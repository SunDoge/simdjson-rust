pub mod error;
pub mod json_parser;
pub mod parsed_json;
pub mod parsed_json_iterator;
pub mod json_ioutil;

pub use error::SimdJsonError;
pub use json_parser::{build_parsed_json, json_parse};
pub use parsed_json::{ParsedJson, DEFUALT_MAX_DEPTH};
pub use parsed_json_iterator::ParsedJsonIterator;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
