use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use cxx::UniquePtr;

use crate::{
    bridge::{check, ffi},
    error::Result,
};

use super::{array::Array, object::Object};

pub struct Value(pub UniquePtr<ffi::OndemandValue>);

impl Value {
    pub fn get_u64(&mut self) -> Result<u64> {
        check!(ffi::ondemand_value_get_uint64, self.0.pin_mut())
    }

    pub fn get_array(&mut self) -> Result<Array> {
        check!(ffi::ondemand_value_get_array, self.0.pin_mut()).map(Array)
    }

    pub fn get_object(&mut self) -> Result<Object> {
        check!(ffi::ondemand_value_get_object, self.0.pin_mut()).map(Object)
    }

    pub fn find_field(&mut self, key: &str) -> Result<Value> {
        check!(ffi::ondemand_value_find_field, self.0.pin_mut(), key).map(Value)
    }

    pub fn find_field_unordered(&mut self, key: &str) -> Result<Value> {
        check!(
            ffi::ondemand_value_find_field_unordered,
            self.0.pin_mut(),
            key
        )
        .map(Value)
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Value").finish()
    }
}
