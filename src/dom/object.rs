use super::element::Element;
use super::parser::Parser;
use crate::error::{SimdJsonError, SimdJsonResult};
use crate::libsimdjson::ffi;
use cxx::UniquePtr;
use std::fmt;
use std::marker::PhantomData;

pub type ObjectIterPtr = UniquePtr<ffi::ObjectIterator>;
pub type ObjectPtr = UniquePtr<ffi::object>;

pub struct Object<'p> {
    ptr: ObjectPtr,
    // _phantom: PhantomData<&'a Parser>,
    parser: &'p Parser,
}

impl<'p> Object<'p> {
    pub fn new(ptr: ObjectPtr, parser: &'p Parser) -> Self {
        Object {
            ptr,
            // _phantom: PhantomData,
            parser,
        }
    }

    pub fn at(&self, json_pointer: &str) -> Result<Element, SimdJsonError> {
        let result = ffi::object_at(&self.ptr, json_pointer);
        // if result.code < 2 {
        //     Ok(Element::from(result.value))
        // } else {
        //     Err(SimdJsonError::from(result.code))
        // }

        check_result!(result, Element, self.parser)
    }

    pub fn at_key(&self, key: &str) -> Result<Element, SimdJsonError> {
        let result = ffi::object_at_key(&self.ptr, key);
        check_result!(result, Element, self.parser)
    }

    pub fn at_key_case_insensitive(&self, key: &str) -> Result<Element, SimdJsonError> {
        let result = ffi::object_at_key_case_insensitive(&self.ptr, key);
        check_result!(result, Element, self.parser)
    }

    pub fn minify(&self) -> String {
        ffi::object_minify(&self.ptr)
    }
}

// impl<'a> From<ObjectPtr> for Object<'a> {
//     fn from(ptr: ObjectPtr) -> Self {
//         Object::new(ptr)
//     }
// }

pub struct ObjectIter<'a, 'p: 'a> {
    pub ptr: ObjectIterPtr,
    object: &'a Object<'p>,
}

impl<'a, 'p> ObjectIter<'a, 'p> {
    pub fn new(object: &'a Object<'p>) -> Self {
        let ptr = ffi::object_get_iterator(&object.ptr);
        ObjectIter { ptr, object }
    }

    pub fn has_next(&self) -> bool {
        ffi::object_iterator_has_next(&self.ptr)
    }

    pub fn key(&self) -> String {
        ffi::object_iterator_key(&self.ptr)
    }

    pub fn value(&self) -> Element<'p> {
        let ptr = ffi::object_iterator_value(&self.ptr);
        Element::new(ptr, self.object.parser)
    }
}

impl<'a, 'p> Iterator for ObjectIter<'a, 'p> {
    type Item = (String, Element<'p>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_next() {
            let result = Some((self.key(), self.value()));
            ffi::object_iterator_next(&mut self.ptr);
            result
        } else {
            None
        }
    }
}

impl<'a, 'p> IntoIterator for &'a Object<'p> {
    type Item = (String, Element<'p>);
    type IntoIter = ObjectIter<'a, 'p>;

    fn into_iter(self) -> Self::IntoIter {
        ObjectIter::new(self)
    }
}

impl<'p> fmt::Display for Object<'p> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.minify())
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
