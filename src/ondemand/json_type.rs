#[derive(Debug, PartialEq)]
pub enum JsonType {
    Array = 1,
    Object,
    Number,
    String,
    Boolean,
    Null,
}

impl From<i32> for JsonType {
    fn from(value: i32) -> Self {
        match value {
            1 => JsonType::Array,
            2 => JsonType::Object,
            3 => JsonType::Number,
            4 => JsonType::String,
            5 => JsonType::Boolean,
            6 => JsonType::Null,
            _ => panic!("Invalid JsonType value: {}", value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum NumberType {
    FloatingPointNumber = 1,
    SignedInteger,
    UnsignedInteger,
}

impl From<i32> for NumberType {
    fn from(value: i32) -> Self {
        match value {
            1 => NumberType::FloatingPointNumber,
            2 => NumberType::SignedInteger,
            3 => NumberType::UnsignedInteger,
            _ => panic!("Invalid NumberType value: {}", value),
        }
    }
}
