use crate::expression::Expression;
use crate::operators;
use serde_json::Value;

pub fn compute_expression(expr: &Expression) -> Value {
    // All other expressions require arguments. Here we compute all existing arguments,
    // regardless of how many are actually needed. Default value handling and the lot is
    // handled by the operators.

    match expr {
        Expression::Constant(value) => (*value).clone(),
        Expression::Equal(args) => {
            let args = compute_arguments(&args);
            Value::Bool(operators::compute_equality(&args))
        }
        Expression::NotEqual(args) => {
            let args = compute_arguments(&args);
            Value::Bool(operators::compute_not_equal(&args))
        }
        Expression::StrictEqual(args) => {
            let args = compute_arguments(&args);
            Value::Bool(operators::compute_strict_equality(&args))
        }
        Expression::StrictNotEqual(args) => {
            let args = compute_arguments(&args);
            Value::Bool(operators::compute_strict_not_equal(&args))
        }
        Expression::Negation(args) => {
            let args = compute_arguments(&args);
            Value::Bool(operators::compute_negation(&args))
        }
        Expression::DoubleNegation(args) => {
            let args = compute_arguments(&args);
            Value::Bool(operators::compute_double_negation(&args))
        }
        Expression::Variable(_) => unimplemented!(),
    }
}

fn compute_arguments(args: &Vec<Expression>) -> Vec<Value> {
    args.iter().map(|arg| compute_expression(arg)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expression::*;
    use serde_json::json;

    #[test]
    fn constant_expression() {
        assert_eq!(compute_expression(&Constant(&json!(1))), json!(1));
    }

    #[test]
    fn equal() {
        assert_eq!(compute_expression(&Equal(vec![])), json!(true));
        assert_eq!(
            compute_expression(&Equal(vec![Constant(&json!(null))])),
            json!(true)
        );
        assert_eq!(
            compute_expression(&Equal(vec![Constant(&json!(1)), Constant(&json!(1))])),
            json!(true)
        );
        assert_eq!(
            compute_expression(&Equal(vec![Constant(&json!(1)), Constant(&json!(2))])),
            json!(false)
        );
    }

    // TODO: Add more or rely on the tests in the operator source files?
}
