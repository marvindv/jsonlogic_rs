use serde_json::Value;

use std::convert::TryFrom;

pub fn compute_variable(args: &Vec<Value>, data: &Value) -> Value {
    let arg = args.get(0).unwrap_or(&Value::Null);

    let value = match arg {
        // Return the complete data, like in the js implementation.
        Value::Null => data.clone(),
        Value::String(s) => match data {
            // TODO: Could we avoid clone here?
            Value::Object(obj) => obj.get(s).cloned().unwrap_or(Value::Null),
            // Try to interpret the string as an index in the given array. If that is not
            // possible return Null.
            Value::Array(_) => s
                .parse::<usize>()
                .map(|index| from_data_by_index(index, data))
                .unwrap_or(Value::Null),
            _ => Value::Null,
        },
        Value::Number(num) => num
            .as_u64()
            .and_then(|index| usize::try_from(index).ok())
            .map(|index| from_data_by_index(index, data))
            .unwrap_or(Value::Null),
        _ => unimplemented!(),
    };

    if value.is_null() {
        if let Some(default) = args.get(1) {
            return default.clone();
        }
    }

    value
}

/// Extracts a value from the given data by index. Data can either be an array or an object
/// containing the stringified index as a key. Otherwise returns `Value::Null`.
fn from_data_by_index(index: usize, data: &Value) -> Value {
    match data {
        Value::Array(arr) => arr.get(index).cloned().unwrap_or(Value::Null),
        Value::Object(obj) => obj.get(&index.to_string()).cloned().unwrap_or(Value::Null),
        _ => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn null_arg() {
        let data = json!({ "a": 5, "b": 6 });
        assert_eq!(compute_variable(&vec![], &data), data);
        assert_eq!(compute_variable(&vec![Value::Null], &data), data);
        assert_eq!(
            compute_variable(&vec![Value::Null, json!(123)], &data),
            data
        );
    }

    #[test]
    fn data_is_object() {
        let data = json!({ "a": 5, "b": 6, "1": 1337 });
        assert_eq!(
            compute_variable(&vec![Value::String(String::from("a"))], &data),
            json!(5)
        );
        assert_eq!(
            compute_variable(&vec![Value::String(String::from("b"))], &data),
            json!(6)
        );
        assert_eq!(compute_variable(&vec![json!(1)], &data), json!(1337));
    }

    #[test]
    fn data_is_array() {
        let data = json!(["foo", "bar"]);
        assert_eq!(compute_variable(&vec![], &data), data);
        assert_eq!(compute_variable(&vec![json!(0)], &data), json!("foo"));
        assert_eq!(compute_variable(&vec![json!(1)], &data), json!("bar"));
        assert_eq!(compute_variable(&vec![json!(2)], &data), json!(null));

        assert_eq!(compute_variable(&vec![json!("1")], &data), json!("bar"));
    }

    #[test]
    fn default_value_array_data() {
        let data = json!(["foo", "bar"]);

        assert_eq!(
            compute_variable(&vec![json!(1), json!("def")], &data),
            json!("bar")
        );
        assert_eq!(
            compute_variable(&vec![json!(2), json!("def")], &data),
            json!("def")
        );
    }

    #[test]
    fn default_value_obj_data() {
        let data = json!({"foo": "bar"});

        assert_eq!(
            compute_variable(&vec![json!("foo"), json!("def")], &data),
            json!("bar")
        );
        assert_eq!(
            compute_variable(&vec![json!("unknown"), json!("def")], &data),
            json!("def")
        );
    }
}
