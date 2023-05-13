use std::fmt::Debug;

use cxx::UniquePtr;

use crate::bridge::ffi::{self, NumberType};

pub struct Number(pub UniquePtr<ffi::OndemandNumber>);

impl Number {
    pub fn get_u64(&self) -> u64 {
        self.0.get_uint64()
    }

    pub fn get_i64(&self) -> i64 {
        self.0.get_int64()
    }

    pub fn get_f64(&self) -> f64 {
        self.0.get_double()
    }

    pub fn get_number_type(&self) -> NumberType {
        self.0.get_number_type()
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Number").finish()
    }
}
