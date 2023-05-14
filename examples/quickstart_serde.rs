use std::dbg;

use serde::{Deserialize, Deserializer};
use simdjson_rust::ondemand::parser::Parser;

#[derive(Debug, Deserialize)]
struct Data(Vec<u64>);

fn main() {
    let data = r#"[1,2,3]"#;
    let mut parser = Parser::default();
    let ps = data.into();
    let mut doc = parser.iterate(&ps).unwrap();
    let mut value = doc.get_value().unwrap();
    let v = Data::deserialize(&mut value).unwrap();
    dbg!(v);
}
