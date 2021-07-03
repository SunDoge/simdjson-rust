use super::element::Element;
use super::parser::Parser;
use crate::error::SimdJsonResult;
use crate::libsimdjson::ffi;
use cxx::UniquePtr;
use std::marker::PhantomData;

pub type DocumentStreamIterPtr = UniquePtr<ffi::DocumentStreamIterator>;
pub type DocumentStreamPtr = UniquePtr<ffi::document_stream>;

pub struct DocumentStream<'a> {
    ptr: DocumentStreamPtr,
    _phantom: PhantomData<&'a Parser>,
    iter: DocumentStreamIterPtr,
}

impl<'a> DocumentStream<'a> {
    pub fn new(ptr: DocumentStreamPtr) -> Self {
        DocumentStream {
            ptr,
            _phantom: PhantomData,
            iter: DocumentStreamIterPtr::null(),
        }
    }
}

impl<'a> From<DocumentStreamPtr> for DocumentStream<'a> {
    fn from(ptr: DocumentStreamPtr) -> Self {
        DocumentStream::new(ptr)
    }
}

impl<'a> Iterator for DocumentStream<'a> {
    type Item = SimdJsonResult<Element<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.is_null() {
            self.iter = ffi::document_stream_get_iterator(self.ptr.pin_mut());
        } else {
            ffi::document_stream_iterator_next(self.iter.pin_mut());
        }

        let result = ffi::document_stream_iterator_deref(self.iter.pin_mut());
        if result.code == 0 && result.value.is_null() {
            None
        } else {
            Some(check_result!(result, Element))
        }
    }
}
