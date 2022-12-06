// use super::libsimdjson::ffi;

// pub type PaddedStringPtr = cxx::UniquePtr<ffi::padded_string>;

// pub struct PaddedString {
//     ptr: PaddedStringPtr,
// }

// impl PaddedString {
//     pub fn new(ptr: PaddedStringPtr) -> Self {
//         PaddedString { ptr }
//     }

//     pub fn as_ptr(&self) -> &PaddedStringPtr {
//         &self.ptr
//     }
// }

// impl From<&str> for PaddedString {
//     fn from(s: &str) -> Self {
//         let ptr = ffi::padded_string_from_string(s);
//         PaddedString { ptr }
//     }
// }

use std::{fmt::Debug, ops::Deref, path::Path};

use cxx::{let_cxx_string, UniquePtr};

use crate::{
    bridge::{
        check,
        ffi::{self, ErrorCode},
    },
    error::Result,
};

pub struct PaddedString(pub UniquePtr<ffi::PaddedString>);

impl PaddedString {
    pub fn load<P>(filename: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        // TODO: this is not optimal but I use to temporarily fix windows ci
        let_cxx_string!(filename_cxx = filename.as_ref().as_os_str().to_str().unwrap());
        // let ps = ffi::padded_string_load(&filename_cxx, &mut code);
        // if code == ErrorCode::SUCCESS {
        //     Ok(Self(ps))
        // } else {
        //     Err(code.into())
        // }
        check!(ffi::padded_string_load, &filename_cxx).map(PaddedString)
    }
}

impl Debug for PaddedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaddedString").finish()
    }
}

impl Deref for PaddedString {
    type Target = UniquePtr<ffi::PaddedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for PaddedString {
    fn from(s: &str) -> Self {
        Self(ffi::padded_string_from_str(&s))
    }
}

#[cfg(test)]
mod tests {

    use crate::error::SimdJsonError;

    use super::*;

    #[test]
    fn load_padded_string() {
        let ps = PaddedString::load("json-examples/twitter.json");
        assert!(ps.is_ok());

        let ps = PaddedString::load("asdf");
        assert!(ps.is_err());
    }
}
