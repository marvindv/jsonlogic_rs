use serde_json::{json, Value};

pub fn compute(args: &[Value]) -> Value {
    let a = args.get(0).unwrap_or(&json!(null));
    let b = args.get(1).unwrap_or(&json!(null));

    Value::Bool(a == b)
}
