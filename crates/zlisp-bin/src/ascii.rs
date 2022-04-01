use crate::error::{Error, ErrorCode, Result};

pub fn from_raw<'a>(v: &'a [u8], start_offset: usize) -> Result<&'a str> {
    if v.len() > 255 {
        return Err(Error::new(ErrorCode::StringTooLong, Some(start_offset)));
    }

    for (offset, b) in (start_offset..).zip(v.iter().copied()) {
        if b == 0 {
            return Err(Error::new(ErrorCode::StringContainsNull, Some(offset)));
        }
        if b == b'"' {
            return Err(Error::new(ErrorCode::StringContainsQuote, Some(offset)));
        }
        if b & 0x80 != 0 {
            return Err(Error::new(
                ErrorCode::StringContainsInvalidByte,
                Some(offset),
            ));
        }
    }

    // SAFETY: v is ASCII, which is also valid UTF-8
    Ok(unsafe { std::str::from_utf8_unchecked(v) })
}

pub fn to_raw<'a>(s: &'a str) -> Result<(&'a [u8], i32)> {
    let v = s.as_bytes();

    let len: i32 = v
        .len()
        .try_into()
        .map_err(|_| Error::new(ErrorCode::StringTooLong, None))?;

    if len > 255 {
        return Err(Error::new(ErrorCode::StringTooLong, None));
    }

    for b in v.iter().copied() {
        if b == 0 {
            return Err(Error::new(ErrorCode::StringContainsNull, None));
        }
        if b == b'"' {
            return Err(Error::new(ErrorCode::StringContainsQuote, None));
        }
        if b & 0x80 != 0 {
            return Err(Error::new(ErrorCode::StringContainsInvalidByte, None));
        }
    }

    Ok((v, len))
}
