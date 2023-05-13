use std::todo;

use cxx::UniquePtr;

use crate::bridge::ffi;

pub struct Number(pub UniquePtr<ffi::OndemandNumber>);

impl Number {
    pub fn get_u64(&mut self) -> u64 {
        todo!()
    }

    pub fn get_i64(&mut self) -> i64 {
        todo!()
    }

    pub fn get_f64(&mut self) -> f64 {
        todo!()
    }
}
