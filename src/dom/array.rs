use super::parser::Parser;
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
}

impl<'a> From<ArrayPtr> for Array<'a> {
    fn from(ptr: ArrayPtr) -> Self {
        Array::new(ptr)
    }
}
