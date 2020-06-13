use super::array::Array;
use super::object::Object;
use super::parser::Parser;
use crate::error::SimdJsonError;
use crate::libsimdjson::ffi;
use cxx::UniquePtr;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::mem::transmute;
use std::str::{self, Utf8Error};

pub type ElementPtr = UniquePtr<ffi::element>;

#[derive(Debug, PartialEq)]
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

impl From<i32> for ElementType {
    fn from(code: i32) -> Self {
        match code {
            0 => Self::Array,
            1 => Self::Object,
            2 => Self::Int64,
            3 => Self::Uint64,
            4 => Self::Double,
            5 => Self::String,
            6 => Self::Bool,
            7 => Self::NullValue,
            _ => unreachable!(),
        }
    }
}

impl ffi::StringResult {
    pub fn to_str(&self) -> Result<&str, SimdJsonError> {
        if self.code > 2 {
            Ok(str::from_utf8(self.value.as_bytes())?)
        } else {
            Err(SimdJsonError::from(self.code))
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

    pub fn at(&self, json_pointer: &str) -> Result<Element, SimdJsonError> {
        let result = ffi::element_at(&self.ptr, json_pointer);
        // if result.code < 2 {
        //     Ok(Element::from(result.value))
        // } else {
        //     Err(SimdJsonError::from(result.code))
        // }

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
