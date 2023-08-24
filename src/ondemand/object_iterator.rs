use simdjson_sys as ffi;
use std::{marker::PhantomData, ptr::NonNull};

use crate::{error::Result, macros::map_result};

use super::{document::Document, field::Field, object::Object, value::Value};

pub struct ObjectIterator<'a> {
    begin: NonNull<ffi::SJ_OD_object_iterator>,
    end: NonNull<ffi::SJ_OD_object_iterator>,
    running: bool,
    _doc: PhantomData<&'a mut Document<'a, 'a>>,
}

impl<'a> ObjectIterator<'a> {
    pub fn new(
        begin: NonNull<ffi::SJ_OD_object_iterator>,
        end: NonNull<ffi::SJ_OD_object_iterator>,
    ) -> Self {
        Self {
            begin,
            end,
            running: false,
            _doc: PhantomData,
        }
    }

    pub fn get(&mut self) -> Result<Field<'a>> {
        map_result!(
            ffi::SJ_OD_object_iterator_get(self.begin.as_mut()),
            ffi::SJ_OD_field_result_error,
            ffi::SJ_OD_field_result_value_unsafe
        )
        .map(Field::new)
    }

    pub fn not_equal(&self) -> bool {
        unsafe { ffi::SJ_OD_object_iterator_not_equal(self.begin.as_ref(), self.end.as_ref()) }
    }

    pub fn step(&mut self) {
        unsafe { ffi::SJ_OD_object_iterator_step(self.begin.as_mut()) }
    }
}

impl<'a> Drop for ObjectIterator<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::SJ_OD_object_iterator_free(self.begin.as_mut());
            ffi::SJ_OD_object_iterator_free(self.end.as_mut());
        }
    }
}

impl<'a> Iterator for ObjectIterator<'a> {
    type Item = Result<Field<'a>>;

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
