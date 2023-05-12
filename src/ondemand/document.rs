use std::fmt::Debug;

use cxx::UniquePtr;

use crate::{
    bridge::{check, check_result, ffi, into_result},
    error::Result,
};

use super::{
    array::Array, object::Object, raw_json_string::RawJsonString, types::JsonType, value::Value,
};

// TODO: we may use transparent
// #[repr(transparent)]
pub struct Document(pub UniquePtr<ffi::OndemandDocument>);

impl Document {
    pub fn at_pointer(&mut self, json_pointer: &str) -> Result<Value> {
        // let_cxx_string!(p = json_pointer);
        // check!(
        //     ffi::ondemand_document_at_pointer,
        //     self.0.pin_mut(),
        //     json_pointer
        // )

        into_result!(ffi::ondemand_document_at_pointer(
            self.0.pin_mut(),
            json_pointer
        ))
        .map(Value)
    }

    pub fn get_object(&mut self) -> Result<Object> {
        check!(ffi::ondemand_document_get_object, self.0.pin_mut()).map(Object)
    }

    pub fn get_array(&mut self) -> Result<Array> {
        check!(ffi::ondemand_document_get_array, self.0.pin_mut()).map(Array)
    }

    pub fn find_field(&mut self, key: &str) -> Result<Value> {
        check!(ffi::ondemand_document_find_field, self.0.pin_mut(), key).map(Value)
    }

    pub fn find_field_unordered(&mut self, key: &str) -> Result<Value> {
        check!(
            ffi::ondemand_document_find_field_unordered,
            self.0.pin_mut(),
            key
        )
        .map(Value)
    }

    pub fn get_u64(&mut self) -> Result<u64> {
        check!(ffi::ondemand_document_get_uint64, self.0.pin_mut())
    }

    pub fn get_u64_in_string(&mut self) -> Result<u64> {
        check!(
            ffi::ondemand_document_get_uint64_in_string,
            self.0.pin_mut()
        )
    }

    pub fn get_i64(&mut self) -> Result<i64> {
        check!(ffi::ondemand_document_get_int64, self.0.pin_mut())
    }

    pub fn get_i64_in_string(&mut self) -> Result<i64> {
        check!(ffi::ondemand_document_get_int64_in_string, self.0.pin_mut())
    }

    pub fn get_f64(&mut self) -> Result<f64> {
        check!(ffi::ondemand_document_get_double, self.0.pin_mut())
    }

    pub fn get_f64_in_string(&mut self) -> Result<f64> {
        check!(
            ffi::ondemand_document_get_double_in_string,
            self.0.pin_mut()
        )
    }
    pub fn get_string(&mut self) -> Result<&str> {
        check!(ffi::ondemand_document_get_string, self.0.pin_mut())
    }
    pub fn get_bool(&mut self) -> Result<bool> {
        check!(ffi::ondemand_document_get_bool, self.0.pin_mut())
    }
    pub fn get_raw_json_string(&mut self) -> Result<RawJsonString> {
        check!(ffi::ondemand_document_get_raw_json_string, self.0.pin_mut()).map(RawJsonString)
    }
    pub fn is_null(&mut self) -> Result<bool> {
        check!(ffi::ondemand_document_is_null, self.0.pin_mut())
    }

    pub fn get_type(&mut self) -> Result<JsonType> {
        check!(ffi::ondemand_document_type, self.0.pin_mut()).map(|x| x.into())
    }

    pub fn is_negative(&mut self) -> bool {
        ffi::ondemand_document_is_negative(self.0.pin_mut())
    }

    pub fn is_scalar(&mut self) -> Result<bool> {
        check_result(|code| ffi::ondemand_document_is_scalar(self.0.pin_mut(), code))
    }
    pub fn is_integer(&mut self) -> Result<bool> {
        check_result(|code| ffi::ondemand_document_is_integer(self.0.pin_mut(), code))
    }
}

impl Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document").finish()
    }
}
