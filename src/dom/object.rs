use super::parser::Parser;
use crate::libsimdjson::ffi;
use cxx::UniquePtr;
use std::marker::PhantomData;

pub type ObjectPtr = UniquePtr<ffi::object>;

pub struct Object<'a> {
    ptr: ObjectPtr,
    _phantom: PhantomData<&'a Parser>,
}
