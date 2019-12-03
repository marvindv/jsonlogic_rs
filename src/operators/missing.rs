use serde_json::Value;

use super::Data;

/// Takes an array of data keys. Returns an array of any keys missing from the data object.
///
/// Can also receive 1 argument that is an array of keys, which typically happens if it's actually
/// acting on the output of another command (like 'if' or 'merge').
/// See https://github.com/jwadhams/json-logic-js/blob/a15f528919346f2ec7d82bd4fc91c41481546c01/logic.js#L145
pub fn compute(args: &[Value], data: &Data) -> Value {
    let mut result: Vec<Value> = vec![];

    // The list of keys to look up is either the first argument if that is an array or the list
    // of all arguments otherwise.
    let keys = match args.get(0) {
        Some(Value::Array(arr)) => arr,
        _ => args,
    };

    for arg in keys.iter() {
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

    #[test]
    fn first_arg_is_array() {
        let data_json = json!({ "a": 5, "b": "foo" });
        let data = &Data::from_json(&data_json);

        assert_eq!(compute(&[json!(["bar"])], data), json!(["bar"]));
        assert_eq!(compute(&[json!(["a"])], data), json!([]));
        assert_eq!(compute(&[json!(["a", "b"])], data), json!([]));
        assert_eq!(compute(&[json!(["a", "b", "c"])], data), json!(["c"]));
    }
}
