use simdjson_sys as ffi;
use std::{marker::PhantomData, ptr::NonNull};

use crate::macros::impl_drop;

use super::document::Document;
use super::parser::Parser;

pub struct Object<'d> {
    ptr: NonNull<ffi::SJ_OD_object>,
    _document: PhantomData<&'d mut Document<'d, 'd>>,
}

impl<'d> Object<'d> {
    pub fn new(ptr: NonNull<ffi::SJ_OD_object>) -> Self {
        Self {
            ptr,
            _document: PhantomData,
        }
    }
}

impl_drop!(Object<'d>, ffi::SJ_OD_object_free);
