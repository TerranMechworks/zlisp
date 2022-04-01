use crate::error::{Error, ErrorCode, Result};

pub fn struct_len(len: usize) -> Result<usize> {
    len.checked_mul(2)
        .ok_or_else(|| Error::new(ErrorCode::SequenceTooLong, None))
}

pub fn map_len(len: Option<usize>) -> Result<Option<usize>> {
    len.map(struct_len).transpose()
}

pub fn require_len(len: Option<usize>) -> Result<usize> {
    len.ok_or_else(|| Error::new(ErrorCode::SequenceMustHaveLength, None))
}

pub fn validate_len(len: usize) -> Result<i32> {
    len.try_into()
        .map_err(|_| Error::new(ErrorCode::SequenceTooLong, None))
}

macro_rules! unsupported {
    ($method:ident, $type:ty) => {
        fn $method(self, _value: $type) -> Result<Self::Ok> {
            Err(Error::new(ErrorCode::UnsupportedType, None))
        }
    };
}

pub(crate) use unsupported;
