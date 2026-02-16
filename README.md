# simdjson-rust

[![Github Actions](https://img.shields.io/github/actions/workflow/status/SunDoge/simdjson-rust/CI.yml?branch=master&style=for-the-badge)](https://github.com/SunDoge/simdjson-rust/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/simdjson-rust?style=for-the-badge)](https://crates.io/crates/simdjson-rust)
[![docs.rs](https://img.shields.io/docsrs/simdjson-rust/latest?style=for-the-badge)](https://docs.rs/simdjson-rust)

This crate currently uses [`simdjson 4.2.4`][simdjson]. You can have a try and give feedback.

If you

- find certain APIs are missing
- encounter memory errors
- experience performance degradation

Please submit an issue.

## Benchmark

Check [SunDoge/json-benchmark](https://github.com/SunDoge/json-benchmark/tree/simdjson-rust)

```
                                DOM                  STRUCT
======= serde_json ======= parse|stringify ===== parse|stringify ====
data/canada.json         400 MB/s   460 MB/s   500 MB/s   390 MB/s
data/citm_catalog.json   540 MB/s   790 MB/s  1000 MB/s  1070 MB/s
data/twitter.json        390 MB/s  1020 MB/s   680 MB/s  1090 MB/s

======= simd-json ======== parse|stringify ===== parse|stringify ====
data/canada.json         400 MB/s   540 MB/s   530 MB/s
data/citm_catalog.json  1080 MB/s  1000 MB/s  1570 MB/s
data/twitter.json       1100 MB/s  1460 MB/s  1180 MB/s

===== simdjson-rust ====== parse|stringify ===== parse|stringify ====
data/canada.json         960 MB/s
data/citm_catalog.json  3110 MB/s
data/twitter.json       3160 MB/s
```

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
    let ps = "[0,1,2,3]".to_padded_string();

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

[`simdjson`][simdjson] now offer two kinds of API, `dom` and `ondemand`.
`dom` will parsed the whole string while `ondemand` only parse what you request.
Due to `ffi`, the overhead of `ondemand` API is relatively high. I have tested `lto` but it only improves a little :(

Thus it is suggestted that

- use `ondemand` if you only want to access a specific part of a large json,
- use `dom` if you want to parse the whole json.


### `padded_string`

[`simdjson`][simdjson] requires the input string to be padded. We must provide a string with `capacity = len + SIMDJSON_PADDING`.
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

### Serde Integration

This crate provides optional serde compatibility with SIMD-accelerated JSON serialization and deserialization.

Enable serde support by adding the feature:

```toml
simdjson-rust = { version = "0.4.0-alpha", features = ["serde_impl"] }
```

#### Deserialization (Parsing JSON to Rust structs)

```rust
use serde::Deserialize;
use simdjson_rust::dom::Parser;
use simdjson_rust::serde::de::from_element;

#[derive(Deserialize)]
struct User {
    name: String,
    age: u32,
    active: bool,
}

let mut parser = Parser::default();
let ps = r#"{"name": "Alice", "age": 30, "active": true}"#.to_padded_string();
let elm = parser.parse(&ps)?;
let user: User = from_element(&elm)?;
```

#### Serialization (Rust structs to JSON)

```rust
use serde::Serialize;
use simdjson_rust::serde::ser::to_string;

#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
    active: bool,
}

let user = User {
    name: "Alice".to_string(),
    age: 30,
    active: true,
};

let json = to_string(&user)?;
// Output: {"name":"Alice","age":30,"active":true}
```

## Other interesting things

There are also pure Rust port of [`simdjson`][simdjson] available here [`simd-json`](https://github.com/simd-lite/simd-json).


[simdjson]: https://github.com/simdjson/simdjson