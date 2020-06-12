use crate::libsimdjson::ffi;
use anyhow::Result;

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

    pub fn at(&self, json_pointer: &str) -> Element {
        unimplemented!();
    }
}
