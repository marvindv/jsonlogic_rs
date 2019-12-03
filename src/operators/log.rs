use serde_json::Value;

/// Logs the first value to console, then passes it through unmodified.
pub fn compute(args: &[Value]) -> Value {
    let a = args.get(0).unwrap_or(&Value::Null);

    println!("{}", a);

    a.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test() {
        assert_eq!(compute(&[]), json!(null));
        assert_eq!(compute(&[json!("foo")]), json!("foo"));
        assert_eq!(compute(&[json!("foo"), json!("bar")]), json!("foo"));
        assert_eq!(
            compute(&[json!({"foo": [1, 2, 3]}), json!("bar")]),
            json!({"foo": [1, 2, 3]})
        );
    }
}
