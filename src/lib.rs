extern crate serde_json;

mod data;
mod expression;
mod operators;

use expression::Expression;
use serde_json::Value;
use std::collections::HashSet;

use data::Data;

/// Applies the given JsonLogic rule to the specified data.
/// If the rule does not use any variables, you may pass `&Value::Null` as the second argument.
///
/// # Example
///
/// ```
/// use serde_json::{json, Value};
///
/// let rule = json!({
///     "===": [
///         2,
///         { "var": "foo" }
///     ]
/// });
///
/// let data = json!({ "foo": 2 });
/// assert_eq!(jsonlogic::apply(&rule, &data), Ok(Value::Bool(true)));
///
/// let data = json!({ "foo": 3 });
/// assert_eq!(jsonlogic::apply(&rule, &data), Ok(Value::Bool(false)));
/// ```
pub fn apply(json_logic: &Value, data: &Value) -> Result<Value, String> {
    let ast = Expression::from_json(json_logic.clone())?;
    let data = Data::from_json(data);
    Ok(ast.compute(&data))
}

// TODO: Add to public api when ready.
#[allow(dead_code)]
fn get_variable_names(json_logic: &Value) -> Result<HashSet<String>, String> {
    let ast = expression::Expression::from_json(json_logic.clone())?;
    ast.get_variable_names()
}
