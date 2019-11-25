use serde_json::Value;

use super::logic;

pub fn compute(args: &[Value]) -> Value {
    let a = match args.get(0) {
        Some(arg) => arg,
        None => return Value::Bool(false),
    };

    let b = match args.get(1) {
        Some(arg) => arg,
        None => return Value::Bool(false),
    };

    Value::Bool(logic::greater_than(a, b))
}
