use simdjson_rust::{ondemand, prelude::*, Result};

fn main() -> Result<()> {
    let ps = load_padded_string("simdjson-sys/simdjson/jsonexamples/twitter.json")?;
    let mut parser = ondemand::Parser::default();
    let mut tweets = parser.iterate(&ps)?;
    println!(
        "{} results.",
        tweets.at_pointer("/search_metadata/count")?.get_uint64()?
    );

    Ok(())
}
