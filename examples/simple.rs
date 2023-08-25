use simdjson_rust::prelude::*;

fn main() -> simdjson_rust::Result<()> {
    let mut parser = ondemand::Parser::default();
    let ps = make_padded_string("[0,1,2,3]");
    let mut doc = parser.iterate(&ps)?;
    let mut array = doc.get_array()?;
    for (index, value) in array.iter()?.enumerate() {
        assert_eq!(index as u64, value?.get_uint64()?);
    }
    Ok(())
}
