use crate::builder::StringBuilder;
use crate::error::SimdJsonError;

use serde::ser::{
    Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant, Serializer,
};

/// Serialize a value to a JSON string using simdjson's SIMD-accelerated builder.
pub fn to_string<T: Serialize>(value: &T) -> Result<String, SimdJsonError> {
    let mut builder = StringBuilder::new();
    value.serialize(&mut BuilderSerializer::new(&mut builder))?;
    builder.into_string()
}

/// Serialize a value to a JSON string with a pre-allocated capacity hint.
pub fn to_string_with_capacity<T: Serialize>(
    value: &T,
    capacity: usize,
) -> Result<String, SimdJsonError> {
    let mut builder = StringBuilder::with_capacity(capacity);
    value.serialize(&mut BuilderSerializer::new(&mut builder))?;
    builder.into_string()
}

pub struct BuilderSerializer<'a> {
    builder: &'a mut StringBuilder,
}

impl<'a> BuilderSerializer<'a> {
    pub fn new(builder: &'a mut StringBuilder) -> Self {
        Self { builder }
    }
}

impl<'a> Serializer for &'a mut BuilderSerializer<'a> {
    type Ok = ();
    type Error = SimdJsonError;

    type SerializeSeq = SeqSerializer<'a>;
    type SerializeTuple = SeqSerializer<'a>;
    type SerializeTupleStruct = SeqSerializer<'a>;
    type SerializeTupleVariant = SeqSerializer<'a>;
    type SerializeMap = MapSerializer<'a>;
    type SerializeStruct = MapSerializer<'a>;
    type SerializeStructVariant = MapSerializer<'a>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.builder.append_bool(v);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.builder.append_i64(v as i64);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.builder.append_i64(v as i64);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.builder.append_i64(v as i64);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.builder.append_i64(v);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.builder.append_u64(v as u64);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.builder.append_u64(v as u64);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.builder.append_u64(v as u64);
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.builder.append_u64(v);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.builder.append_f64(v as f64);
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        if !v.is_finite() {
            return Err(SimdJsonError::Serde(format!(
                "cannot serialize non-finite float: {v}"
            )));
        }
        self.builder.append_f64(v);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.builder.append_string(v);
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.builder.start_array();
        for (i, byte) in v.iter().enumerate() {
            if i > 0 {
                self.builder.append_comma();
            }
            self.builder.append_u64(*byte as u64);
        }
        self.builder.end_array();
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.builder.append_null();
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        self.builder.start_object();
        self.builder.append_string(variant);
        self.builder.append_colon();
        value.serialize(&mut BuilderSerializer::new(self.builder))?;
        self.builder.end_object();
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.builder.start_array();
        Ok(SeqSerializer {
            builder: self.builder,
            first: true,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.builder.start_object();
        self.builder.append_string(variant);
        self.builder.append_colon();
        self.builder.start_array();
        Ok(SeqSerializer {
            builder: self.builder,
            first: true,
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.builder.start_object();
        Ok(MapSerializer {
            builder: self.builder,
            first: true,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.builder.start_object();
        self.builder.append_string(variant);
        self.builder.append_colon();
        self.builder.start_object();
        Ok(MapSerializer {
            builder: self.builder,
            first: true,
        })
    }
}

pub struct SeqSerializer<'a> {
    builder: &'a mut StringBuilder,
    first: bool,
}

impl<'a> SerializeSeq for SeqSerializer<'a> {
    type Ok = ();
    type Error = SimdJsonError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        if !self.first {
            self.builder.append_comma();
        }
        self.first = false;
        value.serialize(&mut BuilderSerializer::new(self.builder))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.builder.end_array();
        Ok(())
    }
}

impl<'a> SerializeTuple for SeqSerializer<'a> {
    type Ok = ();
    type Error = SimdJsonError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<'a> SerializeTupleStruct for SeqSerializer<'a> {
    type Ok = ();
    type Error = SimdJsonError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<'a> SerializeTupleVariant for SeqSerializer<'a> {
    type Ok = ();
    type Error = SimdJsonError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.builder.end_array();
        self.builder.end_object();
        Ok(())
    }
}

pub struct MapSerializer<'a> {
    builder: &'a mut StringBuilder,
    first: bool,
}

impl<'a> SerializeMap for MapSerializer<'a> {
    type Ok = ();
    type Error = SimdJsonError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        if !self.first {
            self.builder.append_comma();
        }
        self.first = false;
        key.serialize(&mut BuilderSerializer::new(self.builder))
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.builder.append_colon();
        value.serialize(&mut BuilderSerializer::new(self.builder))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.builder.end_object();
        Ok(())
    }
}

impl<'a> SerializeStruct for MapSerializer<'a> {
    type Ok = ();
    type Error = SimdJsonError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        if !self.first {
            self.builder.append_comma();
        }
        self.first = false;
        self.builder.append_string(key);
        self.builder.append_colon();
        value.serialize(&mut BuilderSerializer::new(self.builder))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.builder.end_object();
        Ok(())
    }
}

impl<'a> SerializeStructVariant for MapSerializer<'a> {
    type Ok = ();
    type Error = SimdJsonError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.builder.end_object();
        self.builder.end_object();
        Ok(())
    }
}
