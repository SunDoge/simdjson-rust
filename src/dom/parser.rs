use super::element::Element;
use crate::error::SimdJsonError;
use crate::libsimdjson::{ffi, SIMDJSON_MAXSIZE_BYTES};
use crate::padded_string::PaddedString;
use cxx::UniquePtr;
use std::marker::PhantomData;
use std::path::Path;

pub struct Parser {
    ptr: UniquePtr<ffi::parser>,
}

impl Parser {
    pub fn new(max_capacity: usize) -> Parser {
        let parser = ffi::parser_new(max_capacity);
        Parser { ptr: parser }
    }

    // pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<Element, SimdJsonError> {
    //     let result = ffi::parser_load(&mut self.ptr, path.as_ref().to_str().unwrap());
    //     check_result!(result, Element)
    // }

    pub fn parse_str(&mut self, s: &str) -> Result<Element, SimdJsonError> {
        let result = ffi::parser_parse_string(&mut self.ptr, s);
        check_result!(result, Element)
    }

    // pub fn parse_str_padded<T: Into<PaddedString>>(
    //     &mut self,
    //     s: T,
    // ) -> Result<Element, SimdJsonError> {
    //     let result = ffi::parser_parse_padded_string(&mut self.ptr, &s.into().as_ptr());
    //     check_result!(result, Element)
    // }
}

impl Default for Parser {
    fn default() -> Self {
        let parser = ffi::parser_new(SIMDJSON_MAXSIZE_BYTES);
        Parser { ptr: parser }
    }
}
