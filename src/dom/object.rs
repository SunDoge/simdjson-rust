use super::parser::Parser;
use crate::libsimdjson::ffi;
use cxx::UniquePtr;
use std::marker::PhantomData;

pub type ObjectPtr = UniquePtr<ffi::object>;

pub struct Object<'a> {
    ptr: ObjectPtr,
    _phantom: PhantomData<&'a Parser>,
}

impl<'a> Object<'a> {
    pub fn new(ptr: ObjectPtr) -> Self {
        Object {
            ptr,
            _phantom: PhantomData,
        }
    }
}

impl<'a> From<ObjectPtr> for Object<'a> {
    fn from(ptr: ObjectPtr) -> Self {
        Object::new(ptr)
    }
}
