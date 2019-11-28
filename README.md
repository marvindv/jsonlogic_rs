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

## Operator Support

* Accessing Data
    - `var` ✅
    - `missing` ✅
    - `missing_some` ❌ [#4](https://github.com/marvindv/jsonlogic_rs/issues/4)
* Logic and Boolean Operations ✅
    - `if` ✅
    - `==` ✅
    - `===` ✅
    - `!=` ✅
    - `!==` ✅
    - `!` ✅
    - `!!` ✅
    - `or` ✅
    - `and` ✅
* Numeric Operations [#5](https://github.com/marvindv/jsonlogic_rs/issues/5)
    - `>`, `>=`, `<`, and `<=` ✅
    - Between ❌
    - `max` and `min` ❌
    - Arithmetic, `+` `-` `*` `/` ❌
    - `%` ❌
* Array Operations [#6](https://github.com/marvindv/jsonlogic_rs/issues/6)
    - `map`, `reduce` and `filter` ❌
    - `all`, `none` and `some` ❌
    - `merge` ❌
    - `in` ❌
* String Operations [#7](https://github.com/marvindv/jsonlogic_rs/issues/7)
    - `in` ❌
    - `cat` ❌
    - `substr` ❌
* Miscellaneous
    - `log` ❌ [#8](https://github.com/marvindv/jsonlogic_rs/issues/8)