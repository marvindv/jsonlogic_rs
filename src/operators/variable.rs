use serde_json::{Number, Value};

use std::convert::TryFrom;

pub fn compute_variable(args: &Vec<Value>, data: &Value) -> Value {
    let arg = args.get(0).unwrap_or(&Value::Null);

    let value = match arg {
        // Return the complete data, like in the js implementation.
        Value::Null => data.clone(),
        Value::String(arg) => from_data_by_str(arg, data),
        Value::Number(arg) => from_data_by_num(arg, data),
        _ => Value::Null,
    };

    if value.is_null() {
        if let Some(default) = args.get(1) {
            return default.clone();
        }
    }

    value
}

/// Trys to get a value from the given data by the path specified by `accessor`. This can be a
/// simple key or a stringified index for strings and arrays but complex dot-notation access paths
/// are also supported.
fn from_data_by_str(accessor: &String, data: &Value) -> Value {
    let mut data_part = data;
    // While we can traverse through arrays and objects, we can't for a characters. Character access
    // in a string must therefore be the last step in the given accessor. To handle that properly,
    // we save an accessed char in this option.
    let mut prev_step_char: Option<char> = None;

    for step in accessor.split('.') {
        // In the previous step an character from a string was accessed, which must be the last step
        // since a character is considered a primitive here.
        if let Some(_) = prev_step_char {
            return Value::Null;
        }

        let value = match data_part {
            // If the current data_part is an array, try to interpret the current step as an index.
            Value::Array(arr) => step.parse::<usize>().ok().and_then(|index| arr.get(index)),
            // If the current data_part is an object, interpret current step as a key.
            Value::Object(obj) => obj.get(step),
            // If the current data_part is a string, interpret current step as index of a character.
            // This must be the last step.
            Value::String(s) => {
                if let Some(ch) = step
                    .parse::<usize>()
                    .ok()
                    .and_then(|index| s.chars().nth(index))
                {
                    prev_step_char = Some(ch);
                    Some(data_part)
                } else {
                    // String data_part is not long enough.
                    return Value::Null;
                }
            }
            // All other possible types are primitives and since we have still at least one step to
            // do, the accessor string does not match any value in the given data.
            _ => None,
        };

        if let Some(value) = value {
            data_part = value;
        } else {
            return Value::Null;
        }
    }

    // If in the last step a character from a string was accessed, return it.
    if let Some(ch) = prev_step_char {
        return Value::String(ch.to_string());
    }

    // TODO: Could we avoid cloning?
    data_part.clone()
}

/// Extracts a value from the given data by index. Data can either be an array, a string or an
/// object containing the stringified index as a key. Otherwise returns `Value::Null`.
fn from_data_by_num(num: &Number, data: &Value) -> Value {
    num.as_u64()
        .and_then(|index| usize::try_from(index).ok())
        .and_then(|index| match data {
            // Get the element at the given index or Null if there is none.
            Value::Array(arr) => arr.get(index).cloned(),
            // Get the value associated to the key stringified index or Null if there is none.
            Value::Object(obj) => obj.get(&index.to_string()).cloned(),
            // Get the n-th character from the string (where n is index) or Null if the string is
            // not long enough.
            Value::String(s) => s.chars().nth(index).map(|ch| Value::String(ch.to_string())),
            _ => None,
        })
        .unwrap_or(Value::Null)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn invalid_arguments() {
        let data = json!({ "a": 5, "b": 6 });
        assert_eq!(compute_variable(&vec![json!([])], &data), json!(null));
        assert_eq!(compute_variable(&vec![json!({})], &data), json!(null));
        assert_eq!(compute_variable(&vec![json!(true)], &data), json!(null));
        assert_eq!(compute_variable(&vec![json!(false)], &data), json!(null));
    }

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

        let data = json!([{"foo": "bar"}]);
        assert_eq!(
            compute_variable(&vec![json!(0)], &data),
            json!({"foo": "bar"})
        );
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

        let data = json!([{"foo": "bar"}]);
        assert_eq!(compute_variable(&vec![json!("0.foo")], &data), json!("bar"));
        assert_eq!(compute_variable(&vec![json!("1")], &data), json!(null));
        assert_eq!(compute_variable(&vec![json!("1.foo")], &data), json!(null));
        assert_eq!(compute_variable(&vec![json!("0.foo.1")], &data), json!("a"));
        assert_eq!(
            compute_variable(&vec![json!("0.foo.1.0")], &data),
            json!(null)
        );
    }
}
