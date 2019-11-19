use super::strict_equality::compute_strict_equality;
use serde_json::Value;

pub fn compute_strict_not_equal(arguments: &Value) -> Result<Value, String> {
    let strict_equal = compute_strict_equality(arguments)?.as_bool().unwrap();
    Ok(Value::Bool(!strict_equal))
}
