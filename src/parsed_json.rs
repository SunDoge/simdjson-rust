use simdjson_sys as lib;

pub const DEFUALT_MAX_DEPTH: usize = 1024;

pub struct ParsedJson {
    value: lib::ParsedJson,
}

impl ParsedJson {
    pub fn new() -> ParsedJson {
        ParsedJson {
            value: unsafe { lib::ParsedJson::new() },
        }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { self.value.isValid() }
    }

    pub fn allocate_capacity(&mut self, len: usize, max_depth: usize) -> bool {
        unsafe { self.value.allocateCapacity(len, max_depth) }
    }

    pub fn get(&self) -> &lib::ParsedJson {
        &self.value
    }

    pub fn get_mut(&mut self) -> &mut lib::ParsedJson {
        &mut self.value
    }
}


impl Drop for ParsedJson {
    fn drop(&mut self) {
        unsafe {
            self.value.deallocate();
        }
    }
}