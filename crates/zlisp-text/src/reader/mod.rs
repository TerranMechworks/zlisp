mod parse;
mod str_reader;
mod tokenizer;

use crate::error::Result;

/// Deserialize a value from text zlisp data.
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: serde::Deserialize<'a>,
{
    let mut reader = str_reader::StrReader::new(s);
    let v = T::deserialize(&mut reader)?;
    reader.finish()?;
    Ok(v)
}
