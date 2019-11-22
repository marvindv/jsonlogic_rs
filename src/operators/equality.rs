use super::logic;
use serde_json::{json, Value};

pub fn compute_equality(args: &[Value]) -> bool {
    let a = args.get(0).unwrap_or_else(|| &json!(null));
    let b = args.get(1).unwrap_or_else(|| &json!(null));

    logic::is_loose_equal(&a, &b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn default_null() {
        assert_eq!(compute_equality(&[]), true);
        assert_eq!(compute_equality(&[json!(null)]), true);
    }

    #[test]
    fn compute() {
        assert_eq!(compute_equality(&[json!(null), json!(null)]), true);
        assert_eq!(compute_equality(&[json!(1), json!(1)]), true);
        assert_eq!(compute_equality(&[json!(1), json!(2)]), false);
    }
}
