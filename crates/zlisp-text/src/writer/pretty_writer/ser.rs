use super::{Element, Gather, Variant};
use crate::ascii::to_raw;
use crate::error::{Error, ErrorCode, Result};
use crate::writer::ser_common::{map_len, require_len, struct_len, unsupported, validate_len};
use serde::{ser, Serialize};

fn compact(is_compact: bool, len: usize) -> bool {
    is_compact && len < 7
}

impl ser::Serializer for Gather {
    type Ok = Element;
    type Error = Error;

    type SerializeSeq = SeqGather;
    type SerializeTuple = SeqGather;
    type SerializeTupleStruct = SeqGather;

    type SerializeMap = MapGather;
    type SerializeStruct = StructGather;

    type SerializeTupleVariant = TupleEnumGather;
    type SerializeStructVariant = StructEnumGather;

    unsupported!(serialize_bool, bool);
    unsupported!(serialize_i8, i8);
    unsupported!(serialize_i16, i16);
    unsupported!(serialize_i64, i64);
    unsupported!(serialize_u8, u8);
    unsupported!(serialize_u16, u16);
    unsupported!(serialize_u32, u32);
    unsupported!(serialize_u64, u64);
    unsupported!(serialize_f64, f64);
    unsupported!(serialize_char, char);
    unsupported!(serialize_bytes, &[u8]);

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        Ok(Element::Scalar(format!("{}", v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        Ok(Element::Scalar(format!("{:.6}", v)))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        let needs_quoting = to_raw(v)?;
        let value = if needs_quoting {
            format!("\"{}\"", v)
        } else {
            format!("{}", v)
        };
        Ok(Element::Scalar(value))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(self)?;
        Ok(Element::Some(Box::new(v)))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(Element::Unit)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(Element::Unit)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        SeqGather::seq(len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        SeqGather::tuple(len)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_tuple(len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        MapGather::new(len)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        StructGather::new(len)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        // As is done here, serializers are encouraged to treat newtype structs as
        // insignificant wrappers around the data they contain.
        value.serialize(self)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(Element::Enum(variant, Variant::Unit, true))
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(self)?;
        let is_compact = v.is_compact();
        Ok(Element::Enum(
            variant,
            Variant::NewType(Box::new(v)),
            is_compact,
        ))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        TupleEnumGather::new(variant, len)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        StructEnumGather::new(variant, len)
    }
}

pub struct SeqGather {
    inner: Vec<Element>,
    is_compact: bool,
}

impl SeqGather {
    fn seq(len: Option<usize>) -> Result<Self> {
        require_len(len).and_then(validate_len)?;
        Ok(Self {
            inner: Vec::new(),
            is_compact: true,
        })
    }

    fn tuple(len: usize) -> Result<Self> {
        validate_len(len)?;
        Ok(Self {
            inner: Vec::new(),
            is_compact: true,
        })
    }

    fn push(&mut self, v: Element) {
        if !v.is_compact() {
            self.is_compact = false;
        }
        self.inner.push(v);
    }
}

impl ser::SerializeSeq for SeqGather {
    type Ok = Element;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(Gather)?;
        self.push(v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let is_compact = compact(self.is_compact, self.inner.len());
        Ok(Element::Seq(self.inner, is_compact))
    }
}

impl ser::SerializeTuple for SeqGather {
    type Ok = Element;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(Gather)?;
        self.push(v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let is_compact = compact(self.is_compact, self.inner.len());
        Ok(Element::Seq(self.inner, is_compact))
    }
}

impl ser::SerializeTupleStruct for SeqGather {
    type Ok = Element;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(Gather)?;
        self.push(v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let is_compact = compact(self.is_compact, self.inner.len());
        Ok(Element::Seq(self.inner, is_compact))
    }
}

pub struct MapGather {
    inner: Vec<(Element, Element)>,
    key: Option<Element>,
}

impl MapGather {
    fn new(len: Option<usize>) -> Result<Self> {
        validate_len(map_len(len).and_then(require_len)?)?;
        Ok(Self {
            inner: Vec::new(),
            key: None,
        })
    }
}

impl ser::SerializeMap for MapGather {
    type Ok = Element;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let k = key.serialize(Gather)?;
        self.key = Some(k);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let k = self.key.take().unwrap();
        let v = value.serialize(Gather)?;
        self.inner.push((k, v));
        Ok(())
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, key: &K, value: &V) -> Result<()>
    where
        K: Serialize,
        V: Serialize,
    {
        let k = key.serialize(Gather)?;
        let v = value.serialize(Gather)?;
        self.inner.push((k, v));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Element::Map(self.inner))
    }
}

pub struct StructGather {
    inner: Vec<(&'static str, Element)>,
    is_compact: bool,
}

impl StructGather {
    fn new(len: usize) -> Result<Self> {
        validate_len(struct_len(len)?)?;
        Ok(Self {
            inner: Vec::new(),
            is_compact: true,
        })
    }
}

impl ser::SerializeStruct for StructGather {
    type Ok = Element;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(Gather)?;
        if !v.is_compact() {
            self.is_compact = false;
        }
        self.inner.push((key, v));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let is_compact = compact(self.is_compact, self.inner.len().saturating_mul(2));
        Ok(Element::Struct(self.inner, is_compact))
    }
}

pub struct TupleEnumGather {
    variant: &'static str,
    inner: Vec<Element>,
    is_compact: bool,
}

impl TupleEnumGather {
    fn new(variant: &'static str, len: usize) -> Result<Self> {
        validate_len(len)?;
        Ok(Self {
            variant,
            inner: Vec::new(),
            is_compact: true,
        })
    }
}

impl ser::SerializeTupleVariant for TupleEnumGather {
    type Ok = Element;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(Gather)?;
        if !v.is_compact() {
            self.is_compact = false;
        }
        self.inner.push(v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let is_compact = compact(self.is_compact, self.inner.len());
        Ok(Element::Enum(
            self.variant,
            Variant::Tuple(self.inner),
            is_compact,
        ))
    }
}

pub struct StructEnumGather {
    variant: &'static str,
    inner: Vec<(&'static str, Element)>,
    is_compact: bool,
}

impl StructEnumGather {
    fn new(variant: &'static str, len: usize) -> Result<Self> {
        validate_len(struct_len(len)?)?;
        Ok(Self {
            variant,
            inner: Vec::new(),
            is_compact: true,
        })
    }
}

impl ser::SerializeStructVariant for StructEnumGather {
    type Ok = Element;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(Gather)?;
        if !v.is_compact() {
            self.is_compact = false;
        }
        self.inner.push((key, v));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let is_compact = compact(self.is_compact, self.inner.len());
        Ok(Element::Enum(
            self.variant,
            Variant::Struct(self.inner),
            is_compact,
        ))
    }
}
