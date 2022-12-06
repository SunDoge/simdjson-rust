use std::fmt::Debug;

use cxx::UniquePtr;

use crate::{
    bridge::{check, ffi},
    error::Result,
};

pub struct Value(pub UniquePtr<ffi::OndemandValue>);

impl Value {
    pub fn get_u64(&mut self) -> Result<u64> {
        check!(ffi::ondemand_value_get_uint64, self.0.pin_mut())
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Value").finish()
    }
}
