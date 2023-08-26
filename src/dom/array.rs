use std::{marker::PhantomData, ptr::NonNull};

use simdjson_sys as ffi;

use super::{document::Document, element::Element};
use crate::{
    macros::{impl_drop, map_ptr_result},
    Result,
};

pub struct Array<'a> {
    ptr: NonNull<ffi::SJ_DOM_array>,
    _doc: PhantomData<&'a Document>,
}

impl<'a> Array<'a> {
    pub fn new(ptr: NonNull<ffi::SJ_DOM_array>) -> Self {
        Self {
            ptr,
            _doc: PhantomData,
        }
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

    pub fn at_pointer(&self, json_pointer: &str) -> Result<Element<'a>> {
        map_ptr_result!(ffi::SJ_DOM_array_at_pointer(
            self.ptr.as_ptr(),
            json_pointer.as_ptr().cast(),
            json_pointer.len()
        ))
        .map(Element::new)
    }

    pub fn at(&self, index: usize) -> Result<Element<'a>> {
        map_ptr_result!(ffi::SJ_DOM_array_at(self.ptr.as_ptr(), index)).map(Element::new)
    }
}

impl_drop!(Array<'a>, ffi::SJ_DOM_array_free);

pub struct ArrayIter<'a> {
    begin: NonNull<ffi::SJ_DOM_array_iterator>,
    end: NonNull<ffi::SJ_DOM_array_iterator>,
    running: bool,
    _doc: PhantomData<&'a Document>,
}

impl<'a> ArrayIter<'a> {
    pub fn new(
        begin: NonNull<ffi::SJ_DOM_array_iterator>,
        end: NonNull<ffi::SJ_DOM_array_iterator>,
    ) -> Self {
        Self {
            begin,
            end,
            running: false,
            _doc: PhantomData,
        }
    }

    pub fn get(&self) -> Element<'a> {
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

impl<'a> Drop for ArrayIter<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::SJ_DOM_array_iterator_free(self.begin.as_ptr());
            ffi::SJ_DOM_array_iterator_free(self.end.as_ptr());
        }
    }
}

impl<'a> Iterator for ArrayIter<'a> {
    type Item = Element<'a>;

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
