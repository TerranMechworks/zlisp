use crate::ascii::from_raw;
use crate::constants::{FLOAT, INT, LIST, STRING};
use crate::error::{Error, ErrorCode, Result, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    Int(i32),
    Float(f32),
    Str(&'a str),
    List(usize),
}

#[derive(Debug, Clone)]
pub struct SliceReader<'a> {
    input: &'a [u8],
    pub offset: usize,
}

impl<'a> SliceReader<'a> {
    pub const fn new(input: &'a [u8]) -> Self {
        Self { input, offset: 0 }
    }

    fn take_n(&mut self, n: usize) -> Result<&'a [u8]> {
        if self.input.len() >= n {
            // There is no const fn split_at yet: https://github.com/rust-lang/rust/issues/90091
            // PANIC: split_at should not panic, since we have just checked the
            // length
            let (take, input) = self.input.split_at(n);
            self.input = input;
            self.offset += n;
            Ok(take)
        } else {
            let code = ErrorCode::InsufficientData {
                expected: n,
                available: self.input.len(),
            };
            Err(Error::new(code, Some(self.offset)))
        }
    }

    fn take_4(&mut self) -> Result<&'a [u8; 4]> {
        // PANIC: this should be fine, since take_n should return a slice of
        // length 4 (or panic)
        self.take_n(4).map(|take| take.try_into().unwrap())
    }

    fn take_i32(&mut self) -> Result<i32> {
        self.take_4().map(|buf| i32::from_le_bytes(*buf))
    }

    fn take_f32(&mut self) -> Result<f32> {
        self.take_4().map(|buf| f32::from_le_bytes(*buf))
    }

    fn take_str(&mut self) -> Result<&'a str> {
        let offset = self.offset;
        let len = self.take_i32().and_then(|len| {
            len.try_into()
                .map_err(|_| Error::new(ErrorCode::InvalidStringLength, Some(offset)))
        })?;
        let str_offset = self.offset;
        self.take_n(len).and_then(|v| from_raw(v, str_offset))
    }

    fn take_list(&mut self) -> Result<usize> {
        let offset = self.offset;
        self.take_i32()
            .and_then(|len| {
                // for some reason, the length is one bigger than the values in the list
                len.checked_sub(1)
                    .ok_or_else(|| Error::new(ErrorCode::InvalidListLength, Some(offset)))
            })
            .and_then(|len| {
                len.try_into()
                    .map_err(|_| Error::new(ErrorCode::InvalidListLength, Some(offset)))
            })
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        fn expected_int(found: TokenType, offset: usize) -> Error {
            let code = ErrorCode::ExpectedToken {
                expected: TokenType::Int,
                found,
            };
            Error::new(code, Some(offset))
        }

        if self.input.is_empty() {
            return Err(expected_int(TokenType::Eof, self.offset));
        }

        let offset = self.offset;
        let ty = self.take_i32()?;
        match ty {
            INT => self.take_i32(),
            FLOAT => Err(expected_int(TokenType::Float, offset)),
            STRING => Err(expected_int(TokenType::String, offset)),
            LIST => Err(expected_int(TokenType::List, offset)),
            _ => Err(Error::new(ErrorCode::InvalidTokenType, Some(offset))),
        }
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        fn expected_float(found: TokenType, offset: usize) -> Error {
            let code = ErrorCode::ExpectedToken {
                expected: TokenType::Float,
                found,
            };
            Error::new(code, Some(offset))
        }

        if self.input.is_empty() {
            return Err(expected_float(TokenType::Eof, self.offset));
        }

        let offset = self.offset;
        let ty = self.take_i32()?;
        match ty {
            FLOAT => self.take_f32(),
            INT => Err(expected_float(TokenType::Int, offset)),
            STRING => Err(expected_float(TokenType::String, offset)),
            LIST => Err(expected_float(TokenType::List, offset)),
            _ => Err(Error::new(ErrorCode::InvalidTokenType, Some(offset))),
        }
    }

    pub fn read_str(&mut self) -> Result<&'a str> {
        fn expected_str(found: TokenType, offset: usize) -> Error {
            let code = ErrorCode::ExpectedToken {
                expected: TokenType::String,
                found,
            };
            Error::new(code, Some(offset))
        }

        if self.input.is_empty() {
            return Err(expected_str(TokenType::Eof, self.offset));
        }

        let offset = self.offset;
        let ty = self.take_i32()?;
        match ty {
            STRING => self.take_str(),
            INT => Err(expected_str(TokenType::Int, offset)),
            FLOAT => Err(expected_str(TokenType::Float, offset)),
            LIST => Err(expected_str(TokenType::List, offset)),
            _ => Err(Error::new(ErrorCode::InvalidTokenType, Some(offset))),
        }
    }

    pub fn read_list(&mut self) -> Result<(usize, usize)> {
        fn expected_list(found: TokenType, offset: usize) -> Error {
            let code = ErrorCode::ExpectedToken {
                expected: TokenType::List,
                found,
            };
            Error::new(code, Some(offset))
        }

        if self.input.is_empty() {
            return Err(expected_list(TokenType::Eof, self.offset));
        }

        let ty_offset = self.offset;
        let ty = self.take_i32()?;
        let len_offset = self.offset;
        match ty {
            LIST => self.take_list().map(|len| (len, len_offset)),
            INT => Err(expected_list(TokenType::Int, ty_offset)),
            FLOAT => Err(expected_list(TokenType::Float, ty_offset)),
            STRING => Err(expected_list(TokenType::String, ty_offset)),
            _ => Err(Error::new(ErrorCode::InvalidTokenType, Some(ty_offset))),
        }
    }

    pub fn read_any(&mut self) -> Result<Token<'a>> {
        if self.input.is_empty() {
            let code = ErrorCode::ExpectedToken {
                expected: TokenType::Any,
                found: TokenType::Eof,
            };
            return Err(Error::new(code, Some(self.offset)));
        }

        let offset = self.offset;
        let ty = self.take_i32()?;
        match ty {
            INT => self.take_i32().map(Token::Int),
            FLOAT => self.take_f32().map(Token::Float),
            STRING => self.take_str().map(Token::Str),
            LIST => self.take_list().map(Token::List),
            _ => Err(Error::new(ErrorCode::InvalidTokenType, Some(offset))),
        }
    }

    pub fn finish(self) -> Result<()> {
        if self.input.is_empty() {
            Ok(())
        } else {
            Err(Error::new(ErrorCode::TrailingData, Some(self.offset)))
        }
    }

    /// Binary zlisp data must always start with a list of length 1
    pub fn unwrap_outer_list(&mut self) -> Result<()> {
        let (len, offset) = self.read_list()?;
        match len {
            1 => Ok(()),
            _ => Err(Error::new(ErrorCode::InvalidListLength, Some(offset))),
        }
    }
}
