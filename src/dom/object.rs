use super::element::Element;
use super::parser::Parser;
use crate::error::{SimdJsonError, SimdJsonResult};
use crate::libsimdjson::ffi;
use cxx::UniquePtr;
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

    pub fn at(&self, json_pointer: &str) -> Result<Element, SimdJsonError> {
        let result = ffi::object_at(&self.ptr, json_pointer);
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

    pub fn minify(&self) -> &str {
        ffi::object_minify(&self.ptr)
    }
}

impl<'a> From<ObjectPtr> for Object<'a> {
    fn from(ptr: ObjectPtr) -> Self {
        Object::new(ptr)
    }
}

pub struct ObjectIter<'a> {
    ptr: ObjectIterPtr,
    _phantom: PhantomData<&'a Parser>,
}

impl<'a> ObjectIter<'a> {
    pub fn new(ptr: ObjectIterPtr) -> Self {
        ObjectIter {
            ptr,
            _phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for ObjectIter<'a> {
    type Item = (String, Element<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let kvp = ffi::object_iterator_next(&mut self.ptr);
        if kvp.value.is_null() {
            None
        } else {
            Some((kvp.key, Element::from(kvp.value)))
        }
    }
}

impl<'a> IntoIterator for &Object<'a> {
    type Item = (String, Element<'a>);
    type IntoIter = ObjectIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let ptr = ffi::object_get_iterator(&self.ptr);
        ObjectIter::new(ptr)
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
        let elm = parser.parse_str(r#"{"a": true, "b": true}"#)?;
        let obj = elm.get_object()?;

        for (k, v) in &obj {
            println!("k={}, v={}", k, v.get_bool()?);
        }

        Ok(())
    }
}
