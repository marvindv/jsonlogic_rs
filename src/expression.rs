//! {"==": [1]}
//!     Equal (
//!         Constant(1)
//!     )
//!
//! {"==": [1, 2]}
//!     Equal (
//!         Constant(1),
//!         Constant(2)
//!     )
//!
//! {"===": null}, {"===": []}
//!     StrictEqual (
//!     )
//!
//! {"var": ["foo", 5]}
//!     Variable(
//!         "foo",
//!         5
//!     )
//!
//! {"var": "foo"}
//!     Variable(
//!         "foo"
//!     )
//!
//! {"!=": [ {"var": "foo"}, "bar" ]}
//!     NotEqual (
//!         Variable("foo"),
//!         Constant("bar")
//!     )
//!

use crate::operator::Operator;
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Constant(&'a Value),
    Computed(Operator, Vec<Expression<'a>>),
}

impl<'a> Expression<'a> {
    pub fn from_json(json: &Value) -> Result<Expression, String> {
        if !json.is_object() {
            return Ok(Expression::Constant(&json));
        }

        let object = json.as_object().unwrap();
        // If this object has more than one key-value pair, we will return it as is. This replicates
        // the behaviour of the javascript implementation.
        if object.len() != 1 {
            return Ok(Expression::Constant(&json));
        }

        let entry: Vec<(&String, &serde_json::Value)> = object.iter().collect();
        let &(operator_key, value) = entry.get(0).unwrap();
        let operator = Operator::from_str(operator_key)
            .ok_or(format!("Unrecognized operation {}", operator_key))?;

        let arguments: Vec<_> = match value {
            Value::Array(arr) => arr.iter().map(|expr| Expression::from_json(expr)).collect(),
            // Interpret as an empty array.
            Value::Null => Ok(vec![]),
            // If the value is not an array we can only assume that this is a shorthand.
            _ => Expression::from_json(value).and_then(|expr| Ok(vec![expr])),
        }?;

        Ok(Expression::Computed(operator, arguments))
    }
}

#[cfg(test)]
mod tests {
    use super::Expression::*;
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_to_ast() {
        assert_eq!(
            Expression::from_json(&json!({ "==": null })).unwrap(),
            Expression::Computed(Operator::Equal, vec![])
        );

        assert_eq!(
            Expression::from_json(&json!({ "==": [] })).unwrap(),
            Expression::Computed(Operator::Equal, vec![])
        );

        assert_eq!(
            Expression::from_json(&json!({ "==": [1] })).unwrap(),
            Expression::Computed(Operator::Equal, vec![Constant(&json!(1))])
        );

        assert_eq!(
            Expression::from_json(&json!({ "==": [1, 2] })).unwrap(),
            Expression::Computed(
                Operator::Equal,
                vec![Constant(&json!(1)), Constant(&json!(2))]
            )
        );

        assert_eq!(
            Expression::from_json(&json!({"!=": [5, 2]})).unwrap(),
            Expression::Computed(
                Operator::NotEqual,
                vec![Constant(&json!(5)), Constant(&json!(2))]
            )
        );

        assert_eq!(
            Expression::from_json(&json!({"var": ["foo"]})).unwrap(),
            Expression::Computed(Operator::Variable, vec![Constant(&json!("foo"))])
        );

        assert_eq!(
            Expression::from_json(&json!({"==": [{"var": ["foo"]}, "foo"]})).unwrap(),
            Expression::Computed(
                Operator::Equal,
                vec![
                    Expression::Computed(Operator::Variable, vec![Constant(&json!("foo"))]),
                    Expression::Constant(&json!("foo"))
                ]
            )
        );
    }
}
