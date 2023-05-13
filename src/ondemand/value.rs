use super::number::Number;
use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use cxx::UniquePtr;

use crate::{
    bridge::{
        check,
        ffi::{self, JsonType, NumberType},
        into_result,
    },
    error::Result,
};

use super::{array::Array, object::Object};

/// Bind only minimal apis
/// document -> value
/// value -> {
///     number,
///     bool,
///     raw_json_string/&str,
///     object,
///     array,
/// }
/// number -> {u64, i64, f64}
/// object -> [key: value]
/// array -> [value]
///
pub struct Value(pub UniquePtr<ffi::OndemandValue>);

impl Value {
    pub fn get_u64(&mut self) -> Result<u64> {
        into_result!(ffi::ondemand_value_get_uint64(self.0.pin_mut()))
    }

    pub fn get_i64(&mut self) -> Result<i64> {
        into_result!(ffi::ondemand_value_get_int64(self.0.pin_mut()))
    }

    pub fn get_f64(&mut self) -> Result<f64> {
        into_result!(ffi::ondemand_value_get_double(self.0.pin_mut()))
    }

    pub fn get_number_type(&mut self) -> Result<NumberType> {
        into_result!(ffi::ondemand_value_get_number_type(self.0.pin_mut()))
    }

    pub fn get_array(&mut self) -> Result<Array> {
        check!(ffi::ondemand_value_get_array, self.0.pin_mut()).map(Array)
    }

    pub fn get_object(&mut self) -> Result<Object> {
        check!(ffi::ondemand_value_get_object, self.0.pin_mut()).map(Object)
    }

    pub fn get_number(&mut self) -> Result<Number> {
        into_result!(ffi::ondemand_value_get_number(self.0.pin_mut())).map(Number)
    }

    pub fn get_bool(&mut self) -> Result<bool> {
        into_result!(ffi::ondemand_value_get_bool(self.0.pin_mut()))
    }

    pub fn get_string(&mut self) -> Result<&str> {
        check!(ffi::ondemand_value_get_string, self.0.pin_mut())
    }

    pub fn is_null(&mut self) -> Result<bool> {
        into_result!(ffi::ondemand_value_is_null(self.0.pin_mut()))
    }

    pub fn find_field(&mut self, key: &str) -> Result<Value> {
        check!(ffi::ondemand_value_find_field, self.0.pin_mut(), key).map(Value)
    }

    pub fn find_field_unordered(&mut self, key: &str) -> Result<Value> {
        check!(
            ffi::ondemand_value_find_field_unordered,
            self.0.pin_mut(),
            key
        )
        .map(Value)
    }

    pub fn get_type(&mut self) -> Result<JsonType> {
        into_result!(ffi::ondemand_value_type(self.0.pin_mut()))
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Value").finish()
    }
}

// impl Index<&str> for Value {
//     type Output = Self;
//     fn index(&self, index: &str) -> &Self::Output {
//         panic!("can't do this");
//     }
// }

// We need lifetime to do this
// impl IndexMut<&str> for Value {
//     fn index_mut(&mut self, index: &str) -> &mut Self::Output {
//         self.find_field_unordered(index).as_mut().unwrap()
//     }
// }
