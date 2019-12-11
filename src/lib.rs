// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate serde_json;
extern crate web_sys;

mod data;
mod errors;
mod expression;
mod operators;
mod utils;

use expression::Rule;
use serde_json::Value;
use wasm_bindgen::prelude::*;

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
    let rule = Rule::compile(json_logic.clone())?;
    Ok(rule.apply(data))
}

#[wasm_bindgen(js_name = apply)]
pub fn apply_js(json_logic: &JsValue, data: &JsValue) -> Result<JsValue, JsValue> {
    utils::set_panic_hook();
    let ast = Rule::compile_js(json_logic)?;
    ast.apply_js(data)
}
