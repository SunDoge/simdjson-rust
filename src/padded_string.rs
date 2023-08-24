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

pub trait ToPaddedString {
    fn to_padded_string(&self) -> String;
}

pub trait IntoPaddedString {
    fn into_padded_string(self) -> String;
}

impl ToPaddedString for &str {
    fn to_padded_string(&self) -> String {
        make_padded_string(self)
    }
}

impl ToPaddedString for &mut str {
    fn to_padded_string(&self) -> String {
        make_padded_string(self)
    }
}

impl IntoPaddedString for String {
    fn into_padded_string(mut self) -> String {
        if self.capacity() < self.len() + ffi::SIMDJSON_PADDING {
            self.reserve(ffi::SIMDJSON_PADDING);
        }
        self
    }
}

#[cfg(test)]
mod tests {}
