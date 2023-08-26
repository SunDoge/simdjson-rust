use std::ptr::NonNull;

use simdjson_sys as ffi;

use super::Element;
use crate::macros::impl_drop;

pub struct Document {
    ptr: NonNull<ffi::SJ_DOM_document>,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            ptr: unsafe { NonNull::new_unchecked(ffi::SJ_DOM_document_new()) },
        }
    }
}

impl Document {
    pub fn new(ptr: NonNull<ffi::SJ_DOM_document>) -> Self {
        Self { ptr }
    }

    pub fn root(&self) -> Element<'_> {
        Element::new(unsafe {
            NonNull::new_unchecked(ffi::SJ_DOM_document_root(self.ptr.as_ptr()))
        })
    }

    pub fn as_ptr(&self) -> *mut ffi::SJ_DOM_document {
        self.ptr.as_ptr()
    }
}

impl_drop!(Document, ffi::SJ_DOM_document_free);
