use crate::error::SimdJsonError;

/// Remove JSON whitespace outside strings, preserving string contents and
/// number spellings. No parser or input padding is required.
///
/// This is **not JSON validation**: malformed syntax can succeed. Some errors,
/// such as an unclosed string, are detected by simdjson. Empty input succeeds.
///
/// ```
/// use simdjson_rust::minify;
/// assert_eq!(minify(r#" { "hello": "a b", "n": 1.00 } "#).unwrap(),
///            r#"{"hello":"a b","n":1.00}"#);
/// ```
pub fn minify(json: &str) -> Result<String, SimdJsonError> {
    let bytes = minify_bytes(json.as_bytes())?;
    // Minification removes only ASCII whitespace; valid input UTF-8 is preserved.
    // Keep this conversion checked even if the upstream implementation changes.
    String::from_utf8(bytes).map_err(|_| SimdJsonError::Utf8Error)
}

/// Minify bytes without validating JSON syntax or UTF-8.
///
/// Like [`minify`], this removes whitespace outside strings and requires no
/// padding. Invalid UTF-8 bytes are preserved. Returns a newly allocated buffer;
/// on failure, no partial result is returned.
pub fn minify_bytes(json: &[u8]) -> Result<Vec<u8>, SimdJsonError> {
    let mut output = vec![0; json.len()];
    let mut written = 0;
    SimdJsonError::check_code(simdjson_sys::minify_ffi::minify(
        json,
        &mut output,
        &mut written,
    ))?;
    output.truncate(written);
    Ok(output)
}
