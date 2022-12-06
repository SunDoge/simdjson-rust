use std::fmt::Debug;

use cxx::UniquePtr;

use crate::{
    bridge::{check, ffi},
    error::Result,
};

use super::value::Value;

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
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Object").finish()
    }
}
