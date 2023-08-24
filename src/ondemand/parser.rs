use std::ptr::NonNull;

use simdjson_sys as ffi;

use crate::{
    constants::SIMDJSON_MAXSIZE_BYTES,
    error::Result,
    macros::{impl_drop, map_result},
};

use super::document::Document;

pub struct Parser {
    ptr: NonNull<ffi::SJ_OD_parser>,
}

impl Default for Parser {
    fn default() -> Self {
        Parser::new(SIMDJSON_MAXSIZE_BYTES)
    }
}

impl Parser {
    pub fn new(max_capacity: usize) -> Self {
        let ptr = unsafe { NonNull::new_unchecked(ffi::SJ_OD_parser_new(max_capacity)) };
        Self { ptr }
    }

    pub fn iterate<'p, 's>(&'p mut self, padded_string: &'s String) -> Result<Document<'p, 's>> {
        map_result!(
            ffi::SJ_OD_parser_iterate_padded_string_view(
                self.ptr.as_mut(),
                padded_string.as_ptr().cast(),
                padded_string.len(),
                padded_string.capacity()
            ),
            ffi::SJ_OD_document_result_error,
            ffi::SJ_OD_document_result_value_unsafe
        )
        .map(Document::new)
    }
}

impl_drop!(Parser, ffi::SJ_OD_parser_free);

#[cfg(test)]
mod tests {
    use crate::padded_string::make_padded_string;

    use super::*;

    #[test]
    fn test_new() {
        let mut parser = Parser::default();
        let ps = make_padded_string("[1,2,3]");
        let mut doc = parser.iterate(&ps).unwrap();
        doc.get_array().unwrap();
        // doc.get_value().unwrap();
        drop(doc);
        let ps2 = make_padded_string("1");
        let mut doc2 = parser.iterate(&ps2).unwrap();
        let v = doc2.get_int64().unwrap();
        assert_eq!(v, 1);
        let v = doc2.get_uint64().unwrap();
        assert_eq!(v, 1);
    }
}
