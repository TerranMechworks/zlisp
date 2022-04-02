use crate::ascii::to_raw;
use crate::constants::{FLOAT, INT, LIST, MAX_LIST_LEN, STRING};
use crate::error::{Error, ErrorCode, Result};
use std::io::Write;

#[derive(Debug, Clone)]
pub struct IoWriter<W> {
    inner: W,
}

impl<W> IoWriter<W> {
    pub const fn new(inner: W) -> Self {
        Self { inner }
    }
}

impl<W: Write> IoWriter<W> {
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.inner
            .write_all(buf)
            .map_err(|e| Error::new(ErrorCode::IO(e), None))
    }

    pub fn write_i32(&mut self, v: i32) -> Result<()> {
        self.write_all(&INT.to_le_bytes())?;
        self.write_all(&v.to_le_bytes())
    }

    pub fn write_f32(&mut self, v: f32) -> Result<()> {
        self.write_all(&FLOAT.to_le_bytes())?;
        self.write_all(&v.to_le_bytes())
    }

    pub fn write_str(&mut self, v: &str) -> Result<()> {
        let (v, len) = to_raw(v)?;
        self.write_all(&STRING.to_le_bytes())?;
        self.write_all(&len.to_le_bytes())?;
        self.write_all(v)
    }

    pub fn write_list(&mut self, len: Option<usize>) -> Result<()> {
        let len: i32 = len
            .ok_or_else(|| Error::new(ErrorCode::SequenceMustHaveLength, None))
            .and_then(|len| {
                if len > MAX_LIST_LEN {
                    Err(Error::new(ErrorCode::SequenceTooLong, None))
                } else {
                    Ok(len as i32)
                }
            })?;
        // SAFETY: len < i32::MAX
        self.write_list_unchecked(len)
    }

    pub fn write_list_unchecked(&mut self, len: i32) -> Result<()> {
        let count = len + 1;
        self.write_all(&LIST.to_le_bytes())?;
        self.write_all(&count.to_le_bytes())
    }

    pub fn finish(mut self) -> Result<W> {
        self.inner
            .flush()
            .map_err(|e| Error::new(ErrorCode::IO(e), None))?;
        Ok(self.inner)
    }
}
