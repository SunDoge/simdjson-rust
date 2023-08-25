use std::ptr::NonNull;

use simdjson_sys as ffi;

use crate::{
    macros::{impl_drop, map_ptr_result},
    Result,
};

use super::element::Element;

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
}

impl_drop!(Parser, ffi::SJ_DOM_parser_free);
