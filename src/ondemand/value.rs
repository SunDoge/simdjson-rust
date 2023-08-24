use simdjson_sys as ffi;
use std::{marker::PhantomData, ptr::NonNull};

use crate::macros::{impl_drop, map_result};

use super::parser::Parser;
use super::{array::Array, object::Object};
use crate::error::Result;

pub struct Value<'p, 's> {
    ptr: NonNull<ffi::SJ_OD_value>,
    _parser: PhantomData<&'p mut Parser>,
    _padded_string: PhantomData<&'s String>,
}

impl<'p, 's> Value<'p, 's> {
    pub fn new(ptr: NonNull<ffi::SJ_OD_value>) -> Self {
        Self {
            ptr,
            _parser: PhantomData,
            _padded_string: PhantomData,
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

    pub fn get_array(&mut self) -> Result<Array<'p, 's>> {
        map_result!(
            ffi::SJ_OD_value_get_array(self.ptr.as_mut()),
            ffi::SJ_OD_array_result_error,
            ffi::SJ_OD_array_result_value_unsafe
        )
        .map(Array::new)
    }

    pub fn get_object(&mut self) -> Result<Object<'p, 's>> {
        map_result!(
            ffi::SJ_OD_value_get_object(self.ptr.as_mut()),
            ffi::SJ_OD_object_result_error,
            ffi::SJ_OD_object_result_value_unsafe
        )
        .map(Object::new)
    }
}

impl_drop!(Value<'p, 's>, ffi::SJ_OD_value_free);
