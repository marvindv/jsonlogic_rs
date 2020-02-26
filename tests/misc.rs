use jsonlogic::apply;
use serde_json::{json, Value};

#[test]
fn log() {
    assert_eq!(
        apply(&json!({ "log": "foo" }), &Value::Null),
        Ok(json!("foo"))
    );
    assert_eq!(
        apply(&json!({ "log": ["foo"] }), &Value::Null),
        Ok(json!("foo"))
    );
    assert_eq!(
        apply(&json!({ "log": ["foo", "bar"] }), &Value::Null),
        Ok(json!("foo"))
    );
}
