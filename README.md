# simdjson-rust

[![Build Status](https://travis-ci.org/SunDoge/simdjson-rust.svg?branch=master)](https://travis-ci.org/SunDoge/simdjson-rust)
[![Build status](https://ci.appveyor.com/api/projects/status/xiwngkkjvg9dbvgs?svg=true)](https://ci.appveyor.com/project/SunDoge/simdjson-rust)

⚠ This crate is currently updating to support `simdjson 0.3.1`! You can have a try and give feedback.

## Usage

Add this to your `Cargo.toml`

```toml
# In the `[dependencies]` section
simdjson-rust = {git = "https://github.com/SunDoge/simdjson-rust"}
```

Then, get started.

```rust
use simdjson_rust::dom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = dom::Parser::default();
    let tweets = parser.load("json-examples/twitter.json")?;
    println!(
        "{} results.",
        tweets
            .at_key("search_metadata")?
            .at_key("count")?
            .get_u64()?
    );
    Ok(())
}
```

## Roadmap

- [x] ParsedJson
- [x] ParsedJsonIterator
- [x] printjson (impl Display)
- [x] ci
- [ ] tests
- [ ] benchmark