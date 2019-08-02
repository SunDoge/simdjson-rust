use simdjson_sys as lib;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

pub const SIMDJSON_PADDING: usize = 32; // sizeof(__m256i)

pub fn allocate_padded_buffer(length: usize) -> *mut u8 {
    unsafe { lib::simdjson_allocate_padded_buffer(length) as *mut u8 }
}

pub fn get_corpus<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let mut fp = File::open(path)?;
    // Get file length
    let len = fp.metadata()?.len() as usize;
    let ptr = allocate_padded_buffer(len);
    if ptr.is_null() {
        panic!("could not allocate memory");
    }
    let mut buf = unsafe { String::from_raw_parts(ptr, 0, len + SIMDJSON_PADDING) };

    fp.read_to_string(&mut buf)?;
    if buf.len() != len {
        // unsafe { lib::aligned_free(buf.as_mut_ptr()) };
        panic!("could not read the data len: {}-{}", buf.len(), len);
    }

    Ok(buf)
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
