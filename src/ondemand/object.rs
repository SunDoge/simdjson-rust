use simdjson_sys as ffi;
use std::{marker::PhantomData, ptr::NonNull};

use crate::macros::impl_drop;

use super::parser::Parser;

pub struct Object<'p, 's> {
    ptr: NonNull<ffi::SJ_OD_object>,
    _parser: PhantomData<&'p mut Parser>,
    _padded_string: PhantomData<&'s String>,
}

impl<'p, 's> Object<'p, 's> {
    pub fn new(ptr: NonNull<ffi::SJ_OD_object>) -> Self {
        Self {
            ptr,
            _parser: PhantomData,
            _padded_string: PhantomData,
        }
    }
}

impl_drop!(Object<'p, 's>, ffi::SJ_OD_object_free);
