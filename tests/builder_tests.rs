#![cfg(feature = "serde_impl")]

use serde::{Deserialize, Serialize};
use simdjson_rust::builder::StringBuilder;
use simdjson_rust::dom::Parser;
use simdjson_rust::prelude::*;
use simdjson_rust::serde::de::from_element;
use simdjson_rust::serde::ser::{to_string, to_string_with_capacity};

// ---------------------------------------------------------------------------
// Basic StringBuilder usage
// ---------------------------------------------------------------------------

#[test]
fn builder_basic_types() {
    let mut builder = StringBuilder::new();
    builder.start_array();
    builder.append_bool(true);
    builder.append_comma();
    builder.append_i64(42);
    builder.append_comma();
    builder.append_f64(3.15);
    builder.append_comma();
    builder.append_string("hello");
    builder.append_comma();
    builder.append_null();
    builder.end_array();

    let json = builder.view().unwrap();
    assert_eq!(json, r#"[true,42,3.15,"hello",null]"#);
}

#[test]
fn builder_object() {
    let mut builder = StringBuilder::new();
    builder.start_object();
    builder.append_string("name");
    builder.append_colon();
    builder.append_string("Alice");
    builder.append_comma();
    builder.append_string("age");
    builder.append_colon();
    builder.append_i64(30);
    builder.end_object();

    let json = builder.view().unwrap();
    assert_eq!(json, r#"{"name":"Alice","age":30}"#);
}

#[test]
fn builder_nested() {
    let mut builder = StringBuilder::new();
    builder.start_object();
    builder.append_string("data");
    builder.append_colon();
    builder.start_array();
    builder.append_i64(1);
    builder.append_comma();
    builder.append_i64(2);
    builder.append_comma();
    builder.append_i64(3);
    builder.end_array();
    builder.end_object();

    let json = builder.view().unwrap();
    assert_eq!(json, r#"{"data":[1,2,3]}"#);
}

#[test]
fn builder_clear_reuse() {
    let mut builder = StringBuilder::new();
    builder.append_i64(42);
    assert_eq!(builder.view().unwrap(), "42");

    builder.clear();
    builder.append_string("reset");
    assert_eq!(builder.view().unwrap(), r#""reset""#);
}

#[test]
fn builder_display() {
    let mut builder = StringBuilder::new();
    builder.append_bool(true);
    assert_eq!(format!("{builder}"), "true");
}

// ---------------------------------------------------------------------------
// Serde serialization
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Person {
    name: String,
    age: u32,
    active: bool,
}

#[test]
fn serialize_struct() {
    let person = Person {
        name: "Bob".to_string(),
        age: 25,
        active: true,
    };
    let json = to_string(&person).unwrap();
    assert_eq!(json, r#"{"name":"Bob","age":25,"active":true}"#);
}

#[test]
fn serialize_vec() {
    let vec = vec![1, 2, 3, 4, 5];
    let json = to_string(&vec).unwrap();
    assert_eq!(json, "[1,2,3,4,5]");
}

#[test]
fn serialize_nested() {
    #[derive(Serialize)]
    struct Outer {
        inner: Vec<i32>,
        flag: bool,
    }
    let outer = Outer {
        inner: vec![10, 20],
        flag: false,
    };
    let json = to_string(&outer).unwrap();
    assert_eq!(json, r#"{"inner":[10,20],"flag":false}"#);
}

#[test]
fn serialize_option_some() {
    #[derive(Serialize)]
    struct WithOption {
        value: Option<i32>,
    }
    let obj = WithOption { value: Some(42) };
    let json = to_string(&obj).unwrap();
    assert_eq!(json, r#"{"value":42}"#);
}

#[test]
fn serialize_option_none() {
    #[derive(Serialize)]
    struct WithOption {
        value: Option<i32>,
    }
    let obj = WithOption { value: None };
    let json = to_string(&obj).unwrap();
    assert_eq!(json, r#"{"value":null}"#);
}

#[test]
fn serialize_tuple() {
    let tuple = (1, "two", 3.0);
    let json = to_string(&tuple).unwrap();
    assert_eq!(json, r#"[1,"two",3.0]"#);
}

#[test]
fn serialize_unit_enum() {
    #[derive(Serialize)]
    enum Color {
        Red,
        Green,
        Blue,
    }
    let color = Color::Green;
    let json = to_string(&color).unwrap();
    assert_eq!(json, r#""Green""#);
}

#[test]
fn serialize_newtype_enum() {
    #[derive(Serialize)]
    enum Value {
        Number(i32),
    }
    let val = Value::Number(42);
    let json = to_string(&val).unwrap();
    assert_eq!(json, r#"{"Number":42}"#);
}

#[test]
fn serialize_struct_enum() {
    #[derive(Serialize)]
    enum Shape {
        Rectangle { width: f64, height: f64 },
    }
    let shape = Shape::Rectangle {
        width: 10.0,
        height: 5.0,
    };
    let json = to_string(&shape).unwrap();
    assert_eq!(json, r#"{"Rectangle":{"width":10.0,"height":5.0}}"#);
}

#[test]
fn serialize_hashmap() {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("key1", 100);
    map.insert("key2", 200);
    let json = to_string(&map).unwrap();
    // HashMap iteration order is not guaranteed, so check both possibilities
    assert!(
        json == r#"{"key1":100,"key2":200}"# || json == r#"{"key2":200,"key1":100}"#,
        "got: {json}"
    );
}

#[test]
fn serialize_escaped_strings() {
    #[derive(Serialize)]
    struct WithString {
        text: String,
    }
    let obj = WithString {
        text: "hello \"world\"\nline2".to_string(),
    };
    let json = to_string(&obj).unwrap();
    assert!(json.contains(r#"\""#), "should escape quotes");
    assert!(json.contains(r#"\n"#), "should escape newlines");
}

#[test]
fn serialize_unicode() {
    #[derive(Serialize)]
    struct WithEmoji {
        emoji: String,
    }
    let obj = WithEmoji {
        emoji: "Hello ❤️".to_string(),
    };
    let json = to_string(&obj).unwrap();
    assert!(json.contains("❤"), "should preserve unicode");
}

// ---------------------------------------------------------------------------
// Round-trip tests (serialize → parse → deserialize)
// ---------------------------------------------------------------------------

#[test]
fn round_trip_struct() {
    let original = Person {
        name: "Charlie".to_string(),
        age: 35,
        active: false,
    };

    let json = to_string(&original).unwrap();
    let mut parser = Parser::default();
    let ps = json.into_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let deserialized: Person = from_element(&elm).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn round_trip_nested() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Nested {
        values: Vec<i32>,
        metadata: Metadata,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Metadata {
        count: usize,
        valid: bool,
    }

    let original = Nested {
        values: vec![1, 2, 3],
        metadata: Metadata {
            count: 3,
            valid: true,
        },
    };

    let json = to_string(&original).unwrap();
    let mut parser = Parser::default();
    let ps = json.into_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let deserialized: Nested = from_element(&elm).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn round_trip_vec() {
    let original = vec![10, 20, 30, 40, 50];
    let json = to_string(&original).unwrap();
    let mut parser = Parser::default();
    let ps = json.into_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let deserialized: Vec<i32> = from_element(&elm).unwrap();
    assert_eq!(original, deserialized);
}

// ---------------------------------------------------------------------------
// Security: NaN/Infinity rejection
// ---------------------------------------------------------------------------

#[test]
fn serialize_nan_rejected() {
    let result = to_string(&f64::NAN);
    assert!(result.is_err(), "NaN should be rejected");
}

#[test]
fn serialize_infinity_rejected() {
    let result = to_string(&f64::INFINITY);
    assert!(result.is_err(), "Infinity should be rejected");
}

#[test]
fn serialize_neg_infinity_rejected() {
    let result = to_string(&f64::NEG_INFINITY);
    assert!(result.is_err(), "Negative infinity should be rejected");
}

// ---------------------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------------------

#[test]
fn serialize_empty_vec() {
    let vec: Vec<i32> = vec![];
    let json = to_string(&vec).unwrap();
    assert_eq!(json, "[]");
}

#[test]
fn serialize_empty_struct() {
    #[derive(Serialize)]
    struct Empty {}
    let obj = Empty {};
    let json = to_string(&obj).unwrap();
    assert_eq!(json, "{}");
}

#[test]
fn serialize_unit() {
    let json = to_string(&()).unwrap();
    assert_eq!(json, "null");
}

#[test]
fn serialize_newtype_struct() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Wrapper(i32);
    let original = Wrapper(42);
    let json = to_string(&original).unwrap();
    assert_eq!(json, "42");

    let mut parser = Parser::default();
    let ps = json.into_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let deserialized: Wrapper = from_element(&elm).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn serialize_char() {
    let json = to_string(&'A').unwrap();
    assert_eq!(json, r#""A""#);
}

#[test]
fn serialize_bytes() {
    let bytes: &[u8] = &[1, 2, 3];
    let json = to_string(&bytes).unwrap();
    assert_eq!(json, "[1,2,3]");
}

// ---------------------------------------------------------------------------
// Numeric boundaries
// ---------------------------------------------------------------------------

#[test]
fn serialize_i64_min_max() {
    let json = to_string(&i64::MIN).unwrap();
    assert_eq!(json, "-9223372036854775808");

    let json = to_string(&i64::MAX).unwrap();
    assert_eq!(json, "9223372036854775807");
}

#[test]
fn serialize_u64_max() {
    let json = to_string(&u64::MAX).unwrap();
    assert_eq!(json, "18446744073709551615");
}

// ---------------------------------------------------------------------------
// Capacity hint
// ---------------------------------------------------------------------------

#[test]
fn serialize_with_capacity() {
    let person = Person {
        name: "Dave".to_string(),
        age: 40,
        active: true,
    };
    let json = to_string_with_capacity(&person, 128).unwrap();
    assert_eq!(json, r#"{"name":"Dave","age":40,"active":true}"#);
}
