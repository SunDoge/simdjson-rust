use super::parser::Parser;
use crate::error::SimdJsonError;
use crate::libsimdjson::ffi;
use cxx::UniquePtr;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::mem::transmute;
use std::str::{self, Utf8Error};

pub type ElementPtr = UniquePtr<ffi::element>;

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
    fn from(ptr: ElementPtr) -> Element<'a> {
        Element {
            ptr,
            _phantom: PhantomData,
        }
    }
}

impl<'a> Element<'a> {
    // pub fn at(&self, json_pointer: &str) -> Result<Element<'a>, SimdJsonError> {
    //     let result = ffi::element_at(&self.ptr, json_pointer);
    //     // if result.code < 2 {
    //     //     Ok(Element::from(result.value))
    //     // } else {
    //     //     Err(SimdJsonError::from(result.code))
    //     // }

    //     // check_result!(result, Element)
    // }

    // pub fn get_string<'a>(&'a self) -> Result<&'a str, SimdJsonError> {
    //     let result = ffi::element_get_string(&self.ptr);
    //     if result.code < 2 {
    //         // let data = result.value.as_ptr();
    //         // let size = result.value.len();

    //         // Ok(str::from_utf8_unchecked(result.value.as_bytes()))
    //     } else {
    //         Err(SimdJsonError::from(result.code))
    //     }
    // }

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
