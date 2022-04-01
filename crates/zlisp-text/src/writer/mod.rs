mod config;
mod pretty_writer;
mod ser_common;
mod string_writer;

pub use config::{WhitespaceConfig, WhitespaceConfigBuilder};

use crate::error::Result;

/// Serialize a value to text zlisp data.
pub fn to_string<T>(value: &T, config: &WhitespaceConfig<'_>) -> Result<String>
where
    T: ?Sized + serde::Serialize,
{
    let mut serializer = string_writer::StringWriter::new(config);
    value.serialize(&mut serializer)?;
    serializer.finish()
}

/// Serialize a value to text zlisp data.
pub fn to_pretty<T>(value: &T, config: &WhitespaceConfig<'_>) -> Result<String>
where
    T: ?Sized + serde::Serialize,
{
    let element = value.serialize(pretty_writer::Gather)?;
    Ok(pretty_writer::write(element, config))
}
