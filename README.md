# simdjson-rust

[![CI](https://github.com/SunDoge/simdjson-rust/actions/workflows/CI.yml/badge.svg)](https://github.com/SunDoge/simdjson-rust/actions/workflows/CI.yml)

This crate currently uses `simdjson 3.2.3`. You can have a try and give feedback.

## Usage

Add this to your `Cargo.toml`

```toml
# In the `[dependencies]` section
simdjson-rust = {git = "https://github.com/SunDoge/simdjson-rust"}
```

Then, get started.

```rust
use simdjson_rust::{error::Result, ondemand::parser::Parser, padded_string::make_padded_string};

fn main() -> Result<()> {
    let mut parser = Parser::default();
    let ps = make_padded_string("[0,1,2,3]");
    let mut doc = parser.iterate(&ps)?;
    let mut array = doc.get_array()?;
    for (index, value) in array.iter()?.enumerate() {
        assert_eq!(index as u64, value?.get_uint64()?);
    }
    Ok(())
}
```
