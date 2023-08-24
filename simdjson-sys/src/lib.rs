#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const SIMDJSON_PADDING: usize = 64;
pub const SIMDJSON_MAXSIZE_BYTES: usize = 0xFFFFFFFF;
