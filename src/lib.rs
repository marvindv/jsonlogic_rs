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
/// let rule = json!({
///     "===": [
///         2,
///         { "var": "foo" }
///     ]
/// });
///
/// let data = json!({ "foo": 2 });
/// assert_eq!(jsonlogic::apply(&rule, &data), Ok(Value::Bool(true)));
///
/// let data = json!({ "foo": 3 });
/// assert_eq!(jsonlogic::apply(&rule, &data), Ok(Value::Bool(false)));
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
                apply(&json!({ "var": "" }), &json!({ "a": 12, "b": 24 })),
                Ok(json!({ "a": 12, "b": 24 }))
            );

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

            assert_eq!(
                apply(
                    &json!({"if" :[
                      {"merge": [
                        {"missing":["first_name", "last_name"]},
                        {"missing_some":[1, ["cell_phone", "home_phone"] ]}
                      ]},
                      "We require first name, last name, and one phone number.",
                      "OK to proceed"
                    ]}),
                    &json!({"first_name":"Bruce", "last_name":"Wayne"})
                ),
                Ok(json!(
                    "We require first name, last name, and one phone number."
                ))
            );
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
    }

    mod string_operations {
        use super::*;

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
    }

    mod array_operations {
        use super::*;

        #[test]
        fn merge() {
            assert_eq!(
                apply(&json!({"merge":[ [1,2], [3,4] ]}), &Value::Null),
                Ok(json!([1, 2, 3, 4]))
            );
            assert_eq!(
                apply(&json!({"merge":[ 1, 2, [3,4] ]}), &Value::Null),
                Ok(json!([1, 2, 3, 4]))
            );
            assert_eq!(
                apply(
                    &json!({"missing" :
                      { "merge" : [
                        "vin",
                        {"if": [{"var":"financing"}, ["apr", "term"], [] ]}
                      ]}
                    }),
                    &json!({"financing":true})
                ),
                Ok(json!(["vin", "apr", "term"]))
            );
            assert_eq!(
                apply(
                    &json!({"missing" :
                      { "merge" : [
                        "vin",
                        {"if": [{"var":"financing"}, ["apr", "term"], [] ]}
                      ]}
                    }),
                    &json!({"financing":false})
                ),
                Ok(json!(["vin"]))
            );
        }

        #[test]
        fn is_in() {
            assert_eq!(
                apply(
                    &json!({"in":[ "Ringo", ["John", "Paul", "George", "Ringo"] ]}),
                    &Value::Null
                ),
                Ok(json!(true))
            );
        }

        #[test]
        fn map() {
            let rule = json!({ "map": [
              { "var": "integers"},
              { "*": [{ "var": "" }, 2] }
            ]});

            assert_eq!(
                apply(&rule, &json!({ "integers": [1, 2, 3, 4, 5] })),
                Ok(json!([2.0, 4.0, 6.0, 8.0, 10.0]))
            );
            assert_eq!(
                apply(&rule, &json!({ "_integers": [1, 2, 3, 4, 5] })),
                Ok(json!([]))
            );
        }

        #[test]
        fn filter() {
            let rule = json!({ "filter": [
              { "var": "integers"},
              { "%": [{ "var": "" }, 2] }
            ]});

            assert_eq!(
                apply(&rule, &json!({ "integers": [1, 2, 3, 4, 5] })),
                Ok(json!([1, 3, 5]))
            );
            assert_eq!(
                apply(&rule, &json!({ "_integers": [1, 2, 3, 4, 5] })),
                Ok(json!([]))
            );

            let rule = json!({ "filter": [
              { "var": "integers"}
            ]});
            assert_eq!(
                apply(&rule, &json!({ "integers": [1, 2, 3, 4, 5] })),
                Ok(json!([]))
            );
        }

        #[test]
        fn reduce() {
            let rule = json!({ "reduce": [
                { "var": "integers" },
                { "+": [{ "var": "current" }, { "var": "accumulator" }] },
                0
            ]});
            assert_eq!(
                apply(&rule, &json!({ "integers": [1, 2, 3, 4, 5] })),
                Ok(json!(15.0))
            );

            // Return initial value if data is not an array.
            let rule = json!({ "reduce": [
                { "var": "integers" },
                { "+": [{ "var": "current" }, { "var": "accumulator" }] },
                0
            ]});
            assert_eq!(apply(&rule, &json!({ "integers": 5 })), Ok(json!(0)));

            // Default for initial value should be null.
            let rule = json!({ "reduce": [
                { "var": "integers" },
                { "+": [{ "var": "current" }, { "var": "accumulator" }] }
            ]});
            assert_eq!(
                apply(&rule, &json!({ "integers": [1, 2, 3, 4, 5] })),
                Ok(json!(null))
            );

            // Return null without reducer.
            let rule = json!({ "reduce": [
                { "var": "integers" }
            ]});
            assert_eq!(
                apply(&rule, &json!({ "integers": [1, 2, 3, 4, 5] })),
                Ok(json!(null))
            );
        }

        #[test]
        fn all() {
            let rule = json!({ "all": [[1, 2, 3], { ">": [{ "var": "" }, 0] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));

            let rule = json!({ "all": [[1, 2, 3, -4], { "<": [{ "var": "" }, 0] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "all": [[1, 2, 3, "-4"], { "<": [{ "var": "" }, 0] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "all": [[], { "<": [{ "var": "" }, 0] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "all": [[1, 2, 3, -4], { ">": [{ "var": "" }, 0] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "all": [[1, 2, 3, -4], "foo"] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));

            let rule = json!({ "all": [[1, 2, 3, -4], ""] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "all": [] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            // Should work on strings if the test operation works for chars, because of an
            // implementation detail in the JavaScript implementation. The existence of the `length`
            // property is checked on the input, not whether the input is actually an array.
            let rule = json!({ "all": ["foo", "foo"] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));

            let rule = json!({ "all": ["aaa", { "===": [{ "var": "" }, "a"] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));

            let rule = json!({ "all": ["aba", { "===": [{ "var": "" }, "a"] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "all": ["bba", { "===": [{ "var": "" }, "a"] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "all": ["bbb", { "===": [{ "var": "" }, "a"] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));
        }

        #[test]
        fn some() {
            let rule = json!({ "some": [[-1, 0, 1], { ">": [{ "var": "" }, 0] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));

            let rule = json!({ "some": [[-1, 0, "1"], { ">": [{ "var": "" }, 0] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));

            let rule = json!({ "some": [[-1, 0, "1"], { ">": [{ "var": "" }, 1] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "some": [[], { ">": [{ "var": "" }, 1] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "some": [[]] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "some": [[-1, 0, "1"]] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "some": [] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "some": ["foo", { ">": [{ "var": "" }, 0] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "some": ["foo"] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            // Should not work on strings if the test operation works for chars.
            let rule = json!({ "some": ["aaa", { "===": [{ "var": "" }, "a"] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "some": ["aba", { "===": [{ "var": "" }, "a"] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "some": ["bba", { "===": [{ "var": "" }, "a"] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));

            let rule = json!({ "some": ["bbb", { "===": [{ "var": "" }, "a"] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(false)));
        }

        #[test]
        fn some_complex() {
            let rule = json!(
                { "some": [{ "var": "pies" }, { "==": [{ "var": "filling" }, "apple"] }] }
            );
            let data = json!({
                "pies":[
                    { "filling": "pumpkin", "temp": 110 },
                    { "filling": "rhubarb", "temp": 210 },
                    { "filling": "apple", "temp": 310 }
                ]}
            );
            assert_eq!(apply(&rule, &data), Ok(json!(true)));
        }

        #[test]
        fn none() {
            let rule = json!({ "none": [[-3, -2, -1], { ">": [{ "var": "" }, 0] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));

            let rule = json!({ "none": [[-3, -2, -1]] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));

            let rule = json!({ "none": [[], { ">": [{ "var": "" }, 0] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));

            let rule = json!({ "none": [] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));

            let rule = json!({ "none": ["foo"] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));

            let rule = json!({ "none": ["foo", { ">": [{ "var": "" }, 0] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));

            // Should not work (i.e. return true) on strings if the test operation works for chars.
            let rule = json!({ "none": ["aaa", { "===": [{ "var": "" }, "a"] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));

            let rule = json!({ "none": ["aba", { "===": [{ "var": "" }, "a"] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));

            let rule = json!({ "none": ["bba", { "===": [{ "var": "" }, "a"] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));

            let rule = json!({ "none": ["bbb", { "===": [{ "var": "" }, "a"] }] });
            assert_eq!(apply(&rule, &json!(null)), Ok(json!(true)));
        }
    }
}
