#![cfg(feature = "serde_impl")]

use serde::{Deserialize, Serialize};
use simdjson_rust::dom::Parser;
use simdjson_rust::prelude::*;
use simdjson_rust::serde::de::from_element;
use simdjson_rust::serde::value::element_to_value;

// ---------------------------------------------------------------------------
// Basic type deserialization
// ---------------------------------------------------------------------------

#[test]
fn deserialize_bool_array() {
    let mut parser = Parser::default();
    let ps = "[true, false, true]".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let v: Vec<bool> = from_element(&elm).unwrap();
    assert_eq!(v, vec![true, false, true]);
}

#[test]
fn deserialize_integer_array() {
    let mut parser = Parser::default();
    let ps = "[1, 2, 3]".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let v: Vec<u64> = from_element(&elm).unwrap();
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
fn deserialize_signed_integer_array() {
    let mut parser = Parser::default();
    let ps = "[-1, 0, 42]".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let v: Vec<i64> = from_element(&elm).unwrap();
    assert_eq!(v, vec![-1, 0, 42]);
}

#[test]
fn deserialize_float_array() {
    let mut parser = Parser::default();
    let ps = "[1.5, 2.25, 3.125]".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let v: Vec<f64> = from_element(&elm).unwrap();
    assert_eq!(v, vec![1.5, 2.25, 3.125]);
}

#[test]
fn deserialize_string_array() {
    let mut parser = Parser::default();
    let ps = r#"["hello", "world"]"#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let v: Vec<String> = from_element(&elm).unwrap();
    assert_eq!(v, vec!["hello".to_string(), "world".to_string()]);
}

// ---------------------------------------------------------------------------
// Struct deserialization
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Simple {
    name: String,
    age: u32,
    active: bool,
}

#[test]
fn deserialize_struct() {
    let mut parser = Parser::default();
    let ps = r#"{"name": "Alice", "age": 30, "active": true}"#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let s: Simple = from_element(&elm).unwrap();
    assert_eq!(
        s,
        Simple {
            name: "Alice".to_string(),
            age: 30,
            active: true,
        }
    );
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Nested {
    inner: Simple,
    tags: Vec<String>,
}

#[test]
fn deserialize_nested_struct() {
    let mut parser = Parser::default();
    let ps = r#"{"inner": {"name": "Bob", "age": 25, "active": false}, "tags": ["rust", "json"]}"#
        .to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let n: Nested = from_element(&elm).unwrap();
    assert_eq!(
        n,
        Nested {
            inner: Simple {
                name: "Bob".to_string(),
                age: 25,
                active: false,
            },
            tags: vec!["rust".to_string(), "json".to_string()],
        }
    );
}

// ---------------------------------------------------------------------------
// Optional fields
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, PartialEq)]
struct WithOptional {
    required: String,
    optional: Option<i64>,
}

#[test]
fn deserialize_option_some() {
    let mut parser = Parser::default();
    let ps = r#"{"required": "yes", "optional": 42}"#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let w: WithOptional = from_element(&elm).unwrap();
    assert_eq!(
        w,
        WithOptional {
            required: "yes".to_string(),
            optional: Some(42),
        }
    );
}

#[test]
fn deserialize_option_null() {
    let mut parser = Parser::default();
    let ps = r#"{"required": "yes", "optional": null}"#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let w: WithOptional = from_element(&elm).unwrap();
    assert_eq!(
        w,
        WithOptional {
            required: "yes".to_string(),
            optional: None,
        }
    );
}

// ---------------------------------------------------------------------------
// Enum deserialization
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[test]
fn deserialize_unit_enum() {
    let mut parser = Parser::default();
    let ps = r#""Red""#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let c: Color = from_element(&elm).unwrap();
    assert_eq!(c, Color::Red);
}

#[derive(Debug, Deserialize, PartialEq)]
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
}

#[test]
fn deserialize_newtype_enum() {
    let mut parser = Parser::default();
    let ps = r#"{"Circle": 3.15}"#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let s: Shape = from_element(&elm).unwrap();
    assert_eq!(s, Shape::Circle(3.15));
}

#[test]
fn deserialize_struct_enum() {
    let mut parser = Parser::default();
    let ps = r#"{"Rectangle": {"width": 10.0, "height": 5.0}}"#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let s: Shape = from_element(&elm).unwrap();
    assert_eq!(
        s,
        Shape::Rectangle {
            width: 10.0,
            height: 5.0,
        }
    );
}

// ---------------------------------------------------------------------------
// Null / unit
// ---------------------------------------------------------------------------

#[test]
fn deserialize_null() {
    let mut parser = Parser::default();
    let ps = "null".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let v: () = from_element(&elm).unwrap();
    assert_eq!(v, ());
}

// ---------------------------------------------------------------------------
// Tuple
// ---------------------------------------------------------------------------

#[test]
fn deserialize_tuple() {
    let mut parser = Parser::default();
    let ps = r#"[1, "two", 3.0]"#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let v: (u64, String, f64) = from_element(&elm).unwrap();
    assert_eq!(v, (1, "two".to_string(), 3.0));
}

// ---------------------------------------------------------------------------
// HashMap
// ---------------------------------------------------------------------------

#[test]
fn deserialize_hashmap() {
    use std::collections::HashMap;
    let mut parser = Parser::default();
    let ps = r#"{"a": 1, "b": 2}"#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let m: HashMap<String, u64> = from_element(&elm).unwrap();
    assert_eq!(m.get("a"), Some(&1));
    assert_eq!(m.get("b"), Some(&2));
    assert_eq!(m.len(), 2);
}

// ---------------------------------------------------------------------------
// element_to_value conversion
// ---------------------------------------------------------------------------

#[test]
fn element_to_value_scalars() {
    {
        let mut parser = Parser::default();
        let ps = "42".to_padded_string();
        let elm = parser.parse(&ps).unwrap();
        let v = element_to_value(&elm).unwrap();
        assert_eq!(v, serde_json::json!(42));
    }
    {
        let mut parser = Parser::default();
        let ps = "true".to_padded_string();
        let elm = parser.parse(&ps).unwrap();
        let v = element_to_value(&elm).unwrap();
        assert_eq!(v, serde_json::json!(true));
    }
    {
        let mut parser = Parser::default();
        let ps = "null".to_padded_string();
        let elm = parser.parse(&ps).unwrap();
        let v = element_to_value(&elm).unwrap();
        assert_eq!(v, serde_json::json!(null));
    }
    {
        let mut parser = Parser::default();
        let ps = r#""hello""#.to_padded_string();
        let elm = parser.parse(&ps).unwrap();
        let v = element_to_value(&elm).unwrap();
        assert_eq!(v, serde_json::json!("hello"));
    }
}

#[test]
fn element_to_value_nested() {
    let mut parser = Parser::default();
    let ps = r#"{"key": [1, 2, {"nested": true}]}"#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let v = element_to_value(&elm).unwrap();
    assert_eq!(v, serde_json::json!({"key": [1, 2, {"nested": true}]}));
}

// ---------------------------------------------------------------------------
// Round-trip: parse → serde_json::Value → to_string → parse again
// ---------------------------------------------------------------------------

#[test]
fn round_trip_via_serde_json() {
    let input = r#"{"name":"test","values":[1,2,3],"nested":{"flag":true,"nothing":null}}"#;
    let mut parser = Parser::default();
    let ps = input.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let value = element_to_value(&elm).unwrap();
    let serialized = serde_json::to_string(&value).unwrap();

    let reparsed: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    assert_eq!(value, reparsed);
}

// ---------------------------------------------------------------------------
// Display impl
// ---------------------------------------------------------------------------

#[test]
fn element_display() {
    let mut parser = Parser::default();
    let ps = r#"{"a": 1}"#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let displayed = format!("{elm}");
    let reparsed: serde_json::Value = serde_json::from_str(&displayed).unwrap();
    assert_eq!(reparsed, serde_json::json!({"a": 1}));
}

// ---------------------------------------------------------------------------
// Security: numeric overflow protection
// ---------------------------------------------------------------------------

#[test]
fn overflow_u8() {
    let mut parser = Parser::default();
    let ps = "256".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let result: Result<u8, _> = from_element(&elm);
    assert!(result.is_err(), "256 should not fit in u8");
}

#[test]
fn overflow_i8() {
    let mut parser = Parser::default();
    let ps = "128".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let result: Result<i8, _> = from_element(&elm);
    assert!(result.is_err(), "128 should not fit in i8");
}

#[test]
fn overflow_i8_negative() {
    let mut parser = Parser::default();
    let ps = "-129".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let result: Result<i8, _> = from_element(&elm);
    assert!(result.is_err(), "-129 should not fit in i8");
}

#[test]
fn overflow_u16() {
    let mut parser = Parser::default();
    let ps = "65536".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let result: Result<u16, _> = from_element(&elm);
    assert!(result.is_err(), "65536 should not fit in u16");
}

#[test]
fn overflow_u32() {
    let mut parser = Parser::default();
    let ps = "4294967296".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let result: Result<u32, _> = from_element(&elm);
    assert!(result.is_err(), "4294967296 should not fit in u32");
}

#[test]
fn overflow_i32() {
    let mut parser = Parser::default();
    let ps = "2147483648".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let result: Result<i32, _> = from_element(&elm);
    assert!(result.is_err(), "2147483648 should not fit in i32");
}

// ---------------------------------------------------------------------------
// Security: deep nesting protection (element_to_value)
// ---------------------------------------------------------------------------

#[test]
fn deep_nesting_protection() {
    let depth = 200;
    let mut json = String::new();
    for _ in 0..depth {
        json.push_str(r#"{"a":"#);
    }
    json.push('1');
    for _ in 0..depth {
        json.push('}');
    }
    let mut parser = Parser::default();
    let ps = json.into_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let result = element_to_value(&elm);
    assert!(
        result.is_err(),
        "nesting depth of {depth} should exceed the limit"
    );
}

// ---------------------------------------------------------------------------
// Security: escaped strings are handled correctly
// ---------------------------------------------------------------------------

#[test]
fn escaped_strings() {
    let mut parser = Parser::default();
    let ps = r#"{"msg": "hello \"world\"\nline2"}"#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let v = element_to_value(&elm).unwrap();
    let s = v["msg"].as_str().unwrap();
    assert!(s.contains('"'), "should contain literal quote");
    assert!(s.contains('\n'), "should contain literal newline");
}

#[test]
fn unicode_strings() {
    let mut parser = Parser::default();
    let ps = r#"{"emoji": "Hello \u2764\uFE0F"}"#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let v = element_to_value(&elm).unwrap();
    let s = v["emoji"].as_str().unwrap();
    assert!(s.contains('❤'), "should contain heart emoji");
}

// ---------------------------------------------------------------------------
// Security: type mismatch produces errors, not panics
// ---------------------------------------------------------------------------

#[test]
fn type_mismatch_string_as_int() {
    let mut parser = Parser::default();
    let ps = r#""not_a_number""#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let result: Result<i64, _> = from_element(&elm);
    assert!(result.is_err());
}

#[test]
fn type_mismatch_int_as_string() {
    let mut parser = Parser::default();
    let ps = "42".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let result: Result<String, _> = from_element(&elm);
    assert!(result.is_err());
}

#[test]
fn type_mismatch_array_as_struct() {
    let mut parser = Parser::default();
    let ps = "[1, 2, 3]".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let result: Result<Simple, _> = from_element(&elm);
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Security: empty inputs
// ---------------------------------------------------------------------------

#[test]
fn empty_array() {
    let mut parser = Parser::default();
    let ps = "[]".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let v: Vec<i64> = from_element(&elm).unwrap();
    assert!(v.is_empty());
}

#[test]
fn empty_object() {
    use std::collections::HashMap;
    let mut parser = Parser::default();
    let ps = "{}".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let m: HashMap<String, serde_json::Value> = from_element(&elm).unwrap();
    assert!(m.is_empty());
}

// ---------------------------------------------------------------------------
// Large integers at boundaries
// ---------------------------------------------------------------------------

#[test]
fn max_u64() {
    let mut parser = Parser::default();
    let ps = "18446744073709551615".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let v: u64 = from_element(&elm).unwrap();
    assert_eq!(v, u64::MAX);
}

#[test]
fn min_i64() {
    let mut parser = Parser::default();
    let ps = "-9223372036854775808".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let v: i64 = from_element(&elm).unwrap();
    assert_eq!(v, i64::MIN);
}

#[test]
fn max_i64() {
    let mut parser = Parser::default();
    let ps = "9223372036854775807".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let v: i64 = from_element(&elm).unwrap();
    assert_eq!(v, i64::MAX);
}

// ---------------------------------------------------------------------------
// Char deserialization
// ---------------------------------------------------------------------------

#[test]
fn deserialize_char_single() {
    let mut parser = Parser::default();
    let ps = r#""A""#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let c: char = from_element(&elm).unwrap();
    assert_eq!(c, 'A');
}

#[test]
fn deserialize_char_multi_rejects() {
    let mut parser = Parser::default();
    let ps = r#""AB""#.to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let result: Result<char, _> = from_element(&elm);
    assert!(result.is_err(), "multi-char string should fail for char");
}

// ---------------------------------------------------------------------------
// Newtype struct
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, PartialEq)]
struct Wrapper(u64);

#[test]
fn deserialize_newtype_struct() {
    let mut parser = Parser::default();
    let ps = "42".to_padded_string();
    let elm = parser.parse(&ps).unwrap();
    let w: Wrapper = from_element(&elm).unwrap();
    assert_eq!(w, Wrapper(42));
}

// ---------------------------------------------------------------------------
// Memory leak regression test: at_pointer method
// ---------------------------------------------------------------------------

#[test]
fn at_pointer_functionality_test() {
    // Test that at_pointer works correctly and doesn't cause issues
    let json = r#"{
        "users": [
            {"name": "Alice", "age": 30, "active": true},
            {"name": "Bob", "age": 25, "active": false}
        ],
        "metadata": {
            "version": "1.0",
            "count": 2
        },
        "settings": {
            "theme": "dark",
            "notifications": {
                "email": true,
                "push": false
            }
        }
    }"#;

    let mut parser = Parser::default();
    let ps = json.to_padded_string();
    let root = parser.parse(&ps).unwrap();

    // Test basic at_pointer functionality
    let users = root.at_pointer("/users").unwrap();
    assert!(users.get_array().is_ok());

    let first_user = root.at_pointer("/users/0").unwrap();
    assert!(first_user.get_object().is_ok());

    let user_name = root.at_pointer("/users/0/name").unwrap();
    assert_eq!(user_name.get_string().unwrap(), "Alice");

    let user_age = root.at_pointer("/users/0/age").unwrap();
    assert_eq!(user_age.get_uint64().unwrap(), 30);

    let metadata_version = root.at_pointer("/metadata/version").unwrap();
    assert_eq!(metadata_version.get_string().unwrap(), "1.0");

    let count = root.at_pointer("/metadata/count").unwrap();
    assert_eq!(count.get_uint64().unwrap(), 2);

    // Test nested at_pointer calls
    let notifications = root.at_pointer("/settings/notifications").unwrap();
    let email_setting = notifications.at_pointer("/email").unwrap();
    assert_eq!(email_setting.get_bool().unwrap(), true);

    // Test that elements are still accessible after multiple at_pointer calls
    let push_setting = root.at_pointer("/settings/notifications/push").unwrap();
    assert_eq!(push_setting.get_bool().unwrap(), false);

    // Test round-trip through serde to ensure no corruption
    let value = element_to_value(&root).unwrap();
    let serialized = serde_json::to_string(&value).unwrap();
    let reparsed: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    assert_eq!(value, reparsed);
}

#[test]
fn at_pointer_extensive_usage() {
    // Create a deeply nested structure and access many elements via at_pointer
    // This stress test helps ensure no memory corruption or leaks
    let mut json = String::from("{\"root\": ");
    for i in 0..10 {
        json.push_str(&format!("{{\"level_{}\": ", i));
    }
    json.push_str("\"deepest_value\"");
    for _ in 0..10 {
        json.push_str("}");
    }
    json.push('}');

    let mut parser = Parser::default();
    let ps = json.to_padded_string();
    let root = parser.parse(&ps).unwrap();

    // Access elements at various depths
    let mut current_path = "/root".to_string();
    for i in 0..10 {
        let level_path = format!("{}/level_{}", current_path, i);
        let element = root.at_pointer(&level_path).unwrap();
        assert!(element.get_object().is_ok());
        current_path = level_path;
    }

    // Access the final value
    let value_path = format!("{}/level_9", current_path);
    let deepest = root.at_pointer(&(value_path + "/level_9")).unwrap();
    assert_eq!(deepest.get_string().unwrap(), "deepest_value");
}
