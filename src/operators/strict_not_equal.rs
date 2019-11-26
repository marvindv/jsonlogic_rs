use serde_json::{json, Value};

use super::logic;

pub fn compute(args: &[Value]) -> Value {
    let a = args.get(0).unwrap_or(&json!(null));
    let b = args.get(1).unwrap_or(&json!(null));

    Value::Bool(!logic::is_strict_equal(a, b))
}
