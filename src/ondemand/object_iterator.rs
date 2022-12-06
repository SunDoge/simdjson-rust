use std::{fmt::Debug, ops::Deref};

use cxx::UniquePtr;

use crate::{
    bridge::{check, ffi},
    error::Result,
};

use super::{field::Field, value::Value};

pub struct ObjectIterator(pub UniquePtr<ffi::OndemandObjectIterator>);

impl ObjectIterator {
    // pub fn equal(&self, rhs: &Self) -> bool {
    //     ffi::ondemand_object_iterator_equal(self, rhs)
    // }

    pub fn not_equal(&self, rhs: &Self) -> bool {
        ffi::ondemand_object_iterator_not_equal(self, rhs)
    }

    pub fn next(&mut self) -> &mut Self {
        ffi::ondemand_object_iterator_next(self.0.pin_mut());
        self
    }

    pub fn get(&mut self) -> Result<Field> {
        check!(ffi::ondemand_object_iterator_get, self.0.pin_mut()).map(Field)
    }
}

impl Debug for ObjectIterator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjectIterator").finish()
    }
}

impl Deref for ObjectIterator {
    type Target = ffi::OndemandObjectIterator;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
