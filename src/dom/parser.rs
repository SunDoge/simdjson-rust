use super::element::Element;
use crate::error::SimdJsonError;
use crate::libsimdjson::{ffi, SIMDJSON_MAXSIZE_BYTES};
use std::path::Path;

pub struct Parser {
    ptr: cxx::UniquePtr<ffi::parser>,
}

impl Parser {
    pub fn new(max_capacity: usize) -> Parser {
        let parser = ffi::parser_new(max_capacity);
        Parser { ptr: parser }
    }

    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<Element, SimdJsonError> {
        let result = ffi::parser_load(&mut self.ptr, path.as_ref().to_str().unwrap());
        if result.code < 2 {
            Ok(Element::from(result.value))
        } else {
            Err(SimdJsonError::from(result.code))
        }
    }

    pub fn parse_str(&mut self, s: &str) -> Result<Element, SimdJsonError> {
        let result = ffi::parser_parse_string(&mut self.ptr, s);
        if result.code < 2 {
            Ok(Element::from(result.value))
        } else {
            Err(SimdJsonError::from(result.code))
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        let parser = ffi::parser_new(SIMDJSON_MAXSIZE_BYTES);
        Parser { ptr: parser }
    }
}
