#[cfg(test)]
mod number_tests {

    use simdjson_rust::dom;
    use simdjson_rust::error::SimdJsonError;
    use std::mem::transmute;

    // ulp distance
    // Marc B. Reynolds, 2016-2019
    // Public Domain under http://unlicense.org, see link for details.
    // adapted by D. Lemire
    fn f64_ulp_dist(a: f64, b: f64) -> u64 {
        // let ua: u64 = transmute(a);
        // let ub: u64 = transmute(b);
        let (ua, ub): (u64, u64) = unsafe { (transmute(a), transmute(b)) };

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

    fn double_max_digits10() -> usize {
        let x = f64::MANTISSA_DIGITS as f32 * 2.0_f32.log10() + 1.;
        x.ceil() as usize
    }

    #[test]
    fn small_integers() -> Result<(), SimdJsonError> {
        let mut parser = dom::Parser::default();

        for _m in 10..20 {
            for i in -1024..1024 {
                let s = i.to_string();
                let actual = parser.parse(&s)?.get_i64()?;

                assert_eq!(actual, i);
            }
        }

        Ok(())
    }

    #[test]
    fn powers_of_two() -> Result<(), SimdJsonError> {
        let mut parser = dom::Parser::default();

        let mut max_ulp = 0;
        for i in -1075..1024 {
            let expected = (i as f64).powi(2);
            let buf = format!("{:.*e}", double_max_digits10() - 1, expected);
            let actual = parser.parse(&buf)?.get_f64()?;
            let ulp = f64_ulp_dist(actual, expected);
            if ulp > max_ulp {
                max_ulp = ulp;
            }
            assert_eq!(ulp, 0)
        }

        Ok(())
    }
}
