use serde_json::{json, Value};

use super::{Data, Expression};

/// You can use `reduce` to combine all the elements in an array into a single value, like adding
/// up a list of numbers. Note, that inside the logic being used to reduce, var operations only
/// have access to an object like:
///
/// ```ignore
/// {
///     "current" : // this element of the array,
///     "accumulator" : // progress so far, or the initial value
/// }
/// ```
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let initial = match args.get(2) {
        Some(expr) => expr.compute(data),
        None => Value::Null,
    };
    let arr = match args.get(0).map(|arg| arg.compute(data)) {
        Some(Value::Array(arr)) => arr,
        _ => return initial,
    };
    let reducer = match args.get(1) {
        Some(expr) => expr,
        None => &Expression::Constant(&Value::Null),
    };

    let mut accumulator = initial;
    for current in arr.iter() {
        let reduced_value = reducer.compute(&Data::from_json(
            &json!({ "current": current, "accumulator": accumulator }),
        ));
        accumulator = reduced_value;
    }

    accumulator
}
