use serde_json::{json, Value};

pub fn compute_strict_equality(args: &Vec<Value>) -> bool {
    let a = args.get(0).unwrap_or_else(|| &json!(null));
    let b = args.get(1).unwrap_or_else(|| &json!(null));

    a == b
}
