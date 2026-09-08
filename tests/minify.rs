use simdjson_rust::{error::SimdJsonError, minify, minify_bytes};

#[test]
fn preserves_strings_escapes_unicode_and_number_spelling() {
    let input =
        " \n { \"text\": \"你好 🌏 a b\", \"esc\": \"a\\\" b\\\\ c\\t\\n\", \"n\": 1.00e+02 } \r\t";
    let expected = r#"{"text":"你好 🌏 a b","esc":"a\" b\\ c\t\n","n":1.00e+02}"#;
    assert_eq!(minify(input).unwrap(), expected);
    assert_eq!(minify_bytes(input.as_bytes()).unwrap(), expected.as_bytes());
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(input).unwrap(),
        serde_json::from_str::<serde_json::Value>(expected).unwrap()
    );
}

#[test]
fn empty_whitespace_scalars_and_unvalidated_input() {
    for (input, expected) in [
        ("", ""),
        (" \t\r\n", ""),
        (" true ", "true"),
        (" [1, 2] ", "[1,2]"),
        (" { invalid : } ", "{invalid:}"),
    ] {
        assert_eq!(minify(input).unwrap(), expected);
    }
    assert_eq!(
        minify_bytes(&[b' ', b'"', 0xff, b'"', b' ']).unwrap(),
        [b'"', 0xff, b'"']
    );
}

#[test]
fn reports_unclosed_strings() {
    assert_eq!(
        minify(" { \"a\": \"unterminated "),
        Err(SimdJsonError::UnclosedString)
    );
}

#[test]
fn simd_boundaries_and_escaped_quotes() {
    for len in 0..=260 {
        let value = format!("{}\\\" 你好 \n", "a ".repeat(len));
        let json = serde_json::to_string(&value).unwrap();
        let input = format!("{}[ {json}, 1 ]{}", " ".repeat(len), "\n".repeat(len));
        assert_eq!(minify(&input).unwrap(), format!("[{json},1]"));
    }
}

#[test]
fn ffi_checks_output_capacity_and_resets_length_on_error() {
    use simdjson_sys::minify_ffi;
    let mut short = [0xaa; 2];
    let mut written = 99;
    let code = minify_ffi::minify(b" [1] ", &mut short, &mut written);
    assert_eq!(
        SimdJsonError::from_code(code),
        Some(SimdJsonError::Capacity)
    );
    assert_eq!(short, [0xaa; 2]);
    assert_eq!(written, 0);
    let mut output = [0; 8];
    assert_eq!(minify_ffi::minify(b" [1] ", &mut output, &mut written), 0);
    assert_eq!(&output[..written], b"[1]");
    let code = minify_ffi::minify(b"\"oops", &mut output, &mut written);
    assert_eq!(
        SimdJsonError::from_code(code),
        Some(SimdJsonError::UnclosedString)
    );
    assert_eq!(written, 0);
    assert_eq!(minify_ffi::minify(b"", &mut [], &mut written), 0);
    assert_eq!(written, 0);
}
