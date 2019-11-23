mod double_negation;
mod equality;
mod if_else;
mod logic;
mod negation;
mod not_equal;
mod strict_equality;
mod strict_not_equal;
mod variable;

use super::Data;

use double_negation::compute_double_negation;
use equality::compute_equality;
use if_else::compute_if;
use negation::compute_negation;
use not_equal::compute_not_equal;
use strict_equality::compute_strict_equality;
use strict_not_equal::compute_strict_not_equal;
use variable::compute_variable;

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
    ///
    /// If the first argument is null, the data object is returned as is.
    Variable,
    /// Logical negation (“not”). Takes just one argument.
    Negation,
    /// Double negation, or “cast to a boolean.” Takes a single argument.
    DoubleNegation,
    /// The if statement typically takes 3 arguments: a condition (if), what to do if it’s true
    /// (then), and what to do if it’s false (else). If can also take more than 3 arguments, and
    /// will pair up arguments like if/then elseif/then elseif/then else.
    If,
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
            "if" => Some(Operator::If),
            _ => None,
        }
    }

    pub fn compute(self, args: &[Value], data: &Data) -> Value {
        match self {
            Operator::Equal => Value::Bool(compute_equality(&args)),
            Operator::NotEqual => Value::Bool(compute_not_equal(&args)),
            Operator::StrictEqual => Value::Bool(compute_strict_equality(&args)),
            Operator::StrictNotEqual => Value::Bool(compute_strict_not_equal(&args)),
            Operator::Negation => Value::Bool(compute_negation(&args)),
            Operator::DoubleNegation => Value::Bool(compute_double_negation(&args)),
            Operator::Variable => compute_variable(args, data),
            Operator::If => compute_if(args, data),
        }
    }
}

pub trait Computable {
    fn compute(&self, args: &[Value], data: &Data) -> Value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(Operator::from_str("=="), Some(Operator::Equal));
        assert_eq!(Operator::from_str("!="), Some(Operator::NotEqual));
        assert_eq!(Operator::from_str("==="), Some(Operator::StrictEqual));
        assert_eq!(Operator::from_str("!=="), Some(Operator::StrictNotEqual));
        assert_eq!(Operator::from_str("var"), Some(Operator::Variable));
        assert_eq!(Operator::from_str("!"), Some(Operator::Negation));
        assert_eq!(Operator::from_str("!!"), Some(Operator::DoubleNegation));
    }
}
