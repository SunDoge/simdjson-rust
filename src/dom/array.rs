use super::element::Element;
use super::parser::Parser;
use crate::error::{SimdJsonError, SimdJsonResult};
use crate::libsimdjson::ffi;
use cxx::UniquePtr;
use std::fmt;
use std::marker::PhantomData;

pub type ArrayIterPtr = UniquePtr<ffi::ArrayIterator>;
pub type ArrayPtr = UniquePtr<ffi::array>;

pub struct Array<'a> {
    ptr: ArrayPtr,
    _phantom: PhantomData<&'a Parser>,
}

impl<'a> Array<'a> {
    pub fn new(ptr: ArrayPtr) -> Self {
        Array {
            ptr,
            _phantom: PhantomData,
        }
    }

    pub fn at(&self, json_pointer: &str) -> SimdJsonResult<Element> {
        let result = ffi::array_at(&self.ptr, json_pointer);
        check_result!(result, Element)
    }

    pub fn at_index(&self, index: usize) -> SimdJsonResult<Element> {
        let result = ffi::array_at_index(&self.ptr, index);
        check_result!(result, Element)
    }

    pub fn minify(&self) -> String {
        ffi::array_minify(&self.ptr)
    }
}

impl<'a> From<ArrayPtr> for Array<'a> {
    fn from(ptr: ArrayPtr) -> Self {
        Array::new(ptr)
    }
}

pub struct ArrayIter<'a> {
    ptr: ArrayIterPtr,
    // _phantom: PhantomData<&'a Parser>,
    array: &'a Array<'a>,
}

impl<'a> ArrayIter<'a> {
    pub fn new(array: &'a Array<'a>) -> Self {
        let ptr = ffi::array_get_iterator(&array.ptr);
        ArrayIter {
            ptr,
            // _phantom: PhantomData,
            array,
        }
    }
}

impl<'a> Iterator for ArrayIter<'a> {
    type Item = Element<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_ptr = ffi::array_iterator_next(&mut self.ptr);
        if next_ptr.is_null() {
            None
        } else {
            Some(Element::from(next_ptr))
        }
    }
}

impl<'a> IntoIterator for &'a Array<'a> {
    type Item = Element<'a>;
    type IntoIter = ArrayIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayIter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::parser::Parser;
    // use super::element::GetValue;

    #[test]
    fn array_iter() -> Result<(), Box<dyn std::error::Error>> {
        let mut parser = Parser::default();
        let elm = parser.parse("[true, true, true, true]")?;
        let arr = elm.get_array()?;

        assert!(arr.at_index(3)?.get_bool()?);

        let mut c = 0;
        for v in &arr {
            c += 1;
            println!("c={}, v={}", c, v.get_bool()?);

            assert!(v.get_bool()?);
        }

        Ok(())
    }
}

impl<'a> fmt::Display for Array<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.minify())
    }
}
