use simdjson_sys as ffi;
use std::{marker::PhantomData, ptr::NonNull};

use crate::macros::{impl_drop, map_result};
use crate::utils::string_view_to_str;

use super::document::Document;
use super::object_iterator::ObjectIterator;
use super::parser::Parser;
use super::value::Value;
use crate::error::Result;

pub struct Object {
    ptr: NonNull<ffi::SJ_OD_object>,
    // _document: PhantomData<&'a mut Document<'a, 'a>>,
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

    pub fn iter(&mut self) -> Result<ObjectIterator> {
        let begin = map_result!(
            ffi::SJ_OD_object_begin(self.ptr.as_mut()),
            ffi::SJ_OD_object_iterator_result_error,
            ffi::SJ_OD_object_iterator_result_value_unsafe
        )?;
        let end = map_result!(
            ffi::SJ_OD_object_end(self.ptr.as_mut()),
            ffi::SJ_OD_object_iterator_result_error,
            ffi::SJ_OD_object_iterator_result_value_unsafe
        )?;
        Ok(ObjectIterator::new(begin, end))
    }

    pub fn raw_json(&mut self) -> Result<&str> {
        let sv = map_result!(
            ffi::SJ_OD_object_raw_json(self.ptr.as_mut()),
            ffi::STD_string_view_result_error,
            ffi::STD_string_view_result_value_unsafe
        )?;
        Ok(string_view_to_str(sv))
    }
}

impl_drop!(Object, ffi::SJ_OD_object_free);
