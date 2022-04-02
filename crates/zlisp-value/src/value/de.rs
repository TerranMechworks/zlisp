use super::Value;
use serde::de;
use std::fmt;

struct ValueVisitor;

impl<'de> de::Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("any valid zlisp value")
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> {
        Ok(Value::Int(v))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> {
        Ok(Value::Float(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_string(String::from(v))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
        Ok(Value::String(v))
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: de::SeqAccess<'de>,
    {
        let mut vec = visitor
            .size_hint()
            .map_or_else(Vec::new, Vec::with_capacity);
        while let Some(elem) = visitor.next_element()? {
            vec.push(elem);
        }
        Ok(Value::List(vec))
    }
}

impl<'de> de::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}
