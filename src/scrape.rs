use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name};
use serde::Serialize;
#[cfg(feature = "wasm")]
use tsify::Tsify;
use url::Url;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Serialize, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi))]
/// Wrapper around a string to represent a link
pub struct Link(pub String);

#[derive(Serialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi))]
/// Item in the list of updates
pub struct Data {
    /// Title of the update
    pub title: String,
    #[cfg_attr(feature = "wasm", tsify(optional))]
    /// Link to the update if any
    pub link: Option<Link>,
    /// Sub-links in the update
    pub children: Vec<LinkNode>,
    #[cfg_attr(feature = "wasm", tsify(optional))]
    /// Date of the update if written in the update itself
    pub date: Option<String>,
}

#[derive(Serialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi))]
/// Sub-link item with a title and a link in an update
pub struct LinkNode {
    /// Title of the sub-link
    pub title: String,
    /// Link to the sub-link
    pub link: Link,
}

#[derive(Serialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi))]
pub struct Tab {
    /// Title of the tab
    pub title: String,
    /// List of updates in the tab
    pub data: Vec<Data>,
}
#[derive(Serialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi))]
/// Information scraped from the website
pub struct Information(pub Vec<Tab>);

#[cfg_attr(feature = "wasm", wasm_bindgen)]
/// Scrape the given html string of the main DTU website to get the scraped information
pub fn scrape(html: &str) -> Information {
    let document = Document::from(html);

    let api = Url::parse("https://dtu.ac.in").expect("Base url must be valid");
    let base_url = Url::options().base_url(Some(&api));

    let make_link = |link: &str| base_url.parse(link).map(|s| Link(s.to_string())).ok();

    let data = document
        .find(Class("tab_content"))
        .filter_map(|e| {
            let id = e.attr("id")?;

            let selector = format!("#{}", id);
            let tab_title = document.find(Attr("href", selector.as_str())).next()?;

            let tab_data = e.find(Class("latest_tab")).next()?.find(Name("li"));

            let data = tab_data
                .filter_map(|element| {
                    element.find(Name("h6")).next().and_then(|pointer| {
                        let data_title_elem = pointer.first_child()?;

                        let date = get_date(pointer);

                        assert!(
                            data_title_elem.name() == Some("a"),
                            "h6 is not followed by anchor tag as expected"
                        );

                        Some(Data {
                            title: clean_text(data_title_elem),
                            link: data_title_elem.attr("href").and_then(|s| make_link(s)),
                            children: pointer
                                .children()
                                .skip(1)
                                .filter_map(|s| {
                                    s.attr("href").and_then(|link| {
                                        make_link(link).map(|link| LinkNode {
                                            title: clean_text(s),
                                            link,
                                        })
                                    })
                                })
                                .collect(),
                            date,
                        })
                    })
                })
                .collect();

            let tab = Tab {
                title: tab_title.text(),
                data,
            };

            Some(tab)
        })
        .collect();

    Information(data)
}

#[inline]
fn clean_text(s: Node) -> String {
    s.text()
        .trim()
        .trim_matches(['\t', '\n', '\u{a0}', '|', ' '])
        .to_string()
}

#[inline]
fn get_date(s: Node) -> Option<String> {
    s.next().and_then(|date_elem| {
        if date_elem.name() == Some("small") {
            Some(
                date_elem
                    .text()
                    .trim()
                    .trim_start_matches("Date")
                    .trim_start_matches("date")
                    .trim_start_matches([':', ' '])
                    .to_owned(),
            )
        } else {
            None
        }
    })
}
