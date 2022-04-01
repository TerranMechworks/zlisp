mod slice_reader;

use crate::error::Result;

/// Deserialize a value from binary zlisp data.
pub fn from_slice<'a, T>(s: &'a [u8]) -> Result<T>
where
    T: serde::Deserialize<'a>,
{
    let mut reader = slice_reader::SliceReader::new(s);
    reader.unwrap_outer_list()?;
    let v = T::deserialize(&mut reader)?;
    reader.finish()?;
    Ok(v)
}
