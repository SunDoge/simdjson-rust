use std::{fmt::Debug, marker::PhantomData};

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

    // pub fn iter(&mut self) -> Result<IterMut> {
    //     Ok(IterMut {
    //         begin: self.begin()?,
    //         end: self.end()?,
    //     })
    // }
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
