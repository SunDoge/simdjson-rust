use simdjson_sys as lib;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;
use std::slice;

pub const SIMDJSON_PADDING: usize = 32; // sizeof(__m256i)

pub fn allocate_padded_buffer(length: usize) -> Vec<u8> {
    unsafe {
        let ptr = lib::allocate_padded_buffer(length) as *mut u8;
        Vec::from_raw_parts(ptr, length, length + SIMDJSON_PADDING)
    }
}

pub fn get_corpus<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let mut fp = File::open(path)?;
    // Get file length
    let len = fp.metadata()?.len();
    let mut buf = allocate_padded_buffer(len as usize);
    if buf.as_ptr().is_null() {
        panic!("could not allocate memory");
    }

    fp.read(&mut buf)?;
    if buf.len() != len as usize {
        // unsafe { lib::aligned_free(buf.as_mut_ptr()) };
        panic!("could not read the data len: {}-{}", buf.len(), len);
    }

    Ok(unsafe { String::from_utf8_unchecked(buf) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_gitignore() {
        let corpus = get_corpus(".gitignore").unwrap();
        assert_eq!(corpus.len(), 50);
        assert_eq!(corpus.capacity(), 50 + SIMDJSON_PADDING);
        assert_eq!(
            corpus,
            r#"/target
**/*.rs.bk
Cargo.lock
/simdjson-sys/target"#
        )
    }
}
