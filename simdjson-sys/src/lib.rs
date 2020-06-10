#[cxx::bridge(namespace = simdjson::ffi)]
pub mod ffi {
    extern "C" {
        include!("csrc/wrapper.h");
        type parser;
        type element;
        type padded_string;

        fn parser_new(max_capacity: usize) -> UniquePtr<parser>;
        fn parser_load(p: &mut parser, path: &str) -> Result<UniquePtr<element>>;
        fn parser_parse_string(p: &mut parser, s: &str) -> Result<UniquePtr<element>>;
        fn parser_parse_padded_string(p: &mut parser, s: &padded_string) -> Result<UniquePtr<element>>;

        fn padded_string_from_string(s: &str) -> UniquePtr<padded_string>;
    }
}

const SIMDJSON_MAXSIZE_BYTES: usize = 0xFFFFFFFF;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn get_parser() {
        let mut parser = ffi::parser_new(SIMDJSON_MAXSIZE_BYTES);
        let element = ffi::parser_load(&mut parser, "../json-examples/twitter.json").unwrap();
        // dbg!(parser.load);
        let element = ffi::parser_parse_string(&mut parser, "[1]").unwrap();
    }

    #[test]
    fn parse_padded_string() {
        let ps = ffi::padded_string_from_string("[1]");
        let mut parser = ffi::parser_new(SIMDJSON_MAXSIZE_BYTES);
        let element = ffi::parser_parse_padded_string(&mut parser, &ps).unwrap();
    }
}
