use jsonlogic::apply;
use serde_json::{json, Value};

#[test]
fn compare() {
    // Only simple tests here, the hardcore coercion tests are in the logic.rs file.
    assert_eq!(apply(&json!({"<": []}), &Value::Null), Ok(json!(false)));
    assert_eq!(apply(&json!({"<": [1]}), &Value::Null), Ok(json!(false)));
    assert_eq!(apply(&json!({"<": [1, 2]}), &Value::Null), Ok(json!(true)));
    assert_eq!(apply(&json!({"<": [2, 1]}), &Value::Null), Ok(json!(false)));
    assert_eq!(apply(&json!({">": [2, 1]}), &Value::Null), Ok(json!(true)));
    assert_eq!(apply(&json!({">=": [2, 2]}), &Value::Null), Ok(json!(true)));
    assert_eq!(
        apply(&json!({">=": [2, 3]}), &Value::Null),
        Ok(json!(false))
    );
    assert_eq!(apply(&json!({"<=": [2, 2]}), &Value::Null), Ok(json!(true)));
    assert_eq!(
        apply(&json!({"<=": [3, 2]}), &Value::Null),
        Ok(json!(false))
    );
}

#[test]
fn between_exclusive() {
    assert_eq!(
        apply(&json!({"<" : [1, 2, 3]}), &Value::Null),
        Ok(json!(true))
    );
    assert_eq!(
        apply(&json!({"<" : [1, 1, 3]}), &Value::Null),
        Ok(json!(false))
    );
    assert_eq!(
        apply(&json!({"<" : [1, 4, 3]}), &Value::Null),
        Ok(json!(false))
    );
}

#[test]
fn between_inclusive() {
    assert_eq!(
        apply(&json!({"<=" : [1, 2, 3]}), &Value::Null),
        Ok(json!(true))
    );
    assert_eq!(
        apply(&json!({"<=" : [1, 1, 3]}), &Value::Null),
        Ok(json!(true))
    );
    assert_eq!(
        apply(&json!({"<=" : [1, 4, 3]}), &Value::Null),
        Ok(json!(false))
    );
}

#[test]
fn min() {
    assert_eq!(apply(&json!({"min":[1,2,3]}), &Value::Null), Ok(json!(1.0)));
}

#[test]
fn max() {
    assert_eq!(
        apply(&json!({"max":[1,"4",3]}), &Value::Null),
        Ok(json!(4.0))
    )
}

#[test]
#[allow(clippy::approx_constant)]
fn adddition() {
    assert_eq!(apply(&json!({"+":[4, 2]}), &Value::Null), Ok(json!(6.0)));
    assert_eq!(
        apply(&json!({"+":[2,2,2,2,2]}), &Value::Null),
        Ok(json!(10.0))
    );
    assert_eq!(apply(&json!({"+" : "3.14"}), &Value::Null), Ok(json!(3.14)));
}

#[test]
fn substraction() {
    assert_eq!(apply(&json!({"-": [4,2]}), &Value::Null), Ok(json!(2.0)));
    assert_eq!(apply(&json!({"-": [2]}), &Value::Null), Ok(json!(-2.0)));
    assert_eq!(apply(&json!({"-": "-2"}), &Value::Null), Ok(json!(2.0)));
}

#[test]
fn multiplication() {
    assert_eq!(apply(&json!({"*":[4, 2]}), &Value::Null), Ok(json!(8.0)));
    assert_eq!(
        apply(&json!({"*":[2,2,2,2,2]}), &Value::Null),
        Ok(json!(32.0))
    );
    assert_eq!(
        apply(&json!({"*" : "3.14"}), &Value::Null),
        Ok(json!("3.14"))
    );
}

#[test]
fn division() {
    assert_eq!(apply(&json!({"/":[4, 2]}), &Value::Null), Ok(json!(2.0)));
    // null/2 === 0/2 === 0
    assert_eq!(apply(&json!({"/":[null, 2]}), &Value::Null), Ok(json!(0.0)));
    assert_eq!(apply(&json!({"/":[4, 0]}), &Value::Null), Ok(json!(null)));
    // 4/null === 4/0 === null
    assert_eq!(
        apply(&json!({"/":[4, null]}), &Value::Null),
        Ok(json!(null))
    );
}

#[test]
fn modulo() {
    assert_eq!(apply(&json!({"%": [101, 2]}), &Value::Null), Ok(json!(1.0)));
    assert_eq!(
        apply(&json!({"%": [101, null]}), &Value::Null),
        Ok(json!(null))
    );
    assert_eq!(
        apply(&json!({"%": [101, 0]}), &Value::Null),
        Ok(json!(null))
    );
    assert_eq!(
        apply(&json!({"%": [null, 101]}), &Value::Null),
        Ok(json!(0.0))
    );
}
