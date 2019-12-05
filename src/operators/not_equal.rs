use serde_json::{json, Value};

use super::{logic, Data, Expression};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = args
        .get(0)
        .map(|arg| arg.compute(data))
        .unwrap_or(json!(null));
    let b = args
        .get(1)
        .map(|arg| arg.compute(data))
        .unwrap_or(json!(null));

    Value::Bool(!logic::is_abstract_equal(&a, &b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn default_null() {
        assert_eq!(compute_const!(), Value::Bool(false));
        assert_eq!(compute_const!(json!(null)), Value::Bool(false));
    }

    #[test]
    fn test() {
        assert_eq!(compute_const!(json!(null), json!(null)), Value::Bool(false));
        assert_eq!(compute_const!(json!(1), json!(1)), Value::Bool(false));
        assert_eq!(compute_const!(json!(1), json!(2)), Value::Bool(true));
    }
}
