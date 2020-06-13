pub mod element;
pub mod parser;



#[cfg(test)]
mod tests {
    use super::*;
    use super::element::Value;

    #[test]
    fn it_works() {
        let mut parser = parser::Parser::default();
        let value: bool = parser.parse_str("true").unwrap().get().unwrap();
        assert_eq!(value, true);
    }
}