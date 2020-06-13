// use std::error;
// use std::fmt;

// pub use SimdJsonError::*;

// #[derive(Debug)]
// pub enum SimdJsonError {
//     Capacity,
//     Memalloc,
//     TapeError,
// }

// impl fmt::Display for SimdJsonError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             match *self {
//                 Capacity => "This ParsedJson can't support a document that big",
//                 Memalloc => "Error allocating memory, we're most likely out of memory",
//                 TapeError => "Something went wrong while writing to the tape",
//             }
//         )
//     }
// }

// impl From<i32> for SimdJsonError {
//     fn from(int: i32) -> Self {
//         match int {
//             1 => Capacity,
//             2 => Memalloc,
//             3 => TapeError,
//             _ => unreachable!(),
//         }
//     }
// }

// impl error::Error for SimdJsonError {}

use std::str::Utf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SimdJsonError {
    #[error("This parser can't support a document that big")]
    Capacity,

    #[error("Error allocating memory, we're most likely out of memory")]
    MemAlloc,

    #[error("The input is not valid UTF-8")]
    Utf8Error(#[from] Utf8Error),
}

impl From<i32> for SimdJsonError {
    fn from(error_code: i32) -> Self {
        match error_code {
            0 => panic!("No error"),
            1 => panic!("No error and buffer still has more data"),
            2 => SimdJsonError::Capacity,
            3 => SimdJsonError::MemAlloc,
            _ => unimplemented!(),
        }
    }
}
