use simdjson_sys as ffi;
use std::ptr::NonNull;

use crate::macros::impl_drop;

pub struct Element {
    ptr: NonNull<ffi::SJ_DOM_element>,
}

impl Element {
    pub fn new(ptr: NonNull<ffi::SJ_DOM_element>) -> Self {
        Self { ptr }
    }
}

impl_drop!(Element, ffi::SJ_DOM_element_free);
