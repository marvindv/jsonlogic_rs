# jsonlogic_rs

**A [JsonLogic](http://jsonlogic.com/) implementation in Rust.**

To use this library, add

```toml
[dependencies]
jsonlogic = "0.1"
```

to your `Cargo.toml`.

## Usage

```rust
use serde_json::{json, Value};

let rule = json!({"===": [2, {"var": "foo"}]});
assert_eq!(jsonlogic::apply(&rule, &json!({ "foo": 2 })), Ok(Value::Bool(true)));
assert_eq!(jsonlogic::apply(&rule, &json!({ "foo": 3 })), Ok(Value::Bool(false)));
```
