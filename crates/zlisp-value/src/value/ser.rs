use super::Value;
use serde::ser;

impl ser::Serialize for Value {
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
