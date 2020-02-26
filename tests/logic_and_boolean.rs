use jsonlogic::apply;
use serde_json::{json, Value};

// if
mod if_else {
    use super::*;

    #[test]
    fn if_elseif() {
        let logic = json!({"if" : [
          {"<": [{"var":"temp"}, 0] }, "freezing",
          {"<": [{"var":"temp"}, 100] }, "liquid",
          "gas"
        ]});

        assert_eq!(apply(&logic, &json!({ "temp": 50 })), Ok(json!("liquid")));
    }
}

// ==
mod equality {
    use super::*;

    fn test_equality(a: &Value, b: &Value, expect: bool) {
        assert_eq!(
            apply(&json!({ "==": [a, b] }), &Value::Null),
            Ok(Value::Bool(expect))
        );
        assert_eq!(
            apply(&json!({ "==": [b, a] }), &Value::Null),
            Ok(Value::Bool(expect))
        );
    }

    #[test]
    fn simple_equality() {
        assert_eq!(
            apply(&json!({ "==": [] }), &Value::Null),
            Ok(Value::Bool(true))
        );
        // For whatever reason the javascript implementation returns true for `null` instead of an
        // array as the argument.
        assert_eq!(
            apply(&json!({ "==": Value::Null }), &Value::Null),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply(&json!({ "==": [Value::Null] }), &Value::Null),
            Ok(Value::Bool(true))
        );
        test_equality(&json!(null), &json!(null), true);
        test_equality(&json!(1), &json!(1), true);
        test_equality(&json!("foo"), &json!("foo"), true);

        assert_eq!(
            apply(&json!({ "==": 0 }), &Value::Null),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            apply(&json!({ "==": [0] }), &Value::Null),
            Ok(Value::Bool(false))
        );
        test_equality(&json!(1), &json!(0), false);
    }

    #[test]
    fn equality_with_type_coercion() {
        test_equality(&json!(0), &json!(""), true);
        test_equality(&json!(1), &json!("1"), true);
        test_equality(&json!([1]), &json!("1"), true);
        test_equality(&json!([1, 2]), &json!("1,2"), true);
        test_equality(&json!(0), &json!(null), false);
    }
}

// ===
mod strict_equality {
    use super::*;

    fn test_strict_equality(a: &Value, b: &Value, expect: bool) {
        assert_eq!(
            apply(&json!({ "===": [a, b] }), &Value::Null),
            Ok(Value::Bool(expect))
        );
    }

    #[test]
    fn simple_strict_equality() {
        assert_eq!(
            apply(&json!({ "===": [] }), &Value::Null),
            Ok(Value::Bool(true))
        );
        // For whatever reason the javascript implementation returns true for `null` instead of an
        // array as the argument.
        assert_eq!(
            apply(&json!({ "===": Value::Null }), &Value::Null),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply(&json!({ "===": [Value::Null] }), &Value::Null),
            Ok(Value::Bool(true))
        );
        test_strict_equality(&json!(null), &json!(null), true);
        test_strict_equality(&json!(1), &json!(1), true);
        test_strict_equality(&json!("foo"), &json!("foo"), true);

        assert_eq!(
            apply(&json!({ "===": 0 }), &Value::Null),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            apply(&json!({ "===": [0] }), &Value::Null),
            Ok(Value::Bool(false))
        );
        test_strict_equality(&json!(1), &json!(0), false);
    }

    #[test]
    fn strict_equality_with_type_coercion() {
        test_strict_equality(&json!(0), &json!(""), false);
        test_strict_equality(&json!(1), &json!("1"), false);
        test_strict_equality(&json!([1]), &json!("1"), false);
        test_strict_equality(&json!([1, 2]), &json!("1,2"), false);
        test_strict_equality(&json!(0), &json!(null), false);
    }
}

// !=
mod not_equal {
    use super::*;

    fn test_not_equal(a: &Value, b: &Value, expect: bool) {
        assert_eq!(
            apply(&json!({ "!=": [a, b] }), &Value::Null),
            Ok(Value::Bool(expect))
        );
        assert_eq!(
            apply(&json!({ "!=": [b, a] }), &Value::Null),
            Ok(Value::Bool(expect))
        );
    }

    #[test]
    fn simple_not_equal() {
        assert_eq!(
            apply(&json!({ "!=": [] }), &Value::Null),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            apply(&json!({ "!=": Value::Null }), &Value::Null),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            apply(&json!({ "!=": [Value::Null] }), &Value::Null),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            apply(&json!({ "!=": "foo" }), &Value::Null),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply(&json!({ "!=": ["foo"] }), &Value::Null),
            Ok(Value::Bool(true))
        );
        test_not_equal(&json!(null), &json!(null), false);
        test_not_equal(&json!(1), &json!(1), false);
        test_not_equal(&json!("foo"), &json!("foo"), false);

        assert_eq!(
            apply(&json!({ "!=": 0 }), &Value::Null),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply(&json!({ "!=": [0] }), &Value::Null),
            Ok(Value::Bool(true))
        );
        test_not_equal(&json!(1), &json!(0), true);
    }

    #[test]
    fn not_equal_with_type_coercion() {
        test_not_equal(&json!(0), &json!(""), false);
        test_not_equal(&json!(1), &json!("1"), false);
        test_not_equal(&json!([1]), &json!("1"), false);
        test_not_equal(&json!([1, 2]), &json!("1,2"), false);
        test_not_equal(&json!(0), &json!(null), true);
    }
}

// !==
mod strict_not_equal {
    use super::*;

    fn test_strict_not_equal(a: &Value, b: &Value, expect: bool) {
        assert_eq!(
            apply(&json!({ "!==": [a, b] }), &Value::Null),
            Ok(Value::Bool(expect))
        );
    }

    #[test]
    fn simple_strict_equality() {
        assert_eq!(
            apply(&json!({ "!==": [] }), &Value::Null),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            apply(&json!({ "!==": Value::Null }), &Value::Null),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            apply(&json!({ "!==": [Value::Null] }), &Value::Null),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            apply(&json!({ "!==": "foo" }), &Value::Null),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply(&json!({ "!==": ["foo"] }), &Value::Null),
            Ok(Value::Bool(true))
        );
        test_strict_not_equal(&json!(null), &json!(null), false);
        test_strict_not_equal(&json!(1), &json!(1), false);
        test_strict_not_equal(&json!("foo"), &json!("foo"), false);

        assert_eq!(
            apply(&json!({ "!==": 0 }), &Value::Null),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply(&json!({ "!==": [0] }), &Value::Null),
            Ok(Value::Bool(true))
        );
        test_strict_not_equal(&json!(1), &json!(0), true);
    }

    #[test]
    fn strict_equality_with_type_coercion() {
        test_strict_not_equal(&json!(0), &json!(""), true);
        test_strict_not_equal(&json!(1), &json!("1"), true);
        test_strict_not_equal(&json!([1]), &json!("1"), true);
        test_strict_not_equal(&json!([1, 2]), &json!("1,2"), true);
        test_strict_not_equal(&json!(0), &json!(null), true);
    }
}

#[test]
fn negation() {
    assert_eq!(
        apply(
            &json!({
                "!": [true]
            }),
            &Value::Null
        ),
        Ok(Value::Bool(false))
    );

    assert_eq!(
        apply(
            &json!({
                "!": true
            }),
            &Value::Null
        ),
        Ok(Value::Bool(false))
    );
}

#[test]
fn double_negation() {
    assert_eq!(
        apply(
            &json!({
                "!!": [[]]
            }),
            &Value::Null
        ),
        Ok(Value::Bool(false))
    );

    assert_eq!(
        apply(
            &json!({
                "!!": ["0"]
            }),
            &Value::Null
        ),
        Ok(Value::Bool(true))
    );
}

mod or {
    use super::*;

    #[test]
    fn test() {
        // The javascript implementation returns `undefined` for this case but `null` should be
        // fine here.
        assert_eq!(
            apply(
                &json!({
                    "or": []
                }),
                &Value::Null
            ),
            Ok(Value::Null)
        );

        assert_eq!(
            apply(
                &json!({
                    "or": [false]
                }),
                &Value::Null
            ),
            Ok(Value::Bool(false))
        );

        assert_eq!(
            apply(
                &json!({
                    "or": [""]
                }),
                &Value::Null
            ),
            Ok(json!(""))
        );

        assert_eq!(
            apply(
                &json!({
                    "or": ["foo"]
                }),
                &Value::Null
            ),
            Ok(json!("foo"))
        );

        assert_eq!(
            apply(
                &json!({
                    "or": [false, "", 0]
                }),
                &Value::Null
            ),
            Ok(json!(0))
        );

        assert_eq!(
            apply(
                &json!({
                    "or": [false, "", 0, true, false]
                }),
                &Value::Null
            ),
            Ok(Value::Bool(true))
        );

        assert_eq!(
            apply(
                &json!({
                    "or": [false, "", 0, true, "foo", false]
                }),
                &Value::Null
            ),
            Ok(Value::Bool(true))
        );

        assert_eq!(
            apply(
                &json!({
                    "or": [false, "", 0, "foo", true, false]
                }),
                &Value::Null
            ),
            Ok(json!("foo"))
        );
    }
}

mod and {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(apply(&json!({"and": []}), &Value::Null), Ok(json!(null)));
        assert_eq!(
            apply(&json!({"and": [true, true]}), &Value::Null),
            Ok(json!(true))
        );
        assert_eq!(
            apply(&json!({"and": [true, false]}), &Value::Null),
            Ok(json!(false))
        );
        assert_eq!(
            apply(&json!({"and": [true, ""]}), &Value::Null),
            Ok(json!(""))
        );
        assert_eq!(
            apply(&json!({"and": [true, "a", 3]}), &Value::Null),
            Ok(json!(3))
        );
        assert_eq!(
            apply(&json!({"and": [true, "", 3]}), &Value::Null),
            Ok(json!(""))
        );
    }
}
