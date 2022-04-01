use super::StrReader;
use crate::error::{Error, ErrorCode, Result};
use crate::reader::parse::Any;
use crate::reader::tokenizer::Token;
use serde::de::{self, Deserializer as _, Visitor};

macro_rules! unsupported {
    ($method:ident) => {
        fn $method<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            Err(Error::new(
                ErrorCode::UnsupportedType,
                Some(self.location()),
            ))
        }
    };
}

impl<'a, 'de: 'a> de::Deserializer<'de> for &'a mut StrReader<'de> {
    type Error = Error;

    fn is_human_readable(&self) -> bool {
        true
    }

    unsupported!(deserialize_bool);
    unsupported!(deserialize_i8);
    unsupported!(deserialize_i16);
    unsupported!(deserialize_i64);
    unsupported!(deserialize_u8);
    unsupported!(deserialize_u16);
    unsupported!(deserialize_u32);
    unsupported!(deserialize_u64);
    unsupported!(deserialize_f64);
    unsupported!(deserialize_char);
    unsupported!(deserialize_bytes);
    unsupported!(deserialize_byte_buf);
    unsupported!(deserialize_str);

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.read_any()? {
            Any::Int(v) => visitor.visit_i32(v),
            Any::Float(v) => visitor.visit_f32(v),
            Any::String(v) => visitor.visit_string(v),
            Any::ListStart => {
                let v = visitor.visit_seq(UnsizedSeqAccess { deserializer: self })?;
                self.read_list_end()?;
                Ok(v)
            }
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.read_i32()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.read_f32()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.read_string()?)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.read_list(|reader| {
            let span = reader.peek()?;
            match &span.token {
                Token::Text(_) | Token::ListStart => visitor.visit_some(reader),
                Token::ListEnd | Token::Eof => visitor.visit_none(),
            }
        })
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.read_list(|_reader| visitor.visit_unit())
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // As is done here, serializers are encouraged to treat newtype structs
        // as insignificant wrappers around the data they contain.
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.read_list(|deserializer| visitor.visit_seq(UnsizedSeqAccess { deserializer }))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.read_list(|deserializer| visitor.visit_seq(SizedSeqAccess { deserializer, len }))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.read_list(|deserializer| visitor.visit_map(UnsizedSeqAccess { deserializer }))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
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
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // enums variants can be unit, newtype, tuple, and struct
        visitor.visit_enum(self)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct SizedSeqAccess<'a, 'de> {
    deserializer: &'a mut StrReader<'de>,
    len: usize,
}

impl<'a, 'de: 'a> de::SeqAccess<'de> for SizedSeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.len > 0 {
            self.len -= 1;
            let loc = self.deserializer.location();
            seed.deserialize(&mut *self.deserializer)
                .map(Some)
                .map_err(|e| e.attach_location(loc))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

struct UnsizedSeqAccess<'a, 'de> {
    deserializer: &'a mut StrReader<'de>,
}

impl<'a, 'de: 'a> de::SeqAccess<'de> for UnsizedSeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        let span = self.deserializer.peek()?;
        match span.token {
            // list start could be part of the interior type
            Token::Text(_) | Token::ListStart => {
                let loc = self.deserializer.location();
                seed.deserialize(&mut *self.deserializer)
                    .map(Some)
                    .map_err(|e| e.attach_location(loc))
            }
            // handling eof this way just means an error happens when we look
            // for the list end later, and produces a nice error message (the
            // caller must ensure a sequence is terminated with ListEnd)
            Token::ListEnd | Token::Eof => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

impl<'a, 'de: 'a> de::MapAccess<'de> for UnsizedSeqAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        let span = self.deserializer.peek()?;
        match span.token {
            // list start could be part of the interior type
            Token::Text(_) | Token::ListStart => {
                let loc = self.deserializer.location();
                seed.deserialize(&mut *self.deserializer)
                    .map(Some)
                    .map_err(|e| e.attach_location(loc))
            }
            // handling eof this way just means an error happens when we look
            // for the list end later, and produces a nice error message (the
            // caller must ensure a sequence is terminated with ListEnd)
            Token::ListEnd | Token::Eof => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        let loc = self.deserializer.location();
        seed.deserialize(&mut *self.deserializer)
            .map_err(|e| e.attach_location(loc))
    }

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

impl<'a, 'de: 'a> de::EnumAccess<'de> for &'a mut StrReader<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let loc = self.location();
        match seed.deserialize(&mut *self) {
            Ok(v) => Ok((v, self)),
            Err(e) => Err(e.attach_location(loc)),
        }
    }
}

impl<'a, 'de: 'a> de::VariantAccess<'de> for &'a mut StrReader<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        // unit variants are represented in zlisp as `NAME`, and EnumAccess
        // has already read `NAME`, so do nothing here.
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        // newtype variants are represented in zlisp as `NAME ( V )`, and
        // EnumAccess has already read `NAME`, so read ` ( V )` here.
        self.read_list(|reader| {
            let loc = reader.location();
            seed.deserialize(reader).map_err(|e| e.attach_location(loc))
        })
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // tuple variants are represented in zlisp as `NAME ( V ... )`, and
        // EnumAccess has already read `NAME`, so read `( V ... )` here.
        self.deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // struct variants are represented in zlisp as `NAME ( K1 V1 ... )`, and
        // EnumAccess has already read `NAME`, so read `( K1 V1 ... )` here.
        self.deserialize_struct("", fields, visitor)
    }
}
