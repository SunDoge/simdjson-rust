use crate::{macros::impl_drop, utils::string_view_struct_to_str};
use simdjson_sys as ffi;
use std::ptr::NonNull;

use super::Element;

pub struct Object {
    ptr: NonNull<ffi::SJ_DOM_object>,
}

impl Object {
    pub fn new(ptr: NonNull<ffi::SJ_DOM_object>) -> Self {
        Self { ptr }
    }
}

pub struct ObjectIter {
    begin: NonNull<ffi::SJ_DOM_object_iterator>,
    end: NonNull<ffi::SJ_DOM_object_iterator>,
    running: bool,
}

impl ObjectIter {
    pub fn new(
        begin: NonNull<ffi::SJ_DOM_object_iterator>,
        end: NonNull<ffi::SJ_DOM_object_iterator>,
    ) -> Self {
        Self {
            begin,
            end,
            running: false,
        }
    }

    pub fn get(&self) -> (&'static str, Element) {
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

impl Drop for ObjectIter {
    fn drop(&mut self) {
        unsafe {
            ffi::SJ_DOM_object_iterator_free(self.begin.as_ptr());
            ffi::SJ_DOM_object_iterator_free(self.end.as_ptr());
        }
    }
}

impl Iterator for ObjectIter {
    type Item = (&'static str, Element);

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

impl_drop!(Object, ffi::SJ_DOM_object_free);
