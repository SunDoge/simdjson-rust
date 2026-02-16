use crate::dom::{ArrayIter, Element, ElementType, ObjectIter};
use crate::error::SimdJsonError;

use serde::de::{
    Deserialize, DeserializeSeed, Deserializer, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};

fn de_error(msg: &str) -> SimdJsonError {
    SimdJsonError::Serde(msg.to_owned())
}

pub fn from_element<'a, T>(element: &'a Element<'a>) -> Result<T, SimdJsonError>
where
    T: Deserialize<'a>,
{
    let t = T::deserialize(element)?;
    Ok(t)
}

impl<'de, 'a> Deserializer<'de> for &'a Element<'a> {
    type Error = SimdJsonError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_type() {
            ElementType::NullValue => self.deserialize_unit(visitor),
            ElementType::Bool => self.deserialize_bool(visitor),
            ElementType::String => self.deserialize_string(visitor),
            ElementType::UInt64 => self.deserialize_u64(visitor),
            ElementType::Int64 => self.deserialize_i64(visitor),
            ElementType::Array => self.deserialize_seq(visitor),
            ElementType::Object => self.deserialize_map(visitor),
            ElementType::Double => self.deserialize_f64(visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.get_bool()?)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v = self.get_int64()?;
        let narrow =
            i8::try_from(v).map_err(|_| de_error(&format!("i64 value {v} out of range for i8")))?;
        visitor.visit_i8(narrow)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v = self.get_int64()?;
        let narrow = i16::try_from(v)
            .map_err(|_| de_error(&format!("i64 value {v} out of range for i16")))?;
        visitor.visit_i16(narrow)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v = self.get_int64()?;
        let narrow = i32::try_from(v)
            .map_err(|_| de_error(&format!("i64 value {v} out of range for i32")))?;
        visitor.visit_i32(narrow)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.get_int64()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v = self.get_uint64()?;
        let narrow =
            u8::try_from(v).map_err(|_| de_error(&format!("u64 value {v} out of range for u8")))?;
        visitor.visit_u8(narrow)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v = self.get_uint64()?;
        let narrow = u16::try_from(v)
            .map_err(|_| de_error(&format!("u64 value {v} out of range for u16")))?;
        visitor.visit_u16(narrow)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v = self.get_uint64()?;
        let narrow = u32::try_from(v)
            .map_err(|_| de_error(&format!("u64 value {v} out of range for u32")))?;
        visitor.visit_u32(narrow)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.get_uint64()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.get_double()? as f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.get_double()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s = self.get_string()?;
        let mut chars = s.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => visitor.visit_char(c),
            _ => Err(de_error("expected a single character string")),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s = self.get_string()?;
        visitor.visit_str(s)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s = self.get_string()?;
        visitor.visit_string(s.to_owned())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.is_null() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let array = self.get_array()?;
        let iter = array.iter();
        visitor.visit_seq(SeqAccessor(iter))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let object = self.get_object()?;
        let iter = object.iter();
        visitor.visit_map(MapAccessor::new(iter))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_type() {
            ElementType::String => {
                let s = self.get_string()?;
                let de: serde::de::value::StrDeserializer<'_, SimdJsonError> =
                    s.into_deserializer();
                visitor.visit_enum(de)
            }
            ElementType::Object => {
                let object = self.get_object()?;
                let mut iter = object.iter();
                let pair = iter.next();
                drop(iter);
                match pair {
                    Some((variant, value)) => {
                        let variant_owned = String::from(variant);
                        visitor.visit_enum(EnumDeserializer {
                            variant: variant_owned,
                            value,
                        })
                    }
                    None => Err(de_error("expected an object with a single key for enum")),
                }
            }
            _ => Err(de_error("expected a string or object for enum")),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct SeqAccessor<'a>(ArrayIter<'a>);

impl<'de, 'a> SeqAccess<'de> for SeqAccessor<'a> {
    type Error = SimdJsonError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.0.next() {
            Some(element) => seed.deserialize(&element).map(Some),
            None => Ok(None),
        }
    }
}

struct MapAccessor<'a> {
    iter: ObjectIter<'a>,
    pending_value: Option<Element<'a>>,
}

impl<'a> MapAccessor<'a> {
    fn new(iter: ObjectIter<'a>) -> Self {
        Self {
            iter,
            pending_value: None,
        }
    }
}

impl<'de, 'a> MapAccess<'de> for MapAccessor<'a> {
    type Error = SimdJsonError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.pending_value = Some(value);
                let de: serde::de::value::StringDeserializer<SimdJsonError> =
                    String::from(key).into_deserializer();
                seed.deserialize(de).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.pending_value.take() {
            Some(value) => seed.deserialize(&value),
            None => Err(de_error("next_value_seed called before next_key_seed")),
        }
    }
}

struct EnumDeserializer<'a> {
    variant: String,
    value: Element<'a>,
}

impl<'de, 'a> EnumAccess<'de> for EnumDeserializer<'a> {
    type Error = SimdJsonError;
    type Variant = VariantDeserializer<'a>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let de: serde::de::value::StringDeserializer<SimdJsonError> =
            self.variant.into_deserializer();
        let variant = seed.deserialize(de)?;
        Ok((variant, VariantDeserializer(self.value)))
    }
}

struct VariantDeserializer<'a>(Element<'a>);

impl<'de, 'a> VariantAccess<'de> for VariantDeserializer<'a> {
    type Error = SimdJsonError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(de_error(
            "expected a string for unit variant, got an object key",
        ))
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(&self.0)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        serde::Deserializer::deserialize_seq(&self.0, visitor)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        serde::Deserializer::deserialize_map(&self.0, visitor)
    }
}
