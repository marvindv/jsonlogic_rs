# jsonlogic_rs &emsp; [![Build Status]][github] [![Latest Version]][crates.io]

[Build Status]: https://github.com/marvindv/jsonlogic_rs/workflows/build/badge.svg?branch=master
[github]: https://github.com/marvindv/jsonlogic_rs
[Latest Version]: https://img.shields.io/crates/v/jsonlogic.svg
[crates.io]: https://crates.io/crates/jsonlogic

**A [JsonLogic](http://jsonlogic.com/) implementation in Rust.**

To use this library, add

```toml
[dependencies]
jsonlogic = "0.5"
```

to your `Cargo.toml`.

## Usage

```rust
use serde_json::{json, Value};

let rule = json!({
    "===": [
        2,
        { "var": "foo" }
    ]
});

let data = json!({ "foo": 2 });
assert_eq!(jsonlogic::apply(&rule, &data), Ok(Value::Bool(true)));

let data = json!({ "foo": 3 });
assert_eq!(jsonlogic::apply(&rule, &data), Ok(Value::Bool(false)));
```

For detailed informations about all supported operations and their arguments, head over to
[Supported Operations](http://jsonlogic.com/operations.html) on
[jsonlogic.com](http://jsonlogic.com/).

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
* Array Operations
    - `map`, `reduce` and `filter` ✅
    - `all`, `none` and `some` ✅
    - `merge` ✅
    - `in` ✅
* String Operations
    - `in` ✅
    - `cat` ✅
    - `substr` ✅
* Miscellaneous
    - `log` ✅
