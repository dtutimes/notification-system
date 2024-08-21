use std::io::Read;

mod scrape;
use scrape::*;

fn main() {
    let mut arg = Vec::new();
    std::io::stdin()
        .read_to_end(&mut arg)
        .expect("Failed to read from stdin");
    let arg: String = String::from_utf8_lossy(&arg).into_owned();

    let tabs_data = scrape(&arg);

    println!(
        "{}",
        serde_json::to_string(&tabs_data).expect("Non-string keys should not be used")
    );
}
