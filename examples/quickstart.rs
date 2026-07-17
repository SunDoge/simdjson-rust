use simdjson_rust::dom::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "search_metadata": {
            "count": 100
        }
    }"#;
    let mut parser = Parser::default();
    let v = parser.parse_to_value(json)?;
    let count = v
        .get("search_metadata")
        .unwrap()
        .get("count")
        .unwrap()
        .as_u64()
        .unwrap();
    println!("{} results.", count);

    Ok(())
}
