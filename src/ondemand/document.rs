use std::fmt::Debug;

use cxx::{UniquePtr};

use crate::{
    bridge::{check, ffi},
    error::Result,
};

use super::{object::Object, value::Value};

pub struct Document(pub UniquePtr<ffi::OndemandDocument>);

impl Document {
    pub fn at_pointer(&mut self, json_pointer: &str) -> Result<Value> {
        // let_cxx_string!(p = json_pointer);
        check!(ffi::ondemand_document_at_pointer, self.0.pin_mut(), json_pointer).map(Value)
    }

    pub fn get_object(&mut self) -> Result<Object> {
        check!(ffi::ondemand_document_get_object, self.0.pin_mut()).map(Object)
    }
}

impl Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document").finish()
    }
}
