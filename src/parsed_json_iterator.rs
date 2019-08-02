use super::parsed_json::{ParsedJson, JSON_VALUE_MASK};
use simdjson_sys as lib;
use std::borrow::Cow;
use std::ffi;

pub struct ParsedJsonIterator<'a> {
    value: lib::simdjson_ParsedJson_Iterator,
    pj: &'a ParsedJson,
}

impl<'a> ParsedJsonIterator<'a> {
    pub fn new(pj: &ParsedJson) -> ParsedJsonIterator {
        ParsedJsonIterator {
            value: unsafe {
                lib::simdjson_ParsedJson_Iterator::new(pj.get() as *const _ as *mut _)
            },
            pj,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.value.location < self.value.tape_length
    }

    pub fn get_depth(&self) -> usize {
        self.value.depth
    }

    // pub fn get_scope_type(&self) -> u8 {
    //     unsafe { self.value.get_scope_type() }
    // }

    // pub fn move_forward(&mut self) -> bool {
    //     unsafe { self.value.move_forward() }
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

    pub fn is_object_or_array(&self) -> bool {
        self.is_object() || self.is_array()
    }

    pub fn is_object(&self) -> bool {
        self.get_type() == b'{'
    }

    pub fn is_array(&self) -> bool {
        self.get_type() == b'{'
    }

    pub fn is_string(&self) -> bool {
        self.get_type() == b'"'
    }

    pub fn is_integer(&self) -> bool {
        self.get_type() == b'l'
    }

    pub fn is_double(&self) -> bool {
        self.get_type() == b'd'
    }

    pub fn is_true(&self) -> bool {
        self.get_type() == b't'
    }

    pub fn is_false(&self) -> bool {
        self.get_type() == b'f'
    }

    pub fn is_null(&self) -> bool {
        self.get_type() == b'n'
    }

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

    pub fn down(&mut self) -> bool {
        if self.value.location + 1 >= self.value.tape_length {
            return false;
        }

        if self.is_array() || self.is_object() {
            let npos = self.value.current_val as usize & JSON_VALUE_MASK;
            if npos == self.value.location + 2 {
                return false;
            }
            let mut depth_index = unsafe { *self.value.depth_index.add(self.value.depth) };
            depth_index.start_of_scope = self.value.location;
            depth_index.scope_type = self.value.current_type;
            self.value.current_val = unsafe { *self.pj.value.tape.add(self.value.location) };
            self.value.current_type = self.value.current_val.overflowing_shr(56).0 as u8;
            self.value.depth += 1;
            self.value.location += 1;
            return true;
        }
        false
    }

    // pub fn to_start_scope(&mut self) {
    //     unsafe {
    //         self.value.to_start_scope();
    //     }
    // }

    pub fn get_type(&self) -> u8 {
        self.value.current_type
    }
}

impl<'a> Drop for ParsedJsonIterator<'a> {
    fn drop(&mut self) {
        unsafe {
            self.value.destruct();
        }
    }
}
