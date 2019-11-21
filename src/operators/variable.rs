use serde_json::Value;

use std::convert::TryFrom;

pub fn compute_variable(args: &Vec<Value>, data: &Value) -> Value {
    let arg = args.get(0).unwrap_or(&Value::Null);

    let value = match arg {
        // Return the complete data, like in the js implementation.
        Value::Null => data.clone(),
        Value::String(arg) => match data {
            Value::Object(_) => from_object_by_str(arg, data),
            // Try to interpret the string argument we got as an index in the given array or string
            // data. If that is not possible return Null.
            Value::Array(_) | Value::String(_) => arg
                .parse::<usize>()
                .map(|index| from_data_by_index(index, data))
                .unwrap_or(Value::Null),
            _ => Value::Null,
        },
        Value::Number(arg) => arg
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

fn from_object_by_str(accessor: &String, data: &Value) -> Value {
    let mut data_part = data;

    for step in accessor.split('.') {
        if !data_part.is_object() {
            // We still have a step but the remaining data is not an object so nothing we can dive
            // into.
            return Value::Null;
        }

        if let Some(value) = data_part.as_object().unwrap().get(step) {
            data_part = value;
        } else {
            // Property not found.
            return Value::Null;
        }
    }

    // TODO: Could we avoid cloning?
    data_part.clone()
}

/// Extracts a value from the given data by index. Data can either be an array, a string or an
/// object containing the stringified index as a key. Otherwise returns `Value::Null`.
fn from_data_by_index(index: usize, data: &Value) -> Value {
    match data {
        // Get the element at the given index or Null if there is none.
        Value::Array(arr) => arr.get(index).cloned().unwrap_or(Value::Null),
        // Get the value associated to the key stringified index or Null if there is none.
        Value::Object(obj) => obj.get(&index.to_string()).cloned().unwrap_or(Value::Null),
        // Get the n-th character from the string (where n is index) or Null if the string is not
        // long enough.
        Value::String(s) => s
            .chars()
            .nth(index)
            .map(|ch| Value::String(ch.to_string()))
            .unwrap_or(Value::Null),
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
    fn data_is_string() {
        let data = json!("abcderfg");
        assert_eq!(compute_variable(&vec![json!(1)], &data), json!("b"));
        assert_eq!(compute_variable(&vec![json!("1")], &data), json!("b"));
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

    #[test]
    fn nested_object() {
        let data = json!({ "foo": { "bar": "baz" }});

        assert_eq!(
            compute_variable(&vec![json!("foo.bar")], &data),
            json!("baz")
        );
        assert_eq!(
            compute_variable(&vec![json!("foo.bar.baz")], &data),
            json!(null)
        );
        assert_eq!(
            compute_variable(&vec![json!("foo")], &data),
            json!({ "bar": "baz" })
        );
    }
}
