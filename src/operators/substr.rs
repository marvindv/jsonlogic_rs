use serde_json::Value;

use super::{logic, Data, Expression};

/// Gets a portion of a string. Takes two to three arguments.
///
/// The first argument is a string. Any other value will be coerced into a string.
///
/// The second argument is a number (or will be coerced to a number, default to 0) and is the
/// start position to return everything beginning at that index. Give a negative start position
/// to work from the end of the string and then return the rest.
///
/// The third argument limits the length of the returned substring. Give a negative index to
/// stop that many characters before the end.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = match args.get(0).map(|arg| arg.compute(data)) {
        Some(val) => logic::coerce_to_str(&val),
        // Replicates specifics of the javascript implementation.
        None => String::from("undefined"),
    };
    let b = args
        .get(1)
        .map(|arg| arg.compute(data))
        .and_then(|val| logic::coerce_to_f64(&val))
        .map(|f| f as i64)
        .unwrap_or(0);
    let c = args
        .get(2)
        .map(|arg| arg.compute(data))
        .and_then(|val| logic::coerce_to_f64(&val))
        .map(|f| f as i64);

    let len = a.len() as i64;
    let start = if b >= 0 {
        b
    } else {
        // Avoid a negative start index.
        std::cmp::max(len + b, 0)
    };
    let iter = a.chars().skip(start as usize);
    let s = match c {
        Some(c) => {
            let limit = if c >= 0 {
                c
            } else {
                // Avoid a negative limit. We must stop at c bytes before the end.
                let len_after_start = len - start;
                std::cmp::max(len_after_start - c.abs(), 0)
            };

            iter.take(limit as usize).collect()
        }
        None => iter.collect(),
    };

    Value::String(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn basic() {
        assert_eq!(compute_const!(), json!("undefined"));
        assert_eq!(compute_const!(json!(true)), json!("true"));
        assert_eq!(compute_const!(json!("jsonlogic")), json!("jsonlogic"));
        assert_eq!(
            compute_const!(json!("jsonlogic"), json!(0)),
            json!("jsonlogic")
        );
        assert_eq!(compute_const!(json!("jsonlogic"), json!(4)), json!("logic"));
        assert_eq!(
            compute_const!(json!("jsonlogic"), json!(-5)),
            json!("logic")
        );
        assert_eq!(
            compute_const!(json!("jsonlogic"), json!(-50)),
            json!("jsonlogic")
        );

        // Positive limit
        assert_eq!(compute_const!(json!("y̆"), json!(0), json!(1)), json!("y"));
        assert_eq!(
            compute_const!(json!("hallo"), json!(2), json!(2)),
            json!("ll")
        );
        assert_eq!(compute_const!(json!("äüö"), json!(1), json!(1)), json!("ü"));

        // Negative limit c, stop at c characters (i.e. bytes) from the end.
        assert_eq!(
            compute_const!(json!("jsonlogic"), json!(4), json!(-2)),
            json!("log")
        );
        assert_eq!(
            compute_const!(json!("jsonlogic"), json!(4), json!(-3)),
            json!("lo")
        );
        assert_eq!(
            compute_const!(json!("jsonlogic"), json!(4), json!(-4)),
            json!("l")
        );
        assert_eq!(
            compute_const!(json!("jsonlogic"), json!(4), json!(-5)),
            json!("")
        );
        assert_eq!(
            compute_const!(json!("jsonlogic"), json!(4), json!(-6)),
            json!("")
        );

        assert_eq!(
            compute_const!(json!("jsonlogic"), json!(3), json!(-2)),
            json!("nlog")
        );

        // If c is negative and abs(c) > len, string must be empty.
        assert_eq!(
            compute_const!(json!("jsonlogic"), json!(4), json!(-20)),
            json!("")
        );
    }
}
