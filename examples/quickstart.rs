use simdjson_rust::{error::Result, padded_string::PaddedString};

fn main() -> Result<()> {
    // let mut parser = dom::Parser::default();
    // let tweets = parser.load("json-examples/twitter.json")?;
    // println!(
    //     "{} results.",
    //     tweets
    //         .at_key("search_metadata")?
    //         .at_key("count")?
    //         .get_u64()?
    // );

    let ps = PaddedString::load("json-examples/twitter.json")?;

    Ok(())
}
