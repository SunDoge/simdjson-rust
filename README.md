# simdjson-rust

[![Build Status](https://travis-ci.org/SunDoge/simdjson-rust.svg?branch=master)](https://travis-ci.org/SunDoge/simdjson-rust)
[![Build status](https://ci.appveyor.com/api/projects/status/xiwngkkjvg9dbvgs?svg=true)](https://ci.appveyor.com/project/SunDoge/simdjson-rust)

⚠ This crate is currently updating to support `simdjson 0.2.0` on dev branch.

## Usage

Add this to your `Cargo.toml`

```toml
# In the `[dependencies]` section
simdjson-rust = {git = "https://github.com/SunDoge/simdjson-rust"}
```

Then, get started.

```rust
use simdjson_rust::{build_parsed_json, ParsedJsonIterator};

fn main() {
    let data = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    let pj = build_parsed_json(data, true);
    assert!(pj.is_valid());

    let mut pjh = pj.iterator();
    assert!(pjh.is_ok());

    assert!(pjh.down());
    assert_eq!(pjh.get_string(), "name");
    assert!(pjh.move_forward());
    assert_eq!(pjh.get_string(), "John Doe");
}
```

## Roadmap

- [x] ParsedJson
- [x] ParsedJsonIterator
- [ ] printjson (impl Display)
- [ ] ci
- [ ] tests
- [ ] benchmark