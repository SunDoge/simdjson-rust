pub mod de;

#[cfg(test)]
mod tests {
    use super::*;
    // use super::element::GetValue;
    use crate::dom::parser::Parser;
    use serde::Deserialize;

    #[test]
    fn test_element() {
        #[derive(Debug, Deserialize)]
        struct A {
            field1: bool,
        }

        let mut parser = Parser::default();
        // let elm = parser.parse_str(r#"{"field1": true}"#).unwrap();

        let elm = parser.parse_str(r#"[true, false]"#).unwrap();

        let a: Vec<bool> = de::from_element(&elm).unwrap();
        println!("{:?}", a);

        let elm = parser.parse_str(r#"{"field1": false}"#).unwrap();
        let a: A = de::from_element(&elm).unwrap();
        println!("{:?}", a);
    }
}
