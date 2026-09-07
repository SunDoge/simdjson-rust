use simdjson_rust::{
    dom::{Error as ParseError, Parser},
    error::SimdJsonError,
    tape::{Error, TapeRef},
};

#[test]
fn forged_strings_are_checked() {
    let word = [u64::from(b'"') << 56];
    let bytes = [1, 0, 0, 0, 0xff];
    assert!(matches!(
        TapeRef::new(&word, &bytes).get_string(),
        Err(Error::InvalidUtf8 { .. })
    ));
    for bytes in [&[][..], &[4, 0, 0, 0, b'x'][..]] {
        assert!(matches!(
            TapeRef::new(&word, bytes).get_string(),
            Err(Error::StringOutOfBounds { .. })
        ));
    }
    let huge = [(u64::from(b'"') << 56) | 0x00ff_ffff_ffff_ffff];
    assert!(TapeRef::new(&huge, &[]).get_string().is_err());
    assert!(
        TapeRef::new(&[u64::from(b'n') << 56], &bytes)
            .get_string()
            .is_err()
    );
    let mut cursor = TapeRef::new(&word, &bytes);
    cursor.seek_public(usize::MAX);
    assert!(matches!(
        cursor.get_string(),
        Err(Error::OutOfBounds { .. })
    ));
}

#[test]
fn parse_failure_invalidates_tape_and_reports_correct_error() {
    let mut parser = Parser::default();
    assert_eq!(parser.tape_ref().tape_type(), None);
    parser.parse_str(r#"{"old":"value"}"#).unwrap();
    assert!(matches!(
        parser.parse_str(""),
        Err(ParseError::SimdJson {
            source: SimdJsonError::Empty
        })
    ));
    assert_eq!(parser.tape_ref().tape_type(), None);
    assert!(matches!(
        parser.parse_bytes(&[b'"', 0xff, b'"']),
        Err(ParseError::SimdJson {
            source: SimdJsonError::Utf8Error
        })
    ));
    assert_eq!(parser.tape_ref().tape_type(), None);
    parser.parse_str("null").unwrap();
    assert!(parser.parse_padded(&mut String::from("null")).is_err());
    assert_eq!(parser.tape_ref().tape_type(), None);
    assert_eq!(
        parser.parse_to_value(r#""恢复""#).unwrap().as_str(),
        Some("恢复")
    );
}

#[test]
fn padding_preserves_input_and_parser_reuse() {
    let mut parser = Parser::default();
    let mut text = String::with_capacity(128);
    text.push_str(r#""你好🌏""#);
    let original = text.clone();
    for _ in 0..3 {
        let mut tape = parser.parse_padded(&mut text).unwrap();
        assert_eq!(tape.parse_value().unwrap().as_str(), Some("你好🌏"));
        assert_eq!(text, original);
        parser.parse_str("[1,2,3]").unwrap();
    }
    let mut padded = b"true".to_vec();
    padded.resize(4 + simdjson_sys::SIMDJSON_PADDING, 0);
    // SAFETY: 64 initialized padding bytes follow the four-byte input.
    let mut tape = unsafe { parser.parse_bytes_with_padding(&padded[..4]) }.unwrap();
    assert_eq!(tape.parse_value().unwrap().as_bool(), Some(true));
}
