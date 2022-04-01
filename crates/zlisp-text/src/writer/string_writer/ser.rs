use super::StringWriter;
use crate::error::{Error, ErrorCode, Result};
use crate::writer::ser_common::{map_len, require_len, struct_len, unsupported, validate_len};
use serde::{ser, Serialize};

impl<'a, 'b: 'a> ser::Serializer for &'a mut StringWriter<'b, 'b> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

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

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.write_i32(v);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.write_f32(v);
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.write_str(v)
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.write_list_start_unchecked();
        value.serialize(&mut *self)?;
        self.write_list_end();
        Ok(())
    }

    fn serialize_unit(self) -> Result<()> {
        self.write_unit();
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // As is done here, serializers are encouraged to treat newtype structs as
        // insignificant wrappers around the data they contain.
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        variant.serialize(&mut *self)?;
        self.write_list_start_unchecked();
        value.serialize(&mut *self)?;
        self.write_list_end();
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        let count = require_len(len).and_then(validate_len)?;
        self.write_list_start(count)?;
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        let count = validate_len(len)?;
        self.write_list_start(count)?;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        let count = validate_len(len)?;
        variant.serialize(&mut *self)?;
        self.write_list_start(count)?;
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        // a map is key and value, so the length has to be doubled
        self.serialize_seq(map_len(len)?)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        // a struct is key and value, so the length has to be doubled
        self.serialize_tuple(struct_len(len)?)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        variant.serialize(&mut *self)?;
        // a struct is key and value, so the length has to be doubled
        let count = validate_len(struct_len(len)?)?;
        self.write_list_start(count)?;
        Ok(self)
    }
}

impl<'a, 'b: 'a> ser::SerializeSeq for &'a mut StringWriter<'b, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.write_list_end();
        Ok(())
    }
}

impl<'a, 'b: 'a> ser::SerializeTuple for &'a mut StringWriter<'b, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.write_list_end();
        Ok(())
    }
}

impl<'a, 'b: 'a> ser::SerializeTupleStruct for &'a mut StringWriter<'b, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.write_list_end();
        Ok(())
    }
}

impl<'a, 'b: 'a> ser::SerializeTupleVariant for &'a mut StringWriter<'b, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.write_list_end();
        Ok(())
    }
}

impl<'a, 'b: 'a> ser::SerializeMap for &'a mut StringWriter<'b, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, key: &K, value: &V) -> Result<()>
    where
        K: Serialize,
        V: Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.write_list_end();
        Ok(())
    }
}

impl<'a, 'b: 'a> ser::SerializeStruct for &'a mut StringWriter<'b, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.write_list_end();
        Ok(())
    }
}

impl<'a, 'b: 'a> ser::SerializeStructVariant for &'a mut StringWriter<'b, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.write_list_end();
        Ok(())
    }
}
