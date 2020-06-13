use super::parser::Parser;
use crate::libsimdjson::ffi;
use cxx::UniquePtr;
use std::marker::PhantomData;

pub type ArrayPtr = UniquePtr<ffi::array>;

pub struct Array<'a> {
    ptr: ArrayPtr,
    _phantom: PhantomData<&'a Parser>,
}
