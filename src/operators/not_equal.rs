use super::equality::compute_equality;
use serde_json::Value;

pub fn compute_not_equal(arguments: &Value) -> Result<Value, String> {
    let equal = compute_equality(arguments)?.as_bool().unwrap();
    Ok(Value::Bool(!equal))
}
