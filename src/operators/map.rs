use serde_json::Value;

use super::{Data, Expression};

/// You can use `map` to perform an action on every member of an array. Note, that inside the
/// logic being used to map, var operations are relative to the array element being worked on.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let arr = match args.get(0).map(|arg| arg.compute(data)) {
        Some(Value::Array(arr)) => arr,
        _ => Vec::with_capacity(0),
    };
    let op = match args.get(1) {
        Some(expr) => expr,
        None => &Expression::Constant(&Value::Null),
    };

    let mut result = Vec::with_capacity(arr.len());
    for elem in arr.iter() {
        let mapped_value = op.compute(&Data::from_json(elem));
        result.push(mapped_value);
    }

    Value::Array(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn defaults() {
        assert_eq!(compute_const!(), json!([]));
        assert_eq!(
            compute_const!(json!([1, 2, 3, 4, 5])),
            json!([null, null, null, null, null])
        );
    }

    #[test]
    fn complex() -> Result<(), Box<dyn std::error::Error>> {
        let data_json = json!({ "integers": [1, 2, 3, 4, 5] });
        let data = Data::from_json(&data_json);

        assert_eq!(
            compute(
                &[
                    Expression::from_json(&json!({ "var": "integers" }))?,
                    Expression::from_json(&json!({ "*": [{ "var": "" }, 2]}))?,
                ],
                &data,
            ),
            json!([2.0, 4.0, 6.0, 8.0, 10.0])
        );

        assert_eq!(
            compute(
                &[
                    Expression::from_json(&json!({ "var": "integerss" }))?,
                    Expression::from_json(&json!({ "*": [{ "var": "" }, 2]}))?,
                ],
                &data,
            ),
            json!([])
        );

        Ok(())
    }
}
