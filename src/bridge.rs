#[cxx::bridge(namespace = ffi)]
pub mod ffi {

    unsafe extern "C++" {
        include!("include/simdjson_cxx.h");
        fn get_int() -> i32;

        type OndemandParser;
        fn new_ondemand_parser(max_capacity: usize) -> UniquePtr<OndemandParser>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(ffi::get_int(), 1);
    }

    #[test]
    fn new_parser() {
        let parser = ffi::new_ondemand_parser(1024);
        
    }
}
