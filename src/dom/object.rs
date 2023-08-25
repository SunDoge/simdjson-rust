use std::{marker::PhantomData, ptr::NonNull};

use simdjson_sys as ffi;

use super::{Element, Parser};
use crate::{macros::impl_drop, utils::string_view_struct_to_str};

pub struct Object<'a> {
    ptr: NonNull<ffi::SJ_DOM_object>,
    _parser: PhantomData<&'a Parser>,
}

impl<'a> Object<'a> {
    pub fn new(ptr: NonNull<ffi::SJ_DOM_object>) -> Self {
        Self {
            ptr,
            _parser: PhantomData,
        }
    }
}

pub struct ObjectIter<'a> {
    begin: NonNull<ffi::SJ_DOM_object_iterator>,
    end: NonNull<ffi::SJ_DOM_object_iterator>,
    running: bool,
    _parser: PhantomData<&'a Parser>,
}

impl<'a> ObjectIter<'a> {
    pub fn new(
        begin: NonNull<ffi::SJ_DOM_object_iterator>,
        end: NonNull<ffi::SJ_DOM_object_iterator>,
    ) -> Self {
        Self {
            begin,
            end,
            running: false,
            _parser: PhantomData,
        }
    }

    pub fn get(&self) -> (&'a str, Element<'a>) {
        let kv = unsafe { ffi::SJ_DOM_object_iterator_get(self.begin.as_ptr()) };
        let key = string_view_struct_to_str(kv.key);
        let value = Element::new(unsafe { NonNull::new_unchecked(kv.value) });
        (key, value)
    }

    pub fn step(&mut self) {
        unsafe { ffi::SJ_DOM_object_iterator_step(self.begin.as_ptr()) }
    }

    pub fn not_equal(&self) -> bool {
        unsafe { ffi::SJ_DOM_object_iterator_not_equal(self.begin.as_ptr(), self.end.as_ptr()) }
    }
}

impl<'a> Drop for ObjectIter<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::SJ_DOM_object_iterator_free(self.begin.as_ptr());
            ffi::SJ_DOM_object_iterator_free(self.end.as_ptr());
        }
    }
}

impl<'a> Iterator for ObjectIter<'a> {
    type Item = (&'a str, Element<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.running {
            self.step();
        }

        if self.not_equal() {
            self.running = true;
            Some(self.get())
        } else {
            None
        }
    }
}

impl_drop!(Object<'a>, ffi::SJ_DOM_object_free);
