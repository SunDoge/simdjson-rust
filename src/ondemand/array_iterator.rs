use std::{marker::PhantomData, ptr::NonNull};

use simdjson_sys as ffi;

use super::{document::Document, value::Value};
use crate::{error::Result, macros::map_result};

pub struct ArrayIterator<'a> {
    begin: NonNull<ffi::SJ_OD_array_iterator>,
    end: NonNull<ffi::SJ_OD_array_iterator>,
    running: bool,
    _doc: PhantomData<&'a mut Document<'a, 'a>>,
}

impl<'a> ArrayIterator<'a> {
    pub fn new(
        begin: NonNull<ffi::SJ_OD_array_iterator>,
        end: NonNull<ffi::SJ_OD_array_iterator>,
    ) -> Self {
        Self {
            begin,
            end,
            running: false,
            _doc: PhantomData,
        }
    }

    pub fn get(&mut self) -> Result<Value<'a>> {
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

impl<'a> Drop for ArrayIterator<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::SJ_OD_array_iterator_free(self.begin.as_mut());
            ffi::SJ_OD_array_iterator_free(self.end.as_mut());
        }
    }
}

impl<'a> Iterator for ArrayIterator<'a> {
    type Item = Result<Value<'a>>;

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

    #[test]
    fn test_iter() {
        let mut parser = Parser::default();
        let ps = make_padded_string("[1,2,3]");
        let mut doc = parser.iterate(&ps).unwrap();
        // drop(ps);
        let mut arr = doc.get_array().unwrap();
        for (v, num) in arr.iter().unwrap().zip([1u64, 2, 3]) {
            let res = v.unwrap().get_uint64().unwrap();
            assert_eq!(res, num);
        }
    }
}
