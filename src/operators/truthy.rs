use serde_json::Value;

/// See http://jsonlogic.com/truthy.html
pub fn truthy(value: &Value) -> bool {
    if value.is_number() {
        return value.as_f64().unwrap() != 0f64;
    }

    if value.is_array() {
        return value.as_array().unwrap().len() > 0;
    }

    if value.is_string() {
        let s = value.as_str().unwrap();
        return s != "";
    }

    if value.is_null() {
        return false;
    }

    if value.is_boolean() {
        return value.as_bool().unwrap();
    }

    return true;
}
