use simdjson_rust::{error::Result, ondemand::parser::Parser, padded_string::load_padded_string};

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

    let ps = load_padded_string("simdjson-sys/simdjson/jsonexamples/twitter.json")?;
    let mut parser = Parser::default();
    let _tweets = parser.iterate(&ps)?;

    Ok(())
}
