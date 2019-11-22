use serde_json::{Number, Value};

/// See http://jsonlogic.com/truthy.html
pub fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Array(arr) => !arr.is_empty(),
        Value::Bool(b) => *b,
        Value::Null => false,
        Value::Number(num) => num.as_f64().unwrap() != 0f64,
        Value::Object(_) => true,
        Value::String(s) => s != "",
    }
}

// See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Equality_comparisons_and_sameness
pub fn is_loose_equal(a: &Value, b: &Value) -> bool {
    match a {
        Value::Null => b.is_null(),
        Value::Number(a) => match b {
            Value::Null => false,
            Value::Number(b) => a == b,
            Value::String(b) => equal_number_string(a, b),
            Value::Bool(b) => equal_number_boolean(a, *b),
            Value::Array(_) => equal_number_array(a, b),
            Value::Object(_) => false,
        },
        Value::String(a) => match b {
            Value::Null => false,
            Value::Number(b) => equal_number_string(b, a),
            Value::String(b) => a == b,
            Value::Bool(b) => equal_string_boolean(a, *b),
            Value::Array(_) => a == &array_to_str(b),
            Value::Object(_) => false,
        },
        Value::Bool(a) => match b {
            Value::Null => false,
            Value::Number(b) => equal_number_boolean(b, *a),
            Value::String(b) => equal_string_boolean(b, *a),
            Value::Bool(b) => a == b,
            Value::Array(_) => equal_array_bool(b, *a),
            Value::Object(_) => false,
        },
        Value::Array(_) => match b {
            Value::Null => false,
            Value::Number(b) => equal_number_array(b, a),
            Value::String(b) => b == &array_to_str(a),
            Value::Bool(b) => equal_array_bool(a, *b),
            Value::Array(_) => a == b,
            Value::Object(_) => false,
        },
        Value::Object(_) => false,
    }
}

fn equal_array_bool(array_val: &Value, bool_val: bool) -> bool {
    let arr_str = array_to_str(array_val);
    match str_to_u64(&arr_str) {
        // This matches for arrays [1] or [0], ... or [100] of course.
        Some(arr_num) => equal_u64_boolean(arr_num, bool_val),
        // If it is not a number, interpret as a string. Might be [true] or [false].
        None => arr_str == "true",
    }
}

fn equal_number_string(number_val: &Number, str_val: &str) -> bool {
    let num1 = number_val.as_f64().unwrap().to_string();
    let num2 = str_val.trim();

    // case for `0 == ""`
    if num2 == "" {
        num1 == "0"
    } else {
        num1 == num2
    }
}

fn equal_number_boolean(number_val: &Number, bool_val: bool) -> bool {
    let num1 = number_val.as_f64().unwrap();
    if num1.fract() != 0f64 || num1 < 0f64 {
        return false;
    }

    equal_u64_boolean(num1 as u64, bool_val)
}

fn equal_u64_boolean(num1: u64, bool_val: bool) -> bool {
    let num2 = bool_to_u64(bool_val);
    num1 == num2
}

fn equal_string_boolean(string_val: &str, bool_val: bool) -> bool {
    let num2 = bool_to_u64(bool_val);
    match str_to_u64(string_val) {
        Some(num1) => num1 == num2,
        None => false,
    }
}

fn equal_number_array(number_val: &Number, array_val: &Value) -> bool {
    let num1 = number_val.to_string();
    let num2 = array_to_str(array_val);
    num1 == num2
}

fn str_to_u64(s: &str) -> Option<u64> {
    let s = s.trim();
    if s == "" {
        return Some(0);
    }

    s.parse::<u64>().ok()
}

fn bool_to_u64(b: bool) -> u64 {
    if b {
        1
    } else {
        0
    }
}

fn array_to_str(arr: &Value) -> String {
    let s: String = format!("{}", arr);
    // Remove `[` and `]`.
    s[1..s.len() - 1].to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn truthy_values() {
        // See http://jsonlogic.com/truthy.html
        assert_eq!(is_truthy(&json!(0)), false);
        assert_eq!(is_truthy(&json!(-1)), true);
        assert_eq!(is_truthy(&json!(1)), true);
        assert_eq!(is_truthy(&json!([])), false);
        assert_eq!(is_truthy(&json!([1, 2])), true);
        assert_eq!(is_truthy(&json!("")), false);
        assert_eq!(is_truthy(&json!("anything")), true);
        assert_eq!(is_truthy(&json!("0")), true);
        assert_eq!(is_truthy(&json!(["0"])), true);
        assert_eq!(is_truthy(&json!(null)), false);

        assert_eq!(is_truthy(&json!({})), true);
        assert_eq!(is_truthy(&json!(true)), true);
        assert_eq!(is_truthy(&json!(false)), false);
    }

    #[test]
    fn array_as_str() {
        assert_eq!(array_to_str(&json!([1, 2])), "1,2");
    }

    macro_rules! test_loose_equal {
        ($a:expr, $b:expr) => {
            assert_eq!(is_loose_equal(&json!($a), &json!($b)), true);
            assert_eq!(is_loose_equal(&json!($b), &json!($a)), true);
        };
    }

    macro_rules! test_loose_not_equal {
        ($a:expr, $b:expr) => {
            assert_eq!(is_loose_equal(&json!($a), &json!($b)), false);
            assert_eq!(is_loose_equal(&json!($b), &json!($a)), false);
        };
    }

    #[test]
    fn loose_equal_same_type() {
        test_loose_equal!(Value::Null, Value::Null);
        test_loose_equal!(true, true);
        test_loose_equal!(false, false);
        test_loose_equal!("foo", "foo");
        test_loose_equal!(0, 0);
    }

    #[test]
    fn loose_equal_diff_type() {
        test_loose_equal!([1, 2], "1,2");
    }

    #[test]
    fn loose_not_equal() {
        test_loose_not_equal!(0, &Value::Null);
    }

    #[test]
    fn number_boolean() {
        test_loose_equal!(-0, false);
        test_loose_equal!(0, false);

        test_loose_equal!(1, true);
        test_loose_equal!(1.0, true);

        test_loose_not_equal!(-1, true);
        test_loose_not_equal!(0.1 + 0.2, false);
    }

    #[test]
    fn number_string() {
        test_loose_equal!("", 0);
        test_loose_equal!("0", 0);
        test_loose_equal!("17", 17);
        test_loose_equal!("-17", -17);
        test_loose_equal!("   1 ", 1);
        test_loose_equal!("   1.3 ", 1.3);
    }

    #[test]
    fn array_bool() {
        test_loose_equal!([1], true);
        test_loose_equal!([true], true);
    }

    #[test]
    fn string_bool() {
        test_loose_equal!("", false);
        test_loose_equal!("  ", false);
        test_loose_equal!("0", false);
        test_loose_equal!("  0 ", false);
        test_loose_equal!("1", true);
        test_loose_equal!(" 1  ", true);
    }

    #[test]
    fn number_array() {
        test_loose_equal!([1], 1);
        test_loose_equal!([1.2], 1.2);
    }
}
