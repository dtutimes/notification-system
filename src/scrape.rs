use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name};
use serde::Serialize;
use url::Url;

#[derive(Serialize)]
pub struct Link(pub String);

#[derive(Serialize)]
pub struct Data {
    pub title: String,
    pub link: Option<Link>,
    pub children: Vec<LinkNode>,
    pub date: Option<String>,
}
#[derive(Serialize)]
pub struct LinkNode {
    pub title: String,
    pub link: Link,
}

#[derive(Serialize)]
pub struct Tab {
    pub title: String,
    pub data: Vec<Data>,
}

pub fn scrape(html: &str) -> Vec<Tab> {
    let document = Document::from(html);

    let api = Url::parse("https://dtu.ac.in").expect("Base url must be valid");
    let base_url = Url::options().base_url(Some(&api));

    let make_link = |link: &str| base_url.parse(link).map(|s| Link(s.to_string())).ok();

    document
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
        .collect()
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
            Some(date_elem.text().trim().to_owned())
        } else {
            None
        }
    })
}
