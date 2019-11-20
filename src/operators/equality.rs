use super::logic;
use serde_json::{json, Value};

pub fn compute_equality(args: &Vec<Value>) -> bool {
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
        assert_eq!(compute_equality(&vec![]), true);
        assert_eq!(compute_equality(&vec![json!(null)]), true);
    }

    #[test]
    fn compute() {
        assert_eq!(compute_equality(&vec![json!(null), json!(null)]), true);
        assert_eq!(compute_equality(&vec![json!(1), json!(1)]), true);
        assert_eq!(compute_equality(&vec![json!(1), json!(2)]), false);
    }
}
