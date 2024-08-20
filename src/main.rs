use std::io::{self, Read};

use select::document::Document;
use select::predicate::{Attr, Class, Name};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
struct Link(String);

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    title: String,
    link: Option<Link>,
    children: Vec<LinkNode>,
    date: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct LinkNode {
    title: String,
    link: Link,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tab {
    title: String,
    data: Vec<Data>,
}

fn main() {
    let mut arg = Vec::new();
    io::stdin()
        .read_to_end(&mut arg)
        .expect("Failed to read from stdin");
    let arg: String = String::from_utf8_lossy(&arg).into_owned();

    let document = Document::from(arg.as_str());
    // println!("{:?}", document.find(Name("marquee")).count());

    // for element in document.find(Name("marquee")) {
    //     let m: Data = element.try_into().unwrap();
    //     println!("{:?}", m);
    //     break;
    // }

    let api = Url::parse("https://dtu.ac.in").expect("Base url must be valid");
    let base_url = Url::options().base_url(Some(&api));

    let tabs_data: Vec<_> = document
        .find(Class("tab_content"))
        .filter_map(|e| {
            let id = e.attr("id")?;

            let selector = format!("#{}", id);
            let tab_title = document.find(Attr("href", selector.as_str())).next()?;

            let tab_title = tab_title.text();

            let tab_data = e.find(Class("latest_tab")).next()?;

            let tab_data = tab_data.find(Name("li"));

            let data = tab_data
                .filter_map(|element| {
                    if let Some(pointer) = element.find(Name("h6")).next() {
                        let data_title_elem = pointer.first_child()?;

                        let date = pointer.next().and_then(|date_elem| {
                            if date_elem.name() == Some("small") {
                                Some(date_elem.text().trim().to_owned())
                            } else {
                                None
                            }
                        });

                        assert!(
                            data_title_elem.name() == Some("a"),
                            "h6 is not followed by anchor tag as expected"
                        );
                        let data = Data {
                            title: data_title_elem
                                .text()
                                .trim()
                                .trim_matches(['\t', '\n', '\u{a0}', '|', ' '])
                                .to_owned(),
                            link: data_title_elem
                                .attr("href")
                                .and_then(|s| base_url.parse(s).map(|s| Link(s.to_string())).ok()),
                            children: pointer
                                .children()
                                .skip(1)
                                .filter_map(|s| {
                                    s.attr("href").and_then(|link| {
                                        base_url
                                            .parse(link)
                                            .map(|s| Link(s.to_string()))
                                            .map(|link| LinkNode {
                                                title: s
                                                    .text()
                                                    .trim()
                                                    .trim_matches(['\t', '\n', '\u{a0}', '|', ' '])
                                                    .to_string(),
                                                link,
                                            })
                                            .ok()
                                    })
                                })
                                .collect(),
                            date,
                        };

                        Some(data)
                    } else {
                        None
                    }
                })
                .collect();

            let tab = Tab {
                title: tab_title,
                data,
            };

            Some(tab)
        })
        .collect();

    println!(
        "{}",
        serde_json::to_string(&tabs_data).expect("Non-string keys should not be used")
    );
}
