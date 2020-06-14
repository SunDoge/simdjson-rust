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

pub type SimdJsonResult<T> = Result<T, SimdJsonError>;

#[derive(Debug, Error)]
pub enum SimdJsonError {
    #[error("This parser can't support a document that big")]
    Capacity,

    #[error("Error allocating memory, we're most likely out of memory")]
    MemAlloc,

    #[error("Something went wrong while writing to the tape")]
    TapeError,

    #[error("The JSON document was too deep (too many nested objects and arrays)")]
    DepthError,

    #[error("Problem while parsing a string")]
    StringError,

    #[error("Problem while parsing an atom starting with the letter 't'")]
    TAtomError,

    #[error("Problem while parsing an atom starting with the letter 'f'")]
    FAtomError,


    #[error("The JSON element does not have the requested type.")]
    IncorrectType,

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
            4 => SimdJsonError::TapeError,
            5 => SimdJsonError::DepthError,
            6 => SimdJsonError::StringError,
            7 => SimdJsonError::TAtomError,
            8 => SimdJsonError::FAtomError,

            17 => SimdJsonError::IncorrectType,
            x => panic!("Unknown error code: {}", x),
        }
    }
}
