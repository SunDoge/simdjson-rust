use std::{marker::PhantomData, ptr::NonNull};

use simdjson_sys as ffi;

use super::{Element, Parser};
use crate::{
    macros::{impl_drop, map_ptr_result},
    utils::string_view_struct_to_str,
    Result,
};

pub struct DocumentStream {
    ptr: NonNull<ffi::SJ_DOM_document_stream>,
}

impl DocumentStream {
    pub fn new(ptr: NonNull<ffi::SJ_DOM_document_stream>) -> Self {
        Self { ptr }
    }

    pub fn iter(&self) -> DocumentStreamIter {
        let begin =
            unsafe { NonNull::new_unchecked(ffi::SJ_DOM_document_stream_begin(self.ptr.as_ptr())) };
        let end =
            unsafe { NonNull::new_unchecked(ffi::SJ_DOM_document_stream_end(self.ptr.as_ptr())) };
        DocumentStreamIter::new(begin, end)
    }
}

impl_drop!(DocumentStream, ffi::SJ_DOM_document_stream_free);

pub struct DocumentStreamIter<'a> {
    begin: NonNull<ffi::SJ_DOM_document_stream_iterator>,
    end: NonNull<ffi::SJ_DOM_document_stream_iterator>,
    running: bool,
    _parser: PhantomData<&'a DocumentStream>,
}

impl<'a> DocumentStreamIter<'a> {
    pub fn new(
        begin: NonNull<ffi::SJ_DOM_document_stream_iterator>,
        end: NonNull<ffi::SJ_DOM_document_stream_iterator>,
    ) -> Self {
        Self {
            begin,
            end,
            running: false,
            _parser: PhantomData,
        }
    }

    pub fn get(&self) -> Result<Element<'a>> {
        map_ptr_result!(ffi::SJ_DOM_document_stream_iterator_get(
            self.begin.as_ptr()
        ))
        .map(Element::new)
    }

    pub fn step(&mut self) {
        unsafe { ffi::SJ_DOM_document_stream_iterator_step(self.begin.as_ptr()) }
    }

    pub fn not_equal(&self) -> bool {
        unsafe {
            ffi::SJ_DOM_document_stream_iterator_not_equal(self.begin.as_ptr(), self.end.as_ptr())
        }
    }
}

impl<'a> Iterator for DocumentStreamIter<'a> {
    type Item = Result<Element<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.running {
            self.step();
        }

        if self.not_equal() {
            self.running = true;
            Some(self.get())
        } else {
            None
        }
    }
}
