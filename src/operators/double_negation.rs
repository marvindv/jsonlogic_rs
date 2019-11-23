use serde_json::{json, Value};

use super::logic;

/// Double negation, or "cast to a boolean". Takes a single argument.
pub fn compute(args: &[Value]) -> Value {
    let a = args.get(0).unwrap_or(&json!(null));

    Value::Bool(logic::is_truthy(a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(compute(&[]), Value::Bool(false));
        assert_eq!(compute(&[json!(null)]), Value::Bool(false));
        assert_eq!(compute(&[json!(false)]), Value::Bool(false));
        assert_eq!(compute(&[json!(true)]), Value::Bool(true));
    }
}
