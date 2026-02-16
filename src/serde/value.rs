use serde_json::{Map, Number, Value};

use crate::dom::{Element, ElementType};
use crate::error::SimdJsonError;

const MAX_NESTING_DEPTH: usize = 128;

/// Convert a DOM `Element` into a `serde_json::Value`.
///
/// Enforces a maximum nesting depth to guard against stack overflow from
/// adversarial inputs with extreme nesting (e.g. `[[[[...]]]]` repeated
/// thousands of times).
pub fn element_to_value(element: &Element<'_>) -> Result<Value, SimdJsonError> {
    element_to_value_inner(element, 0)
}

fn element_to_value_inner(element: &Element<'_>, depth: usize) -> Result<Value, SimdJsonError> {
    if depth > MAX_NESTING_DEPTH {
        return Err(SimdJsonError::Serde(format!(
            "nesting depth exceeds maximum of {MAX_NESTING_DEPTH}"
        )));
    }

    match element.get_type() {
        ElementType::NullValue => Ok(Value::Null),
        ElementType::Bool => Ok(Value::Bool(element.get_bool()?)),
        ElementType::String => Ok(Value::String(element.get_string()?.to_owned())),
        ElementType::Int64 => {
            let v = element.get_int64()?;
            Ok(Value::Number(Number::from(v)))
        }
        ElementType::UInt64 => {
            let v = element.get_uint64()?;
            Ok(Value::Number(Number::from(v)))
        }
        ElementType::Double => {
            let v = element.get_double()?;
            match Number::from_f64(v) {
                Some(n) => Ok(Value::Number(n)),
                None => Err(SimdJsonError::Serde(format!(
                    "cannot represent {v} as a JSON number (NaN or Infinity)"
                ))),
            }
        }
        ElementType::Array => {
            let array = element.get_array()?;
            let mut vec = Vec::with_capacity(array.size());
            for child in array.iter() {
                vec.push(element_to_value_inner(&child, depth + 1)?);
            }
            Ok(Value::Array(vec))
        }
        ElementType::Object => {
            let object = element.get_object()?;
            let mut map = Map::new();
            for (key, child) in object.iter() {
                map.insert(
                    String::from(key),
                    element_to_value_inner(&child, depth + 1)?,
                );
            }
            Ok(Value::Object(map))
        }
    }
}
