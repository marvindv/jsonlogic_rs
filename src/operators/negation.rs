use super::logic;
use serde_json::{json, Value};

/// Logical negation (“not”). Takes just one argument.
pub fn compute_negation(args: &Vec<Value>) -> bool {
    let a = args.get(0).unwrap_or_else(|| &json!(null));

    !logic::is_truthy(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute() {
        assert_eq!(compute_negation(&vec![]), true);
        assert_eq!(compute_negation(&vec![json!(null)]), true);
        assert_eq!(compute_negation(&vec![json!(false)]), true);
        assert_eq!(compute_negation(&vec![json!(true)]), false);
    }
}
