use std::marker::PhantomData;

// use std::slice::IterMut;
use crate::error::Result;

use super::{array_iterator::ArrayIterator, value::Value};

// pub trait CxxIterator {
//     type Item;
//     fn not_equal(&mut self, rhs: &Self) -> bool;
//     fn next(&mut self) -> &mut Self;
//     fn get(&mut self) -> Self::Item;
// }

// pub struct IterMut<'a, T>
// where
//     T: CxxIterator,
// {
//     begin: T,
//     end: T,
//     curr: Option<T::Item>,
//     source: &'a mut T,
// }

// impl<'a, T> IterMut<'a, T>
// where
//     T: CxxIterator,
// {
//     pub fn new(begin: T, end: T) -> Self {
//         Self {
//             begin,
//             end,
//             curr: None,
//             _marker: PhantomData,
//         }
//     }
// }

// impl<'a, T> Iterator for IterMut<'a, T>
// where
//     T: CxxIterator,
// {
//     type Item = &'a mut T::Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.curr.is_some() {
//             self.begin.next();
//         }

//         if self.begin.not_equal(&self.end) {
//             self.curr = Some(self.begin.get());
//             self.curr.as_mut()
//         } else {
//             None
//         }
//     }
// }

// impl CxxIterator for ArrayIterator {
//     type Item = Result<Value>;

//     fn not_equal(&mut self, rhs: &Self) -> bool {
//         self.not_equal(rhs)
//     }

//     fn get(&mut self) -> Self::Item {
//         self.get()
//     }

//     fn next(&mut self) -> &mut Self {
//         self.next()
//     }
// }
