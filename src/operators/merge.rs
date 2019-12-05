use serde_json::Value;

use super::{Data, Expression};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let mut result: Vec<Value> = vec![];

    for arg in args {
        let arg = arg.compute(data);
        match arg {
            Value::Array(arr) => result.extend(arr.iter().cloned()),
            _ => result.push(arg),
        };
    }

    Value::Array(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn merge() {
        assert_eq!(compute_const!(), json!([]));
        assert_eq!(
            compute_const!(json!([1, 2]), json!([3, 4])),
            json!([1, 2, 3, 4])
        );
        assert_eq!(
            compute_const!(json!(1), json!(2), json!([3, 4])),
            json!([1, 2, 3, 4])
        );
    }
}
