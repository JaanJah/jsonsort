use serde_json::{Map, Value};
use std::collections::BTreeMap;

pub fn sort_json_properties(json: &Value) -> Value {
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
