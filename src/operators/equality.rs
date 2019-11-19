use serde_json::Value;

pub fn compute_equality(arguments: &Value) -> Result<Value, String> {
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
    // TODO: We are the type annotations needed for code completion.
    let first: &Value = arr.get(0).unwrap_or_else(|| &Value::Null);
    let second: &Value = arr.get(1).unwrap_or_else(|| &Value::Null);

    return Ok(Value::Bool(is_equal(first, second)));
}

// See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Equality_comparisons_and_sameness
pub fn is_equal(a: &Value, b: &Value) -> bool {
    if a.is_null() {
        b.is_null()
    } else if a.is_number() {
        if b.is_null() {
            false
        } else if b.is_number() {
            a.as_f64().unwrap() == b.as_f64().unwrap()
        } else if b.is_string() {
            equal_number_string(a, b)
        } else if b.is_boolean() {
            equal_number_boolean(a, b)
        } else if b.is_array() {
            equal_number_array(a, b)
        } else if b.is_object() {
            false
        } else {
            unreachable!()
        }
    } else if a.is_string() {
        if b.is_null() {
            false
        } else if b.is_number() {
            equal_number_string(b, a)
        } else if b.is_string() {
            a == b
        } else if b.is_boolean() {
            equal_string_boolean(a, b)
        } else if b.is_array() {
            a.as_str().unwrap() == array_to_str(b)
        } else if b.is_object() {
            false
        } else {
            unreachable!()
        }
    } else if a.is_boolean() {
        if b.is_null() {
            false
        } else if b.is_number() {
            bool_to_number(a.as_bool().unwrap()) == b.as_f64().unwrap()
        } else if b.is_string() {
            equal_string_boolean(b, a)
        } else if b.is_boolean() {
            a == b
        } else if b.is_array() {
            false
        } else if b.is_object() {
            false
        } else {
            unreachable!()
        }
    } else if a.is_array() {
        if b.is_null() {
            false
        } else if b.is_number() {
            equal_number_array(b, a)
        } else if b.is_string() {
            &array_to_str(a) == b.as_str().unwrap()
        } else if b.is_boolean() {
            false
        } else if b.is_array() {
            a == b
        } else if b.is_object() {
            false
        } else {
            unreachable!()
        }
    } else if a.is_object() {
        false
    } else {
        unreachable!()
    }
}

fn equal_number_string(number_val: &Value, str_val: &Value) -> bool {
    let num1 = number_val.as_f64().unwrap();
    match str_to_number(str_val.as_str().unwrap()) {
        Some(num2) => num1 == num2,
        None => false,
    }
}

fn equal_number_boolean(number_val: &Value, bool_val: &Value) -> bool {
    let num1 = number_val.as_f64().unwrap();
    let num2 = bool_to_number(bool_val.as_bool().unwrap());
    num1 == num2
}

fn equal_string_boolean(string_val: &Value, bool_val: &Value) -> bool {
    let num2 = bool_to_number(bool_val.as_bool().unwrap());
    match str_to_number(string_val.as_str().unwrap()) {
        Some(num1) => num1 == num2,
        None => false,
    }
}

fn equal_number_array(number_val: &Value, array_val: &Value) -> bool {
    let num1 = number_val.as_f64().unwrap();
    match str_to_number(&array_to_str(array_val)) {
        Some(num2) => num1 == num2,
        None => false,
    }
}

fn str_to_number(s: &str) -> Option<f64> {
    let s = s.trim();
    if s == "" {
        return Some(0f64);
    }
    match s.parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

fn bool_to_number(b: bool) -> f64 {
    if b {
        1f64
    } else {
        0f64
    }
}

fn array_to_str(arr: &Value) -> String {
    //let arr = arr.as_array().unwrap();
    let s: String = format!("{}", arr);
    // Remove `[` and `]`.
    s[1..s.len() - 1].to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn array_as_str() {
        assert_eq!(array_to_str(&json!([1, 2])), "1,2");
    }

    #[test]
    fn equal() {
        // TODO: test for each is_equal(a,b) <=> is_equal(b,a)
        let null = Value::Null;
        assert_eq!(is_equal(&null, &null), true);
        assert_eq!(is_equal(&json!(true), &json!(true)), true);
        assert_eq!(is_equal(&json!(false), &json!(false)), true);
        assert_eq!(is_equal(&json!("foo"), &json!("foo")), true);
        assert_eq!(is_equal(&json!(0), &json!(0)), true);
        assert_eq!(is_equal(&json!(0), &json!(false)), true);
        assert_eq!(is_equal(&json!(""), &json!(false)), true);
        assert_eq!(is_equal(&json!(""), &json!(0)), true);
        assert_eq!(is_equal(&json!("0"), &json!(0)), true);
        assert_eq!(is_equal(&json!("17"), &json!(17)), true);
        assert_eq!(is_equal(&json!([1, 2]), &json!("1,2")), true);
        assert_eq!(is_equal(&json!([1]), &json!(1)), true);
        assert_eq!(is_equal(&json!(0), &Value::Null), false);
        // TODO:
        //assert_eq!(is_equal(&json!([1]), &json!(true)), true);
        //assert_eq!(is_equal(&json!([true]), &json!(true)), true);
    }
}
