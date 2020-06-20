use super::libsimdjson::ffi;
use crate::error::SimdJsonError;
use std::path::Path;

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

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, SimdJsonError> {
        let result = ffi::padded_string_load(path.as_ref().to_str().ok_or(SimdJsonError::IoError)?);
        if result.code < 2 {
            Ok(PaddedString::new(result.value))
        } else {
            Err(SimdJsonError::from(result.code))
        }
    }
}

impl From<&str> for PaddedString {
    fn from(s: &str) -> Self {
        let ptr = ffi::padded_string_from_string(s);
        PaddedString { ptr }
    }
}
