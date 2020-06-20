use super::libsimdjson::ffi;

pub struct Implementation;

impl Implementation {
    pub fn set_by_name(name: &str) {
        ffi::active_implementation_set_by_name(name);
    }

    pub fn name() -> String {
        ffi::active_implementation_name()
    }

    pub fn description() -> String {
        ffi::active_implementation_description()
    }
}
