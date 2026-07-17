//! Integration tests for simdjson-rust tape parsing.

#[cfg(test)]
mod tape_integration {
    use simdjson_rust::{dom::Parser, tape::TapeType};

    #[test]
    fn parse_null_returns_root_node() {
        let mut p = Parser::default();
        let tape = p.parse_str("null").unwrap();
        assert_eq!(tape.tape_type(), Some(TapeType::Root));
    }

    #[test]
    fn parse_object_tape_structure() {
        let mut p = Parser::default();
        let tape = p.parse_str(r#"{"answer":42}"#).unwrap();
        assert_eq!(tape.tape_type(), Some(TapeType::Root));
    }

    #[test]
    fn parse_to_value_null() {
        let mut p = Parser::default();
        let v = p.parse_to_value("null").unwrap();
        assert!(v.is_null());
    }

    #[test]
    fn parse_to_value_bool() {
        let mut p = Parser::default();
        assert_eq!(p.parse_to_value("true").unwrap().as_bool(), Some(true));
        assert_eq!(p.parse_to_value("false").unwrap().as_bool(), Some(false));
    }

    #[test]
    fn parse_to_value_integers() {
        let mut p = Parser::default();
        assert_eq!(p.parse_to_value("42").unwrap().as_u64(), Some(42));
        assert_eq!(p.parse_to_value("-100").unwrap().as_i64(), Some(-100));
    }

    #[test]
    fn parse_to_value_float() {
        let mut p = Parser::default();
        let v = p.parse_to_value("1.5").unwrap();
        assert_eq!(v.as_f64().unwrap(), 1.5_f64);
    }

    #[test]
    fn parse_to_value_string() {
        let mut p = Parser::default();
        let v = p.parse_to_value(r#""hello world""#).unwrap();
        assert_eq!(v.as_str().unwrap(), "hello world");
    }

    #[test]
    fn parse_to_value_array() {
        let mut p = Parser::default();
        let v = p.parse_to_value("[1, 2, 3]").unwrap();
        assert!(v.as_array().is_some());
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0].as_u64(), Some(1));
        assert_eq!(arr[1].as_u64(), Some(2));
        assert_eq!(arr[2].as_u64(), Some(3));
    }

    #[test]
    fn parse_to_value_object() {
        let mut p = Parser::default();
        let v = p
            .parse_to_value(r#"{"answer": 42, "ratio": 1.5, "ok": true}"#)
            .unwrap();
        assert_eq!(v.get("answer").unwrap().as_u64(), Some(42));
        assert_eq!(v.get("ratio").unwrap().as_f64(), Some(1.5));
        assert_eq!(v.get("ok").unwrap().as_bool(), Some(true));
    }

    #[test]
    fn parse_to_value_nested() {
        let mut p = Parser::default();
        let v = p
            .parse_to_value(r#"{"outer": {"inner": [1, 2, {"deep": true}]}}"#)
            .unwrap();
        let outer = v.get("outer").unwrap();
        let inner = outer.get("inner").unwrap();
        let arr = inner.as_array().unwrap();
        assert_eq!(arr[0].as_u64(), Some(1));
        assert_eq!(arr[1].as_u64(), Some(2));
        assert_eq!(arr[2].get("deep").unwrap().as_bool(), Some(true));
    }

    #[test]
    fn parse_to_value_unicode() {
        let mut p = Parser::default();
        let v = p
            .parse_to_value(r#"{"emoji": "🦀", "chinese": "你好"}"#)
            .unwrap();
        assert_eq!(v.get("emoji").unwrap().as_str().unwrap(), "🦀");
        assert_eq!(v.get("chinese").unwrap().as_str().unwrap(), "你好");
    }

    #[cfg(feature = "serde")]
    mod with_serde {
        use serde::Deserialize;
        use simdjson_rust::serde::{from_bytes, from_str};

        #[derive(Debug, Deserialize, PartialEq)]
        struct Person {
            name: String,
            age: u64,
            active: bool,
        }

        #[test]
        fn from_str_struct() {
            let p: Person = from_str(r#"{"name": "Alice", "age": 30, "active": true}"#).unwrap();
            assert_eq!(
                p,
                Person {
                    name: "Alice".to_string(),
                    age: 30,
                    active: true
                }
            );
        }

        #[test]
        fn from_bytes_struct() {
            let json = br#"{"name": "Bob", "age": 25, "active": false}"#;
            let p: Person = from_bytes(json).unwrap();
            assert_eq!(
                p,
                Person {
                    name: "Bob".to_string(),
                    age: 25,
                    active: false
                }
            );
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct WithOptional {
            required: String,
            optional: Option<i64>,
        }

        #[test]
        fn from_str_with_null_optional() {
            let v: WithOptional = from_str(r#"{"required": "test", "optional": null}"#).unwrap();
            assert_eq!(v.required, "test");
            assert_eq!(v.optional, None);
        }

        #[test]
        fn from_str_with_some_optional() {
            let v: WithOptional = from_str(r#"{"required": "test", "optional": 99}"#).unwrap();
            assert_eq!(v.optional, Some(99));
        }

        #[derive(Debug, Deserialize, PartialEq)]
        #[serde(rename_all = "lowercase")]
        enum Status {
            Ok,
            Error,
        }

        #[test]
        fn from_str_unit_enum() {
            let s: Status = from_str(r#""ok""#).unwrap();
            assert_eq!(s, Status::Ok);
            let e: Status = from_str(r#""error""#).unwrap();
            assert_eq!(e, Status::Error);
        }

        #[test]
        fn from_str_vec_of_structs() {
            let v: Vec<Person> = from_str(
                r#"[{"name":"A","age":1,"active":true},{"name":"B","age":2,"active":false}]"#,
            )
            .unwrap();
            assert_eq!(v.len(), 2);
            assert_eq!(v[0].name, "A");
            assert_eq!(v[1].age, 2);
        }
    }
}
