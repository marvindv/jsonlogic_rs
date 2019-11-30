use serde_json::{Number, Value};

use super::logic;

/// "-", takes two numbers and returns the substraction of the them.
/// If only one argument is passed, returns the negation of that argument.
/// Returns `Value::Null` one of the arguments cannot be coerced into a number.
pub fn compute(args: &[Value]) -> Value {
    let a = match args.get(0) {
        Some(arg) => arg,
        None => return Value::Null,
    };

    match args.get(1) {
        None => compute_negation(&logic::coerce_to_f64(a)),
        Some(b) => compute_substraction(&logic::coerce_to_f64(a), &logic::coerce_to_f64(b)),
    }
}

fn compute_negation(a: &Option<f64>) -> Value {
    match a {
        Some(a) => Value::Number(Number::from_f64(-1f64 * a).unwrap()),
        None => Value::Null,
    }
}

fn compute_substraction(a: &Option<f64>, b: &Option<f64>) -> Value {
    match (a, b) {
        (Some(a), Some(b)) => match Number::from_f64(a - b) {
            Some(num) => Value::Number(num),
            None => Value::Null,
        },
        _ => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn null() {
        assert_eq!(compute(&[]), Value::Null);
        assert_eq!(compute(&[json!("a")]), Value::Null);
    }

    #[test]
    fn negation() {
        assert_eq!(compute(&[json!(1)]), json!(-1.0));
        assert_eq!(compute(&[json!("")]), json!(-0.0));
        assert!(logic::is_strict_equal(&compute(&[json!("")]), &json!(0)));
        assert_eq!(compute(&[json!("-5")]), json!(5.0));
    }

    #[test]
    fn substraction() {
        assert_eq!(compute(&[json!(1), json!(2)]), json!(-1.0));
        assert_eq!(compute(&[json!(4), json!(2)]), json!(2.0));
        assert_eq!(compute(&[json!(4), json!(-2)]), json!(6.0));
    }
}
