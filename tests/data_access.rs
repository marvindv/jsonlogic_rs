use jsonlogic::apply;
use serde_json::json;

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
