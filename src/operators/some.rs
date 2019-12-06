use serde_json::Value;

use super::{logic, Data, Expression};

/// Takes an array as the first argument and a condition as the second argument. Returns `true`
/// if the condition evaluates to a truthy value for at least one element of the first
/// parameter.
///
/// `var` operations inside the second argument expression are relative to the array element
/// being tested.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let arr = match args.get(0).map(|arg| arg.compute(data)) {
        Some(Value::Array(arr)) => arr,
        _ => return Value::Bool(false),
    };
    let condition = match args.get(1) {
        Some(expr) => expr,
        None => return Value::Bool(false),
    };

    for elem in arr.iter() {
        let result = condition.compute(&Data::from_json(&elem));
        if logic::is_truthy(&result) {
            return Value::Bool(true);
        }
    }

    // Condition is falsy for all elements.
    Value::Bool(false)
}
