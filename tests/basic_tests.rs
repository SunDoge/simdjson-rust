#[cfg(test)]
mod number_tests {

    use simdjson_rust::dom;
    use simdjson_rust::error::SimdJsonError;

    // ulp distance
    // Marc B. Reynolds, 2016-2019
    // Public Domain under http://unlicense.org, see link for details.
    // adapted by D. Lemire
    #[allow(dead_code)]
    fn f64_ulp_dist(a: f64, b: f64) -> u64 {
        // let ua: u64 = transmute(a);
        // let ub: u64 = transmute(b);
        let (ua, ub): (u64, u64) = (a.to_bits(), b.to_bits());

        if (ub ^ ua) as i64 >= 0 {
            if (ua - ub) as i64 >= 0 {
                ua - ub
            } else {
                ub - ua
            }
        } else {
            ua + ub + 0x80000000
        }
    }

    #[test]
    fn small_integers() -> Result<(), SimdJsonError> {
        let mut parser = dom::parser::Parser::default();

        for _m in 10..20 {
            for i in -1024..1024 {
                let s = i.to_string();
                let actual = parser.parse(&s)?.get_i64()?;

                assert_eq!(actual, i);
            }
        }

        Ok(())
    }
}
