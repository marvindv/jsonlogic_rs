mod and;
mod double_negation;
mod equality;
mod greater_equal_than;
mod greater_than;
mod if_else;
mod less_equal_than;
mod less_than;
mod logic;
mod missing;
mod negation;
mod not_equal;
mod or;
mod strict_equality;
mod strict_not_equal;
mod variable;

use serde_json::Value;

use super::Data;

/// Represents a JsonLogic operator.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
    /// Tests abstract equality as specified in
    /// https://www.ecma-international.org/ecma-262/#sec-abstract-equality-comparison, with type
    /// coercion. Requires two arguments.
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
    /// Takes an arbitrary number of arguments. Returns the first truthy argument or the last
    /// argument.
    Or,
    /// Takes an arbitrary number of arguments. Returns the first falsy argument or the last
    /// argument.
    And,
    /// Less than. Takes exactly 2 arguments, otherwise returns `false`.
    LessThan,
    /// Less or equal than. Takes exactly 2 arguments, otherwise returns `false`.
    LessEqualThan,
    /// Greater than. Takes exactly 2 arguments, otherwise returns `false`.
    GreaterThan,
    /// Greater or equal than. Takes exactly 2 arguments, otherwise returns `false`.
    GreaterEqualThan,
    /// Takes an array of data keys to search for (same format as `var`). Returns an array of any
    /// keys that are missing from the data object, or an empty array.
    Missing,
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
            "or" => Some(Operator::Or),
            "and" => Some(Operator::And),
            "<" => Some(Operator::LessThan),
            "<=" => Some(Operator::LessEqualThan),
            ">" => Some(Operator::GreaterThan),
            ">=" => Some(Operator::GreaterEqualThan),
            "missing" => Some(Operator::Missing),
            _ => None,
        }
    }

    pub fn compute(self, args: &[Value], data: &Data) -> Value {
        match self {
            Operator::Equal => equality::compute(args),
            Operator::NotEqual => not_equal::compute(args),
            Operator::StrictEqual => strict_equality::compute(args),
            Operator::StrictNotEqual => strict_not_equal::compute(args),
            Operator::Negation => negation::compute(args),
            Operator::DoubleNegation => double_negation::compute(args),
            Operator::Variable => variable::compute(args, data),
            Operator::If => if_else::compute(args),
            Operator::Or => or::compute(args),
            Operator::And => and::compute(args),
            Operator::LessThan => less_than::compute(args),
            Operator::LessEqualThan => less_equal_than::compute(args),
            Operator::GreaterThan => greater_than::compute(args),
            Operator::GreaterEqualThan => greater_equal_than::compute(args),
            Operator::Missing => missing::compute(args, data),
        }
    }
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
        assert_eq!(Operator::from_str("if"), Some(Operator::If));
        assert_eq!(Operator::from_str("or"), Some(Operator::Or));
        assert_eq!(Operator::from_str("and"), Some(Operator::And));
        assert_eq!(Operator::from_str("<"), Some(Operator::LessThan));
        assert_eq!(Operator::from_str("<="), Some(Operator::LessEqualThan));
        assert_eq!(Operator::from_str(">"), Some(Operator::GreaterThan));
        assert_eq!(Operator::from_str(">="), Some(Operator::GreaterEqualThan));
        assert_eq!(Operator::from_str("missing"), Some(Operator::Missing));
    }
}
