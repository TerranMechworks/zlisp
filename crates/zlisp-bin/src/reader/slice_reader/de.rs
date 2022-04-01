use super::{SliceReader, Token};
use crate::error::{Error, ErrorCode, Result};
use serde::de::{self, Deserializer as _, Visitor};

macro_rules! unsupported {
    ($method:ident) => {
        fn $method<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            Err(Error::new(ErrorCode::UnsupportedType, Some(self.offset)))
        }
    };
}

impl<'a, 'de: 'a> de::Deserializer<'de> for &'a mut SliceReader<'de> {
    type Error = Error;

    fn is_human_readable(&self) -> bool {
        false
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

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.read_any()? {
            Token::Int(v) => visitor.visit_i32(v),
            Token::Float(v) => visitor.visit_f32(v),
            Token::Str(v) => visitor.visit_borrowed_str(v),
            Token::List(len) => visitor.visit_seq(SizedSeqAccess {
                deserializer: self,
                len,
            }),
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

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.read_str()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (len, offset) = self.read_list()?;
        match len {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            _ => {
                let code = ErrorCode::ExpectedListOfLength {
                    expected_min: 0,
                    expected_max: 1,
                    found: len,
                };
                Err(Error::new(code, Some(offset)))
            }
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (len, offset) = self.read_list()?;
        match len {
            0 => visitor.visit_unit(),
            _ => {
                let code = ErrorCode::ExpectedListOfLength {
                    expected_min: 0,
                    expected_max: 0,
                    found: len,
                };
                Err(Error::new(code, Some(offset)))
            }
        }
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
        let (len, _offset) = self.read_list()?;
        visitor.visit_seq(SizedSeqAccess {
            deserializer: self,
            len,
        })
    }

    fn deserialize_tuple<V>(self, tuple_len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (list_len, offset) = self.read_list()?;
        if list_len != tuple_len {
            let code = ErrorCode::ExpectedListOfLength {
                expected_min: tuple_len,
                expected_max: tuple_len,
                found: list_len,
            };
            return Err(Error::new(code, Some(offset)));
        }
        visitor.visit_seq(SizedSeqAccess {
            deserializer: self,
            len: tuple_len,
        })
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
        let (len, _offset) = self.read_list()?;
        visitor.visit_map(SizedSeqAccess {
            deserializer: self,
            len,
        })
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
        // using the sized map access here would be good, but this breaks for
        // optional fields. we have to defer to serde's mapping logic here.
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
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct SizedSeqAccess<'a, 'de> {
    deserializer: &'a mut SliceReader<'de>,
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
            let offset = self.deserializer.offset;
            seed.deserialize(&mut *self.deserializer)
                .map(Some)
                .map_err(|e| e.attach_offset(offset))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

impl<'a, 'de: 'a> de::MapAccess<'de> for SizedSeqAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.len == 0 {
            Ok(None)
        } else if self.len < 2 {
            Err(Error::new(
                ErrorCode::ExpectedKeyValuePair,
                Some(self.deserializer.offset),
            ))
        } else {
            self.len -= 2;
            let offset = self.deserializer.offset;
            seed.deserialize(&mut *self.deserializer)
                .map(Some)
                .map_err(|e| e.attach_offset(offset))
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        let offset = self.deserializer.offset;
        seed.deserialize(&mut *self.deserializer)
            .map_err(|e| e.attach_offset(offset))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

impl<'a, 'de: 'a> de::EnumAccess<'de> for &'a mut SliceReader<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let offset = self.offset;
        match seed.deserialize(&mut *self) {
            Ok(v) => Ok((v, self)),
            Err(e) => Err(e.attach_offset(offset)),
        }
    }
}

impl<'a, 'de: 'a> de::VariantAccess<'de> for &'a mut SliceReader<'de> {
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
        let (len, offset) = self.read_list()?;
        if len != 1 {
            let code = ErrorCode::ExpectedListOfLength {
                expected_min: 1,
                expected_max: 1,
                found: len,
            };
            return Err(Error::new(code, Some(offset)));
        }
        let offset = self.offset;
        seed.deserialize(&mut *self)
            .map_err(|e| e.attach_offset(offset))
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // tuple variants are represented in zlisp as `NAME ( V ... )`, and
        // EnumAccess has already read `NAME`, so read `( V ... )` here.
        self.deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // struct variants are represented in zlisp as `NAME ( K V ... )`, and
        // EnumAccess has already read `NAME`, so read `( K V ... )` here.
        let (len, _offset) = self.read_list()?;
        // Warning: do not compare len to the fields, this would break for e.g.
        // optional fields.
        visitor.visit_map(SizedSeqAccess {
            deserializer: &mut *self,
            len,
        })
    }
}
