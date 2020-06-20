// pub mod error;
// pub mod json_ioutil;
// pub mod json_parser;
// pub mod parsed_json;
// pub mod parsed_json_iterator;

// pub use error::SimdJsonError;
// pub use json_parser::{build_parsed_json, json_parse};
// pub use parsed_json::{ParsedJson, DEFUALT_MAX_DEPTH};
// pub use parsed_json_iterator::ParsedJsonIterator;

pub mod dom;
pub mod error;
pub mod implementation;
pub mod libsimdjson;
pub mod padded_string;
// pub mod serde;

use self::implementation::Implementation;
// use std::sync::RwLock;

pub static ACTIVE_IMPLEMENTATION: Implementation = Implementation;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
