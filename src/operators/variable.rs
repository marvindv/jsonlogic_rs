use serde_json::Value;

use super::Data;

pub fn compute(args: &[Value], data: &Data) -> Value {
    let arg = args.get(0).unwrap_or(&Value::Null);

    if let Value::Null = arg {
        return data.get_plain().clone();
    }

    data.get_value(arg)
        .unwrap_or_else(|| args.get(1).cloned().unwrap_or(Value::Null))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn invalid_arguments() {
        let data_json = json!({ "a": 5, "b": 6 });
        let data = Data::from_json(&data_json);
        assert_eq!(compute(&[json!([])], &data), json!(null));
        assert_eq!(compute(&[json!({})], &data), json!(null));
        assert_eq!(compute(&[json!(true)], &data), json!(null));
        assert_eq!(compute(&[json!(false)], &data), json!(null));
    }

    #[test]
    fn null_arg() {
        let data_json = json!({ "a": 5, "b": 6 });
        let data = Data::from_json(&data_json);
        assert_eq!(compute(&[], &data), data_json);
        assert_eq!(compute(&[Value::Null], &data), data_json);
        assert_eq!(compute(&[Value::Null, json!(123)], &data), data_json);
    }

    #[test]
    fn data_is_object() {
        let data_json = json!({ "a": 5, "b": 6, "1": 1337 });
        let data = Data::from_json(&data_json);
        assert_eq!(
            compute(&[Value::String(String::from("a"))], &data),
            json!(5)
        );
        assert_eq!(
            compute(&[Value::String(String::from("b"))], &data),
            json!(6)
        );
        assert_eq!(compute(&[json!(1)], &data), json!(1337));
    }

    #[test]
    fn data_is_string() {
        let data_json = json!("abcderfg");
        let data = Data::from_json(&data_json);
        assert_eq!(compute(&[json!(1)], &data), json!("b"));
        assert_eq!(compute(&[json!("1")], &data), json!("b"));
    }

    #[test]
    fn data_is_array() {
        let data_json = json!(["foo", "bar"]);
        let data = Data::from_json(&data_json);
        assert_eq!(compute(&[], &data), data_json);
        assert_eq!(compute(&[json!(0)], &data), json!("foo"));
        assert_eq!(compute(&[json!(1)], &data), json!("bar"));
        assert_eq!(compute(&[json!(2)], &data), json!(null));

        assert_eq!(compute(&[json!("1")], &data), json!("bar"));

        let data_json = json!([{"foo": "bar"}]);
        let data = Data::from_json(&data_json);
        assert_eq!(compute(&[json!(0)], &data), json!({"foo": "bar"}));
    }

    #[test]
    fn default_value_array_data() {
        let data_json = json!(["foo", "bar"]);
        let data = Data::from_json(&data_json);

        assert_eq!(compute(&[json!(1), json!("def")], &data), json!("bar"));
        assert_eq!(compute(&[json!(2), json!("def")], &data), json!("def"));
    }

    #[test]
    fn default_value_obj_data() {
        let data_json = json!({"foo": "bar"});
        let data = Data::from_json(&data_json);

        assert_eq!(compute(&[json!("foo"), json!("def")], &data), json!("bar"));
        assert_eq!(
            compute(&[json!("unknown"), json!("def")], &data),
            json!("def")
        );
    }

    #[test]
    fn nested_object() {
        let data_json = json!({ "foo": { "bar": "baz" }});
        let data = Data::from_json(&data_json);

        assert_eq!(compute(&[json!("foo.bar")], &data), json!("baz"));
        assert_eq!(compute(&[json!("foo.bar.baz")], &data), json!(null));
        assert_eq!(compute(&[json!("foo")], &data), json!({ "bar": "baz" }));

        let data_json = json!([{"foo": "bar"}]);
        let data = Data::from_json(&data_json);
        assert_eq!(compute(&[json!("0.foo")], &data), json!("bar"));
        assert_eq!(compute(&[json!("1")], &data), json!(null));
        assert_eq!(compute(&[json!("1.foo")], &data), json!(null));
        assert_eq!(compute(&[json!("0.foo.1")], &data), json!("a"));
        assert_eq!(compute(&[json!("0.foo.1.0")], &data), json!(null));
    }
}
