//! Simdjson error code translations and strong type definitions.

use snafu::prelude::*;

/// Strongly typed error enum representing C++ simdjson error codes.
#[derive(Debug, Snafu, Clone, Copy, PartialEq, Eq)]
pub enum SimdJsonError {
    #[snafu(display("CAPACITY: This parser can't support a document that big"))]
    Capacity,

    #[snafu(display("MEMALLOC: Memory allocation failure"))]
    MemAlloc,

    #[snafu(display("TAPE_ERROR: The JSON is so badly formed we couldn't build a tape"))]
    TapeError,

    #[snafu(display("DEPTH_ERROR: Your document exceeds the maximum depth"))]
    DepthError,

    #[snafu(display("STRING_ERROR: Problem while parsing a string"))]
    StringError,

    #[snafu(display("T_ATOM_ERROR: Problem while parsing an atom starting with 't'"))]
    TAtomError,

    #[snafu(display("F_ATOM_ERROR: Problem while parsing an atom starting with 'f'"))]
    FAtomError,

    #[snafu(display("N_ATOM_ERROR: Problem while parsing an atom starting with 'n'"))]
    NAtomError,

    #[snafu(display("NUMBER_ERROR: Problem while parsing a number"))]
    NumberError,

    #[snafu(display("UTF8_ERROR: The input is not valid UTF-8"))]
    Utf8Error,

    #[snafu(display("UNINITIALIZED: Uninitialized or an empty parser"))]
    Uninitialized,

    #[snafu(display("EMPTY: No structural element found"))]
    Empty,

    #[snafu(display("UNESCAPED_CHARS: Found unescaped characters in a string"))]
    UnescapedChars,

    #[snafu(display("UNCLOSED_STRING: Unclosed string"))]
    UnclosedString,

    #[snafu(display("UNSUPPORTED_ARCHITECTURE: Unsupported architecture"))]
    UnsupportedArchitecture,

    #[snafu(display("INCORRECT_TYPE: Element has the wrong type for this operation"))]
    IncorrectType,

    #[snafu(display("NUMBER_OUT_OF_RANGE: Number is too large or too small"))]
    NumberOutOfRange,

    #[snafu(display("INDEX_OUT_OF_BOUNDS: Array index is too large"))]
    IndexOutOfBounds,

    #[snafu(display("NO_SUCH_FIELD: The key was not found in the object"))]
    NoSuchField,

    #[snafu(display("IO_ERROR: Error reading file"))]
    IoError,

    #[snafu(display("INVALID_JSON_POINTER: Invalid JSON pointer syntax"))]
    InvalidJsonPointer,

    #[snafu(display("INVALID_URI_FRAGMENT: Fragment is not valid"))]
    InvalidUriFragment,

    #[snafu(display("UNEXPECTED_ERROR: Something went wrong, this is a bug in simdjson"))]
    UnexpectedError,

    #[snafu(display("UNKNOWN_ERROR: Unknown error code {code}"))]
    Unknown { code: i32 },
}

impl SimdJsonError {
    /// Convert a raw C++ simdjson error code into a strongly typed
    /// `SimdJsonError`.
    ///
    /// If the code is `0` (SUCCESS), this returns `None`. For all other codes,
    /// this returns `Some(SimdJsonError)`.
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            0 => None,
            1 => Some(Self::Capacity),
            2 => Some(Self::MemAlloc),
            3 => Some(Self::TapeError),
            4 => Some(Self::DepthError),
            5 => Some(Self::StringError),
            6 => Some(Self::TAtomError),
            7 => Some(Self::FAtomError),
            8 => Some(Self::NAtomError),
            9 => Some(Self::NumberError),
            10 => Some(Self::Utf8Error),
            11 => Some(Self::Uninitialized),
            12 => Some(Self::Empty),
            13 => Some(Self::UnescapedChars),
            14 => Some(Self::UnclosedString),
            15 => Some(Self::UnsupportedArchitecture),
            16 => Some(Self::IncorrectType),
            17 => Some(Self::NumberOutOfRange),
            18 => Some(Self::IndexOutOfBounds),
            19 => Some(Self::NoSuchField),
            20 => Some(Self::IoError),
            21 => Some(Self::InvalidJsonPointer),
            22 => Some(Self::InvalidUriFragment),
            23 => Some(Self::UnexpectedError),
            other => Some(Self::Unknown { code: other }),
        }
    }

    pub fn check_code(code: i32) -> Result<(), Self> {
        match Self::from_code(code) {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }
}
