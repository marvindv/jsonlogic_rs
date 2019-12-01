mod addition;
mod and;
mod cat;
mod division;
mod double_negation;
mod equality;
mod greater_equal_than;
mod greater_than;
mod if_else;
mod is_substr;
mod less_equal_than;
mod less_than;
mod logic;
mod max;
mod min;
mod missing;
mod missing_some;
mod modulo;
mod multiplication;
mod negation;
mod not_equal;
mod or;
mod strict_equality;
mod strict_not_equal;
mod subtraction;
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
    /// Takes a minimum number of data keys that are required, and an array of keys to search for
    /// (same format as `var` or `missing`). Returns an empty array if the minimum is met, or an
    /// array of the missing keys otherwise.
    MissingSome,
    /// Return the minimum from a list of values. Matches the javascript `Math.min` implementation.
    Min,
    /// Return the maximum from a list of values. Matches the javascript `Math.max` implementation.
    Max,
    /// +, takes an arbitrary number of arguments and sums them up. If just one argument is passed,
    /// it will be cast to a number. Returns `Value::Null` if one argument cannot be coerced into a
    /// number.
    Addition,
    /// -, if just one argument is passed, it gets negated.
    Subtraction,
    /// *, takes an arbitrary number of arguments and multiplicates them.
    Multiplication,
    /// /
    Division,
    /// %, Finds the remainder after the first argument is divided by the second argument.
    Modulo,
    /// Expects two string arguments. Tests, whether the first argument is a substring of the
    /// second argument.
    In,
    /// Concatenate all the supplied arguments. Note that this is not a join or implode operation,
    /// there is no "glue" string.
    Cat,
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
            "missing_some" => Some(Operator::MissingSome),
            "min" => Some(Operator::Min),
            "max" => Some(Operator::Max),
            "+" => Some(Operator::Addition),
            "-" => Some(Operator::Subtraction),
            "*" => Some(Operator::Multiplication),
            "/" => Some(Operator::Division),
            "%" => Some(Operator::Modulo),
            "in" => Some(Operator::In),
            "cat" => Some(Operator::Cat),
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
            Operator::MissingSome => missing_some::compute(args, data),
            Operator::Min => min::compute(args),
            Operator::Max => max::compute(args),
            Operator::Addition => addition::compute(args),
            Operator::Subtraction => subtraction::compute(args),
            Operator::Multiplication => multiplication::compute(args),
            Operator::Division => division::compute(args),
            Operator::Modulo => modulo::compute(args),
            Operator::In => is_substr::compute(args),
            Operator::Cat => cat::compute(args),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::cognitive_complexity)]
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
        assert_eq!(
            Operator::from_str("missing_some"),
            Some(Operator::MissingSome)
        );
        assert_eq!(Operator::from_str("min"), Some(Operator::Min));
        assert_eq!(Operator::from_str("max"), Some(Operator::Max));
        assert_eq!(Operator::from_str("+"), Some(Operator::Addition));
        assert_eq!(Operator::from_str("-"), Some(Operator::Subtraction));
        assert_eq!(Operator::from_str("*"), Some(Operator::Multiplication));
        assert_eq!(Operator::from_str("/"), Some(Operator::Division));
        assert_eq!(Operator::from_str("%"), Some(Operator::Modulo));
        assert_eq!(Operator::from_str("in"), Some(Operator::In));
        assert_eq!(Operator::from_str("cat"), Some(Operator::Cat));
    }
}
