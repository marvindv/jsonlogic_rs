use serde_json::Value;

pub fn compute(args: &[Value]) -> Value {
    let mut result: Vec<Value> = vec![];

    for arg in args.iter() {
        match arg {
            Value::Array(arr) => result.extend(arr.iter().cloned()),
            _ => result.push(arg.clone()),
        };
    }

    Value::Array(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn merge() {
        assert_eq!(compute(&[]), json!([]));
        assert_eq!(
            compute(&[json!([1, 2]), json!([3, 4])]),
            json!([1, 2, 3, 4])
        );
        assert_eq!(
            compute(&[json!(1), json!(2), json!([3, 4])]),
            json!([1, 2, 3, 4])
        );
    }
}
