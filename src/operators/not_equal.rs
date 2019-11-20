use super::equality::compute_equality;
use serde_json::Value;

pub fn compute_not_equal(args: &Vec<Value>) -> bool {
    !compute_equality(args)
}
