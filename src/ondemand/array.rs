use std::fmt::Debug;

use cxx::UniquePtr;

use crate::{
    bridge::{check, ffi},
    error::Result,
};

use super::{array_iterator::ArrayIterator, iterator::Iterate, value::Value};

pub struct Array(pub UniquePtr<ffi::OndemandArray>);

impl Array {
    // pub fn new(ptr: UniquePtr<ffi::OndemandArray>) -> Self {
    //     Self(ptr)
    // }

    pub fn begin(&mut self) -> Result<ArrayIterator> {
        check!(ffi::ondemand_array_begin, self.0.pin_mut()).map(ArrayIterator)
    }

    pub fn end(&mut self) -> Result<ArrayIterator> {
        check!(ffi::ondemand_array_end, self.0.pin_mut()).map(ArrayIterator)
    }

    // pub fn iter(&mut self) -> Result<IterMut> {
    //     Ok(IterMut {
    //         begin: self.begin()?,
    //         end: self.end()?,
    //     })
    // }

    pub fn iterate(&mut self) -> Result<Iterate<ArrayIterator>> {
        Ok(Iterate::new(self.begin()?, self.end()?))
    }

    pub fn at(&mut self, index: usize) -> Result<Value> {
        check!(ffi::ondemand_array_at, self.0.pin_mut(), index).map(Value)
    }
}

impl Debug for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Array").finish()
    }
}

// pub struct IterMut {
//     pub begin: ArrayIterator,
//     pub end: ArrayIterator,

// }

// impl Iterator for IterMut {
//     type Item = &mut Value;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.begin.not_equal(&self.end) {
//             Some(&mut self.begin.next())
//         } else {
//             None
//         }
//     }
// }