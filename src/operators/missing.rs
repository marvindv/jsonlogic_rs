use serde_json::Value;

use super::Data;

/// Takes an array of data keys. Returns an array of any keys missing from the data object.
pub fn compute(args: &[Value], data: &Data) -> Value {
    let mut result: Vec<Value> = vec![];

    for arg in args.iter() {
        // TODO: Even tough we only look for the existence, the value to the key will be cloned.
        // Something like Data::has_value without cloning would help.
        if data.get_value(arg).is_none() {
            result.push(arg.clone());
        }
    }

    Value::Array(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tests() {
        let data_json = json!({ "a": 5, "b": "foo" });
        let data = &Data::from_json(&data_json);

        assert_eq!(compute(&[], data), json!([]));
        assert_eq!(compute(&[json!("bar")], data), json!(["bar"]));
        assert_eq!(compute(&[json!("a")], data), json!([]));
        assert_eq!(compute(&[json!("a"), json!("b")], data), json!([]));
        assert_eq!(
            compute(&[json!("a"), json!("b"), json!("c")], data),
            json!(["c"])
        );
    }
}
