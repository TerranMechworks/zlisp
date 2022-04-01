mod private;
mod ser;

use crate::writer::config::WhitespaceConfig;

pub struct Gather;

#[derive(Debug, Clone)]
pub enum Variant {
    Unit,
    NewType(Box<Element>),
    Tuple(Vec<Element>),
    Struct(Vec<(&'static str, Element)>),
}

#[derive(Debug, Clone)]
pub enum Element {
    Unit,
    Scalar(String),
    Some(Box<Element>),
    Seq(Vec<Element>, bool),
    Map(Vec<(Element, Element)>),
    Struct(Vec<(&'static str, Element)>, bool),
    Enum(&'static str, Variant, bool),
}

impl Element {
    pub fn is_compact(&self) -> bool {
        match self {
            Self::Scalar(_) | Self::Unit => true,
            Self::Some(inner) => inner.is_compact(),
            Self::Seq(_, v) => *v,
            Self::Map(_) => false,
            Self::Struct(_, v) => *v,
            Self::Enum(_, _, v) => *v,
        }
    }
}

pub fn write(element: Element, config: &WhitespaceConfig<'_>) -> String {
    let writer = private::PrettyWriter::new(config);
    writer.write(element)
}
