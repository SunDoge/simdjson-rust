use simdjson_rust::prelude::*;
use simdjson_rust::{dom, ondemand};

fn main() -> simdjson_rust::Result<()> {
    let ps = make_padded_string("[0,1,2,3]");

    // ondemand api.
    {
        let mut parser = ondemand::Parser::default();
        let mut doc = parser.iterate(&ps)?;
        let mut array = doc.get_array()?;
        for (index, value) in array.iter()?.enumerate() {
            assert_eq!(index as u64, value?.get_uint64()?);
        }
    }

    // dom api.
    {
        let mut parser = dom::Parser::default();
        let elem = parser.parse(&ps)?;
        let arr = elem.get_array()?;
        for (index, value) in arr.iter().enumerate() {
            assert_eq!(index as u64, value.get_uint64()?);
        }
    }

    Ok(())
}
