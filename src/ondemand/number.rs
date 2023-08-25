use std::{marker::PhantomData, ptr::NonNull};

use simdjson_sys as ffi;

use super::{Document, NumberType};
use crate::macros::impl_drop;

pub struct Number<'a> {
    ptr: NonNull<ffi::SJ_OD_number>,
    _doc: PhantomData<&'a mut Document<'a, 'a>>,
}

impl<'a> Number<'a> {
    pub fn new(ptr: NonNull<ffi::SJ_OD_number>) -> Self {
        Self {
            ptr,
            _doc: PhantomData,
        }
    }

    pub fn get_uint64(&mut self) -> u64 {
        unsafe { ffi::SJ_OD_number_get_uint64(self.ptr.as_mut()) }
    }

    pub fn get_int64(&mut self) -> i64 {
        unsafe { ffi::SJ_OD_number_get_int64(self.ptr.as_mut()) }
    }

    pub fn get_double(&mut self) -> f64 {
        unsafe { ffi::SJ_OD_number_get_double(self.ptr.as_mut()) }
    }

    pub fn get_number_type(&mut self) -> NumberType {
        unsafe { ffi::SJ_OD_number_get_number_type(self.ptr.as_mut()) }.into()
    }
}

impl_drop!(Number<'a>, ffi::SJ_OD_number_free);
