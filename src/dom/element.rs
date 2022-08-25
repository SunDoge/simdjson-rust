use super::array::Array;
use super::object::Object;
use super::parser::Parser;
use crate::error::SimdJsonError;
use crate::libsimdjson::ffi;
use cxx::UniquePtr;

use std::fmt;
use std::marker::PhantomData;

use std::str::{self};

pub type ElementPtr = UniquePtr<ffi::element>;

#[derive(Debug, PartialEq, Eq)]
pub enum ElementType {
    ///< dom::array
    Array,
    ///< dom::object
    Object,
    ///< int64_t
    Int64,
    ///< uint64_t: any integer that fits in uint64_t but *not* int64_t
    Uint64,
    ///< double: Any number with a "." or "e" that fits in double.
    Double,
    ///< std::string_view
    String,
    ///< bool
    Bool,
    ///< null
    NullValue,
}

impl From<u8> for ElementType {
    fn from(code: u8) -> Self {
        match code as char {
            '[' => Self::Array,
            '{' => Self::Object,
            'l' => Self::Int64,
            'u' => Self::Uint64,
            'd' => Self::Double,
            '"' => Self::String,
            't' => Self::Bool,
            'n' => Self::NullValue,
            _ => unreachable!(),
        }
    }
}

pub struct Element<'a> {
    ptr: ElementPtr,
    // ptr: &'a ffi::element,
    _phantom: PhantomData<&'a Parser>,
}

impl<'a> From<ElementPtr> for Element<'a> {
    fn from(ptr: ElementPtr) -> Self {
        Element::new(ptr)
    }
}

impl<'a> Element<'a> {
    fn new(ptr: ElementPtr) -> Self {
        Element {
            ptr,
            _phantom: PhantomData,
        }
    }

    pub fn at_pointer(&self, json_pointer: &str) -> Result<Element, SimdJsonError> {
        let result = ffi::element_at_pointer(&self.ptr, json_pointer);
        // if result.code < 2 {
        //     Ok(Element::from(result.value))
        // } else {
        //     Err(SimdJsonError::from(result.code))
        // }

        check_result!(result, Element)
    }

    pub fn at_key(&self, key: &str) -> Result<Element, SimdJsonError> {
        let result = ffi::element_at_key(&self.ptr, key);
        check_result!(result, Element)
    }

    pub fn at_index(&self, index: usize) -> Result<Element, SimdJsonError> {
        let result = ffi::element_at_index(&self.ptr, index);
        check_result!(result, Element)
    }

    pub fn get_string(&self) -> Result<String, SimdJsonError> {
        let result = ffi::element_get_string(&self.ptr);
        check_result!(result)
    }

    pub fn get_bool(&self) -> Result<bool, SimdJsonError> {
        let result = ffi::element_get_bool(&self.ptr);
        check_result!(result)
    }

    pub fn get_u64(&self) -> Result<u64, SimdJsonError> {
        let result = ffi::element_get_u64(&self.ptr);
        check_result!(result)
    }

    pub fn get_i64(&self) -> Result<i64, SimdJsonError> {
        let result = ffi::element_get_i64(&self.ptr);
        check_result!(result)
    }

    pub fn get_f64(&self) -> Result<f64, SimdJsonError> {
        let result = ffi::element_get_f64(&self.ptr);
        check_result!(result)
    }

    pub fn get_object(&self) -> Result<Object, SimdJsonError> {
        let result = ffi::element_get_object(&self.ptr);
        check_result!(result, Object)
    }

    pub fn get_array(&self) -> Result<Array, SimdJsonError> {
        let result = ffi::element_get_array(&self.ptr);
        check_result!(result, Array)
    }

    pub fn get_type(&self) -> ElementType {
        ElementType::from(ffi::element_get_type(&self.ptr))
    }

    pub fn is_null(&self) -> bool {
        ffi::element_is_null(&self.ptr)
    }

    pub fn minify(&self) -> String {
        ffi::element_minify(&self.ptr)
    }
}

// pub trait GetValue<T> {
//     fn get(&self) -> Result<T, SimdJsonError>;
// }

// impl GetValue<bool> for Element {
//     fn get(&self) -> Result<bool, SimdJsonError> {
//         let result = ffi::element_get_bool(&self.ptr);
//         check_result!(result)
//     }
// }

impl<'a> fmt::Display for Element<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.minify())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_at_pointer() {
        let mut parser = Parser::default();
        let doc = parser.parse("{\"foo\": 1, \"bar\": 2}").unwrap();
        let elem = doc.at_pointer("/foo").unwrap();
        assert_eq!(elem.get_i64().unwrap(), 1);
        let elem = doc.at_pointer("/bar").unwrap();
        assert_eq!(elem.get_i64().unwrap(), 2);

        let doc = parser.parse("{\"foo\": [-1, -2, -3]}").unwrap();
        let elem = doc.at_pointer("/foo/1").unwrap();
        assert_eq!(elem.get_i64().unwrap(), -2);
    }
}
