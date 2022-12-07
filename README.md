# simdjson-rust

[![CI](https://github.com/SunDoge/simdjson-rust/actions/workflows/CI.yml/badge.svg)](https://github.com/SunDoge/simdjson-rust/actions/workflows/CI.yml)

This crate currently uses `simdjson 3.0.1`. You can have a try and give feedback.

## Usage

Add this to your `Cargo.toml`

```toml
# In the `[dependencies]` section
simdjson-rust = {git = "https://github.com/SunDoge/simdjson-rust"}
```

Then, get started.

```rust
use simdjson_rust::{error::Result, ondemand::parser::Parser, padded_string::PaddedString};

fn main() -> Result<()> {
    let mut parser = Parser::default();
    let json = PaddedString::load("json-examples/twitter.json")?;
    let mut tweets = parser.iterate(&json)?;
    let v = tweets.at_pointer("/search_metadata/count")?.get_u64()?;
    println! {"{} results.", v};
    Ok(())
}
```

## Roadmap

- [ ] ci
- [ ] `serde`
- [ ] tests
- [ ] benchmark