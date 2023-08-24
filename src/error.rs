use thiserror::Error;

pub type Result<T> = std::result::Result<T, SimdJsonError>;

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

    #[error(
        "simdjson does not have an implementation supported by this CPU architecture (perhaps \
         it's a non-SIMD CPU?)."
    )]
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

    #[error("todo")]
    UnexpectedError,

    #[error("todo")]
    ParserInUse,

    #[error("todo")]
    OutOfOrderIteration,

    #[error("todo")]
    InsufficientPadding,

    #[error("todo")]
    IncompleteArrayOrObject,

    #[error("todo")]
    ScalarDocumentAsValue,

    #[error("todo")]
    OutOfBounds,

    #[error("todo")]
    TailingContent,

    #[error("todo")]
    NumErrorCodes,

    #[error("todo")]
    StdIoError(#[from] std::io::Error),
}

impl From<i32> for SimdJsonError {
    fn from(error_code: i32) -> Self {
        match error_code {
            1 => SimdJsonError::Capacity,
            2 => SimdJsonError::MemAlloc,
            3 => SimdJsonError::TapeError,
            4 => SimdJsonError::DepthError,
            5 => SimdJsonError::StringError,
            6 => SimdJsonError::TAtomError,
            7 => SimdJsonError::FAtomError,
            8 => SimdJsonError::NAtomError,
            9 => SimdJsonError::NumberError,
            10 => SimdJsonError::Utf8Error,
            11 => SimdJsonError::Uninitialized,
            12 => SimdJsonError::Empty,
            13 => SimdJsonError::UnescapedChars,
            14 => SimdJsonError::UnclosedString,
            15 => SimdJsonError::UnsupportedArchitecture,
            16 => SimdJsonError::IncorrectType,
            17 => SimdJsonError::NumberOutOfRange,
            18 => SimdJsonError::IndexOutOfBounds,
            19 => SimdJsonError::NoSuchField,
            20 => SimdJsonError::IoError,
            21 => SimdJsonError::InvalidJsonPointer,
            22 => SimdJsonError::InvalidUriFragment,
            23 => SimdJsonError::UnexpectedError,
            24 => SimdJsonError::ParserInUse,
            25 => SimdJsonError::OutOfOrderIteration,
            26 => SimdJsonError::InsufficientPadding,
            27 => SimdJsonError::IncompleteArrayOrObject,
            28 => SimdJsonError::ScalarDocumentAsValue,
            29 => SimdJsonError::OutOfBounds,
            30 => SimdJsonError::TailingContent,
            31 => SimdJsonError::NumErrorCodes,
            x => panic!("Unknown error code: {}", x),
        }
    }
}
