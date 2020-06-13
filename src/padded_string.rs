use super::libsimdjson::ffi;

pub type PaddedStringPtr = cxx::UniquePtr<ffi::padded_string>;

pub struct PaddedString {
    ptr: PaddedStringPtr,
}

impl PaddedString {
    pub fn new(ptr: PaddedStringPtr) -> Self {
        PaddedString { ptr }
    }

    pub fn as_ptr(&self) -> &PaddedStringPtr {
        &self.ptr
    }
}

impl From<&str> for PaddedString {
    fn from(s: &str) -> Self {
        let ptr = ffi::padded_string_from_string(s);
        PaddedString { ptr }
    }
}

