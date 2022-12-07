use std::fmt::Debug;

use cxx::UniquePtr;

use crate::{
    bridge::{check, ffi},
    error::Result,
};

use super::{iterator::Iterate, object_iterator::ObjectIterator, value::Value};

pub struct Object(pub UniquePtr<ffi::OndemandObject>);

impl Object {
    pub fn at_pointer(&mut self, json_pointer: &str) -> Result<Value> {
        check!(
            ffi::ondemand_object_at_pointer,
            self.0.pin_mut(),
            json_pointer
        )
        .map(Value)
    }

    pub fn begin(&mut self) -> Result<ObjectIterator> {
        check!(ffi::ondemand_object_begin, self.0.pin_mut()).map(ObjectIterator)
    }

    pub fn end(&mut self) -> Result<ObjectIterator> {
        check!(ffi::ondemand_object_end, self.0.pin_mut()).map(ObjectIterator)
    }

    pub fn iterate(&mut self) -> Result<Iterate<ObjectIterator>> {
        Ok(Iterate::new(self.begin()?, self.end()?))
    }

    pub fn raw_json(&mut self) -> Result<&str> {
        check!(ffi::ondemand_object_raw_json, self.0.pin_mut())
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Object").finish()
    }
}