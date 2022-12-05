use std::fmt::Debug;

use cxx::UniquePtr;

use crate::bridge::ffi;

pub struct Document(pub UniquePtr<ffi::OndemandDocument>);

impl Document {}

impl Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document").finish()
    }
}
