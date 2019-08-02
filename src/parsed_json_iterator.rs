use super::parsed_json::ParsedJson;
use simdjson_sys as lib;
use std::borrow::Cow;
use std::ffi;

pub struct ParsedJsonIterator<'a> {
    value: lib::simdjson_ParsedJson_Iterator,
    _pj: &'a ParsedJson,
}

impl<'a> ParsedJsonIterator<'a> {
    pub fn new(pj: &ParsedJson) -> ParsedJsonIterator {
        ParsedJsonIterator {
            value: unsafe {
                lib::simdjson_ParsedJson_Iterator::new(pj.get() as *const _ as *mut _)
            },
            _pj: pj,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.value.location < self.value.tape_length
    }

    // pub fn get_depth(&self) -> usize {
    //     unsafe { self.value.get_depth() }
    // }

    // pub fn get_scope_type(&self) -> u8 {
    //     unsafe { self.value.get_scope_type() }
    // }

    // pub fn move_forward(&mut self) -> bool {
    //     unsafe { self.value.move_forward() }
    // }

    // pub fn get_type(&self) -> u8 {
    //     unsafe { self.value.get_type() }
    // }

    // pub fn get_integer(&self) -> i64 {
    //     unsafe { self.value.get_integer() }
    // }

    // pub fn get_string(&self) -> Cow<str> {
    //     unsafe { ffi::CStr::from_ptr(self.value.get_string()) }.to_string_lossy()
    // }

    // pub fn get_double(&self) -> f64 {
    //     unsafe { self.value.get_double() }
    // }

    // pub fn is_object_or_array(&self) -> bool {
    //     unsafe { self.value.is_object_or_array() }
    // }
    // pub fn is_object(&self) -> bool {
    //     unsafe { self.value.is_object() }
    // }
    // pub fn is_array(&self) -> bool {
    //     unsafe { self.value.is_array() }
    // }
    // pub fn is_string(&self) -> bool {
    //     unsafe { self.value.is_string() }
    // }
    // pub fn is_integer(&self) -> bool {
    //     unsafe { self.value.is_integer() }
    // }
    // pub fn is_double(&self) -> bool {
    //     unsafe { self.value.is_double() }
    // }

    // pub fn move_to_key(&mut self, key: &str) -> bool {
    //     unsafe {
    //         self.value
    //             .move_to_key(ffi::CString::new(key).unwrap().into_raw())
    //     }
    // }

    // pub fn next(&mut self) -> bool {
    //     unsafe { self.value.next() }
    // }

    // pub fn prev(&mut self) -> bool {
    //     unsafe { self.value.prev() }
    // }

    // pub fn up(&mut self) -> bool {
    //     unsafe { self.value.up() }
    // }

    // pub fn down(&mut self) -> bool {
    //     unsafe { self.value.down() }
    // }

    // pub fn to_start_scope(&mut self) {
    //     unsafe {
    //         self.value.to_start_scope();
    //     }
    // }
}

impl<'a> Drop for ParsedJsonIterator<'a> {
    fn drop(&mut self) {
        unsafe {
            self.value.destruct();
        }
    }
}
