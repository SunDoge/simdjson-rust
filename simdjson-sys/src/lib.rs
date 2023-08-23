#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            let s = "hello world";
            let ps = SJ_padded_string_new(s.as_ptr().cast(), s.len());
            SJ_padded_string_free(ps);
        }
    }
}
