use serde_json::{json, Value};

use super::logic;

/// Logical negation ("not"). Takes just one argument.
pub fn compute(args: &[Value]) -> Value {
    let a = args.get(0).unwrap_or(&json!(null));

    Value::Bool(!logic::is_truthy(a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(compute(&[]), Value::Bool(true));
        assert_eq!(compute(&[json!(null)]), Value::Bool(true));
        assert_eq!(compute(&[json!(false)]), Value::Bool(true));
        assert_eq!(compute(&[json!(true)]), Value::Bool(false));
    }
}
