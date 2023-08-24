use std::{marker::PhantomData, ptr::NonNull};

use simdjson_sys as ffi;

use super::{document::Document, value::Value};
use crate::{
    error::Result,
    macros::{impl_drop, map_result},
    utils::string_view_to_str,
};

pub struct Field<'a> {
    ptr: NonNull<ffi::SJ_OD_field>,
    _doc: PhantomData<&'a mut Document<'a, 'a>>,
}

impl<'a> Field<'a> {
    pub fn new(ptr: NonNull<ffi::SJ_OD_field>) -> Self {
        Self {
            ptr,
            _doc: PhantomData,
        }
    }

    pub fn unescaped_key(&mut self, allow_replacement: bool) -> Result<&'a str> {
        let sv = map_result!(
            ffi::SJ_OD_field_unescaped_key(self.ptr.as_mut(), allow_replacement),
            ffi::STD_string_view_result_error,
            ffi::STD_string_view_result_value_unsafe
        )?;
        Ok(string_view_to_str(sv))
    }

    // Double free error.
    // pub fn value(&mut self) -> Value {
    //     let ptr = unsafe {
    //         let ptr = ffi::SJ_OD_field_value(self.ptr.as_mut());
    //         NonNull::new_unchecked(ptr)
    //     };
    //     Value::new(ptr)
    // }

    pub fn take_value(self) -> Value<'a> {
        let ptr = unsafe {
            let ptr = ffi::SJ_OD_field_take_value(self.ptr.as_ptr());
            NonNull::new_unchecked(ptr)
        };

        Value::new(ptr)
    }
}

impl_drop!(Field<'a>, ffi::SJ_OD_field_free);
