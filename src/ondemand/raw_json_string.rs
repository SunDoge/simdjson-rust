use std::fmt::Debug;

use cxx::UniquePtr;

use crate::{
    bridge::{check, ffi},
    error::Result,
};

pub struct RawJsonString(pub UniquePtr<ffi::OndemandRawJsonString>);

impl Debug for RawJsonString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RawJsonString").finish()
    }
}
