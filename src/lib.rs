extern crate serde_json;

use serde_json::Value;

mod computation;
mod expression;
mod operator;
mod operators;

pub fn apply(json: &Value) -> Result<Value, String> {
    let ast = expression::Expression::from_json(json)?;
    Ok(computation::compute_expression(&ast))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn simple_values() {
        let num = json!(1);
        assert_eq!(apply(&num), Ok(num));

        let string = json!("foo");
        assert_eq!(apply(&string), Ok(string));

        let boolean = json!(true);
        assert_eq!(apply(&boolean), Ok(boolean));
    }

    // ==
    mod equality {
        use super::*;

        fn test_equality(a: &Value, b: &Value, expect: bool) {
            assert_eq!(apply(&json!({ "==": [a, b] })), Ok(Value::Bool(expect)));
            assert_eq!(apply(&json!({ "==": [b, a] })), Ok(Value::Bool(expect)));
        }

        #[test]
        fn simple_equality() {
            assert_eq!(apply(&json!({ "==": [] })), Ok(Value::Bool(true)));
            // For whatever reason the javascript implementation returns true for `null` instead of an
            // array as the argument.
            assert_eq!(apply(&json!({ "==": Value::Null })), Ok(Value::Bool(true)));
            assert_eq!(
                apply(&json!({ "==": [Value::Null] })),
                Ok(Value::Bool(true))
            );
            test_equality(&json!(null), &json!(null), true);
            test_equality(&json!(1), &json!(1), true);
            test_equality(&json!("foo"), &json!("foo"), true);

            assert_eq!(apply(&json!({ "==": 0 })), Ok(Value::Bool(false)));
            assert_eq!(apply(&json!({ "==": [0] })), Ok(Value::Bool(false)));
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

    // !=
    mod not_equal {
        use super::*;

        fn test_not_equal(a: &Value, b: &Value, expect: bool) {
            assert_eq!(apply(&json!({ "!=": [a, b] })), Ok(Value::Bool(expect)));
            assert_eq!(apply(&json!({ "!=": [b, a] })), Ok(Value::Bool(expect)));
        }

        #[test]
        fn simple_not_equal() {
            assert_eq!(apply(&json!({ "!=": [] })), Ok(Value::Bool(false)));
            assert_eq!(apply(&json!({ "!=": Value::Null })), Ok(Value::Bool(false)));
            assert_eq!(
                apply(&json!({ "!=": [Value::Null] })),
                Ok(Value::Bool(false))
            );
            assert_eq!(apply(&json!({ "!=": "foo" })), Ok(Value::Bool(true)));
            assert_eq!(apply(&json!({ "!=": ["foo"] })), Ok(Value::Bool(true)));
            test_not_equal(&json!(null), &json!(null), false);
            test_not_equal(&json!(1), &json!(1), false);
            test_not_equal(&json!("foo"), &json!("foo"), false);

            assert_eq!(apply(&json!({ "!=": 0 })), Ok(Value::Bool(true)));
            assert_eq!(apply(&json!({ "!=": [0] })), Ok(Value::Bool(true)));
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

    // ===
    mod strict_equality {
        use super::*;

        fn test_strict_equality(a: &Value, b: &Value, expect: bool) {
            assert_eq!(apply(&json!({ "===": [a, b] })), Ok(Value::Bool(expect)));
        }

        #[test]
        fn simple_strict_equality() {
            assert_eq!(apply(&json!({ "===": [] })), Ok(Value::Bool(true)));
            // For whatever reason the javascript implementation returns true for `null` instead of an
            // array as the argument.
            assert_eq!(apply(&json!({ "===": Value::Null })), Ok(Value::Bool(true)));
            assert_eq!(
                apply(&json!({ "===": [Value::Null] })),
                Ok(Value::Bool(true))
            );
            test_strict_equality(&json!(null), &json!(null), true);
            test_strict_equality(&json!(1), &json!(1), true);
            test_strict_equality(&json!("foo"), &json!("foo"), true);

            assert_eq!(apply(&json!({ "===": 0 })), Ok(Value::Bool(false)));
            assert_eq!(apply(&json!({ "===": [0] })), Ok(Value::Bool(false)));
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

    // !==
    mod strict_not_equal {
        use super::*;

        fn test_strict_not_equal(a: &Value, b: &Value, expect: bool) {
            assert_eq!(apply(&json!({ "!==": [a, b] })), Ok(Value::Bool(expect)));
        }

        #[test]
        fn simple_strict_equality() {
            assert_eq!(apply(&json!({ "!==": [] })), Ok(Value::Bool(false)));
            assert_eq!(
                apply(&json!({ "!==": Value::Null })),
                Ok(Value::Bool(false))
            );
            assert_eq!(
                apply(&json!({ "!==": [Value::Null] })),
                Ok(Value::Bool(false))
            );
            assert_eq!(apply(&json!({ "!==": "foo" })), Ok(Value::Bool(true)));
            assert_eq!(apply(&json!({ "!==": ["foo"] })), Ok(Value::Bool(true)));
            test_strict_not_equal(&json!(null), &json!(null), false);
            test_strict_not_equal(&json!(1), &json!(1), false);
            test_strict_not_equal(&json!("foo"), &json!("foo"), false);

            assert_eq!(apply(&json!({ "!==": 0 })), Ok(Value::Bool(true)));
            assert_eq!(apply(&json!({ "!==": [0] })), Ok(Value::Bool(true)));
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
}
