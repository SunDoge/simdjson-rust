use super::element::Element;
use super::parser::Parser;
use crate::error::{SimdJsonError, SimdJsonResult};
use crate::libsimdjson::ffi;
use cxx::UniquePtr;
use std::fmt;
use std::marker::PhantomData;

pub type ObjectIterPtr = UniquePtr<ffi::ObjectIterator>;
pub type ObjectPtr = UniquePtr<ffi::object>;

pub struct Object<'a> {
    ptr: ObjectPtr,
    _phantom: PhantomData<&'a Parser>,
}

impl<'a> Object<'a> {
    pub fn new(ptr: ObjectPtr) -> Self {
        Object {
            ptr,
            _phantom: PhantomData,
        }
    }

    pub fn at_pointer(&self, json_pointer: &str) -> Result<Element, SimdJsonError> {
        let result = ffi::object_at_pointer(&self.ptr, json_pointer);
        // if result.code < 2 {
        //     Ok(Element::from(result.value))
        // } else {
        //     Err(SimdJsonError::from(result.code))
        // }

        check_result!(result, Element)
    }

    pub fn at_key(&self, key: &str) -> Result<Element, SimdJsonError> {
        let result = ffi::object_at_key(&self.ptr, key);
        check_result!(result, Element)
    }

    pub fn at_key_case_insensitive(&self, key: &str) -> Result<Element, SimdJsonError> {
        let result = ffi::object_at_key_case_insensitive(&self.ptr, key);
        check_result!(result, Element)
    }

    pub fn minify(&self) -> String {
        ffi::object_minify(&self.ptr)
    }
}

impl<'a> From<ObjectPtr> for Object<'a> {
    fn from(ptr: ObjectPtr) -> Self {
        Object::new(ptr)
    }
}

pub struct ObjectIter<'a> {
    pub ptr: ObjectIterPtr,
    object: &'a Object<'a>,
}

impl<'a> ObjectIter<'a> {
    pub fn new(object: &'a Object<'a>) -> Self {
        let ptr = ffi::object_get_iterator(&object.ptr);
        ObjectIter { ptr, object }
    }

    pub fn has_next(&self) -> bool {
        ffi::object_iterator_has_next(&self.ptr)
    }

    pub fn key(&self) -> String {
        ffi::object_iterator_key(&self.ptr)
    }

    pub fn value(&self) -> Element<'a> {
        ffi::object_iterator_value(&self.ptr).into()
    }
}

impl<'a> Iterator for ObjectIter<'a> {
    type Item = (String, Element<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        ffi::object_iterator_next(self.ptr.pin_mut());
        if self.has_next() {
            None
        } else {
            Some((self.key(), self.value()))
        }
    }
}

impl<'a> IntoIterator for &'a Object<'a> {
    type Item = (String, Element<'a>);
    type IntoIter = ObjectIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ObjectIter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::parser::Parser;
    // use super::element::GetValue;

    #[test]
    fn object_iter() -> Result<(), Box<dyn std::error::Error>> {
        let mut parser = Parser::default();
        let elm = parser.parse(r#"{"a": true, "b": true}"#)?;
        let obj = elm.get_object()?;

        for (k, v) in &obj {
            println!("k={}, v={}", k, v.get_bool()?);
        }

        Ok(())
    }
}

impl<'a> fmt::Display for Object<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.minify())
    }
}
