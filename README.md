# simdjson-rust

[![Github Actions](https://img.shields.io/github/actions/workflow/status/SunDoge/simdjson-rust/CI.yml?branch=master&style=for-the-badge)](https://github.com/SunDoge/simdjson-rust/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/simdjson-rust?style=for-the-badge)](https://crates.io/crates/simdjson-rust)
[![docs.rs](https://img.shields.io/docsrs/simdjson-rust/latest?style=for-the-badge)](https://docs.rs/simdjson-rust)

This crate currently uses `simdjson 3.2.3`. You can have a try and give feedback.

If you

- find certain APIs are missing
- encounter memory errors
- experience performance degradation

Please submit an issue.

## Usage

Add this to your `Cargo.toml`

```toml
# In the `[dependencies]` section
simdjson-rust = {git = "https://github.com/SunDoge/simdjson-rust"}
```

Then, get started.

```rust
use simdjson_rust::{ondemand::Parser, prelude::*};

fn main() -> simdjson_rust::Result<()> {
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
