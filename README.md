# simdjson-rust

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

    let mut iter = ParsedJsonIterator::new(&pj);
    assert!(iter.is_ok());

    assert!(iter.down());
    assert_eq!(iter.get_string(), "name");
    assert!(iter.move_forward());
    assert_eq!(iter.get_string(), "John Doe");
}
```

## Roadmap

- [x] ParsedJson
- [x] ParsedJsonIterator
- [ ] printjson (impl Display)
- [ ] ci
- [ ] tests
- [ ] benchmark