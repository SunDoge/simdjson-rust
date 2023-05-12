use std::fmt::Debug;

use cxx::UniquePtr;

use crate::{
    bridge::{
        ffi::{self},
        into_result,
    },
    constants::SIMDJSON_MAXSIZE_BYTES,
    error::Result,
    padded_string::PaddedString,
};

use super::document::Document;

pub struct Parser(pub UniquePtr<ffi::OndemandParser>);

impl Parser {
    pub fn new(max_capacity: usize) -> Self {
        Self(ffi::ondemand_parser_new(max_capacity))
    }

    pub fn iterate(&mut self, padded_string: &PaddedString) -> Result<Document> {
        // let mut code = ErrorCode::SUCCESS;
        // let doc = ffi::ondemand_parser_iterate(self.0.pin_mut(), &padded_string, &mut code);
        // if code == ErrorCode::SUCCESS {
        //     Ok(Document(doc))
        // } else {
        //     Err(code.into())
        // }

        // check!(
        //     ffi::ondemand_parser_iterate,
        //     self.0.pin_mut(),
        //     padded_string
        // )
        // .map(Document)

        let res = ffi::ondemand_parser_iterate(self.0.pin_mut(), padded_string);
        into_result!(res).map(Document)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new(SIMDJSON_MAXSIZE_BYTES)
    }
}

impl Debug for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parser").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_parser() {
        let mut parser = Parser::default();
        let ps = PaddedString::load("json-examples/twitter.json").unwrap();
        let doc = parser.iterate(&ps);
        dbg!(&doc);
        assert!(doc.is_ok());
    }
}
