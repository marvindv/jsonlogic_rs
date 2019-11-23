use serde_json::Value;

use super::logic;

/// Takes an arbitrary number of arguments. Returns the first falsy argument or the last
/// argument.
pub fn compute(args: &[Value]) -> Value {
    for arg in args {
        if !logic::is_truthy(arg) {
            return arg.clone();
        }
    }

    args.last().cloned().unwrap_or(Value::Null)
}
