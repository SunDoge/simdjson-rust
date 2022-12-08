use crate::bridge::ffi::OndemandJsonType;

pub enum JsonType {
    Array = 1,
    Object,
    Number,
    String,
    Bool,
    Null,
}

impl From<OndemandJsonType> for JsonType {
    fn from(json_type: OndemandJsonType) -> Self {
        match json_type {
            OndemandJsonType::array => Self::Array,
            OndemandJsonType::object => Self::Object,
            OndemandJsonType::number => Self::Number,
            OndemandJsonType::string => Self::String,
            OndemandJsonType::boolean => Self::Bool,
            OndemandJsonType::null => Self::Null,
            _ => panic!("unknown json type: {:?}", json_type),
        }
    }
}
