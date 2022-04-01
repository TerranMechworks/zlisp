use serde::{de, ser};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Any {
    Int(i32),
    Float(f32),
    String(String),
    List(Vec<Any>),
}

impl Any {
    pub fn str(v: &str) -> Self {
        Self::String(v.to_owned())
    }
}

struct ValueVisitor;

impl<'de> de::Visitor<'de> for ValueVisitor {
    type Value = Any;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("any valid zlisp value")
    }

    fn visit_i32<E>(self, v: i32) -> Result<Any, E> {
        Ok(Any::Int(v))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Any, E> {
        Ok(Any::Float(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Any, E>
    where
        E: de::Error,
    {
        self.visit_string(String::from(v))
    }

    fn visit_string<E>(self, v: String) -> Result<Any, E> {
        Ok(Any::String(v))
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Any, V::Error>
    where
        V: de::SeqAccess<'de>,
    {
        let mut vec = visitor
            .size_hint()
            .map_or_else(|| Vec::new(), |capacity| Vec::with_capacity(capacity));
        while let Some(elem) = visitor.next_element()? {
            vec.push(elem);
        }
        Ok(Any::List(vec))
    }
}

impl<'de> de::Deserialize<'de> for Any {
    fn deserialize<D>(deserializer: D) -> Result<Any, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

impl ser::Serialize for Any {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *self {
            Self::Int(v) => serializer.serialize_i32(v),
            Self::Float(v) => serializer.serialize_f32(v),
            Self::String(ref s) => serializer.serialize_str(s),
            Self::List(ref v) => v.serialize(serializer),
        }
    }
}
