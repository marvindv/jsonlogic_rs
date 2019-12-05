use serde_json::Value;

use super::{logic, Data, Expression};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let mut result = String::new();

    for arg in args {
        let val = arg.compute(data);
        result.push_str(&logic::coerce_to_str(&val));
    }

    Value::String(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn test() {
        assert_eq!(compute_const!(), json!(""));
        assert_eq!(compute_const!(json!(1), json!(2), json!(3)), json!("123"));
        assert_eq!(compute_const!(json!("foo"), json!("bar")), json!("foobar"));
        assert_eq!(
            compute_const!(json!("foo"), json!([1, 2]), json!("bar")),
            json!("foo1,2bar")
        );
    }
}
