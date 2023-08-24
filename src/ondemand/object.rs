use simdjson_sys as ffi;
use std::{marker::PhantomData, ptr::NonNull};

use crate::macros::{impl_drop, map_result};

use super::document::Document;
use super::parser::Parser;
use super::value::Value;
use crate::error::Result;

pub struct Object {
    ptr: NonNull<ffi::SJ_OD_object>,
    // _document: PhantomData<&'d mut Document<'d, 'd>>,
}

impl Object {
    pub fn new(ptr: NonNull<ffi::SJ_OD_object>) -> Self {
        Self {
            ptr,
            // _document: PhantomData,
        }
    }

    pub fn at_pointer(&mut self, json_pointer: &str) -> Result<Value> {
        map_result!(
            ffi::SJ_OD_object_at_pointer(
                self.ptr.as_mut(),
                json_pointer.as_ptr().cast(),
                json_pointer.len()
            ),
            ffi::SJ_OD_value_result_error,
            ffi::SJ_OD_value_result_value_unsafe
        )
        .map(Value::new)
    }
}

impl_drop!(Object, ffi::SJ_OD_object_free);
