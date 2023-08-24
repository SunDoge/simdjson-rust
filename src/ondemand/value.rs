use std::{marker::PhantomData, ptr::NonNull};

use simdjson_sys as ffi;

use super::{array::Array, document::Document, object::Object};
use crate::{
    error::Result,
    macros::{impl_drop, map_result},
    utils::string_view_to_str,
};

pub struct Value<'a> {
    ptr: NonNull<ffi::SJ_OD_value>,
    _doc: PhantomData<&'a mut Document<'a, 'a>>,
}

impl<'a> Value<'a> {
    pub fn new(ptr: NonNull<ffi::SJ_OD_value>) -> Self {
        Self {
            ptr,
            _doc: PhantomData,
        }
    }

    pub fn get_uint64(&mut self) -> Result<u64> {
        map_result!(
            primitive,
            ffi::SJ_OD_value_get_uint64(self.ptr.as_mut()),
            ffi::uint64_t_result_error,
            ffi::uint64_t_result_value_unsafe
        )
    }

    pub fn get_int64(&mut self) -> Result<i64> {
        map_result!(
            primitive,
            ffi::SJ_OD_value_get_int64(self.ptr.as_mut()),
            ffi::int64_t_result_error,
            ffi::int64_t_result_value_unsafe
        )
    }

    pub fn get_bool(&mut self) -> Result<bool> {
        map_result!(
            primitive,
            ffi::SJ_OD_value_get_bool(self.ptr.as_mut()),
            ffi::bool_result_error,
            ffi::bool_result_value_unsafe
        )
    }

    pub fn get_double(&mut self) -> Result<f64> {
        map_result!(
            primitive,
            ffi::SJ_OD_value_get_double(self.ptr.as_mut()),
            ffi::double_result_error,
            ffi::double_result_value_unsafe
        )
    }

    pub fn get_array(&mut self) -> Result<Array<'a>> {
        map_result!(
            ffi::SJ_OD_value_get_array(self.ptr.as_mut()),
            ffi::SJ_OD_array_result_error,
            ffi::SJ_OD_array_result_value_unsafe
        )
        .map(Array::new)
    }

    pub fn get_object(&mut self) -> Result<Object<'a>> {
        map_result!(
            ffi::SJ_OD_value_get_object(self.ptr.as_mut()),
            ffi::SJ_OD_object_result_error,
            ffi::SJ_OD_object_result_value_unsafe
        )
        .map(Object::new)
    }

    pub fn get_string(&mut self, allow_replacement: bool) -> Result<&'a str> {
        let sv = map_result!(
            ffi::SJ_OD_value_get_string(self.ptr.as_mut(), allow_replacement),
            ffi::STD_string_view_result_error,
            ffi::STD_string_view_result_value_unsafe
        )?;
        Ok(string_view_to_str(sv))
    }

    pub fn at_pointer(&mut self, json_pointer: &str) -> Result<Value<'a>> {
        map_result!(
            ffi::SJ_OD_value_at_pointer(
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

impl_drop!(Value<'a>, ffi::SJ_OD_value_free);
