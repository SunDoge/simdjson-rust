mod macros;

pub mod builder;
pub mod dom;
mod error;
pub mod ondemand;
pub mod padded_string;
pub mod prelude;
mod utils;

pub use error::{Result, SimdJsonError};
pub use simdjson_sys::{SIMDJSON_MAXSIZE_BYTES, SIMDJSON_PADDING};

#[cfg(feature = "serde_impl")]
pub mod serde;

#[cfg(test)]
mod tests {}
