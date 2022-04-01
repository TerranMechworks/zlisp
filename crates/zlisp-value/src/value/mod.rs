mod de;
mod display;
mod from;
mod ser;

use std::fmt;

/// Represents any valid zlisp value.
#[derive(Clone, PartialEq)]
pub enum Value {
    /// Represents an integer.
    Int(i32),
    /// Represents a float.
    Float(f32),
    /// Represents a string.
    String(String),
    /// Represents a list.
    List(Vec<Value>),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(v) => f.debug_tuple("Int").field(v).finish(),
            Self::Float(v) => f.debug_tuple("Float").field(v).finish(),
            Self::String(v) => f.debug_tuple("String").field(v).finish(),
            Self::List(v) => f.debug_list().entries(v.iter()).finish(),
        }
    }
}
