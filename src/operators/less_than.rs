use serde_json::Value;

use super::logic;

pub fn compute(args: &[Value]) -> Value {
    let a = match args.get(0) {
        Some(arg) => arg,
        None => return Value::Bool(false),
    };

    let b = match args.get(1) {
        Some(arg) => arg,
        None => return Value::Bool(false),
    };

    let result = match args.get(2) {
        Some(c) => compute_between_exclusive(a, b, c),
        None => compute_less_than(a, b),
    };

    Value::Bool(result)
}

fn compute_less_than(a: &Value, b: &Value) -> bool {
    logic::less_than(a, b)
}

/// Checks whether the value `b` is between `a` and `c`, excluding the bounds.
fn compute_between_exclusive(a: &Value, b: &Value, c: &Value) -> bool {
    logic::less_than(a, b) && logic::less_than(b, c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn less_than() {
        assert_eq!(compute(&[]), Value::Bool(false));
        assert_eq!(compute(&[json!(1)]), Value::Bool(false));
        assert_eq!(compute(&[json!(1), json!(2)]), Value::Bool(true));
        assert_eq!(compute(&[json!(2), json!(2)]), Value::Bool(false));
    }

    #[test]
    fn between_exclusive() {
        assert_eq!(compute(&[json!(1), json!(2), json!(3)]), Value::Bool(true));
        assert_eq!(compute(&[json!(1), json!(2), json!(2)]), Value::Bool(false));
        assert_eq!(compute(&[json!(2), json!(2), json!(3)]), Value::Bool(false));
    }
}
