use serde_json::{Number, Value};

use super::{logic, Data, Expression};

/// *, takes an arbitrary number of arguments and multiplicates them. Returns `Value::Null` if one
/// argument cannot be coerced into a number or if no arguments are passed.
/// If only one argument is specified, it is returned as is, to match the behaviour of the
/// javascript implementation of JsonLogic.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    match args {
        [] => Value::Null,
        [arg] => arg.compute(data),
        _ => {
            let mut result = 1f64;

            for arg in args {
                // Use parseFloat like in the javascript implementation.
                // parseFloat(null) is NaN, whereas coerce_to_f64 would return 0.
                match logic::parse_float(&arg.compute(data)) {
                    Some(num) => result *= num,
                    None => return Value::Null,
                }
            }

            Value::Number(Number::from_f64(result).unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn test() {
        assert_eq!(compute_const!(), Value::Null);
        assert_eq!(compute_const!(Value::Null), Value::Null);
        assert_eq!(compute_const!(json!("foo")), json!("foo"));
        assert_eq!(compute_const!(json!("6")), json!("6"));
        assert_eq!(compute_const!(json!(4), json!(2)), json!(8.0));
        assert_eq!(
            compute_const!(json!(4), json!(2), json!(2), json!(2)),
            json!(32.0)
        );
    }
}
