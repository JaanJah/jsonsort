#[cfg(test)]
mod tests {
    use serde_json::{Value,from_str,to_string};
    use jsonsort::sort_json_properties;

    #[test]
    fn test_sort() {
        let input = r#"{"second":1,"first":0,"third":2}"#;
        let expected_result = r#"{"first":0,"second":1,"third":2}"#;

        let json: Value = from_str(input).unwrap();
        let sorted_json = sort_json_properties(&json);
        let sorted_json_str = to_string(&sorted_json).unwrap();

        assert_eq!(sorted_json_str, expected_result);
    }
}
