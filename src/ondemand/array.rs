use std::fmt::Debug;

use cxx::UniquePtr;

use crate::{
    bridge::{check, ffi},
    error::Result,
};

use super::{array_iterator::ArrayIterator, value::Value};

pub struct Array(pub UniquePtr<ffi::OndemandArray>);

impl Array {
    pub fn begin(&mut self) -> Result<ArrayIterator> {
        check!(ffi::ondemand_array_begin, self.0.pin_mut()).map(ArrayIterator)
    }

    pub fn end(&mut self) -> Result<ArrayIterator> {
        check!(ffi::ondemand_array_end, self.0.pin_mut()).map(ArrayIterator)
    }

    pub fn iter(&mut self) -> Result<Iter> {
        Ok(Iter {
            begin: self.begin()?,
            end: self.end()?,
        })
    }
}

impl Debug for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Array").finish()
    }
}

pub struct Iter {
    pub begin: ArrayIterator,
    pub end: ArrayIterator,
}

impl Iterator for Iter {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin.not_equal(&self.end) {
            Some(self.begin.next())
        } else {
            None
        }
    }
}
