use simdjson_sys as ffi;
use std::{marker::PhantomData, ptr::NonNull};

use super::array::Array;

pub struct ArrayIterator<'a> {
    begin: NonNull<ffi::SJ_OD_array_iterator>,
    end: NonNull<ffi::SJ_OD_array_iterator>,
    _array: PhantomData<&'a mut Array<'a>>,
}

impl<'a> ArrayIterator<'a> {
    pub fn current(&self) {}
}

impl<'a> Drop for ArrayIterator<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::SJ_OD_array_iterator_free(self.begin.as_mut());
            ffi::SJ_OD_array_iterator_free(self.end.as_mut());
        }
    }
}
