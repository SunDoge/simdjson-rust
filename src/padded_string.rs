use std::ptr::NonNull;

use simdjson_sys as ffi;

use crate::{
    error::Result,
    macros::{check_result, impl_drop},
};

pub struct PaddedString {
    ptr: NonNull<ffi::SJ_padded_string>,
}

impl PaddedString {
    pub fn load(path: &str) -> Result<Self> {
        let c_path = std::ffi::CString::new(path).unwrap();
        check_result!(
            ffi::SJ_padded_string_load(c_path.as_ptr()),
            ffi::SJ_padded_string_result_error,
            ffi::SJ_padded_string_result_value
        )
        .map(|ptr| PaddedString {
            ptr: NonNull::new(ptr).unwrap(),
        })
    }
}

impl_drop!(PaddedString, ffi::SJ_padded_string_free);
