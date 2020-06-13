use crate::error::SimdJsonError;
use crate::libsimdjson::ffi;

pub type ElementPtr = cxx::UniquePtr<ffi::element>;

pub struct Element {
    ptr: ElementPtr,
}

impl From<ElementPtr> for Element {
    fn from(ptr: ElementPtr) -> Element {
        Element { ptr }
    }
}

impl Element {
    pub fn at(&self, json_pointer: &str) -> Result<Element, SimdJsonError> {
        let result = ffi::element_at(&self.ptr, json_pointer);
        if result.code < 2 {
            Ok(Element::from(result.value))
        } else {
            Err(SimdJsonError::from(result.code))
        }
    }
}

pub trait Value<T> {
    fn get(&self) -> Result<T, SimdJsonError>;
}

impl Value<bool> for Element {
    fn get(&self) -> Result<bool, SimdJsonError> {
        let result = ffi::element_get_bool(&self.ptr);
        if result.code < 2 {
            Ok(result.value)
        } else {
            Err(SimdJsonError::from(result.code))
        }
    }
}
