use serde_json::Value;

use super::{logic, Data, Expression};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    match args.len() {
        // Return the condition, for whatever reason.
        0..=1 => args
            .get(0)
            .map(|arg| arg.compute(data))
            .unwrap_or(Value::Null),
        // Normal if/then/else, with default null.
        2..=3 => {
            let condition = args
                .get(0)
                .map(|arg| arg.compute(data))
                .unwrap_or(Value::Null);
            if logic::is_truthy(&condition) {
                args.get(1).map(|arg| arg.compute(data)).unwrap()
            } else {
                args.get(2)
                    .map(|arg| arg.compute(data))
                    .unwrap_or(Value::Null)
            }
        }
        // Now the arguments are pairs of condition and then value. The last argument is the
        // else value.
        // TODO: Actually the logic of this arm computes the other cases properly. Test whether
        // the short circuit cases have a performance benefit or not.
        _ => {
            let mut args = args.iter();

            loop {
                let condition_or_else_val = args
                    .next()
                    .map(|arg| arg.compute(data))
                    .unwrap_or(Value::Null);
                let then_val = args.next();

                match then_val {
                    // The value behind arg1 is the last argument to the if operator, which is
                    // the else argument. Since we come until here, no other (else-)if condition
                    // was truthy. Therefore return the value of arg1.
                    None => return condition_or_else_val,
                    // If the condition (arg1) is truthy, return the then value (arg2).
                    // Otherwise just continue with the next pair.
                    Some(then_val) => {
                        if logic::is_truthy(&condition_or_else_val) {
                            return then_val.compute(data);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn simple() {
        assert_eq!(compute_const!(), Value::Null);
        assert_eq!(compute_const!(Value::Null), Value::Null);
    }

    #[test]
    fn one_arg() {
        assert_eq!(compute_const!(json!(true)), json!(true));
        assert_eq!(compute_const!(json!(false)), json!(false));
        assert_eq!(compute_const!(json!("foo")), json!("foo"));
    }

    #[test]
    fn two_args() {
        assert_eq!(compute_const!(json!(true), json!(5)), json!(5));
        assert_eq!(compute_const!(json!(false), json!(5)), json!(null));
        assert_eq!(compute_const!(json!("foo"), json!("bar")), json!("bar"));
    }

    #[test]
    fn three_args() {
        assert_eq!(compute_const!(json!(true), json!(5), json!(6)), json!(5));
        assert_eq!(compute_const!(json!(false), json!(5), json!(6)), json!(6));
    }

    #[test]
    fn more() {
        assert_eq!(
            compute_const!(json!(false), json!(5), json!(true), json!(6)),
            json!(6)
        );
        assert_eq!(
            compute_const!(json!(false), json!(5), json!(false), json!(6)),
            json!(null)
        );
        assert_eq!(
            compute_const!(json!(false), json!(5), json!(false), json!(6), json!(7)),
            json!(7)
        );
        assert_eq!(
            compute_const!(
                json!(false),
                json!(5),
                json!(false),
                json!(6),
                json!(7),
                json!(8)
            ),
            json!(8)
        );
    }
}
