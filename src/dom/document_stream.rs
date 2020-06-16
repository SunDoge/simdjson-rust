use super::element::Element;
use super::parser::Parser;
use crate::libsimdjson::ffi;
use cxx::UniquePtr;
use std::marker::PhantomData;

pub type DocumentStreamIteratorPtr = UniquePtr<ffi::DocumentStreamIterator>;

pub struct DocumentStreamIter<'a> {
    ptr: DocumentStreamIteratorPtr,
    _phantom: PhantomData<&'a Parser>,
}

impl<'a> From<DocumentStreamIteratorPtr> for DocumentStreamIter<'a> {
    fn from(ptr: DocumentStreamIteratorPtr) -> Self {
        DocumentStreamIter::new(ptr)
    }
}

impl<'a> DocumentStreamIter<'a> {
    pub fn new(ptr: DocumentStreamIteratorPtr) -> Self {
        DocumentStreamIter {
            ptr,
            _phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for DocumentStreamIter<'a> {
    type Item = Element<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = ffi::document_stream_iterator_next(&mut self.ptr);
        if result.value.is_null() {
            None
        } else {
            Some(result.value.into())
        }
    }
}
