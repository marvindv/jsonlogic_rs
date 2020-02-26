use jsonlogic::apply;
use serde_json::{json, Value};

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
