use super::logic;
use serde_json::{json, Value};

/// Double negation, or "cast to a boolean."" Takes a single argument.
pub fn compute_double_negation(args: &Vec<Value>) -> bool {
    let a = args.get(0).unwrap_or_else(|| &json!(null));

    logic::is_truthy(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute() {
        assert_eq!(compute_double_negation(&vec![]), false);
        assert_eq!(compute_double_negation(&vec![json!(null)]), false);
        assert_eq!(compute_double_negation(&vec![json!(false)]), false);
        assert_eq!(compute_double_negation(&vec![json!(true)]), true);
    }
}
