use simdjson_sys as ffi;
use std::{marker::PhantomData, ptr::NonNull};

use crate::macros::impl_drop;

use super::parser::Parser;

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
}

impl_drop!(Value<'p, 's>, ffi::SJ_OD_value_free);
