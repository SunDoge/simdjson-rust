use std::{marker::PhantomData, ptr::NonNull};

use simdjson_sys as ffi;

use super::{Element, Parser};
use crate::{macros::impl_drop, utils::string_view_struct_to_str};

pub struct Document {
    ptr: NonNull<ffi::SJ_DOM_document>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            ptr: unsafe { NonNull::new_unchecked(ffi::SJ_DOM_document_new()) },
        }
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
