use serde_json::Value;

use super::{Data, Expression};

/// Logs the first value to console, then passes it through unmodified.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = args
        .get(0)
        .map(|arg| arg.compute(data))
        .unwrap_or(Value::Null);

    println!("{}", a);

    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn test() {
        assert_eq!(compute_const!(), json!(null));
        assert_eq!(compute_const!(json!("foo")), json!("foo"));
        assert_eq!(compute_const!(json!("foo"), json!("bar")), json!("foo"));
        assert_eq!(
            compute_const!(json!({"foo": [1, 2, 3]}), json!("bar")),
            json!({"foo": [1, 2, 3]})
        );
    }
}
