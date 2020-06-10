// pub mod error;
// pub mod json_ioutil;
// pub mod json_parser;
// pub mod parsed_json;
// pub mod parsed_json_iterator;

// pub use error::SimdJsonError;
// pub use json_parser::{build_parsed_json, json_parse};
// pub use parsed_json::{ParsedJson, DEFUALT_MAX_DEPTH};
// pub use parsed_json_iterator::ParsedJsonIterator;

#[cxx::bridge(namespace = simdjson::ffi)]
pub mod ffi {
    extern "C" {
        include!("csrc/wrapper.h");
        type parser;

        fn hello();

        fn parser_new(max_capacity: usize) -> UniquePtr<parser>;
    }

}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
