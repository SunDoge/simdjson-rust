use std::ptr::NonNull;

use simdjson_sys as ffi;

use crate::{Result, SimdJsonError};

const DEFAULT_INITIAL_CAPACITY: usize = 1024;

/// A high-performance JSON string builder backed by simdjson's SIMD-accelerated
/// string_builder. Provides efficient JSON serialization with automatic string
/// escaping and structural token management.
pub struct StringBuilder {
    ptr: NonNull<ffi::SJ_string_builder>,
}

impl StringBuilder {
    /// Creates a new StringBuilder with the default initial capacity (1KB).
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_INITIAL_CAPACITY)
    }

    /// Creates a new StringBuilder with the specified initial capacity in bytes.
    pub fn with_capacity(capacity: usize) -> Self {
        let ptr = unsafe { NonNull::new_unchecked(ffi::SJ_string_builder_new(capacity)) };
        Self { ptr }
    }

    /// Clears the contents of the builder, resetting it to empty while retaining
    /// the allocated capacity.
    pub fn clear(&mut self) {
        unsafe { ffi::SJ_string_builder_clear(self.ptr.as_ptr()) }
    }

    /// Appends a boolean value (`true` or `false`).
    pub fn append_bool(&mut self, v: bool) {
        unsafe { ffi::SJ_string_builder_append_bool(self.ptr.as_ptr(), v) }
    }

    /// Appends a signed 64-bit integer.
    pub fn append_i64(&mut self, v: i64) {
        unsafe { ffi::SJ_string_builder_append_int64(self.ptr.as_ptr(), v) }
    }

    /// Appends an unsigned 64-bit integer.
    pub fn append_u64(&mut self, v: u64) {
        unsafe { ffi::SJ_string_builder_append_uint64(self.ptr.as_ptr(), v) }
    }

    /// Appends a 64-bit floating-point number.
    pub fn append_f64(&mut self, v: f64) {
        unsafe { ffi::SJ_string_builder_append_double(self.ptr.as_ptr(), v) }
    }

    /// Appends the JSON `null` literal.
    pub fn append_null(&mut self) {
        unsafe { ffi::SJ_string_builder_append_null(self.ptr.as_ptr()) }
    }

    /// Appends a single character (unescaped).
    pub fn append_char(&mut self, c: char) {
        unsafe { ffi::SJ_string_builder_append_char(self.ptr.as_ptr(), c as i8) }
    }

    /// Appends a string with JSON escaping and surrounding double quotes.
    /// This uses SIMD-accelerated escaping for performance.
    pub fn append_string(&mut self, s: &str) {
        unsafe {
            ffi::SJ_string_builder_escape_and_append_with_quotes(
                self.ptr.as_ptr(),
                s.as_ptr().cast(),
                s.len(),
            )
        }
    }

    /// Appends the opening brace `{` for a JSON object.
    pub fn start_object(&mut self) {
        unsafe { ffi::SJ_string_builder_start_object(self.ptr.as_ptr()) }
    }

    /// Appends the closing brace `}` for a JSON object.
    pub fn end_object(&mut self) {
        unsafe { ffi::SJ_string_builder_end_object(self.ptr.as_ptr()) }
    }

    /// Appends the opening bracket `[` for a JSON array.
    pub fn start_array(&mut self) {
        unsafe { ffi::SJ_string_builder_start_array(self.ptr.as_ptr()) }
    }

    /// Appends the closing bracket `]` for a JSON array.
    pub fn end_array(&mut self) {
        unsafe { ffi::SJ_string_builder_end_array(self.ptr.as_ptr()) }
    }

    /// Appends a comma `,` separator.
    pub fn append_comma(&mut self) {
        unsafe { ffi::SJ_string_builder_append_comma(self.ptr.as_ptr()) }
    }

    /// Appends a colon `:` separator (for object key-value pairs).
    pub fn append_colon(&mut self) {
        unsafe { ffi::SJ_string_builder_append_colon(self.ptr.as_ptr()) }
    }

    /// Appends raw bytes without escaping. Use with caution — the caller must
    /// ensure the content is valid JSON.
    pub fn append_raw(&mut self, s: &str) {
        unsafe { ffi::SJ_string_builder_append_raw(self.ptr.as_ptr(), s.as_ptr().cast(), s.len()) }
    }

    /// Returns a view of the written JSON buffer as a string slice.
    pub fn view(&self) -> Result<&str> {
        unsafe {
            let result = ffi::SJ_string_builder_view(self.ptr.as_ptr());
            if result.error != 0 {
                return Err(SimdJsonError::from(result.error));
            }
            let slice = std::slice::from_raw_parts(result.value.data.cast(), result.value.len);
            Ok(std::str::from_utf8_unchecked(slice))
        }
    }

    /// Validates that the content is valid UTF-8.
    pub fn validate_unicode(&self) -> bool {
        unsafe { ffi::SJ_string_builder_validate_unicode(self.ptr.as_ptr()) }
    }

    /// Returns the current size of the written JSON buffer in bytes.
    pub fn size(&self) -> usize {
        unsafe { ffi::SJ_string_builder_size(self.ptr.as_ptr()) }
    }

    /// Consumes the builder and returns the JSON string.
    pub fn into_string(self) -> Result<String> {
        let s = self.view()?.to_owned();
        Ok(s)
    }
}

impl Default for StringBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StringBuilder {
    fn drop(&mut self) {
        unsafe { ffi::SJ_string_builder_free(self.ptr.as_ptr()) }
    }
}

impl std::fmt::Display for StringBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.view() {
            Ok(s) => write!(f, "{s}"),
            Err(e) => write!(f, "<error: {e}>"),
        }
    }
}
