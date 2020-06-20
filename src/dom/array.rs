use super::element::Element;
use super::parser::Parser;
use crate::error::{SimdJsonError, SimdJsonResult};
use crate::libsimdjson::ffi;
use cxx::UniquePtr;
use std::fmt;
use std::marker::PhantomData;

pub type ArrayIterPtr = UniquePtr<ffi::ArrayIterator>;
pub type ArrayPtr = UniquePtr<ffi::array>;

pub struct Array<'p> {
    ptr: ArrayPtr,
    // _phantom: PhantomData<&'a Parser>,
    parser: &'p Parser,
}

impl<'p> Array<'p> {
    pub fn new(ptr: ArrayPtr, parser: &'p Parser) -> Self {
        Array {
            ptr,
            // _phantom: PhantomData,
            parser,
        }
    }

    pub fn at(&self, json_pointer: &str) -> SimdJsonResult<Element> {
        let result = ffi::array_at(&self.ptr, json_pointer);
        check_result!(result, Element, self.parser)
    }

    pub fn at_index(&self, index: usize) -> SimdJsonResult<Element> {
        let result = ffi::array_at_index(&self.ptr, index);
        check_result!(result, Element, self.parser)
    }

    pub fn minify(&self) -> String {
        ffi::array_minify(&self.ptr)
    }
}

// impl<'p> From<ArrayPtr> for Array<'a> {
//     fn from(ptr: ArrayPtr) -> Self {
//         Array::new(ptr)
//     }
// }

pub struct ArrayIter<'a, 'p: 'a> {
    ptr: ArrayIterPtr,
    // _phantom: PhantomData<&'a Parser>,
    array: &'a Array<'p>,
}

impl<'a, 'p> ArrayIter<'a, 'p> {
    pub fn new(array: &'a Array<'p>) -> Self {
        let ptr = ffi::array_get_iterator(&array.ptr);
        ArrayIter { ptr, array }
    }
}

impl<'a, 'p> Iterator for ArrayIter<'a, 'p> {
    type Item = Element<'p>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_ptr = ffi::array_iterator_next(&mut self.ptr);
        if next_ptr.is_null() {
            None
        } else {
            Some(Element::new(next_ptr, &self.array.parser))
        }
    }
}

impl<'a, 'p> IntoIterator for &'a Array<'p> {
    type Item = Element<'p>;
    type IntoIter = ArrayIter<'a, 'p>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayIter::new(self)
    }
}

impl<'p> fmt::Display for Array<'p> {
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
