use crate::macros::{impl_drop, map_ptr_result};
use simdjson_sys as ffi;
use std::ptr::NonNull;

use super::element::Element;
use crate::Result;

pub struct Array {
    ptr: NonNull<ffi::SJ_DOM_array>,
}

impl Array {
    pub fn new(ptr: NonNull<ffi::SJ_DOM_array>) -> Self {
        Self { ptr }
    }

    pub fn iter(&self) -> ArrayIter {
        let begin = unsafe { NonNull::new_unchecked(ffi::SJ_DOM_array_begin(self.ptr.as_ptr())) };
        let end = unsafe { NonNull::new_unchecked(ffi::SJ_DOM_array_end(self.ptr.as_ptr())) };
        ArrayIter::new(begin, end)
    }

    pub fn size(&self) -> usize {
        unsafe { ffi::SJ_DOM_array_size(self.ptr.as_ptr()) }
    }

    pub fn number_of_slots(&self) -> usize {
        unsafe { ffi::SJ_DOM_array_number_of_slots(self.ptr.as_ptr()) }
    }

    pub fn at_pointer(&self, json_pointer: &str) -> Result<Element> {
        map_ptr_result!(ffi::SJ_DOM_array_at_pointer(
            self.ptr.as_ptr(),
            json_pointer.as_ptr().cast(),
            json_pointer.len()
        ))
        .map(Element::new)
    }

    pub fn at(&self, index: usize) -> Result<Element> {
        map_ptr_result!(ffi::SJ_DOM_array_at(self.ptr.as_ptr(), index)).map(Element::new)
    }
}

impl_drop!(Array, ffi::SJ_DOM_array_free);

pub struct ArrayIter {
    begin: NonNull<ffi::SJ_DOM_array_iterator>,
    end: NonNull<ffi::SJ_DOM_array_iterator>,
    running: bool,
}

impl ArrayIter {
    pub fn new(
        begin: NonNull<ffi::SJ_DOM_array_iterator>,
        end: NonNull<ffi::SJ_DOM_array_iterator>,
    ) -> Self {
        Self {
            begin,
            end,
            running: false,
        }
    }

    pub fn get(&self) -> Element {
        let ptr = unsafe { ffi::SJ_DOM_array_iterator_get(self.begin.as_ptr()) };
        Element::new(unsafe { NonNull::new_unchecked(ptr) })
    }

    pub fn step(&mut self) {
        unsafe { ffi::SJ_DOM_array_iterator_step(self.begin.as_ptr()) }
    }

    pub fn not_equal(&self) -> bool {
        unsafe { ffi::SJ_DOM_array_iterator_not_equal(self.begin.as_ptr(), self.end.as_ptr()) }
    }
}

impl Drop for ArrayIter {
    fn drop(&mut self) {
        unsafe {
            ffi::SJ_DOM_array_iterator_free(self.begin.as_ptr());
            ffi::SJ_DOM_array_iterator_free(self.end.as_ptr());
        }
    }
}

impl Iterator for ArrayIter {
    type Item = Element;

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
