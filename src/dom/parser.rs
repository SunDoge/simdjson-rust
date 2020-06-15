use super::document_stream::DocumentStreamIter;
use super::element::Element;
use crate::error::SimdJsonError;
use crate::libsimdjson::{ffi, DEFAULT_BATCH_SIZE, SIMDJSON_MAXSIZE_BYTES};
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

    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<Element, SimdJsonError> {
        let result = ffi::parser_load(&mut self.ptr, path.as_ref().to_str().unwrap());
        check_result!(result, Element)
    }

    pub fn parse(&mut self, s: &str) -> Result<Element, SimdJsonError> {
        let result = ffi::parser_parse_string(&mut self.ptr, s);
        check_result!(result, Element)
    }

    pub fn parse_padded(&mut self, s: &PaddedString) -> Result<Element, SimdJsonError> {
        let result = ffi::parser_parse_padded_string(&mut self.ptr, s.as_ptr());
        check_result!(result, Element)
    }

    pub fn load_many<P: AsRef<Path>>(
        &mut self,
        path: P,
        batch_size: usize,
    ) -> Result<DocumentStreamIter, SimdJsonError> {
        let iter_ptr = ffi::parser_load_many(
            &mut self.ptr,
            path.as_ref()
                .to_str()
                .ok_or(SimdJsonError::InvalidUriFragment)?,
            batch_size,
        );
        Ok(iter_ptr.into())
    }

    pub fn load_many_default<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<DocumentStreamIter, SimdJsonError> {
        self.load_many(path, DEFAULT_BATCH_SIZE)
    }

    pub fn parse_many(
        &mut self,
        s: &str,
        batch_size: usize,
    ) -> Result<DocumentStreamIter, SimdJsonError> {
        let iter_ptr = ffi::parser_parse_many(&mut self.ptr, s, batch_size);
        Ok(iter_ptr.into())
    }

    pub fn parse_many_padded(
        &mut self,
        s: &PaddedString,
        batch_size: usize,
    ) -> Result<DocumentStreamIter, SimdJsonError> {
        let iter_ptr = ffi::parser_parse_many_padded(&mut self.ptr, s.as_ptr(), batch_size);
        Ok(iter_ptr.into())
    }

    pub fn parse_many_default(&mut self, s: &str) -> Result<DocumentStreamIter, SimdJsonError> {
        self.parse_many(s, DEFAULT_BATCH_SIZE)
    }

    pub fn parse_many_padded_default(
        &mut self,
        s: &PaddedString,
    ) -> Result<DocumentStreamIter, SimdJsonError> {
        self.parse_many_padded(s, DEFAULT_BATCH_SIZE)
    }
}

impl Default for Parser {
    fn default() -> Self {
        let parser = ffi::parser_new(SIMDJSON_MAXSIZE_BYTES);
        Parser { ptr: parser }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use super::element::GetValue;

    #[test]
    fn parse_padded_string() {
        let mut parser = Parser::default();
        let value: bool = parser
            .parse_padded(&"true".into())
            .unwrap()
            .get_bool()
            .unwrap();
        assert_eq!(value, true);
    }

    #[test]
    fn borrow_checker() {
        let mut parser = Parser::default();
        let elm = parser.parse_padded(&"true".into()).unwrap();
        elm.get_bool();
        let new_elm = parser.parse("false").unwrap();
        // elm.get_bool(); // This won't pass
    }
}
