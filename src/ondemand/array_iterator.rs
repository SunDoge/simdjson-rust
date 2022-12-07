use std::{fmt::Debug, ops::Deref};

use cxx::UniquePtr;

use crate::{
    bridge::{check, ffi},
    error::Result,
};

use super::value::Value;

pub struct ArrayIterator(pub UniquePtr<ffi::OndemandArrayIterator>);

impl ArrayIterator {
    pub fn equal(&self, rhs: &Self) -> bool {
        ffi::ondemand_array_iterator_equal(self, rhs)
    }

    pub fn not_equal(&self, rhs: &Self) -> bool {
        ffi::ondemand_array_iterator_not_equal(self, rhs)
    }

    pub fn next(&mut self) -> &mut Self {
        ffi::ondemand_array_iterator_next(self.0.pin_mut());
        self
    }

    pub fn get(&mut self) -> Result<Value> {
        check!(ffi::ondemand_array_iterator_get, self.0.pin_mut()).map(Value)
    }
}

impl Debug for ArrayIterator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArrayIterator").finish()
    }
}

impl Deref for ArrayIterator {
    type Target = ffi::OndemandArrayIterator;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
