use simdjson_sys as ffi;
use std::{marker::PhantomData, ptr::NonNull};

use super::{document::Document, value::Value};
use crate::{
    error::Result,
    macros::{impl_drop, map_result},
};

use super::parser::Parser;

pub struct Array<'d> {
    ptr: NonNull<ffi::SJ_OD_array>,
    _document: PhantomData<&'d mut Document<'d, 'd>>,
}

impl<'d> Array<'d> {
    pub fn new(ptr: NonNull<ffi::SJ_OD_array>) -> Self {
        Self {
            ptr,
            _document: PhantomData,
        }
    }

    pub fn count_elements(&mut self) -> Result<usize> {
        map_result!(
            primitive,
            ffi::SJ_OD_array_count_elements(self.ptr.as_mut()),
            ffi::size_t_result_error,
            ffi::size_t_result_value_unsafe
        )
    }

    pub fn is_empty(&mut self) -> Result<bool> {
        map_result!(
            primitive,
            ffi::SJ_OD_array_is_empty(self.ptr.as_mut()),
            ffi::bool_result_error,
            ffi::bool_result_value_unsafe
        )
    }

    pub fn at(&mut self, index: usize) -> Result<Value> {
        map_result!(
            ffi::SJ_OD_array_at(self.ptr.as_mut(), index),
            ffi::SJ_OD_value_result_error,
            ffi::SJ_OD_value_result_value_unsafe
        )
        .map(Value::new)
    }

    pub fn reset(&mut self) -> Result<bool> {
        map_result!(
            primitive,
            ffi::SJ_OD_array_reset(self.ptr.as_mut()),
            ffi::bool_result_error,
            ffi::bool_result_value_unsafe
        )
    }

    fn begin(&mut self) {

    }

    fn end(&self) {
        
    }
    
}

impl_drop!(Array<'d>, ffi::SJ_OD_array_free);
