//! # simdjson-rust
//!
//! High-performance JSON parsing for Rust, backed by the
//! [simdjson](https://github.com/simdjson/simdjson) C++ library.
//!
//! ## Quick Start
//!
//! ```rust
//! use simdjson_rust::dom::Parser;
//!
//! let mut parser = Parser::default();
//! let value = parser
//!     .parse_to_value(r#"{"hello": "world", "n": 42}"#)
//!     .unwrap();
//! println!("{value:?}");
//! ```
//!
//! ## Serde Support
//!
//! With the default `serde` feature you can deserialize directly into any
//! `serde::Deserialize` type:
//!
//! ```rust
//! # #[cfg(feature = "serde")]
//! # {
//! use serde::Deserialize;
//! use simdjson_rust::serde::from_str;
//!
//! #[derive(Deserialize, Debug)]
//! struct Config {
//!     name: String,
//!     value: u64,
//! }
//!
//! let cfg: Config = from_str(r#"{"name": "example", "value": 99}"#).unwrap();
//! println!("{cfg:?}");
//! # }
//! ```

mod minify;
pub use minify::{minify, minify_bytes};

pub mod dom;
pub mod error;
pub mod tape;

#[cfg(feature = "serde")]
pub mod serde;

/// Re-export the simdjson-sys padding constant.
pub use simdjson_sys::SIMDJSON_PADDING;
