pub mod de;

#[cfg(test)]
mod tests {
    use super::*;
    // use super::element::GetValue;
    use crate::dom::parser::Parser;
    use serde::Deserialize;

    #[test]
    fn test_element() -> Result<(), Box<dyn std::error::Error>> {
        

        let mut parser = Parser::default();
        let elm = parser.parse_str(r#"[true, false]"#)?;
        let a: Vec<bool> = de::from_element(&elm)?;
        assert_eq!(vec![true, false], a);

        #[derive(Debug, Deserialize)]
        struct A {
            field1: bool,
        }
        let elm = parser.parse_str(r#"{"field1": false}"#)?;
        let a: A = de::from_element(&elm)?;
        assert_eq!(a.field1, false);

        Ok(())
    }
}
