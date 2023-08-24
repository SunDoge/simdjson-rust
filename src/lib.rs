mod macros;

mod error;
pub mod ondemand;
pub mod padded_string;
pub mod prelude;
pub mod utils;

pub use error::{Result, SimdJsonError};

// pub mod serde;

#[cfg(test)]
mod tests {}
