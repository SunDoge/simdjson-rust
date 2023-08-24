use simdjson_sys as ffi;
use std::{marker::PhantomData, ptr::NonNull};

use crate::{error::Result, macros::map_result};

use super::{array::Array, value::Value};

pub struct ArrayIterator {
    begin: NonNull<ffi::SJ_OD_array_iterator>,
    end: NonNull<ffi::SJ_OD_array_iterator>,
    running: bool,
    // _array: PhantomData<&'a mut Array<'a>>,
}

impl ArrayIterator {
    pub fn new(
        begin: NonNull<ffi::SJ_OD_array_iterator>,
        end: NonNull<ffi::SJ_OD_array_iterator>,
    ) -> Self {
        Self {
            begin,
            end,
            running: false,
            // _array: PhantomData,
        }
    }

    pub fn get(&mut self) -> Result<Value> {
        map_result!(
            ffi::SJ_OD_array_iterator_get(self.begin.as_mut()),
            ffi::SJ_OD_value_result_error,
            ffi::SJ_OD_value_result_value_unsafe
        )
        .map(Value::new)
    }

    pub fn not_equal(&self) -> bool {
        unsafe { ffi::SJ_OD_array_iterator_not_equal(self.begin.as_ref(), self.end.as_ref()) }
    }

    pub fn step(&mut self) {
        unsafe { ffi::SJ_OD_array_iterator_step(self.begin.as_mut()) }
    }
}

impl Drop for ArrayIterator {
    fn drop(&mut self) {
        unsafe {
            ffi::SJ_OD_array_iterator_free(self.begin.as_mut());
            ffi::SJ_OD_array_iterator_free(self.end.as_mut());
        }
    }
}

impl Iterator for ArrayIterator {
    type Item = Result<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.running {
            self.step();
        }

        if self.not_equal() {
            self.running = true;
            Some(self.get())
        } else {
            self.running = false;
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ondemand::parser::Parser, padded_string::make_padded_string};

    use super::*;

    #[test]
    fn test_iter() {
        let mut parser = Parser::default();
        let ps = make_padded_string("[1,2,3]");
        let mut doc = parser.iterate(&ps).unwrap();
        let mut arr = doc.get_array().unwrap();
        for (v, num) in arr.iter().unwrap().zip([1u64, 2, 3]) {
            let res = v.unwrap().get_uint64().unwrap();
            assert_eq!(res, num);
        }
    }
}
