use serde_json::Value;

use super::{logic, Data, Expression};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = match args.get(0) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };

    let b = match args.get(1) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };

    let result = match args.get(2) {
        Some(c) => compute_between_inclusive(&a, &b, &c.compute(data)),
        None => compute_less_equal_than(&a, &b),
    };

    Value::Bool(result)
}

fn compute_less_equal_than(a: &Value, b: &Value) -> bool {
    logic::less_equal_than(a, b)
}

/// Checks whether the value `b` is between `a` and `c`, including the bounds.
fn compute_between_inclusive(a: &Value, b: &Value, c: &Value) -> bool {
    logic::less_equal_than(a, b) && logic::less_equal_than(b, c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn less_equal_than() {
        assert_eq!(compute_const!(), Value::Bool(false));
        assert_eq!(compute_const!(json!(1)), Value::Bool(false));
        assert_eq!(compute_const!(json!(1), json!(2)), Value::Bool(true));
        assert_eq!(compute_const!(json!(2), json!(2)), Value::Bool(true));
        assert_eq!(compute_const!(json!(3), json!(2)), Value::Bool(false));
    }

    #[test]
    fn between_inclusive() {
        assert_eq!(
            compute_const!(json!(1), json!(2), json!(3)),
            Value::Bool(true)
        );
        assert_eq!(
            compute_const!(json!(1), json!(2), json!(2)),
            Value::Bool(true)
        );
        assert_eq!(
            compute_const!(json!(2), json!(2), json!(3)),
            Value::Bool(true)
        );
        assert_eq!(
            compute_const!(json!(2), json!(4), json!(3)),
            Value::Bool(false)
        );
    }
}
