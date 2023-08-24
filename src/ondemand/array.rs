use simdjson_sys as ffi;
use std::{marker::PhantomData, ptr::NonNull};

use crate::macros::impl_drop;

use super::parser::Parser;

pub struct Array<'p, 's> {
    ptr: NonNull<ffi::SJ_OD_array>,
    _parser: PhantomData<&'p mut Parser>,
    _padded_string: PhantomData<&'s String>,
}

impl<'p, 's> Array<'p, 's> {
    pub fn new(ptr: NonNull<ffi::SJ_OD_array>) -> Self {
        Self {
            ptr,
            _parser: PhantomData,
            _padded_string: PhantomData,
        }
    }
}

impl_drop!(Array<'p, 's>, ffi::SJ_OD_array_free);
