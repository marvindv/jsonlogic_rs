use jsonlogic::apply;
use serde_json::{json, Value};

#[test]
fn simple_values() {
    let num = json!(1);
    assert_eq!(apply(&num, &Value::Null), Ok(num));

    let string = json!("foo");
    assert_eq!(apply(&string, &Value::Null), Ok(string));

    let boolean = json!(true);
    assert_eq!(apply(&boolean, &Value::Null), Ok(boolean));
}
