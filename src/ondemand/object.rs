use std::{marker::PhantomData, ptr::NonNull};

use simdjson_sys as ffi;

use super::{document::Document, object_iterator::ObjectIterator, value::Value};
use crate::{
    error::Result,
    macros::{impl_drop, map_result},
    utils::string_view_to_str,
};

pub struct Object<'a> {
    ptr: NonNull<ffi::SJ_OD_object>,
    _doc: PhantomData<&'a mut Document<'a, 'a>>,
}

impl<'a> Object<'a> {
    pub fn new(ptr: NonNull<ffi::SJ_OD_object>) -> Self {
        Self {
            ptr,
            _doc: PhantomData,
        }
    }

    pub fn at_pointer(&mut self, json_pointer: &str) -> Result<Value<'a>> {
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

    pub fn iter(&mut self) -> Result<ObjectIterator<'a>> {
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

    pub fn raw_json(&mut self) -> Result<&'a str> {
        let sv = map_result!(
            ffi::SJ_OD_object_raw_json(self.ptr.as_mut()),
            ffi::STD_string_view_result_error,
            ffi::STD_string_view_result_value_unsafe
        )?;
        Ok(string_view_to_str(sv))
    }

    pub fn find_field(&mut self, key: &str) -> Result<Value<'a>> {
        map_result!(
            ffi::SJ_OD_object_find_field(self.ptr.as_mut(), key.as_ptr().cast(), key.len()),
            ffi::SJ_OD_value_result_error,
            ffi::SJ_OD_value_result_value_unsafe
        )
        .map(Value::new)
    }

    pub fn find_field_unordered(&mut self, key: &str) -> Result<Value<'a>> {
        map_result!(
            ffi::SJ_OD_object_find_field_unordered(
                self.ptr.as_mut(),
                key.as_ptr().cast(),
                key.len()
            ),
            ffi::SJ_OD_value_result_error,
            ffi::SJ_OD_value_result_value_unsafe
        )
        .map(Value::new)
    }

    pub fn count_fields(&mut self) -> Result<usize> {
        map_result!(
            primitive,
            ffi::SJ_OD_object_count_fields(self.ptr.as_mut()),
            ffi::size_t_result_error,
            ffi::size_t_result_value_unsafe
        )
    }

    pub fn is_empty(&mut self) -> Result<bool> {
        map_result!(
            primitive,
            ffi::SJ_OD_object_is_empty(self.ptr.as_mut()),
            ffi::bool_result_error,
            ffi::bool_result_value_unsafe
        )
    }

    pub fn reset(&mut self) -> Result<bool> {
        map_result!(
            primitive,
            ffi::SJ_OD_object_reset(self.ptr.as_mut()),
            ffi::bool_result_error,
            ffi::bool_result_value_unsafe
        )
    }
}

impl_drop!(Object<'a>, ffi::SJ_OD_object_free);
