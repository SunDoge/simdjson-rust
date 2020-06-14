use serde::de;
use std::fmt::Display;
use std::str::Utf8Error;
use thiserror::Error;

pub type SimdJsonResult<T> = Result<T, SimdJsonError>;

#[derive(Debug, Error)]
pub enum SimdJsonError {
    #[error("Message: {0}")]
    Message(String),

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

    #[error("Problem while parsing an atom starting with the letter 'n'")]
    NAtomError,

    #[error("Problem while parsing a number")]
    NumberError,

    #[error("The input is not valid UTF-8")]
    Utf8Error,

    #[error("Uninitialized")]
    Uninitialized,

    #[error("Empty: no JSON found")]
    Empty,

    #[error("Within strings, some characters must be escaped, we found unescaped characters")]
    UnescapedChars,

    #[error("A string is opened, but never closed.")]
    UnclosedString,

    #[error("simdjson does not have an implementation supported by this CPU architecture (perhaps it's a non-SIMD CPU?).")]
    UnsupportedArchitecture,

    #[error("The JSON element does not have the requested type.")]
    IncorrectType,

    #[error("The JSON number is too large or too small to fit within the requested type.")]
    NumberOutOfRange,

    #[error("Attempted to access an element of a JSON array that is beyond its length.")]
    IndexOutOfBounds,

    #[error("The JSON field referenced does not exist in this object.")]
    NoSuchField,

    #[error("Error reading the file.")]
    IoError,

    #[error("Invalid JSON pointer syntax.")]
    InvalidJsonPointer,

    #[error("Invalid URI fragment syntax.")]
    InvalidUriFragment,

    #[error(
        "Unexpected error, consider reporting this problem as you may have found a bug in simdjson"
    )]
    UnexpectedError,
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
            9 => SimdJsonError::NAtomError,
            10 => SimdJsonError::NumberError,
            11 => SimdJsonError::Utf8Error,
            12 => SimdJsonError::Uninitialized,
            13 => SimdJsonError::Empty,
            14 => SimdJsonError::UnescapedChars,
            15 => SimdJsonError::UnclosedString,
            16 => SimdJsonError::UnsupportedArchitecture,
            17 => SimdJsonError::IncorrectType,
            18 => SimdJsonError::NumberOutOfRange,
            19 => SimdJsonError::IndexOutOfBounds,
            20 => SimdJsonError::NoSuchField,
            21 => SimdJsonError::IoError,
            22 => SimdJsonError::InvalidJsonPointer,
            23 => SimdJsonError::InvalidUriFragment,
            24 => SimdJsonError::UnexpectedError,
            x => panic!("Unknown error code: {}", x),
        }
    }
}

impl de::Error for SimdJsonError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Message(msg.to_string())
    }
}
