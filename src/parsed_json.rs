use super::parsed_json_iterator::ParsedJsonIterator;
use simdjson_sys as lib;
use std::fmt;

pub const DEFAULT_MAX_DEPTH: usize = 1024;
pub const JSON_VALUE_MASK: usize = 0xFFFFFFFFFFFFFF;

pub struct ParsedJson {
    pub value: lib::simdjson_ParsedJson,
}

impl ParsedJson {
    pub fn new() -> ParsedJson {
        ParsedJson {
            value: unsafe { lib::simdjson_ParsedJson::new() },
        }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { self.value.is_valid() }
    }

    pub fn allocate_capacity(&mut self, len: usize, max_depth: usize) -> bool {
        unsafe { self.value.allocate_capacity(len, max_depth) }
    }

    pub fn get(&self) -> &lib::simdjson_ParsedJson {
        &self.value
    }

    pub fn get_mut(&mut self) -> &mut lib::simdjson_ParsedJson {
        &mut self.value
    }

    // pub fn iterator(&self) -> ParsedJsonIterator {
    //     ParsedJsonIterator::new(self)
    // }
}

// Clippy says I should add a Default for it.
impl Default for ParsedJson {
    fn default() -> ParsedJson {
        ParsedJson::new()
    }
}

impl fmt::Display for ParsedJson {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.is_valid() {
            return Err(fmt::Error);
        }
        write!(f, "{}", "not implemented yet")
    }
}

impl Drop for ParsedJson {
    fn drop(&mut self) {
        unsafe {
            self.value.deallocate();
        }
    }
}
