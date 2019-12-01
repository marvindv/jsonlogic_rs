use serde_json::{json, Value};

use super::logic;

/// Expects two string arguments. Tests, whether the first argument is a substring of the
/// second argument.
pub fn compute(args: &[Value]) -> Value {
    let a = match args.get(0) {
        Some(arg) => logic::coerce_to_str(arg),
        None => return json!(false),
    };

    // The second argument must be a string, no coercion.
    let b = match args.get(1) {
        Some(Value::String(s)) => s,
        _ => return json!(false),
    };

    Value::Bool(b.contains(&a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(compute(&[]), Value::Bool(false));
        assert_eq!(compute(&[Value::Null]), Value::Bool(false));
        assert_eq!(compute(&[json!("foo")]), Value::Bool(false));

        assert_eq!(compute(&[json!("foo"), json!("foobar")]), Value::Bool(true));
        assert_eq!(
            compute(&[json!({}), json!("{}.toString() === '[object Object]'")]),
            Value::Bool(true)
        );
    }
}
