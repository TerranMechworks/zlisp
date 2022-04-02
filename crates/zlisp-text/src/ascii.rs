use crate::constants::MAX_STRING_LEN;
use crate::error::{Error, ErrorCode, Location, Result};

pub fn from_raw(s: &str, loc: Location) -> Result<()> {
    let v = s.as_bytes();
    // SAFETY: MAX_STRING_LEN < i32::MAX, usize::MIN > i32::MIN
    if v.len() > MAX_STRING_LEN {
        return Err(Error::new(ErrorCode::StringTooLong, Some(loc)));
    }

    for b in v.iter().copied() {
        if b == 0 {
            return Err(Error::new(ErrorCode::StringContainsNull, Some(loc)));
        }
        if b == b'"' {
            return Err(Error::new(ErrorCode::StringContainsQuote, Some(loc)));
        }
        if b & 0x80 != 0 {
            return Err(Error::new(ErrorCode::StringContainsInvalidChar, Some(loc)));
        }
    }

    Ok(())
}

pub fn to_raw(s: &str) -> Result<bool> {
    // empty strings must always be quoted, otherwise they will disappear
    if s.is_empty() {
        return Ok(true);
    }

    let v = s.as_bytes();
    // SAFETY: MAX_STRING_LEN < i32::MAX, usize::MIN > i32::MIN
    if v.len() > MAX_STRING_LEN {
        return Err(Error::new(ErrorCode::StringTooLong, None));
    }

    let mut needs_quoting = false;
    let mut possible_number = true;
    for b in v.iter().copied() {
        match b {
            b'\0' => Err(Error::new(ErrorCode::StringContainsNull, None)),
            b'"' => Err(Error::new(ErrorCode::StringContainsQuote, None)),
            b' ' | b'\t' | b'\r' | b'\n' | b'(' | b')' => {
                possible_number = false;
                needs_quoting = true;
                Ok(())
            }
            b'-' | b'+' | b'.' | b'0'..=b'9' => {
                // possible number remains true
                Ok(())
            }
            _ if b.is_ascii() => {
                possible_number = false;
                Ok(())
            }
            _ => Err(Error::new(ErrorCode::StringContainsInvalidChar, None)),
        }?;
    }

    // if the string needs quoting, it is. also, if the string *could* be a
    // number, we quote it regardless. this avoids actually needing to parse
    // the string to an integer or a float, which is expensive. the downside is
    // there may be false positives, but worst case is a string is quoted when
    // it didn't need to be.
    Ok(needs_quoting || possible_number)
}
