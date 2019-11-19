extern crate serde_json;

use serde_json::Value;

mod operators;

enum Operator {
    /// Tests equality, with type coercion. Requires two arguments.
    Equality,
    /// Tests strict equality. Requires two arguments.
    StrictEquality,
    /// Tests not-equal, with type coercion.
    NotEqual,
    /// Tests strict not-equal.
    StrictNotEqual,
}

impl Operator {
    /// Returns the Operator matching the given string representation. Returns None if the given
    /// string matches no known operator.
    fn from_str(s: &str) -> Option<Operator> {
        match s {
            "==" => Some(Operator::Equality),
            "===" => Some(Operator::StrictEquality),
            "!=" => Some(Operator::NotEqual),
            "!==" => Some(Operator::StrictNotEqual),
            _ => None,
        }
    }
}

fn compute_double_negation(argument: &Value) -> Result<Value, String> {
    unimplemented!();
}

pub fn apply(json: &serde_json::Value) -> Result<serde_json::Value, String> {
    if !json.is_object() {
        // Return simple values.
        // TODO: Avoid cloning if possible.
        return Ok(json.clone());
    }

    let object = match json.as_object() {
        Some(v) => v,
        None => unreachable!(),
    };

    // If this object has more than one key-value pair, we will return it as is. This replicates the
    // behaviour of the javascript implementation.
    if object.len() != 1 {
        // TODO: Avoid cloning if possible.
        return Ok(json.clone());
    }

    let entry: Vec<(&String, &serde_json::Value)> = object.iter().collect();

    let &(operator_key, value) = match entry.get(0) {
        Some(v) => v,
        None => unreachable!(),
    };
    let operator = match Operator::from_str(operator_key) {
        Some(o) => o,
        None => return Err(format!("Unrecognized operation {}", operator_key)),
    };

    // TODO: To allow nested expressions, process all values here and pass them as an array to the
    // operators as an vector.

    match operator {
        Operator::Equality => operators::compute_equality(value),
        Operator::StrictEquality => operators::compute_strict_equality(value),
        _ => panic!("Not implemented"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn truthy_values() {
        // See http://jsonlogic.com/truthy.html
        assert_eq!(operators::truthy(&json!(0)), false);
        assert_eq!(operators::truthy(&json!(-1)), true);
        assert_eq!(operators::truthy(&json!(1)), true);
        assert_eq!(operators::truthy(&json!([])), false);
        assert_eq!(operators::truthy(&json!([1, 2])), true);
        assert_eq!(operators::truthy(&json!("")), false);
        assert_eq!(operators::truthy(&json!("anything")), true);
        assert_eq!(operators::truthy(&json!("0")), true);
        assert_eq!(operators::truthy(&Value::Null), false);

        assert_eq!(operators::truthy(&json!({})), true);
        assert_eq!(operators::truthy(&json!(true)), true);
        assert_eq!(operators::truthy(&json!(false)), false);
    }

    #[test]
    fn simple_values() {
        let num = json!(1);
        assert_eq!(apply(&num), Ok(num));

        let string = json!("foo");
        assert_eq!(apply(&string), Ok(string));

        let boolean = json!(true);
        assert_eq!(apply(&boolean), Ok(boolean));
    }

    //
    // ==
    //

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
        assert_eq!(
            apply(&json!({ "==": [Value::Null, Value::Null] })),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply(&json!({
                "==": [1, 1]
            })),
            Ok(Value::Bool(true))
        );

        assert_eq!(apply(&json!({ "==": 0 })), Ok(Value::Bool(false)));
        assert_eq!(apply(&json!({ "==": [0] })), Ok(Value::Bool(false)));
        assert_eq!(
            apply(&json!({
                "==": [1, 0]
            })),
            Ok(Value::Bool(false))
        );
    }

    #[test]
    fn equality_with_type_coercion() {
        assert_eq!(
            apply(&json!({
                "==": [0, ""]
            })),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply(&json!({
                "==": [[1, 2], "1,2"]
            })),
            Ok(Value::Bool(true))
        );

        assert_eq!(
            apply(&json!({
                "==": [0, null]
            })),
            Ok(Value::Bool(false))
        );
    }

    //
    // ===
    //

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
        assert_eq!(
            apply(&json!({ "===": [Value::Null, Value::Null] })),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply(&json!({
                "===": [1, 1]
            })),
            Ok(Value::Bool(true))
        );

        assert_eq!(apply(&json!({ "===": 0 })), Ok(Value::Bool(false)));
        assert_eq!(apply(&json!({ "===": [0] })), Ok(Value::Bool(false)));
        assert_eq!(
            apply(&json!({
                "===": [1, 0]
            })),
            Ok(Value::Bool(false))
        );
    }

    #[test]
    fn strict_equality_with_type_coercion() {
        assert_eq!(
            apply(&json!({
                "===": [0, ""]
            })),
            Ok(Value::Bool(false))
        );

        assert_eq!(
            apply(&json!({
                "===": [0, null]
            })),
            Ok(Value::Bool(false))
        );
    }
}
