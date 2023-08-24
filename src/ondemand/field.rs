use simdjson_sys as ffi;
use std::ptr::NonNull;

use crate::error::Result;
use crate::macros::{impl_drop, map_result};
use crate::utils::string_view_to_str;

use super::value::Value;

pub struct Field {
    ptr: NonNull<ffi::SJ_OD_field>,
}

impl Field {
    pub fn new(ptr: NonNull<ffi::SJ_OD_field>) -> Self {
        Self { ptr }
    }

    pub fn unescaped_key(&mut self, allow_replacement: bool) -> Result<&str> {
        let sv = map_result!(
            ffi::SJ_OD_field_unescaped_key(self.ptr.as_mut(), allow_replacement),
            ffi::STD_string_view_result_error,
            ffi::STD_string_view_result_value_unsafe
        )?;
        // let s = unsafe {
        //     let s = std::slice::from_raw_parts(
        //         ffi::STD_string_view_data(sv.as_ptr()).cast(),
        //         ffi::STD_string_view_size(sv.as_ptr()),
        //     );
        //     std::str::from_utf8_unchecked(s)
        // };
        // unsafe { ffi::STD_string_view_free(sv.as_ptr()) };

        let s = string_view_to_str(sv);
        Ok(s)
    }

    // Double free error.
    // pub fn value(&mut self) -> Value {
    //     let ptr = unsafe {
    //         let ptr = ffi::SJ_OD_field_value(self.ptr.as_mut());
    //         NonNull::new_unchecked(ptr)
    //     };
    //     Value::new(ptr)
    // }

    pub fn take_value(self) -> Value {
        let ptr = unsafe {
            let ptr = ffi::SJ_OD_field_take_value(self.ptr.as_ptr());
            NonNull::new_unchecked(ptr)
        };

        Value::new(ptr)
    }
}

impl_drop!(Field, ffi::SJ_OD_field_free);
