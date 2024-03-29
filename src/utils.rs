use std::ptr::NonNull;

use simdjson_sys as ffi;

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

#[inline]
pub fn string_view_struct_to_str<'a>(sv: ffi::SJ_string_view) -> &'a str {
    unsafe {
        let s = std::slice::from_raw_parts(sv.data.cast(), sv.len);
        std::str::from_utf8_unchecked(s)
    }
}
