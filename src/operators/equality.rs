use serde_json::{json, Value};

use super::logic;

pub fn compute(args: &[Value]) -> Value {
    let a = args.get(0).unwrap_or(&json!(null));
    let b = args.get(1).unwrap_or(&json!(null));

    Value::Bool(logic::is_loose_equal(&a, &b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn default_null() {
        assert_eq!(compute(&[]), Value::Bool(true));
        assert_eq!(compute(&[json!(null)]), Value::Bool(true));
    }

    #[test]
    fn test() {
        assert_eq!(compute(&[json!(null), json!(null)]), Value::Bool(true));
        assert_eq!(compute(&[json!(1), json!(1)]), Value::Bool(true));
        assert_eq!(compute(&[json!(1), json!(2)]), Value::Bool(false));
    }
}
