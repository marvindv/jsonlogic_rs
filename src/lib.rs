extern crate serde_json;

mod data;
mod expression;
mod operators;

use serde_json::Value;
use std::collections::HashSet;

use data::Data;

/// Applies the given JsonLogic rule to the specified data.
/// If the rule does not use any variables, you may pass `&Value::Null` as the second argument.
///
/// # Example
///
/// ```
/// use serde_json::{json, Value};
///
/// let rule = json!({"===": [2, {"var": "foo"}]});
/// assert_eq!(jsonlogic::apply(&rule, &json!({ "foo": 2 })), Ok(Value::Bool(true)));
/// assert_eq!(jsonlogic::apply(&rule, &json!({ "foo": 3 })), Ok(Value::Bool(false)));
/// ```
pub fn apply(json_logic: &Value, data: &Value) -> Result<Value, String> {
    let ast = expression::Expression::from_json(json_logic)?;
    let data = Data::from_json(data);
    Ok(ast.compute(&data))
}

// TODO: Add to public api when ready.
#[allow(dead_code)]
fn get_variable_names(json_logic: &Value) -> Result<HashSet<String>, String> {
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
        assert_eq!(apply(&num, &Value::Null), Ok(num));

        let string = json!("foo");
        assert_eq!(apply(&string, &Value::Null), Ok(string));

        let boolean = json!(true);
        assert_eq!(apply(&boolean, &Value::Null), Ok(boolean));
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

    // var
    mod variable {
        use super::*;

        #[test]
        fn simple() {
            assert_eq!(
                apply(&json!({ "var": "a" }), &json!({ "a": 12, "b": 24 })),
                Ok(json!(12))
            );

            assert_eq!(
                apply(&json!({ "var": ["a"] }), &json!({ "a": 12, "b": 24 })),
                Ok(json!(12))
            );

            assert_eq!(
                apply(
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
                apply(&json!({ "var": ["nope"] }), &json!({ "a": 12, "b": 24 })),
                Ok(json!(null))
            );
            assert_eq!(
                apply(&json!({ "var": ["nope", 5] }), &json!({ "a": 12, "b": 24 })),
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
                apply(
                    &logic,
                    &json!({
                        "var1": "foo",
                        "var2": "bar"
                    })
                ),
                Ok(json!(false))
            );

            assert_eq!(
                apply(
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

    // missing
    mod missing {
        use super::*;

        #[test]
        fn test() {
            assert_eq!(
                apply(
                    &json!({"missing":["a", "b", "6.foo.1", "6.foo.3"]}),
                    &json!({"a":"apple", "c":"carrot", "6": {"foo": "bar"}})
                ),
                Ok(json!(["b", "6.foo.3"]))
            );

            assert_eq!(
                apply(
                    &json!({"if":[
                      {"missing":["a", "b"]},
                      "Not enough fruit",
                      "OK to proceed"
                    ]}),
                    &json!({"a":"apple", "b":"banana"})
                ),
                Ok(json!("OK to proceed"))
            );
        }
    }

    // missing_some
    mod missing_some {
        use super::*;

        #[test]
        fn test() {
            assert_eq!(
                apply(
                    &json!({"missing_some":[1, ["a", "b", "c"]]}),
                    &json!({"a":"apple"})
                ),
                Ok(json!([]))
            );

            assert_eq!(
                apply(
                    &json!({"missing_some":[2, ["a", "b", "c"]]}),
                    &json!({"a":"apple"})
                ),
                Ok(json!(["b", "c"]))
            );

            // TODO(#6): add after merge is implemented
            // assert_eq!(
            //     apply(
            //         &json!({"if" :[
            //           {"merge": [
            //             {"missing":["first_name", "last_name"]},
            //             {"missing_some":[1, ["cell_phone", "home_phone"] ]}
            //           ]},
            //           "We require first name, last name, and one phone number.",
            //           "OK to proceed"
            //         ]}),
            //         &json!({"first_name":"Bruce", "last_name":"Wayne"})
            //     ),
            //     Ok(json!(
            //         "We require first name, last name, and one phone number."
            //     ))
            // );
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

    mod numeric_operations {
        use super::*;

        #[test]
        fn test() {
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
    }
}
