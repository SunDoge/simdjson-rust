use simdjson_sys as ffi;
use std::ptr::NonNull;

use crate::{
    macros::{impl_drop, map_primitive_result, map_ptr_result},
    utils::string_view_struct_to_str,
    Result,
};

use super::{array::Array, object::Object};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementType {
    Array = '[' as _,
    Object = '{' as _,
    Int64 = 'l' as _,
    UInt64 = 'u' as _,
    Double = 'd' as _,
    String = '"' as _,
    Bool = 't' as _,
    NullValue = 'n' as _,
}

impl From<i32> for ElementType {
    fn from(value: i32) -> Self {
        match value as u8 as char {
            '[' => Self::Array,
            '{' => Self::Object,
            'l' => Self::Int64,
            'u' => Self::UInt64,
            'd' => Self::Double,
            '"' => Self::String,
            't' => Self::Bool,
            'n' => Self::NullValue,
            _ => unreachable!(),
        }
    }
}

pub struct Element {
    ptr: NonNull<ffi::SJ_DOM_element>,
}

impl Element {
    pub fn new(ptr: NonNull<ffi::SJ_DOM_element>) -> Self {
        Self { ptr }
    }

    pub fn get_type(&self) -> ElementType {
        unsafe { ElementType::from(ffi::SJ_DOM_element_type(self.ptr.as_ptr())) }
    }

    pub fn get_array(&self) -> Result<Array> {
        map_ptr_result!(ffi::SJ_DOM_element_get_array(self.ptr.as_ptr())).map(Array::new)
    }

    pub fn get_object(&self) -> Result<Object> {
        map_ptr_result!(ffi::SJ_DOM_element_get_object(self.ptr.as_ptr())).map(Object::new)
    }

    pub fn get_string(&self) -> Result<&str> {
        map_primitive_result!(ffi::SJ_DOM_element_get_string(self.ptr.as_ptr()))
            .map(|sv| string_view_struct_to_str(sv))
    }

    pub fn get_int64(&self) -> Result<i64> {
        map_primitive_result!(ffi::SJ_DOM_element_get_int64(self.ptr.as_ptr()))
    }

    pub fn get_uint64(&self) -> Result<u64> {
        map_primitive_result!(ffi::SJ_DOM_element_get_uint64(self.ptr.as_ptr()))
    }

    pub fn get_double(&self) -> Result<f64> {
        map_primitive_result!(ffi::SJ_DOM_element_get_double(self.ptr.as_ptr()))
    }

    pub fn get_bool(&self) -> Result<bool> {
        map_primitive_result!(ffi::SJ_DOM_element_get_bool(self.ptr.as_ptr()))
    }

    pub fn at_pointer(&self, json_pointer: &str) -> Result<Element> {
        map_ptr_result!(ffi::SJ_DOM_element_at_pointer(
            self.ptr.as_ptr(),
            json_pointer.as_ptr().cast(),
            json_pointer.len()
        ))
        .map(Element::new)
    }
}

impl_drop!(Element, ffi::SJ_DOM_element_free);
