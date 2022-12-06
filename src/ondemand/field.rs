use std::fmt::Debug;

use cxx::UniquePtr;

use crate::{
    bridge::{check, ffi},
    error::Result,
};

pub struct Field(pub UniquePtr<ffi::OndemandField>);

impl Field {
    pub fn unescaped_key(&mut self) -> Result<&str> {
        check!(ffi::ondemand_field_unescaped_key, self.0.pin_mut())
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field").finish()
    }
}
