use serde_json::{Number, Value};
use std::convert::TryFrom;

/// Contains a JSON value that is passed as data for the evaluation JsonLogic expression.
pub struct Data<'a>(&'a Value);

impl<'a> Data<'a> {
    /// Creates a new struct from the given json value.
    pub fn from_json(data: &Value) -> Data {
        Data(data)
    }

    /// Creates an empty struct, encapsulating a null value.
    #[allow(dead_code)]
    pub fn empty() -> Data<'static> {
        Data(&Value::Null)
    }

    /// Gets the plain json data that is encapsulated by this struct.
    pub fn get_plain(&self) -> &Value {
        self.0
    }

    /// Tries to get part of the encapsulate data by the given path.
    ///
    /// The path may be either a json string or a number.
    /// The string may contain multiple steps (separated by a dot) to acccess nested values inside
    /// objects, arrays or even characters in a string.
    ///
    /// TODO: Would it be possible to clone only if necessary?
    pub fn get_value(&self, path: &Value) -> Option<Value> {
        match path {
            Value::String(path) => self.by_string(path),
            Value::Number(number) => self.by_number(number),
            _ => None,
        }
    }

    /// Trys to get a value from the given data by the path. This can be a simple key or a
    /// stringified index for strings and arrays but complex dot-notation access paths are also
    /// supported.
    fn by_string(&self, path: &str) -> Option<Value> {
        let mut data_part = self.0;

        // While we can traverse through arrays and objects, we can't for a characters. Character
        // access in a string must therefore be the last step in the given path. To handle that
        // properly, we save an accessed char in this option.
        let mut prev_step_char: Option<char> = None;

        for step in path.split('.') {
            // In the previous step an character from a string was accessed, which must be the last
            // step since a character is considered a primitive here.
            if prev_step_char.is_some() {
                return None;
            }

            // Depending on the current type of data_part, we cast the current step string in the
            // type we need to access the data_part.
            let option = match data_part {
                // If the current data_part is an array, try to interpret the current step as an
                // index.
                Value::Array(arr) => step.parse::<usize>().ok().and_then(|index| arr.get(index)),
                // If the current data_part is an object, interpret current step as a key.
                Value::Object(obj) => obj.get(step),
                // If the current data_part is a string, interpret current step as index of a
                // character. This must be the last step.
                Value::String(s) => {
                    if let Some(ch) = step
                        .parse::<usize>()
                        .ok()
                        .and_then(|index| s.chars().nth(index))
                    {
                        prev_step_char = Some(ch);
                        Some(data_part)
                    } else {
                        // String data_part is not long enough or data_part cannot be parsed into a
                        // number.
                        return None;
                    }
                }
                // All other possible types are primitives and since we have still at least one step
                // to do, the accessor string does not match any value in the given data.
                _ => None,
            };

            if let Some(value) = option {
                data_part = value;
            } else {
                return None;
            }
        }

        // If in the last step a character from a string was accessed, return it.
        if let Some(ch) = prev_step_char {
            return Some(Value::String(ch.to_string()));
        }

        // TODO: Could we avoid cloning?
        Some(data_part.clone())
    }

    /// Extracts a value from the given data by index. Data can either be an array, a string or an
    /// object containing the stringified index as a key. Otherwise returns `None`.
    fn by_number(&self, num: &Number) -> Option<Value> {
        match self.0 {
            Value::Array(arr) => num
                .as_u64()
                .and_then(|index| usize::try_from(index).ok())
                .and_then(|index| arr.get(index).cloned()),
            Value::Object(obj) => obj.get(&num.to_string()).cloned(),
            Value::String(s) => num
                .as_u64()
                .and_then(|index| usize::try_from(index).ok())
                .and_then(|index| s.chars().nth(index))
                .map(|ch| Value::String(ch.to_string())),
            _ => None,
        }
    }
}

// TODO: Move tests from variable operator to this file.
