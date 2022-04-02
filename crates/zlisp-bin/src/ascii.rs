use crate::constants::MAX_STRING_LEN;
use crate::error::{Error, ErrorCode, Result};

pub fn from_raw(v: &[u8], start_offset: usize) -> Result<&str> {
    // SAFETY: MAX_STRING_LEN < i32::MAX, usize::MIN > i32::MIN
    if v.len() > MAX_STRING_LEN {
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

pub fn to_raw(s: &str) -> Result<(&[u8], i32)> {
    let v = s.as_bytes();

    if v.len() > MAX_STRING_LEN {
        return Err(Error::new(ErrorCode::StringTooLong, None));
    }
    // SAFETY: MAX_STRING_LEN < i32::MAX, usize::MIN > i32::MIN
    let len = v.len() as i32;

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
