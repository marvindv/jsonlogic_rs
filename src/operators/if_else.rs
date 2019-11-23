use super::{logic, Data};
use serde_json::Value;

pub fn compute_if(args: &[Value], _: &Data) -> Value {
    match args.len() {
        // Return the condition, for whatever reason.
        0..=1 => args.get(0).cloned().unwrap_or(Value::Null),
        // Normal if/then/else, with default null.
        2..=3 => {
            let condition = args.get(0).unwrap_or(&Value::Null);
            if logic::is_truthy(condition) {
                args.get(1).unwrap().clone()
            } else {
                args.get(2).cloned().unwrap_or(Value::Null)
            }
        }
        // Now the arguments are pairs of condition and then value. The last argument is the
        // else value.
        // TODO: Actually the logic of this arm computes the other cases properly. Test whether
        // the short circuit cases have a performance benefit or not.
        _ => {
            let mut args = args.iter();

            loop {
                let arg1 = args.next();
                let arg2 = args.next();

                match arg2 {
                    // The value behind arg1 is the last argument to the if operator, which is
                    // the else argument. Since we come until here, no other (else-)if condition
                    // was truthy. Therefore return the value of arg1.
                    None => return arg1.cloned().unwrap_or(Value::Null),
                    Some(then) => {
                        let condition = arg1.unwrap_or(&Value::Null);
                        // If the condition (arg1) is truthy, return the then value (arg2).
                        // Otherwise just continue with the next pair.
                        if logic::is_truthy(condition) {
                            return then.clone();
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
    use serde_json::json;

    #[test]
    fn simple() {
        let data = &Data::empty();
        let null = json!(null);

        assert_eq!(compute_if(&[], data), null);
        assert_eq!(compute_if(&[Value::Null], data), null);
    }

    #[test]
    fn one_arg() {
        let data = &Data::empty();

        assert_eq!(compute_if(&[json!(true)], data), json!(true));
        assert_eq!(compute_if(&[json!(false)], data), json!(false));
        assert_eq!(compute_if(&[json!("foo")], data), json!("foo"));
    }

    #[test]
    fn two_args() {
        let data = &Data::empty();

        assert_eq!(compute_if(&[json!(true), json!(5)], data), json!(5));
        assert_eq!(compute_if(&[json!(false), json!(5)], data), json!(null));
        assert_eq!(
            compute_if(&[json!("foo"), json!("bar")], data),
            json!("bar")
        );
    }

    #[test]
    fn three_args() {
        let data = &Data::empty();

        assert_eq!(
            compute_if(&[json!(true), json!(5), json!(6)], data),
            json!(5)
        );
        assert_eq!(
            compute_if(&[json!(false), json!(5), json!(6)], data),
            json!(6)
        );
    }

    #[test]
    fn more() {
        let data = &Data::empty();

        assert_eq!(
            compute_if(&[json!(false), json!(5), json!(true), json!(6)], data),
            json!(6)
        );
        assert_eq!(
            compute_if(&[json!(false), json!(5), json!(false), json!(6)], data),
            json!(null)
        );
        assert_eq!(
            compute_if(
                &[json!(false), json!(5), json!(false), json!(6), json!(7)],
                data
            ),
            json!(7)
        );
        assert_eq!(
            compute_if(
                &[
                    json!(false),
                    json!(5),
                    json!(false),
                    json!(6),
                    json!(7),
                    json!(8)
                ],
                data
            ),
            json!(8)
        );
    }
}