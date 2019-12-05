use serde_json::Value;

use super::{logic, Data, Expression};

/// Takes an arbitrary number of arguments. Returns the first falsy argument or the last
/// argument.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let args = args.iter().map(|arg| arg.compute(data));
    let mut last = None;

    for arg in args {
        if !logic::is_truthy(&arg) {
            return arg;
        }

        last = Some(arg);
    }

    last.unwrap_or(Value::Null)
}
