# jsonlogic_rs &emsp; [![Build Status]][github] [![Latest Version]][crates.io]

[Build Status]: https://github.com/marvindv/jsonlogic_rs/workflows/build/badge.svg?branch=master
[github]: https://github.com/marvindv/jsonlogic_rs
[Latest Version]: https://img.shields.io/crates/v/jsonlogic.svg
[crates.io]: https://crates.io/crates/jsonlogic

**A [JsonLogic](http://jsonlogic.com/) implementation in Rust.**

To use this library, add

```toml
[dependencies]
jsonlogic = "0.4"
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
    - `missing_some` ✅
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
* Numeric Operations
    - `>`, `>=`, `<`, and `<=` ✅
    - Between ✅
    - `max` and `min` ✅
    - Arithmetic, `+` `-` `*` `/` ✅
    - `%` ✅
* Array Operations [#6](https://github.com/marvindv/jsonlogic_rs/issues/6)
    - `map`, `reduce` and `filter` ✅
    - `all`, `none` and `some` ❌
    - `merge` ✅
    - `in` ✅
* String Operations
    - `in` ✅
    - `cat` ✅
    - `substr` ✅
* Miscellaneous
    - `log` ✅
