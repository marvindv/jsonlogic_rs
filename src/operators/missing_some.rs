use serde_json::Value;

use super::{logic, Data, Expression};

/// Takes a minimum number of data keys that are required, and an array of keys to search for
/// (same format as `var` or `missing`). Returns an empty array if the minimum is met, or an array
/// of the missing keys otherwise.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let mut min_num = args
        .get(0)
        .map(|arg| arg.compute(data))
        .and_then(|arg| logic::coerce_to_f64(&arg))
        .map(|arg| arg.ceil() as u64)
        .unwrap_or(0);

    let keys = match args.get(1).map(|arg| arg.compute(data)) {
        Some(arg) => match arg {
            Value::Array(keys) => keys,
            _ => return Value::Array(vec![]),
        },
        None => return Value::Array(vec![]),
    };

    let mut result: Vec<&Value> = vec![];

    for arg in keys.iter() {
        if min_num < 1 {
            return Value::Array(vec![]);
        }

        if data.get_value(arg).is_some() {
            min_num -= 1;
        } else {
            result.push(arg);
        }
    }

    Value::Array(result.iter().map(|&el| el.clone()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const_with_data;
    use serde_json::json;

    #[test]
    fn test() {
        let data_json = json!({"a": 5, "b": 6});
        let data = Data::from_json(&data_json);

        assert_eq!(compute_const_with_data!(&[], &data), json!([]));
        assert_eq!(compute_const_with_data!(&[json!("a")], &data), json!([]));
        assert_eq!(compute_const_with_data!(&[json!(1)], &data), json!([]));
        assert_eq!(
            compute_const_with_data!(&[json!(1), json!([])], &data),
            json!([])
        );
        assert_eq!(
            compute_const_with_data!(&[json!(0), json!(["a"])], &data),
            json!([])
        );
        assert_eq!(
            compute_const_with_data!(&[json!(1), json!(["a"])], &data),
            json!([])
        );
        assert_eq!(
            compute_const_with_data!(&[json!(1), json!(["c"])], &data),
            json!(["c"])
        );
        assert_eq!(
            compute_const_with_data!(&[json!(2), json!(["a", "b", "c"])], &data),
            json!([])
        );
        assert_eq!(
            compute_const_with_data!(&[json!(2), json!(["a", "c", "d"])], &data),
            json!(["c", "d"])
        );

        assert_eq!(
            compute_const_with_data!(&[json!(1.9), json!(["a", "b", "d", "e"])], &data),
            json!([])
        );
        assert_eq!(
            compute_const_with_data!(&[json!(2), json!(["a", "b", "d", "e"])], &data),
            json!([])
        );
        assert_eq!(
            compute_const_with_data!(&[json!(2.1), json!(["a", "b", "d", "e"])], &data),
            json!(["d", "e"])
        );
    }
}
