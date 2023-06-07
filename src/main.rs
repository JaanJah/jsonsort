use std::io::{self, BufRead};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;

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

fn sort_json_properties(json: &Value) -> Value {
    match json {
        Value::Object(obj) => {
            // Convert the JSON object to a BTreeMap for sorted iteration
            let sorted_obj: Map<String, Value> =
                obj.iter()
                    .collect::<BTreeMap<_, _>>()
                    .into_iter()
                    .map(|(k, v)| (k.clone(), sort_json_properties(v)))
                    .collect();

            Value::Object(sorted_obj)
        }
        Value::Array(arr) => {
            // Recursively sort elements in JSON arrays
            let sorted_arr: Vec<Value> = arr.iter().map(sort_json_properties).collect();
            Value::Array(sorted_arr)
        }
        other => other.clone(),
    }
}
