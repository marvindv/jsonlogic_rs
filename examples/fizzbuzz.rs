extern crate jsonlogic;
extern crate serde_json;

use std::error::Error;

use serde_json::json;

fn main() -> Result<(), Box<dyn Error>> {
    let rule = json!({
        "if": [
            {"==": [{ "%": [{ "var": "i" }, 15] }, 0]},
            "fizzbuzz",
            {"==": [{ "%": [{ "var": "i" }, 3] }, 0]},
            "fizz",
            {"==": [{ "%": [{ "var": "i" }, 5] }, 0]},
            "buzz",
            { "var": "i" }
        ]
    });

    for i in 1..=30 {
        println!("{}", jsonlogic::apply(&rule, &json!({ "i": i }))?);
    }

    Ok(())
}
