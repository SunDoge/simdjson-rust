use std::fmt::Debug;

use cxx::{let_cxx_string, UniquePtr};

use crate::{
    bridge::{check, ffi},
    error::Result,
};

use super::value::Value;

pub struct Document(pub UniquePtr<ffi::OndemandDocument>);

impl Document {
    pub fn at_pointer(&mut self, json_pointer: &str) -> Result<Value> {
        let_cxx_string!(p = json_pointer);
        check!(ffi::ondemand_document_at_pointer, self.0.pin_mut(), &p).map(Value)
    }
}

impl Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document").finish()
    }
}
