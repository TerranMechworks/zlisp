use super::Value;

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Self::Int(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Self::Float(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Self::String(v.to_owned())
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Self::List(v)
    }
}

impl From<&[Value]> for Value {
    fn from(v: &[Value]) -> Self {
        Self::List(v.to_vec())
    }
}

impl<const N: usize> From<&[Value; N]> for Value {
    fn from(v: &[Value; N]) -> Self {
        Self::List(v.to_vec())
    }
}
