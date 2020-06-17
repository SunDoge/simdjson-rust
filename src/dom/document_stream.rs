use super::element::Element;
use super::parser::Parser;
use crate::error::SimdJsonError;
use crate::libsimdjson::ffi;
use cxx::UniquePtr;
use std::marker::PhantomData;

pub type DocumentStreamIteratorPtr = UniquePtr<ffi::DocumentStreamIterator>;
pub type DocumentStreamPtr = UniquePtr<ffi::document_stream>;

pub struct DocumentStream<'a> {
    pub ptr: DocumentStreamPtr,
    _phantom: PhantomData<&'a Parser>,
}

impl<'a> From<DocumentStreamPtr> for DocumentStream<'a> {
    fn from(ptr: DocumentStreamPtr) -> Self {
        DocumentStream {
            ptr,
            _phantom: PhantomData,
        }
    }
}

pub struct DocumentStreamIter<'a> {
    ptr: DocumentStreamIteratorPtr,
    _phantom: PhantomData<&'a Parser>,
}

// impl<'a> From<DocumentStreamIteratorPtr> for DocumentStreamIter<'a> {
//     fn from(ptr: DocumentStreamIteratorPtr) -> Self {
//         DocumentStreamIter::new(ptr)
//     }
// }

impl<'a> DocumentStreamIter<'a> {
    pub fn new(document_stream: &mut DocumentStream) -> Self {
        let ptr = ffi::document_stream_get_iterator(&mut document_stream.ptr);
        DocumentStreamIter {
            ptr,
            _phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for DocumentStreamIter<'a> {
    type Item = Result<Element<'a>, SimdJsonError>;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("Before doc stream iter next");
        // let result = ffi::document_stream_iterator_next(&mut self.ptr);
        // if result.value.is_null() {
        //     None
        // } else {
        //     Some(result.value.into())
        // }

        if ffi::document_stream_iterator_has_next(&self.ptr) {
            let result = ffi::document_stream_iterator_value(&mut self.ptr);
            ffi::document_stream_iterator_next(&mut self.ptr);
            if result.code < 2 {
                Some(Ok(result.value.into()))
            } else {
                Some(Err(SimdJsonError::from(result.code)))
            }
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a mut DocumentStream<'a> {
    type Item = Result<Element<'a>, SimdJsonError>;
    type IntoIter = DocumentStreamIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DocumentStreamIter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom;
    use crate::error::SimdJsonError;

    #[test]
    fn parse_many() -> Result<(), SimdJsonError> {
        let mut parser = dom::Parser::default();
        // let mut dc = parser.load_many("json-examples/amazon_cellphones.ndjson", 100)?;
        let mut dc = parser.load_many_default("json-examples/amazon_cellphones.ndjson")?;


        let mut dci = dc.into_iter();
        for r in dci {
            println!("{}", r.unwrap());
            // break;
        }

        Ok(())
    }
}
