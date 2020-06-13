use super::element::Element;
use super::parser::Parser;
use crate::error::{SimdJsonError, SimdJsonResult};
use crate::libsimdjson::ffi;
use cxx::UniquePtr;
use std::marker::PhantomData;

pub type ArrayPtr = UniquePtr<ffi::array>;

pub struct Array<'a> {
    ptr: ArrayPtr,
    _phantom: PhantomData<&'a Parser>,
}

impl<'a> Array<'a> {
    pub fn new(ptr: ArrayPtr) -> Self {
        Array {
            ptr,
            _phantom: PhantomData,
        }
    }

    pub fn at(&self, json_pointer: &str) -> SimdJsonResult<Element> {
        let result = ffi::array_at(&self.ptr, json_pointer);
        check_result!(result, Element)
    }

    pub fn at_index(&self, index: usize) -> SimdJsonResult<Element> {
        let result = ffi::array_at_index(&self.ptr, index);
        check_result!(result, Element)
    }
}

impl<'a> From<ArrayPtr> for Array<'a> {
    fn from(ptr: ArrayPtr) -> Self {
        Array::new(ptr)
    }
}
