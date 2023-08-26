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
simdjson-rust = "0.3.0"
```

Then, get started.

```rust
use simdjson_rust::prelude::*;
use simdjson_rust::{dom, ondemand};

fn main() -> simdjson_rust::Result<()> {
    let ps = make_padded_string("[0,1,2,3]");

    // ondemand api.
    {
        let mut parser = ondemand::Parser::default();
        let mut doc = parser.iterate(&ps)?;
        let mut array = doc.get_array()?;
        for (index, value) in array.iter()?.enumerate() {
            assert_eq!(index as u64, value?.get_uint64()?);
        }
    }

    // dom api.
    {
        let mut parser = dom::Parser::default();
        let elem = parser.parse(&ps)?;
        let arr = elem.get_array()?;
        for (index, value) in arr.iter().enumerate() {
            assert_eq!(index as u64, value.get_uint64()?);
        }
    }

    Ok(())
}
```

### `dom` and `ondemand` 

`simdjson` now offer two kinds of API, `dom` and `ondemand`.
`dom` will parsed the whole string while `ondemand` only parse what you request.
Due to `ffi`, the overhead of `ondemand` API is relatively high. I have tested `lto` but it only improves a little :(

Thus it is suggestted that

- use `ondemand` if you only want to access a specific part of a large json,
- use `dom` if you want to parse the whole json.


### `padded_string`

`simdjson` requires the input string to be padded. We must provide a string with `capacity = len + SIMDJSON_PADDING`.
We provide utils to do so.

```rust
use simdjson_rust::prelude::*;

fn main() -> simdjson_rust::Result<()> {
    let ps = make_padded_string("[0,1,2,3]");
    let ps = "[0,1,2,3]".to_padded_string();
    // or reuse a buffer.
    let unpadded = String::from("[1,2,3,4]");
    let ps = unpadded.into_padded_string();
    // or load from file.
    let ps = load_padded_string("test.json")?;
    Ok(())
}
```
