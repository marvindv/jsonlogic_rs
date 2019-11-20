use crate::operators;
use serde_json::Value;

/// Represents a JsonLogic operator.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
    /// Tests equality, with type coercion. Requires two arguments.
    Equal,
    /// Tests strict equality. Requires two arguments.
    StrictEqual,
    /// Tests not-equal, with type coercion.
    NotEqual,
    /// Tests strict not-equal.
    StrictNotEqual,
    /// Retrieve data from the provided data object.
    Variable,
    /// Logical negation (“not”). Takes just one argument.
    Negation,
    /// Double negation, or “cast to a boolean.” Takes a single argument.
    DoubleNegation,
}

impl Operator {
    /// Returns the Operator matching the given string representation. Returns None if the given
    /// string matches no known operator.
    pub fn from_str(s: &str) -> Option<Operator> {
        match s {
            "==" => Some(Operator::Equal),
            "===" => Some(Operator::StrictEqual),
            "!=" => Some(Operator::NotEqual),
            "!==" => Some(Operator::StrictNotEqual),
            "var" => Some(Operator::Variable),
            "!" => Some(Operator::Negation),
            "!!" => Some(Operator::DoubleNegation),
            _ => None,
        }
    }

    pub fn compute(&self, args: &Vec<Value>) -> Value {
        match self {
            Operator::Equal => Value::Bool(operators::compute_equality(&args)),
            Operator::NotEqual => Value::Bool(operators::compute_not_equal(&args)),
            Operator::StrictEqual => Value::Bool(operators::compute_strict_equality(&args)),
            Operator::StrictNotEqual => Value::Bool(operators::compute_strict_not_equal(&args)),
            Operator::Negation => Value::Bool(operators::compute_negation(&args)),
            Operator::DoubleNegation => Value::Bool(operators::compute_double_negation(&args)),
            Operator::Variable => unimplemented!(),
        }
    }
}
