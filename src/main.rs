use std::io::{self, BufRead};
use jsonsort::sort_json_properties;
use serde_json::{Value};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        if let Ok(line) = line {
            let json: Value = serde_json::from_str(&*line).unwrap();
            let sorted_json = sort_json_properties(&json);
            let sorted_json_str = serde_json::to_string(&sorted_json).unwrap();
            println!("{}", sorted_json_str);
        }
    }
}
