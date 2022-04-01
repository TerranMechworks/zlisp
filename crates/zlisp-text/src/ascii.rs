use crate::error::{Error, ErrorCode, Location, Result};

pub fn from_raw<'a>(s: &'a str, loc: Location) -> Result<()> {
    let v = s.as_bytes();
    if v.len() > 255 {
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

pub fn to_raw<'a>(s: &'a str) -> Result<bool> {
    let v = s.as_bytes();

    let len: i32 = v
        .len()
        .try_into()
        .map_err(|_| Error::new(ErrorCode::StringTooLong, None))?;

    if len > 255 {
        return Err(Error::new(ErrorCode::StringTooLong, None));
    }

    // TODO: figure out when strings need quoting

    let mut needs_quoting = false;
    for b in v.iter().copied() {
        match b {
            b'\0' => Err(Error::new(ErrorCode::StringContainsNull, None)),
            b'"' => Err(Error::new(ErrorCode::StringContainsQuote, None)),
            b' ' | b'\t' | b'\r' | b'\n' | b'(' | b')' => {
                needs_quoting = true;
                Ok(())
            }
            _ if b.is_ascii() => Ok(()),
            _ => Err(Error::new(ErrorCode::StringContainsInvalidChar, None)),
        }?;
    }

    Ok(needs_quoting)
}
