use std::{io::Read, os::unix::prelude::OsStrExt, path::Path, ptr::NonNull};

use simdjson_sys as ffi;

use crate::{
    constants::SIMDJSON_PADDING,
    error::Result,
    macros::{impl_drop, map_result},
};

pub struct PaddedString {
    ptr: NonNull<ffi::SJ_padded_string>,
}

impl PaddedString {
    pub fn new(s: &str) -> Self {
        let ptr = unsafe {
            let ptr = ffi::SJ_padded_string_new(s.as_ptr().cast(), s.len());
            NonNull::new_unchecked(ptr)
        };
        Self { ptr }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let bytes = path.as_ref().as_os_str().as_bytes();
        let c_path = std::ffi::CString::new(bytes).unwrap();
        map_result!(
            ffi::SJ_padded_string_load(c_path.as_ptr()),
            ffi::SJ_padded_string_result_error,
            ffi::SJ_padded_string_result_value_unsafe
        )
        .map(|ptr| PaddedString { ptr })
    }

    pub fn len(&self) -> usize {
        unsafe { ffi::SJ_padded_string_length(self.ptr.as_ref()) }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let data = ffi::SJ_padded_string_u8data(self.ptr.as_ref());
            let s = std::slice::from_raw_parts(data, self.len());
            std::str::from_utf8_unchecked(s)
        }
    }
}

impl_drop!(PaddedString, ffi::SJ_padded_string_free);

pub fn make_padded_string(s: &str) -> String {
    let mut ps = String::with_capacity(s.len() + SIMDJSON_PADDING);
    ps.push_str(s);
    ps
}

pub fn load_padded_string<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let len = file.metadata()?.len() as usize;
    let mut buf = String::with_capacity(len + SIMDJSON_PADDING);
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = "{}";
        let ps = PaddedString::new(s);
        assert_eq!(s, ps.as_str());
    }
}
