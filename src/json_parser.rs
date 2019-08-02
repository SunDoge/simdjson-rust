use super::error::SimdJsonError;
use super::parsed_json::{ParsedJson, DEFUALT_MAX_DEPTH};
use simdjson_sys as lib;

pub fn json_parse(
    s: &str,
    pj: &mut ParsedJson,
    realloc_if_needed: bool,
) -> Result<(), SimdJsonError> {
    // let ret = unsafe {
    //     lib::simdjson_json_parse_ptr(s.as_ptr(), s.len(), pj.get_mut(), realloc_if_needed)
    //     };

    let ret = unsafe {
        // if let Some(simdjson_json_parse) = lib::simdjson_json_parse_ptr {
        //     simdjson_json_parse(s.as_ptr(), s.len(), pj.get_mut(), realloc_if_needed)
        // } else {
        //     0
        // }
        lib::simdjson_json_parse_ptr.unwrap()(s.as_ptr(), s.len(), pj.get_mut(), realloc_if_needed)
    };
    if ret == 0 {
        Ok(())
    } else {
        Err(ret.into())
    }
}

// [TODO] Make another fn return Result<ParsedJson, SimdJsonError> instead of an invalid pj.
pub fn build_parsed_json(s: &str, realloc_if_needed: bool) -> ParsedJson {
    let mut pj = ParsedJson::new();
    let ok = pj.allocate_capacity(s.len(), DEFUALT_MAX_DEPTH);

    if ok {
        let res = json_parse(s, &mut pj, realloc_if_needed);
        assert_eq!(res.is_ok(), pj.is_valid());
    } else {
        eprintln!("failure during memory allocation ");
    }
    pj
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_parsed() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let pj = build_parsed_json(data, true);
        assert!(pj.is_valid());

        let mut pjh = pj.iterator();
        assert!(pjh.is_ok());

        // [TODO] impl inline methods
        // assert!(pjh.down());
        // assert_eq!(pjh.get_string(), "name");
        // assert!(pjh.move_forward());
        // assert_eq!(pjh.get_string(), "John Doe");
    }
}
