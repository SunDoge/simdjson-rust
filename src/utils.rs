use simdjson_sys as ffi;
use std::ptr::NonNull;

#[inline]
pub fn string_view_to_str<'a>(sv: NonNull<ffi::STD_string_view>) -> &'a str {
    let s = unsafe {
        let s = std::slice::from_raw_parts(
            ffi::STD_string_view_data(sv.as_ptr()).cast(),
            ffi::STD_string_view_size(sv.as_ptr()),
        );
        std::str::from_utf8_unchecked(s)
    };
    unsafe { ffi::STD_string_view_free(sv.as_ptr()) };
    s
}
