use jsonlogic::apply;
use serde_json::{json, Value};

// in
#[test]
fn is_in() {
    assert_eq!(
        apply(&json!({"in":["Spring", "Springfield"]}), &Value::Null),
        Ok(json!(true))
    );

    assert_eq!(
        apply(&json!({"in":["spring", "Springfield"]}), &Value::Null),
        Ok(json!(false))
    );
}

// cat
#[test]
fn cat() {
    assert_eq!(
        apply(&json!({"cat": ["I love", " pie"]}), &Value::Null),
        Ok(json!("I love pie"))
    );

    assert_eq!(
        apply(
            &json!({"cat": ["I love ", {"var":"filling"}, " pie"]}),
            &json!({"filling":"apple", "temp":110})
        ),
        Ok(json!("I love apple pie"))
    );
}

// substr
#[test]
fn substr() {
    assert_eq!(
        apply(&json!({"substr": ["jsonlogic", 4]}), &Value::Null),
        Ok(json!("logic"))
    );
    assert_eq!(
        apply(&json!({"substr": ["jsonlogic", -5]}), &Value::Null),
        Ok(json!("logic"))
    );
    assert_eq!(
        apply(&json!({"substr": ["jsonlogic", 1, 3]}), &Value::Null),
        Ok(json!("son"))
    );
    assert_eq!(
        apply(&json!({"substr": ["jsonlogic", 4, -2]}), &Value::Null),
        Ok(json!("log"))
    );
}
