use serde_json::{json, Value};

use super::logic;

/// Expects two arguments. Tests either for substring or whether an array contains an element.
///
/// If the second argument is an array, tests that the first argument is a member of the array.
///
/// If the second argument is a string, tests that the first argument is a substring.
pub fn compute(args: &[Value]) -> Value {
    let a = match args.get(0) {
        Some(arg) => arg,
        None => return json!(false),
    };

    let result = match args.get(1) {
        // Second argument is an array: test whether the first argument is a member of the array.
        Some(Value::String(b)) => b.contains(&logic::coerce_to_str(a)),
        // Second argument is a string: test whether the first argument (coerced into a string) is
        // a substring of the second argument.
        Some(Value::Array(b)) => b.iter().any(|el| logic::is_strict_equal(el, a)),
        _ => false,
    };
    Value::Bool(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(compute(&[]), Value::Bool(false));
        assert_eq!(compute(&[Value::Null]), Value::Bool(false));
        assert_eq!(compute(&[json!("foo")]), Value::Bool(false));
    }

    #[test]
    fn substr() {
        assert_eq!(compute(&[json!("foo"), json!("foobar")]), Value::Bool(true));
        assert_eq!(
            compute(&[json!({}), json!("{}.toString() === '[object Object]'")]),
            Value::Bool(true)
        );
    }

    #[test]
    fn array() {
        assert_eq!(compute(&[json!("foo"), json!(["foo"])]), Value::Bool(true));
        assert_eq!(
            compute(&[json!("foo"), json!(["foo", "bar"])]),
            Value::Bool(true)
        );
        assert_eq!(
            compute(&[json!(1.0), json!(["foo", 1, "bar"])]),
            Value::Bool(true)
        );

        assert_eq!(
            compute(&[json!(1), json!(["foo", "1", true, "bar"])]),
            Value::Bool(false)
        );
        assert_eq!(
            compute(&[json!(1.1), json!(["foo", 1, "bar"])]),
            Value::Bool(false)
        );
        assert_eq!(compute(&[json!({}), json!([{}])]), Value::Bool(false));
    }
}
