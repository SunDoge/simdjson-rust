use std::ptr::NonNull;

use simdjson_sys as ffi;

use crate::macros::impl_drop;

pub struct PaddedString {
    ptr: NonNull<ffi::SJ_padded_string>,
}

impl PaddedString {}

impl_drop!(PaddedString, ffi::SJ_padded_string_free);
