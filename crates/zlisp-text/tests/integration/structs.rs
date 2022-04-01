use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct UnitStruct;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct NewTypeStruct(pub i32);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TupleStruct(pub i32, pub i32);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Struct {
    pub a: i32,
    pub b: i32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct OptStruct {
    #[serde(default)]
    pub a: i32,
    #[serde(default)]
    pub b: i32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum UnitVariant {
    V,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum NewTypeVariant {
    V(i32),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum TupleVariant {
    V(i32, i32),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum StructVariant {
    V { a: i32, b: i32 },
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum OptStructVariant {
    V {
        #[serde(default)]
        a: i32,
        #[serde(default)]
        b: i32,
    },
}

pub struct Bytes<'a>(pub &'a [u8]);

impl<'a> serde::ser::Serialize for Bytes<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_bytes(self.0)
    }
}
