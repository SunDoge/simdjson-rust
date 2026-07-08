#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use simdjson_rust::serde::from_str;

#[cfg(feature = "serde")]
#[derive(Debug, Deserialize, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use simdjson_rust::dom::Parser;

    // 1. Parsing directly to our zero-copy DOM Value enum (Always available)
    let mut parser = Parser::default();
    let val = parser.parse_to_value(r#"{"points": [10, 20], "name": "rust"}"#)?;

    let name = val.get("name").unwrap().as_str().unwrap();
    let points = val.get("points").unwrap().as_array().unwrap();
    println!("Name: {}", name);
    println!(
        "Points: [{}, {}]",
        points[0].as_i64().unwrap(),
        points[1].as_i64().unwrap()
    );

    // 2. Direct zero-copy deserialization into custom structs via Serde
    #[cfg(feature = "serde")]
    {
        let point: Point = from_str(r#"{"x": 100, "y": -200}"#)?;
        println!("Deserialized point: {:?}", point);
    }

    Ok(())
}
