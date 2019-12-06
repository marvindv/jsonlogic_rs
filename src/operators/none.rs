use serde_json::Value;

use super::{logic, Data, Expression};

/// Takes an array as the first argument and a condition as the second argument. Returns `true`
/// if the condition evaluates to a falsy value for each element of the first parameter.
///
/// `var` operations inside the second argument expression are relative to the array element
/// being tested.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let arr = match args.get(0).map(|arg| arg.compute(data)) {
        Some(Value::Array(arr)) => arr,
        _ => return Value::Bool(true),
    };
    let condition = match args.get(1) {
        Some(expr) => expr,
        None => return Value::Bool(true),
    };

    for elem in arr.iter() {
        let result = condition.compute(&Data::from_json(&elem));
        if logic::is_truthy(&result) {
            return Value::Bool(false);
        }
    }

    // Condition is truthy for all elements.
    Value::Bool(true)
}
