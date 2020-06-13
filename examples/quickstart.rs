use simdjson_rust::dom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = dom::parser::Parser::default();
    let tweets = parser.load("json-examples/twitter.json")?;
    println!(
        "{} results.",
        tweets.at("search_metadata/count")?.get_u64()?
    );
    Ok(())
}
