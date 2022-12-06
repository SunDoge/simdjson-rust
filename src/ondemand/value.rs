use std::fmt::Debug;

use cxx::UniquePtr;

use crate::{
    bridge::{check, ffi},
    error::Result,
};

use super::array::Array;

pub struct Value(pub UniquePtr<ffi::OndemandValue>);

impl Value {
    pub fn get_u64(&mut self) -> Result<u64> {
        check!(ffi::ondemand_value_get_uint64, self.0.pin_mut())
    }

    pub fn get_array(&mut self) -> Result<Array> {
        check!(ffi::ondemand_value_get_array, self.0.pin_mut()).map(Array)
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Value").finish()
    }
}
