use simdjson_rust::dom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = dom::Parser::default();
    let tweets = parser.load("json-examples/twitter.json")?;
    println!(
        "{} results.",
        tweets
            .at_key("search_metadata")?
            .at_key("count")?
            .get_u64()?
    );
    Ok(())
}
