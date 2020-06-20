#[macro_use]
mod utils;

pub mod array;
// pub mod document_stream;
pub mod element;
pub mod object;
pub mod parser;

pub use self::parser::Parser;

#[cfg(test)]
mod tests {
    use super::*;
    // use super::element::GetValue;

    #[test]
    fn it_works() {
        let mut parser = parser::Parser::default();
        let elm = parser.parse("true").unwrap();
        let value: bool = elm.get_bool().unwrap();
        assert_eq!(value, true);
        assert_eq!(elm.get_type(), element::ElementType::Bool);
    }
}
