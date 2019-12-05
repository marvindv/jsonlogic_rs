use serde_json::Value;

use super::{Data, Expression};

/// Takes an array of data keys. Returns an array of any keys missing from the data object.
///
/// Can also receive 1 argument that is an array of keys, which typically happens if it's actually
/// acting on the output of another command (like 'if' or 'merge').
/// See https://github.com/jwadhams/json-logic-js/blob/a15f528919346f2ec7d82bd4fc91c41481546c01/logic.js#L145
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let mut result: Vec<Value> = vec![];
    let mut args = args.iter().map(|arg| arg.compute(data));

    // The list of keys to look up is either the first argument if that is an array or the list
    // of all arguments otherwise.
    let first = args.next();
    let keys = match first {
        // The first argument is an array, so use its values as keys.
        Some(Value::Array(arr)) => arr,
        // The first argument is something else, so interpret the arguments as keys.
        Some(first) => {
            let mut keys = Vec::with_capacity(args.len());
            keys.push(first);
            keys.append(&mut args.collect());
            keys
        }
        // No argument, return an empty array.
        _ => return Value::Array(result),
    };

    for key in keys.iter() {
        // TODO: Even tough we only look for the existence, the value to the key will be cloned.
        // Something like Data::has_value without cloning would help.
        if data.get_value(key).is_none() {
            result.push(key.clone());
        }
    }

    Value::Array(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const_with_data;
    use serde_json::json;

    #[test]
    fn tests() {
        let data_json = json!({ "a": 5, "b": "foo" });
        let data = &Data::from_json(&data_json);

        assert_eq!(compute_const_with_data!(&[], data), json!([]));
        assert_eq!(
            compute_const_with_data!(&[json!("bar")], data),
            json!(["bar"])
        );
        assert_eq!(compute_const_with_data!(&[json!("a")], data), json!([]));
        assert_eq!(
            compute_const_with_data!(&[json!("a"), json!("b")], data),
            json!([])
        );
        assert_eq!(
            compute_const_with_data!(&[json!("a"), json!("b"), json!("c")], data),
            json!(["c"])
        );
    }

    #[test]
    fn first_arg_is_array() {
        let data_json = json!({ "a": 5, "b": "foo" });
        let data = &Data::from_json(&data_json);

        assert_eq!(
            compute_const_with_data!(&[json!(["bar"])], data),
            json!(["bar"])
        );
        assert_eq!(compute_const_with_data!(&[json!(["a"])], data), json!([]));
        assert_eq!(
            compute_const_with_data!(&[json!(["a", "b"])], data),
            json!([])
        );
        assert_eq!(
            compute_const_with_data!(&[json!(["a", "b", "c"])], data),
            json!(["c"])
        );
    }
}
