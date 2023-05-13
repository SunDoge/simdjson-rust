use std::todo;

use cxx::UniquePtr;

use crate::bridge::ffi;

pub struct Number(pub UniquePtr<ffi::OndemandNumber>);

impl Number {
    pub fn get_u64(&mut self) -> u64 {
        ffi::ondemand_number_get_u64(self.0.pin_mut())
    }

    pub fn get_i64(&mut self) -> i64 {
        ffi::ondemand_number_get_i64(self.0.pin_mut())
    }

    pub fn get_f64(&mut self) -> f64 {
        ffi::ondemand_number_get_f64(self.0.pin_mut())
    }
}
