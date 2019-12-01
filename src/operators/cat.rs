use serde_json::Value;

use super::logic;

pub fn compute(args: &[Value]) -> Value {
    let mut result = String::new();

    for arg in args.iter() {
        result.push_str(&logic::coerce_to_str(arg));
    }

    Value::String(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test() {
        assert_eq!(compute(&[]), json!(""));
        assert_eq!(compute(&[json!(1), json!(2), json!(3)]), json!("123"));
        assert_eq!(compute(&[json!("foo"), json!("bar")]), json!("foobar"));
        assert_eq!(
            compute(&[json!("foo"), json!([1, 2]), json!("bar")]),
            json!("foo1,2bar")
        );
    }
}
