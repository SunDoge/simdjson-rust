use std::ptr::NonNull;

use ffi::DEFAULT_BATCH_SIZE;
use simdjson_sys as ffi;

use super::{document::Document, document_stream::DocumentStream, element::Element};
use crate::{
    macros::{impl_drop, map_ptr_result},
    Result,
};

pub struct Parser {
    ptr: NonNull<ffi::SJ_DOM_parser>,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new(ffi::SIMDJSON_MAXSIZE_BYTES)
    }
}

impl Parser {
    pub fn new(max_capacity: usize) -> Self {
        let ptr = unsafe { NonNull::new_unchecked(ffi::SJ_DOM_parser_new(max_capacity)) };
        Self { ptr }
    }

    pub fn parse(&mut self, padded_string: &String) -> Result<Element> {
        map_ptr_result!(ffi::SJ_DOM_parser_parse(
            self.ptr.as_ptr(),
            padded_string.as_ptr().cast(),
            padded_string.len()
        ))
        .map(Element::new)
    }

    pub fn parse_into_document(
        &mut self,
        doc: &mut Document,
        padded_string: &String,
    ) -> Result<Element> {
        map_ptr_result!(ffi::SJ_DOM_parser_parse_into_document(
            self.ptr.as_ptr(),
            doc.as_ptr(),
            padded_string.as_ptr().cast(),
            padded_string.len()
        ))
        .map(Element::new)
    }

    pub fn parse_many(&mut self, padded_string: &String) -> Result<DocumentStream> {
        self.parse_batch(padded_string, DEFAULT_BATCH_SIZE)
    }

    pub fn parse_batch(
        &mut self,
        padded_string: &String,
        batch_size: usize,
    ) -> Result<DocumentStream> {
        map_ptr_result!(ffi::SJ_DOM_parser_parse_many(
            self.ptr.as_ptr(),
            padded_string.as_ptr().cast(),
            padded_string.len(),
            batch_size
        ))
        .map(DocumentStream::new)
    }
}

impl_drop!(Parser, ffi::SJ_DOM_parser_free);
