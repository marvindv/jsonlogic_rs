use serde_json::{Number, Value};

use super::{logic, Data, Expression};

/// Returns the largest of the given numbers. Arguments that are no numbers are coerced into
/// numbers. If one argument cannot be coerced or there are no arguments, `Value::Null` will be
/// returned.
/// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/max
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let mut max: Option<f64> = None;

    for arg in args {
        let arg = arg.compute(data);
        match (logic::coerce_to_f64(&arg), max) {
            (Some(num), Some(current_max)) => {
                if num > current_max {
                    max = Some(num);
                }
            }
            (Some(num), None) => max = Some(num),
            (None, _) => return Value::Null,
        }
    }

    match max {
        Some(max) => Value::Number(Number::from_f64(max).unwrap()),
        None => Value::Null,
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
        assert_eq!(compute_const!(json!("foo")), Value::Null);
        assert_eq!(compute_const!(json!("1"), json!(-2)), json!(1.0));
        assert_eq!(
            compute_const!(json!(1), json!("-2"), json!("foo"), json!(-4)),
            Value::Null
        );
        assert_eq!(compute_const!(json!(null)), json!(0.0));
        assert_eq!(compute_const!(json!(-4)), json!(-4.0));
        assert_eq!(compute_const!(json!(null), json!(2), json!(-4)), json!(2.0));
    }
}
