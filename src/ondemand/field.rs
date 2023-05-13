use std::{fmt::Debug, pin::Pin};

use cxx::UniquePtr;

use crate::{
    bridge::{check, ffi},
    error::Result,
};

use super::{raw_json_string::RawJsonString, value::Value};

pub struct Field(pub UniquePtr<ffi::OndemandField>);

impl Field {
    // If you want value, you must get value first, than get the key
    // Yes, it's really wired.
    // pub fn unescaped_key(&mut self) -> Result<&str> {
    //     check!(ffi::ondemand_field_unescaped_key, self.0.pin_mut())
    // }

    pub fn value(&mut self) -> Value {
        Value(ffi::ondemand_field_value(self.0.pin_mut()))
    }

    pub fn key(&self) -> RawJsonString {
        RawJsonString(ffi::ondemand_field_key(&self.0))
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field").finish()
    }
}
