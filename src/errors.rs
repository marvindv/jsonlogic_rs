use std::error;
use std::fmt;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub enum JsonLogicError {
    UnknownOperator(String),
    Other(String),
    SerializationError(serde_json::Error),
}

pub type JsonLogicResult<T> = std::result::Result<T, JsonLogicError>;

impl fmt::Display for JsonLogicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonLogicError::UnknownOperator(op) => write!(f, "unknown operator '{}'", op),
            JsonLogicError::Other(s) => write!(f, "{}", s),
            JsonLogicError::SerializationError(err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for JsonLogicError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<serde_json::Error> for JsonLogicError {
    fn from(error: serde_json::Error) -> Self {
        JsonLogicError::SerializationError(error)
    }
}

impl From<JsonLogicError> for JsValue {
    fn from(error: JsonLogicError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

impl From<JsonLogicError> for String {
    fn from(error: JsonLogicError) -> Self {
        error.to_string()
    }
}
