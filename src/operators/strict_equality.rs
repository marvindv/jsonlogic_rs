use serde_json::Value;

pub fn compute_strict_equality(arguments: &Value) -> Result<Value, String> {
    // Return true if the argument is null. Return false if the arguments are not are not null or
    // an array. This replicates the behaviour of the javascript implementation.
    if arguments.is_null() {
        return Ok(Value::Bool(true));
    }

    let arr = match arguments.as_array() {
        Some(arr) => arr,
        None => return Ok(Value::Bool(false)),
    };

    // Requires two arguments. More arguments are ignored. Non existing arguments default to null.
    let first = arr.get(0).unwrap_or_else(|| &Value::Null);
    let second = arr.get(1).unwrap_or_else(|| &Value::Null);

    return Ok(Value::Bool(first == second));
}
