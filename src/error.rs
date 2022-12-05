use thiserror::Error;

pub type Result<T> = std::result::Result<T, SimdJsonError>;
use crate::bridge::ffi::ErrorCode;



#[derive(Debug, Error)]
pub enum SimdJsonError {
    #[error("This parser can't support a document that big")]
    Capacity,

    #[error("Error allocating memory, we're most likely out of memory")]
    Memalloc,

    #[error("The JSON document has an improper structure: missing or superfluous commas, braces, missing keys, etc.")]
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
    TrailingContent,
}

impl From<ErrorCode> for SimdJsonError {
    fn from(error_code: ErrorCode) -> Self {
        match error_code {
            ErrorCode::SUCCESS => panic!("No error"),
            ErrorCode::CAPACITY => Self::Capacity,
            ErrorCode::MEMALLOC => Self::Memalloc,
            ErrorCode::TAPE_ERROR => Self::TapeError,
            ErrorCode::DEPTH_ERROR => Self::DepthError,
            ErrorCode::STRING_ERROR => Self::StringError,
            ErrorCode::T_ATOM_ERROR => Self::TAtomError,
            ErrorCode::F_ATOM_ERROR => Self::FAtomError,
            ErrorCode::N_ATOM_ERROR => Self::NAtomError,
            ErrorCode::NUMBER_ERROR => Self::NumberError,
            ErrorCode::UTF8_ERROR => Self::Utf8Error,
            ErrorCode::UNINITIALIZED => Self::Uninitialized,
            ErrorCode::EMPTY => Self::Empty,
            ErrorCode::UNESCAPED_CHARS => Self::UnescapedChars,
            ErrorCode::UNCLOSED_STRING => Self::UnclosedString,
            ErrorCode::UNSUPPORTED_ARCHITECTURE => Self::UnsupportedArchitecture,
            ErrorCode::INCORRECT_TYPE => Self::IncorrectType,
            ErrorCode::NUMBER_OUT_OF_RANGE => Self::NumberOutOfRange,
            ErrorCode::INDEX_OUT_OF_BOUNDS => Self::IndexOutOfBounds,
            ErrorCode::NO_SUCH_FIELD => Self::NoSuchField,
            ErrorCode::IO_ERROR => Self::IoError,
            ErrorCode::INVALID_JSON_POINTER => Self::InvalidJsonPointer,
            ErrorCode::INVALID_URI_FRAGMENT => Self::InvalidUriFragment,
            ErrorCode::UNEXPECTED_ERROR => Self::UnexpectedError,
            ErrorCode::PARSER_IN_USE => Self::ParserInUse,
            ErrorCode::OUT_OF_ORDER_ITERATION => Self::OutOfOrderIteration,
            ErrorCode::INSUFFICIENT_PADDING => Self::InsufficientPadding,
            ErrorCode::INCOMPLETE_ARRAY_OR_OBJECT => Self::IncompleteArrayOrObject,
            ErrorCode::SCALAR_DOCUMENT_AS_VALUE => Self::ScalarDocumentAsValue,
            ErrorCode::OUT_OF_BOUNDS => Self::OutOfBounds,
            ErrorCode::TRAILING_CONTENT => Self::TrailingContent,
            x => panic!("Unknown error code: {:?}", x),
        }
    }
}
