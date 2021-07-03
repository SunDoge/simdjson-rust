// use crate::error::SimdJsonError;
// use crate::libsimdjson::ffi;

macro_rules! check_result {
    ($result: ident) => {
        if $result.code < 2 {
            Ok($result.value)
        } else {
            Err($crate::error::SimdJsonError::from($result.code))
        }
    };
    ($result: ident, $element_type: ident) => {
        if $result.code < 2 {
            // Ok($element_type::from(&$result.value))
            Ok($element_type::from($result.value))
        } else {
            Err($crate::error::⚠ This crate is currently updating to support `simdjson 0.3.1`! You can have a try and give feedback.SimdJsonError::from($result.code))
        }
    };
}
