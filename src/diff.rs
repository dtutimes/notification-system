use serde::Serialize;
use std::collections::HashMap;
#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::*;
#[derive(Serialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi))]
pub enum Update {
    Added,
    Removed,
    Modified,
    Unchanged,
    Inherit,
}

impl From<Data> for DataUpdate {
    fn from(value: Data) -> Self {
        Self {
            title: value.title,
            link: value.link,
            children: value
                .children
                .into_iter()
                .map(|c| LinkNodeUpdate::from(c))
                .collect(),
            date: value.date,
            update: Update::Inherit,
        }
    }
}


#[derive(Serialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi))]
pub struct TabUpdate {
    pub title: String,
    pub data: Vec<DataUpdate>,
    pub update: Update,
}


#[derive(Serialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi))]
pub struct LinkNodeUpdate {
    pub title: String,
    pub link: Link,
    pub update: Update,
}

impl From<LinkNode> for LinkNodeUpdate {
    fn from(value: LinkNode) -> Self {
        Self {
            title: value.title,
            link: value.link,
            update: Update::Inherit,
        }
    }
}


#[derive(Serialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi))]
pub struct DataUpdate {
    pub title: String,
    pub link: Option<Link>,
    pub children: Vec<LinkNodeUpdate>,
    pub date: Option<String>,
    pub update: Update,
}


#[derive(Serialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi))]
pub struct InformationUpdate(pub Vec<TabUpdate>);

// #[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn difference(newer: Information, older: Information) -> InformationUpdate {
    InformationUpdate(diff_tabs(newer.0, older.0))
}

fn diff_tabs(newer: Vec<Tab>, older: Vec<Tab>) -> Vec<TabUpdate> {
    let mut newer_mp: HashMap<String, Vec<Data>> = HashMap::from_iter(
        newer
            .into_iter()
            .map(|tab_data| (tab_data.title, tab_data.data)),
    );

    let mut diff: Vec<TabUpdate> = older
        .into_iter()
        .map(|tab_data| {
            if let Some(newer_tab_data) = newer_mp.remove(&tab_data.title) {
                TabUpdate {
                    title: tab_data.title,
                    data: diff_data(newer_tab_data, tab_data.data),
                    // this tab already existed
                    update: Update::Unchanged,
                }
            } else {
                // it isn't here now
                TabUpdate {
                    title: tab_data.title,
                    data: tab_data
                        .data
                        .into_iter()
                        .map(|s| DataUpdate::from(s))
                        .collect(),
                    // existed in older
                    update: Update::Removed,
                }
            }
        })
        .collect();

    diff.extend(newer_mp.into_iter().map(|(title, data)| {
        TabUpdate {
            title,
            data: data.into_iter().map(|s| DataUpdate::from(s)).collect(),
            // exist in new only
            update: Update::Added,
        }
    }));

    diff
}

fn diff_data(newer: Vec<Data>, older: Vec<Data>) -> Vec<DataUpdate> {
    struct DataStorage {
        link: Option<Link>,
        children: Vec<LinkNode>,
        date: Option<String>,
    }

    let mut newer_mp: HashMap<String, DataStorage> =
        HashMap::from_iter(newer.into_iter().map(|tab_data| {
            (
                tab_data.title,
                DataStorage {
                    link: tab_data.link,
                    children: tab_data.children,
                    date: tab_data.date,
                },
            )
        }));

    let mut diff: Vec<DataUpdate> = older
        .into_iter()
        .map(|old_data| {
            if let Some(new_data) = newer_mp.remove(&old_data.title) {
                let update = if new_data.link == old_data.link && new_data.date == old_data.date {
                    Update::Unchanged
                } else {
                    Update::Modified
                };
                DataUpdate {
                    title: old_data.title,
                    // this title already existed
                    update,
                    link: new_data.link,
                    children: diff_link_node(new_data.children, old_data.children),
                    date: new_data.date,
                }
            } else {
                // it isn't here now
                DataUpdate {
                    title: old_data.title,
                    link: old_data.link,
                    children: old_data
                        .children
                        .into_iter()
                        .map(|s| LinkNodeUpdate::from(s))
                        .collect(),
                    date: old_data.date,
                    update: Update::Removed,
                }
            }
        })
        .collect();

    diff.extend(newer_mp.into_iter().map(|(title, data)| {
        DataUpdate {
            title,
            link: data.link,
            children: data
                .children
                .into_iter()
                .map(|s| LinkNodeUpdate::from(s))
                .collect(),
            date: data.date,
            update: Update::Added,
        }
    }));

    diff
}

fn diff_link_node(newer: Vec<LinkNode>, older: Vec<LinkNode>) -> Vec<LinkNodeUpdate> {
    let mut newer_mp: HashMap<String, Link> = HashMap::from_iter(
        newer
            .into_iter()
            .map(|tab_data| (tab_data.title, tab_data.link)),
    );
    let mut diff: Vec<LinkNodeUpdate> = older
        .into_iter()
        .map(|old_data| {
            if let Some(newer_link) = newer_mp.remove(&old_data.title) {
                let update = if newer_link == old_data.link {
                    Update::Unchanged
                } else {
                    Update::Modified
                };

                LinkNodeUpdate {
                    title: old_data.title,
                    link: newer_link,
                    update,
                }
            } else {
                LinkNodeUpdate {
                    title: old_data.title,
                    link: old_data.link,
                    update: Update::Removed,
                }
            }
        })
        .collect();

    diff.extend(newer_mp.into_iter().map(|(title, link)| LinkNodeUpdate {
        title,
        link,
        update: Update::Added,
    }));

    diff
}
