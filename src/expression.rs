use crate::operators::Operator;
use crate::Data;
use serde_json::Value;
use std::collections::HashSet;

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
            .ok_or_else(|| format!("Unrecognized operation {}", operator_key))?;

        let arguments: Vec<_> = match value {
            Value::Array(arr) => arr.iter().map(|expr| Expression::from_json(expr)).collect(),
            // Interpret as an empty array.
            Value::Null => Ok(vec![]),
            // If the value is not an array we can only assume that this is a shorthand.
            _ => Expression::from_json(value).and_then(|expr| Ok(vec![expr])),
        }?;

        Ok(Expression::Computed(operator, arguments))
    }

    /// Computes the expression and returns value it evaluates to.
    pub fn compute(&self) -> Value {
        self.compute_with_data(&Data::from_json(&Value::Null))
    }

    /// Computes the expression and returns value it evaluates to.
    pub fn compute_with_data(&self, data: &Data) -> Value {
        match self {
            Expression::Constant(value) => (*value).clone(),
            Expression::Computed(operator, args) => {
                let args: Vec<Value> = args.iter().map(|arg| arg.compute_with_data(data)).collect();
                operator.compute(&args, data)
            }
        }
    }

    /// Returns a set that contains all variable names that occure in this expression and its child
    /// expressions. Errors if a variable operator
    ///
    /// - has not a string as its argument (TODO: numbers are ok for when data is an array)
    /// - has a non static argument
    ///
    /// While the latter is valid for computation, it is currently not implemented to analyze the
    /// variable name for that.
    pub fn get_variable_names(&self) -> Result<HashSet<String>, String> {
        let mut variable_names: HashSet<String> = HashSet::new();

        self.insert_var_names(&mut variable_names)?;
        Ok(variable_names)
    }

    fn insert_var_names(&self, names: &mut HashSet<String>) -> Result<(), String> {
        match self {
            Expression::Constant(_) => Ok(()),
            Expression::Computed(operator, args) => {
                if let Operator::Variable = operator {
                    let first_expr = args
                        .get(0)
                        .ok_or("found Variable operator without arguments")?;
                    if let Expression::Constant(name_value) = first_expr {
                        let name = name_value
                            .as_str()
                            .ok_or("found Variable operator with non string argument")?;
                        names.insert(name.to_owned());
                        return Ok(());
                    } else {
                        return Err(String::from(
                            "found Variable operator with non static argument",
                        ));
                    }
                }

                // For all other operations analyze the arguments recursive.
                args.iter()
                    .map(|expr| expr.insert_var_names(names))
                    .collect()
            }
        }
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

    #[test]
    fn get_variable_names_error() {
        assert_eq!(
            Expression::Computed(
                Operator::Variable,
                vec![Expression::Computed(
                    Operator::Variable,
                    vec![Expression::Constant(&json!("foo"))]
                )]
            )
            .get_variable_names(),
            Err(String::from(
                "found Variable operator with non static argument"
            ))
        );

        assert_eq!(
            Expression::Computed(Operator::Variable, vec![Expression::Constant(&json!(1))])
                .get_variable_names(),
            Err(String::from(
                "found Variable operator with non string argument"
            ))
        );

        assert_eq!(
            Expression::Computed(Operator::Variable, vec![]).get_variable_names(),
            Err(String::from("found Variable operator without arguments"))
        );
    }

    #[test]
    fn get_variable_names() {
        assert_eq!(
            Expression::Constant(&json!("foo")).get_variable_names(),
            Ok(HashSet::new())
        );

        assert_eq!(
            Expression::Computed(
                Operator::Variable,
                vec![Expression::Constant(&json!("foo"))]
            )
            .get_variable_names(),
            Ok(["foo".to_owned()].iter().cloned().collect::<HashSet<_>>())
        );

        assert_eq!(
            Expression::Computed(
                Operator::Equal,
                vec![
                    Expression::Constant(&json!("a value")),
                    Expression::Computed(
                        Operator::Variable,
                        vec![Expression::Constant(&json!("foo"))]
                    )
                ]
            )
            .get_variable_names(),
            Ok(["foo".to_owned()].iter().cloned().collect::<HashSet<_>>())
        );

        assert_eq!(
            Expression::Computed(
                Operator::Equal,
                vec![
                    Expression::Computed(
                        Operator::Variable,
                        vec![Expression::Constant(&json!("foo"))]
                    ),
                    Expression::Computed(
                        Operator::Variable,
                        vec![Expression::Constant(&json!("foo"))]
                    )
                ]
            )
            .get_variable_names(),
            Ok(["foo".to_owned()].iter().cloned().collect::<HashSet<_>>())
        );

        assert_eq!(
            Expression::Computed(
                Operator::Equal,
                vec![
                    Expression::Computed(
                        Operator::Variable,
                        vec![Expression::Constant(&json!("bar"))]
                    ),
                    Expression::Computed(
                        Operator::Variable,
                        vec![Expression::Constant(&json!("foo"))]
                    )
                ]
            )
            .get_variable_names(),
            Ok(["foo".to_owned(), "bar".to_owned()]
                .iter()
                .cloned()
                .collect::<HashSet<_>>())
        );
    }

    mod compute {
        use super::*;

        #[test]
        fn constant_expression() {
            assert_eq!(Constant(&json!(1)).compute(), json!(1));
        }

        #[test]
        fn equal() {
            assert_eq!(Computed(Operator::Equal, vec![]).compute(), json!(true));
            assert_eq!(
                Computed(Operator::Equal, vec![Constant(&json!(null))]).compute(),
                json!(true)
            );
            assert_eq!(
                Computed(
                    Operator::Equal,
                    vec![Constant(&json!(1)), Constant(&json!(1))]
                )
                .compute(),
                json!(true)
            );
            assert_eq!(
                Computed(
                    Operator::Equal,
                    vec![Constant(&json!(1)), Constant(&json!(2))]
                )
                .compute(),
                json!(false)
            );
        }
    }
}
