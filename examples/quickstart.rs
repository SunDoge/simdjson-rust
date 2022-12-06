use simdjson_rust::{ondemand::parser::Parser, padded_string::PaddedString};

fn main() {
    let mut parser = Parser::default();
    let json = PaddedString::load("json-examples/twitter.json").unwrap();
    let tweets = parser.iterate(&json).unwrap();
    // TODO
}
