use serde_json::{Number, Value};

use super::{logic, Data, Expression};

/// +, takes an arbitrary number of arguments and sums them up. If just one argument is passed, it
/// will be cast to a number. Returns `Value::Null` if one argument cannot be coerced into a
/// number.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let mut result = 0f64;

    for arg in args.iter() {
        // Use parseFloat like in the javascript implementation.
        // parseFloat(null) is NaN, whereas coerce_to_f64 would return 0.
        match logic::parse_float(&arg.compute(data)) {
            Some(num) => result += num,
            None => return Value::Null,
        }
    }

    Value::Number(Number::from_f64(result).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn test() {
        assert_eq!(compute_const!(), json!(0.0));
        assert_eq!(compute_const!(Value::Null), Value::Null);
        assert_eq!(compute_const!(json!("foo")), Value::Null);
        assert_eq!(compute_const!(json!("6")), json!(6.0));
        assert_eq!(compute_const!(json!(4), json!(2)), json!(6.0));
        assert_eq!(
            compute_const!(json!(4), json!(2), json!(2), json!(2)),
            json!(10.0)
        );
    }
}
