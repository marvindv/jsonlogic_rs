extern crate serde_json;

mod data;
mod expression;
mod operators;

use serde_json::Value;
use std::collections::HashSet;

use data::Data;

pub fn apply(json_logic: &Value) -> Result<Value, String> {
    apply_with_data(json_logic, &Value::Null)
}

pub fn apply_with_data(json_logic: &Value, data: &Value) -> Result<Value, String> {
    let ast = expression::Expression::from_json(json_logic)?;
    let data = Data::from_json(data);
    Ok(ast.compute_with_data(&data))
}

pub fn get_variable_names(json_logic: &Value) -> Result<HashSet<String>, String> {
    let ast = expression::Expression::from_json(json_logic)?;
    ast.get_variable_names()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashSet;

    #[test]
    fn simple_values() {
        let num = json!(1);
        assert_eq!(apply(&num), Ok(num));

        let string = json!("foo");
        assert_eq!(apply(&string), Ok(string));

        let boolean = json!(true);
        assert_eq!(apply(&boolean), Ok(boolean));
    }

    #[test]
    fn var_names() {
        let json_logic = json!({ "!==": [{ "var": "foo" }, { "var": ["bar", 5] }] });
        let names: HashSet<_> = [String::from("foo"), String::from("bar")]
            .iter()
            .cloned()
            .collect();
        assert_eq!(get_variable_names(&json_logic).unwrap(), names);
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

    // var
    mod variable {
        use super::*;

        #[test]
        fn simple() {
            assert_eq!(
                apply_with_data(&json!({ "var": "a" }), &json!({ "a": 12, "b": 24 })),
                Ok(json!(12))
            );

            assert_eq!(
                apply_with_data(&json!({ "var": ["a"] }), &json!({ "a": 12, "b": 24 })),
                Ok(json!(12))
            );

            assert_eq!(
                apply_with_data(
                    &json!({
                        "==": [
                            { "var": "var1" },
                            "foo"
                        ]
                    }),
                    &json!({ "var1": "foo"})
                ),
                Ok(json!(true))
            );
        }

        #[test]
        fn default() {
            assert_eq!(
                apply_with_data(&json!({ "var": ["nope"] }), &json!({ "a": 12, "b": 24 })),
                Ok(json!(null))
            );
            assert_eq!(
                apply_with_data(&json!({ "var": ["nope", 5] }), &json!({ "a": 12, "b": 24 })),
                Ok(json!(5))
            );
        }

        #[test]
        fn complex() {
            let logic = json!({
                "==": [
                    { "var": "var1" },
                    {
                        "var": [
                            "noneVar",
                            { "var": "var2" }
                        ]
                    }
                ]
            });

            assert_eq!(
                apply_with_data(
                    &logic,
                    &json!({
                        "var1": "foo",
                        "var2": "bar"
                    })
                ),
                Ok(json!(false))
            );

            assert_eq!(
                apply_with_data(
                    &logic,
                    &json!({
                        "var1": "foo",
                        "var2": "foo"
                    })
                ),
                Ok(json!(true))
            );
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
