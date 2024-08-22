use std::{fs, io::Read};

mod diff;
mod scrape;
use diff::*;

macro_rules! print_diff {
    ($i: ident, $j: ident) => {
        let diff = difference(&$i, &$j, Configuration::default());
        let diff_json = serde_json::to_string(&diff).expect("JSON keys should be valid string");
        println!("{}", diff_json);
        fs::write("old_state.json", $i).expect("Should be able to save");
    };
}

fn main() {
    let mut arg = Vec::new();
    std::io::stdin()
        .read_to_end(&mut arg)
        .expect("Failed to read from stdin");
    let arg: String = String::from_utf8_lossy(&arg).into_owned();

    if let Ok(f) = fs::read("old_state.json") {
        let old_state = String::from_utf8_lossy(&f);
        print_diff!(arg, old_state);
    } else {
        print_diff!(arg, arg);
    };
}
