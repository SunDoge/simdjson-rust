use std::{io::Read, path::Path};

use simdjson_sys as ffi;

pub fn make_padded_string(s: &str) -> String {
    let mut ps = String::with_capacity(s.len() + ffi::SIMDJSON_PADDING);
    ps.push_str(s);
    ps
}

pub fn load_padded_string<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let len = file.metadata()?.len() as usize;
    let mut buf = String::with_capacity(len + ffi::SIMDJSON_PADDING);
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod tests {}
