use serde_json::Value;

/// See http://jsonlogic.com/truthy.html
pub fn truthy(value: &Value) -> bool {
    match value {
        Value::Array(arr) => arr.len() > 0,
        Value::Bool(b) => *b,
        Value::Null => false,
        Value::Number(num) => num.as_f64().unwrap() != 0f64,
        Value::Object(_) => true,
        Value::String(s) => s != "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn truthy_values() {
        // See http://jsonlogic.com/truthy.html
        assert_eq!(truthy(&json!(0)), false);
        assert_eq!(truthy(&json!(-1)), true);
        assert_eq!(truthy(&json!(1)), true);
        assert_eq!(truthy(&json!([])), false);
        assert_eq!(truthy(&json!([1, 2])), true);
        assert_eq!(truthy(&json!("")), false);
        assert_eq!(truthy(&json!("anything")), true);
        assert_eq!(truthy(&json!("0")), true);
        assert_eq!(truthy(&json!(null)), false);

        assert_eq!(truthy(&json!({})), true);
        assert_eq!(truthy(&json!(true)), true);
        assert_eq!(truthy(&json!(false)), false);
    }
}
