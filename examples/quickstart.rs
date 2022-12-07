use simdjson_rust::{error::Result, ondemand::parser::Parser, padded_string::PaddedString};

fn main() -> Result<()> {
    let mut parser = Parser::default();
    let json = PaddedString::load("json-examples/twitter.json")?;
    let mut tweets = parser.iterate(&json)?;
    let v = tweets.at_pointer("/search_metadata/count")?.get_u64()?;
    println! {"{} results.", v};
    Ok(())
}
