use serde_json::{Number, Value};

use super::logic;

/// +, takes an arbitrary number of arguments and sums them up. If just one argument is passed, it
/// will be cast to a number. Returns `Value::Null` if one argument cannot be coerced into a
/// number.
pub fn compute(args: &[Value]) -> Value {
    let mut result = 0f64;

    for arg in args.iter() {
        // Use parseFloat like in the javascript implementation.
        // parseFloat(null) is NaN, whereas coerce_to_f64 would return 0.
        match logic::parse_float(arg) {
            Some(num) => result += num,
            None => return Value::Null,
        }
    }

    Value::Number(Number::from_f64(result).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test() {
        assert_eq!(compute(&[]), json!(0.0));
        assert_eq!(compute(&[Value::Null]), Value::Null);
        assert_eq!(compute(&[json!("foo")]), Value::Null);
        assert_eq!(compute(&[json!("6")]), json!(6.0));
        assert_eq!(compute(&[json!(4), json!(2)]), json!(6.0));
        assert_eq!(
            compute(&[json!(4), json!(2), json!(2), json!(2)]),
            json!(10.0)
        );
    }
}
