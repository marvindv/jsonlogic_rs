use super::strict_equality::compute_strict_equality;
use serde_json::Value;

pub fn compute_strict_not_equal(args: &Vec<Value>) -> bool {
    !compute_strict_equality(args)
}
