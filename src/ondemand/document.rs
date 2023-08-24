use std::{marker::PhantomData, ptr::NonNull};

use simdjson_sys as ffi;

use super::{array::Array, object::Object, parser::Parser, value::Value};
use crate::{
    error::Result,
    macros::{impl_drop, map_result},
    utils::string_view_to_str,
};

pub struct Document<'p, 's> {
    ptr: NonNull<ffi::SJ_OD_document>,
    _parser: PhantomData<&'p mut Parser>,
    _padded_string: PhantomData<&'s String>,
}
impl<'p, 's> Document<'p, 's> {
    pub fn new(ptr: NonNull<ffi::SJ_OD_document>) -> Self {
        Self {
            ptr,
            _parser: PhantomData,
            _padded_string: PhantomData,
        }
    }

    pub fn get_uint64(&mut self) -> Result<u64> {
        map_result!(
            primitive,
            ffi::SJ_OD_document_get_uint64(self.ptr.as_mut()),
            ffi::uint64_t_result_error,
            ffi::uint64_t_result_value_unsafe
        )
    }

    pub fn get_int64(&mut self) -> Result<i64> {
        map_result!(
            primitive,
            ffi::SJ_OD_document_get_int64(self.ptr.as_mut()),
            ffi::int64_t_result_error,
            ffi::int64_t_result_value_unsafe
        )
    }

    pub fn get_bool(&mut self) -> Result<bool> {
        map_result!(
            primitive,
            ffi::SJ_OD_document_get_bool(self.ptr.as_mut()),
            ffi::bool_result_error,
            ffi::bool_result_value_unsafe
        )
    }

    pub fn get_double(&mut self) -> Result<f64> {
        map_result!(
            primitive,
            ffi::SJ_OD_document_get_double(self.ptr.as_mut()),
            ffi::double_result_error,
            ffi::double_result_value_unsafe
        )
    }

    pub fn get_value<'a>(&mut self) -> Result<Value<'a>> {
        map_result!(
            ffi::SJ_OD_document_get_value(self.ptr.as_mut()),
            ffi::SJ_OD_value_result_error,
            ffi::SJ_OD_value_result_value_unsafe
        )
        .map(Value::new)
    }

    pub fn get_array<'a>(&mut self) -> Result<Array<'a>> {
        map_result!(
            ffi::SJ_OD_document_get_array(self.ptr.as_mut()),
            ffi::SJ_OD_array_result_error,
            ffi::SJ_OD_array_result_value_unsafe
        )
        .map(Array::new)
    }

    pub fn get_object<'a>(&mut self) -> Result<Object<'a>> {
        map_result!(
            ffi::SJ_OD_document_get_object(self.ptr.as_mut()),
            ffi::SJ_OD_object_result_error,
            ffi::SJ_OD_object_result_value_unsafe
        )
        .map(Object::new)
    }

    pub fn get_wobbly_string<'a>(&mut self) -> Result<&'a str> {
        let sv = map_result!(
            ffi::SJ_OD_document_get_wobbly_string(self.ptr.as_mut()),
            ffi::STD_string_view_result_error,
            ffi::STD_string_view_result_value_unsafe
        )?;
        Ok(string_view_to_str(sv))
    }

    pub fn get_string<'a>(&mut self) -> Result<&'a str> {
        let sv = map_result!(
            ffi::SJ_OD_document_get_string(self.ptr.as_mut()),
            ffi::STD_string_view_result_error,
            ffi::STD_string_view_result_value_unsafe
        )?;
        Ok(string_view_to_str(sv))
    }
}

impl_drop!(Document<'p, 's>, ffi::SJ_OD_document_free);
