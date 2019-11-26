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

/// See https://www.ecma-international.org/ecma-262/#sec-strict-equality-comparison
pub fn is_strict_equal(a: &Value, b: &Value) -> bool {
    use Value::*;

    match (a, b) {
        (Array(_), Array(_)) => false,
        (Bool(a), Bool(b)) => a == b,
        (Null, Null) => true,
        (Number(a), Number(b)) => equal_numbers(a, b),
        (Object(_), Object(_)) => false,
        (String(a), String(b)) => a == b,
        _ => false,
    }
}

// See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Equality_comparisons_and_sameness
// and https://www.ecma-international.org/ecma-262/#sec-abstract-equality-comparison
#[allow(clippy::float_cmp)]
pub fn is_abstract_equal(a: &Value, b: &Value) -> bool {
    use Value::*;

    match (a, b) {
        // 1. Strict equal for same types.
        (Array(_), Array(_))
        | (Bool(_), Bool(_))
        | (Null, Null)
        | (Number(_), Number(_))
        | (Object(_), Object(_))
        | (String(_), String(_)) => is_strict_equal(a, b),
        // short-circuit only one operand being null
        (Null, _) | (_, Null) => false,
        // 4. If Type(a) is number and Type(b) is string, return a == ToNumber(b).
        (Number(a), String(_)) => coerce_to_f64(b)
            .map(|b| a.as_f64().unwrap() == b)
            .unwrap_or(false),
        // 5. If Type(a) is string and Type(b) is number, return ToNumber(a) == b.
        (String(_), Number(b)) => coerce_to_f64(a)
            .map(|a| a == b.as_f64().unwrap())
            .unwrap_or(false),
        // 6. If Type(a) is bool return ToNumber(a)==b
        (Bool(_), _) => coerce_to_f64(a)
            .map(|a| is_abstract_equal(&Value::Number(serde_json::Number::from_f64(a).unwrap()), b))
            .unwrap_or(false),
        // 7. If Type(b) is bool return a==ToNumber(b)
        (_, Bool(_)) => coerce_to_f64(b)
            .map(|b| is_abstract_equal(a, &Value::Number(serde_json::Number::from_f64(b).unwrap())))
            .unwrap_or(false),
        // 8. something with object
        // if non array object:
        //   An object is never equal to something else, including another object, since
        //   ToPrimitive(object) does not work for json.
        (Object(_), _) | (_, Object(_)) => false,
        // if array:
        //   the only possible operand types that are still possible are Number and String
        (String(a), Array(b)) | (Array(b), String(a)) => a == &arr_to_primitive_str(b),
        (Number(_), Array(b)) => is_abstract_equal(a, &Value::String(arr_to_primitive_str(b))),
        (Array(a), Number(_)) => is_abstract_equal(&Value::String(arr_to_primitive_str(a)), b),
    }
}

// See https://www.ecma-international.org/ecma-262/#sec-abstract-relational-comparison
pub fn less_than(a: &Value, b: &Value) -> bool {
    use Value::*;

    match (a, b) {
        (Null, Null) => false,
        (Bool(false), Bool(true)) => true,
        (Bool(_), Bool(_)) => false,
        (Object(_), _) | (_, Object(_)) => false,
        (String(a), String(b)) => a < b,
        // Combinations where both operands will be coerced to strings:
        //   Arrays will be converted to a primitive (i.e. a string). (1.)
        //   Strings will be compared lexically. (3.)
        (Array(_), Array(_)) | (Array(_), String(_)) | (String(_), Array(_)) => {
            coerce_to_str(a) < coerce_to_str(b)
        }
        // Combinations where both operands will be coerced to numbers:
        //   In every other combination the operands will be converted to numbers in the end. (4.)
        (Null, _) | (_, Null) | (Number(_), _) | (_, Number(_)) | (Bool(_), _) | (_, Bool(_)) => {
            match (coerce_to_f64(a), coerce_to_f64(b)) {
                (Some(a), Some(b)) => a < b,
                _ => false,
            }
        }
    }
}

pub fn less_equal_than(a: &Value, b: &Value) -> bool {
    less_than(a, b) || is_abstract_equal(a, b)
}

pub fn greater_than(a: &Value, b: &Value) -> bool {
    !less_equal_than(a, b)
}

pub fn greater_equal_than(a: &Value, b: &Value) -> bool {
    !less_than(a, b)
}

#[allow(clippy::float_cmp)]
fn equal_numbers(a: &Number, b: &Number) -> bool {
    // Avoid float compare if possible.
    if a.is_u64() && b.is_u64() {
        a.as_u64().unwrap() == b.as_u64().unwrap()
    } else if a.is_i64() && b.is_i64() {
        a.as_i64().unwrap() == b.as_i64().unwrap()
    } else {
        a.as_f64().unwrap() == b.as_f64().unwrap()
    }
}

/// The javascript operation `String(val)`.
fn coerce_to_str(val: &Value) -> String {
    match val {
        Value::Array(arr) => arr_to_primitive_str(arr),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::from("null"),
        Value::Number(num) => num.to_string(),
        Value::Object(_) => String::from("[object Object]"),
        Value::String(s) => s.to_string(),
    }
}

/// `Number(val)` in javascript or as named in the standard `ToNumber(val)`.
fn coerce_to_f64(val: &Value) -> Option<f64> {
    match val {
        Value::Array(arr) => match &arr[..] {
            [] => Some(0f64),
            // I don't really understand why Number([true]) is NaN but thats the way it is.
            [el] => match el {
                Value::Array(_) | Value::Null | Value::Number(_) | Value::String(_) => {
                    coerce_to_f64(el)
                }
                _ => None,
            },
            _ => None,
        },
        Value::Bool(true) => Some(1f64),
        Value::Bool(false) => Some(0f64),
        Value::Null => Some(0f64),
        Value::Number(num) => num.as_f64(),
        Value::Object(_) => None,
        Value::String(s) => {
            let s = s.trim();
            if s == "" {
                Some(0f64)
            } else {
                s.parse::<f64>().ok()
            }
        }
    }
}

// In the end this is 7.1.1.1 OrdinaryToPrimitive for array(-objects)
// from https://www.ecma-international.org/ecma-262/#sec-ordinarytoprimitive with hint=number
// but in the end it boils down to arr.toString()
fn arr_to_primitive_str(arr: &[Value]) -> String {
    arr.iter()
        .map(|el| coerce_to_str(el))
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    mod strict_equal {
        use super::*;

        macro_rules! test_strict_equal {
            ($a:expr, $b:expr) => {
                assert!(is_strict_equal(&json!($a), &json!($b)));
                assert!(is_strict_equal(&json!($b), &json!($a)));
            };
        }

        macro_rules! test_strict_not_equal {
            ($a:expr, $b:expr) => {
                assert!(!is_strict_equal(&json!($a), &json!($b)));
                assert!(!is_strict_equal(&json!($b), &json!($a)));
            };
        }

        #[test]
        fn same_type_string() {
            test_strict_equal!("", "");
            test_strict_not_equal!(" ", "");
            test_strict_equal!("a", "a");
            test_strict_not_equal!("a", "b");
        }

        #[test]
        fn same_type_number() {
            test_strict_equal!(0, 0);
            test_strict_equal!(0, 0.0);
            test_strict_equal!(-0, 0);
            test_strict_not_equal!(-1, 1);
            test_strict_not_equal!(1.1, 1);
        }

        #[test]
        fn same_type_bool() {
            test_strict_equal!(true, true);
            test_strict_equal!(false, false);
            test_strict_not_equal!(false, true);
        }

        #[test]
        fn same_type_object() {
            assert!(!is_strict_equal(&json!({}), &json!({}),));
            assert!(!is_strict_equal(
                &json!({"foo": "bar"}),
                &json!({"foo": "bar"}),
            ));
            assert!(!is_strict_equal(
                &json!({"foo": "bar"}),
                &json!({"foo":  1}),
            ));
        }

        #[test]
        fn same_type_array() {
            assert!(!is_strict_equal(&json!([]), &json!([])));
            assert!(!is_strict_equal(&json!([1]), &json!([1])));
            assert!(!is_strict_equal(&json!([1]), &json!([2])));
        }

        #[test]
        fn different_type() {
            test_strict_not_equal!("", 0);
            test_strict_not_equal!("1", 1);
            test_strict_not_equal!("true", true);
            test_strict_not_equal!(1, true);
            assert!(!is_strict_equal(&json!(null), &json!(false)));
        }
    }

    mod abstract_equal {
        use super::*;

        macro_rules! test_abstract_equal {
            ($a:expr, $b:expr) => {
                assert!(is_abstract_equal(&json!($a), &json!($b)));
                assert!(is_abstract_equal(&json!($b), &json!($a)));
            };
        }
        macro_rules! test_abstract_not_equal {
            ($a:expr, $b:expr) => {
                assert!(!is_abstract_equal(&json!($a), &json!($b)),);
                assert!(!is_abstract_equal(&json!($b), &json!($a)),);
            };
        }
        #[test]
        fn loose_equal_same_type() {
            test_abstract_equal!(Value::Null, Value::Null);
            test_abstract_equal!(true, true);
            test_abstract_equal!(false, false);
            test_abstract_equal!("foo", "foo");
            test_abstract_equal!(0, 0);
            test_abstract_equal!(0, -0);
            test_abstract_equal!(0, 0.0);
            test_abstract_equal!(0.2, 0.2);
        }
        #[test]
        fn loose_equal_diff_type() {
            test_abstract_equal!([1, 2], "1,2");
        }
        #[test]
        fn loose_not_equal() {
            test_abstract_not_equal!(0, &Value::Null);
        }
        #[test]
        fn number_boolean() {
            test_abstract_equal!(-0, false);
            test_abstract_equal!(0, false);
            test_abstract_equal!(1, true);
            test_abstract_equal!(1.0, true);
            test_abstract_not_equal!(-1, true);
            test_abstract_not_equal!(0.1 + 0.2, false);
        }
        #[test]
        fn number_string() {
            test_abstract_equal!("", 0);
            test_abstract_equal!("0", 0);
            test_abstract_equal!("-0", 0);
            test_abstract_equal!("+0", 0);
            test_abstract_equal!("0.0", 0);
            test_abstract_equal!("+0.0", 0);
            test_abstract_equal!("-0.0", 0);
            test_abstract_equal!("17", 17);
            test_abstract_equal!("-17", -17);
            test_abstract_equal!("   1 ", 1);
            test_abstract_equal!("   1.3 ", 1.3);
        }
        #[test]
        fn array_bool() {
            test_abstract_equal!([1], true);
            test_abstract_not_equal!([true], true);
        }
        #[test]
        fn string_bool() {
            test_abstract_equal!("", false);
            test_abstract_equal!("  ", false);
            test_abstract_equal!("0", false);
            test_abstract_equal!("  0 ", false);
            test_abstract_equal!("1", true);
            test_abstract_equal!(" 1  ", true);
        }
        #[test]
        fn number_array() {
            test_abstract_equal!([1], 1);
            test_abstract_equal!([1.2], 1.2);
        }
    }

    mod test_less_than {
        use super::*;

        macro_rules! less_than {
            ($a:expr, $b:expr, $result:expr) => {
                assert_eq!(less_than(&json!($a), &json!($b)), $result);
            };
        }

        #[test]
        fn same_type() {
            // number < number
            assert_eq!(less_than(&json!(1), &json!(2)), true);
            assert_eq!(less_than(&json!(2), &json!(2)), false);
            assert_eq!(less_than(&json!(3), &json!(2)), false);

            // string < string
            assert_eq!(less_than(&json!("a"), &json!("b")), true);
            assert_eq!(less_than(&json!("b"), &json!("b")), false);
            assert_eq!(less_than(&json!("c"), &json!("b")), false);

            // null < null
            assert_eq!(less_than(&json!(null), &json!(null)), false);

            // bool < bool
            assert_eq!(less_than(&json!(false), &json!(true)), true);
            assert_eq!(less_than(&json!(true), &json!(false)), false);
            assert_eq!(less_than(&json!(true), &json!(true)), false);
            assert_eq!(less_than(&json!(false), &json!(false)), false);
        }

        #[test]
        fn number_string() {
            // number < string, string is casted to number
            assert_eq!(less_than(&json!(1), &json!("b")), false);
            assert_eq!(less_than(&json!(1), &json!("1")), false);
            assert_eq!(less_than(&json!(-1), &json!("")), true);
            assert_eq!(less_than(&json!(1), &json!("12")), true);

            // string < number, string is casted to number
            assert_eq!(less_than(&json!("b"), &json!(1)), false);
            assert_eq!(less_than(&json!("1"), &json!(1)), false);
            assert_eq!(less_than(&json!(""), &json!(-1)), false);
            assert_eq!(less_than(&json!("12"), &json!(1)), false);
        }

        #[test]
        fn array_number() {
            // array < number, cast array to number
            assert_eq!(less_than(&json!([1]), &json!(12)), true);
            assert_eq!(less_than(&json!([2]), &json!(12)), true);
            assert_eq!(less_than(&json!([[2]]), &json!(12)), true);
            assert_eq!(less_than(&json!([[2], 3]), &json!(12)), false);

            // number < array, cast array to number
            assert_eq!(less_than(&json!(1), &json!([12])), true);
            assert_eq!(less_than(&json!(2), &json!([12])), true);
            assert_eq!(less_than(&json!(2), &json!([[12]])), true);
            assert_eq!(less_than(&json!(2), &json!([10, [12]])), false);
        }

        #[test]
        fn multi_elem_arrays() {
            // Multiple element arrays are converted to string and lexicographically compared.
            assert_eq!(less_than(&json!([1, 2]), &json!([3, 4])), true);
            assert_eq!(less_than(&json!([3, 4]), &json!([1, 2])), false);
            assert_eq!(less_than(&json!([1, 2, 2]), &json!([2, 2])), true);
        }

        #[test]
        fn bool_number() {
            // bool < number, bool is converted to number
            assert_eq!(less_than(&json!(false), &json!(1)), true);
            assert_eq!(less_than(&json!(true), &json!(1)), false);
            assert_eq!(less_than(&json!(true), &json!(2)), true);

            // number < bool, bool is converted to number
            assert_eq!(less_than(&json!(-1), &json!(false)), true);
            assert_eq!(less_than(&json!(1), &json!(true)), false);
            assert_eq!(less_than(&json!(0), &json!(true)), true);
        }

        #[test]
        fn bool_string() {
            // bool < string, bool is converted to number, string is converted to number
            assert_eq!(less_than(&json!(false), &json!("1")), true);
            assert_eq!(less_than(&json!(true), &json!("1")), false);
            assert_eq!(less_than(&json!(true), &json!("2")), true);
            assert_eq!(less_than(&json!(true), &json!("foo")), false);

            // string < bool, bool is converted to number, string is converted to number
            assert_eq!(less_than(&json!("-1"), &json!(false)), true);
            assert_eq!(less_than(&json!("1"), &json!(true)), false);
            assert_eq!(less_than(&json!("0"), &json!(true)), true);
            assert_eq!(less_than(&json!("foo"), &json!(true)), false);
        }

        #[test]
        fn bool_array() {
            less_than!(false, [true], false);
            less_than!(false, [false], false);
            less_than!(false, [0], false);
            less_than!(false, [1], true);
            less_than!(false, [1, 2], false);
            less_than!(true, [true], false);
            less_than!(true, [false], false);
            less_than!(true, [0], false);
            less_than!(true, [1], false);
            less_than!(true, [2], true);
            less_than!(true, [2, 3], false);
        }

        #[test]
        fn string_array() {
            assert_eq!(less_than(&json!([1]), &json!("12")), true);
            assert_eq!(less_than(&json!([2]), &json!("12")), false);
        }

        #[test]
        fn with_null() {
            // null < *, * is converted to number, null is treated as 0
            macro_rules! null_less_than {
                ($a:expr, $b:expr) => {
                    assert_eq!(less_than(&json!(null), &json!($a)), $b);
                };
            }

            macro_rules! is_less_than_null {
                ($a:expr, $b:expr) => {
                    assert_eq!(less_than(&json!($a), &json!(null)), $b);
                };
            }

            null_less_than!(1, true);
            null_less_than!("5", true);
            null_less_than!(true, true);

            null_less_than!({}, false);
            null_less_than!([-5], false);
            null_less_than!(["-5"], false);
            null_less_than!([5], true);
            null_less_than!(["5"], true);

            is_less_than_null!(-1, true);
            is_less_than_null!(1, false);
            is_less_than_null!("-1", true);
            is_less_than_null!("1", false);

            is_less_than_null!({}, false);
            is_less_than_null!([-5], true);
            is_less_than_null!(["-5"], true);
            is_less_than_null!([5], false);
            is_less_than_null!(["5"], false);
        }
    }

    #[test]
    fn test_less_equal_than() {
        assert_eq!(less_equal_than(&json!(1), &json!(1)), true);
        assert_eq!(less_equal_than(&json!([1]), &json!("1")), true);
        assert_eq!(less_equal_than(&json!([1]), &json!("12")), true);

        assert_eq!(less_equal_than(&json!(2), &json!(1)), false);
        assert_eq!(less_equal_than(&json!([2]), &json!("12")), false);
    }

    #[test]
    fn test_greater_than() {
        assert_eq!(greater_than(&json!(2), &json!(1)), true);
        assert_eq!(greater_than(&json!(2), &json!(2)), false);
        assert_eq!(greater_than(&json!(2), &json!(3)), false);
    }

    #[test]
    fn test_greater_equal_than() {
        assert_eq!(greater_equal_than(&json!(2), &json!(1)), true);
        assert_eq!(greater_equal_than(&json!(2), &json!(2)), true);
        assert_eq!(greater_equal_than(&json!(2), &json!(3)), false);
    }

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
    fn value_as_string_coercion() {
        assert_eq!(coerce_to_str(&json!(true)), "true");
        assert_eq!(coerce_to_str(&json!(false)), "false");
        assert_eq!(coerce_to_str(&json!([false])), "false");
        assert_eq!(coerce_to_str(&json!([true])), "true");
        assert_eq!(coerce_to_str(&json!(null)), "null");
        assert_eq!(coerce_to_str(&json!({})), "[object Object]");

        assert_eq!(coerce_to_str(&json!([1, 2])), "1,2");
        // String([[1,2], [3, 4]]) === "1,2,3,4"
        assert_eq!(coerce_to_str(&json!([[1, 2], [3, 4]])), "1,2,3,4");
        // String([[1,2], [[true, 4]], 5]) === '1,2,true,4,5'
        assert_eq!(
            coerce_to_str(&json!([[1, 2], [[true, 4]], 5])),
            "1,2,true,4,5"
        );
    }

    #[test]
    fn value_as_f64_coercion() {
        assert_eq!(coerce_to_f64(&json!([[[5]]])), Some(5f64));
        assert_eq!(coerce_to_f64(&json!([[[5], 6]])), None);
        assert_eq!(coerce_to_f64(&json!([[[1, 2]]])), None);
        assert_eq!(coerce_to_f64(&json!(true)), Some(1f64));
        assert_eq!(coerce_to_f64(&json!([true])), None);
        assert_eq!(coerce_to_f64(&json!(false)), Some(0f64));
        assert_eq!(coerce_to_f64(&json!([false])), None);
        assert_eq!(coerce_to_f64(&json!("1")), Some(1f64));
        assert_eq!(coerce_to_f64(&json!(["1"])), Some(1f64));
        assert_eq!(coerce_to_f64(&json!(null)), Some(0f64));
        assert_eq!(coerce_to_f64(&json!([null])), Some(0f64));
    }
}
