use crate::error::{Location, Result, TokenType};
use crate::reader::parse::{parse_any, parse_f32, parse_i32, parse_string, Any};
use crate::reader::tokenizer::{Span, Token, Tokenizer};

#[derive(Debug, Clone)]
pub struct StrReader<'a> {
    inner: Tokenizer<'a>,
    buffer: Option<Span<'a>>,
}

impl<'a> StrReader<'a> {
    pub const fn new(input: &'a str) -> Self {
        Self {
            inner: Tokenizer::new(input),
            buffer: None,
        }
    }

    fn next_span(&mut self) -> Result<Span<'a>> {
        if let Some(span) = self.buffer.take() {
            Ok(span)
        } else {
            self.inner.read_token()
        }
    }

    pub fn peek(&mut self) -> Result<Span<'a>> {
        if let Some(span) = self.buffer.as_ref() {
            Ok(span.clone())
        } else {
            let span = self.inner.read_token()?;
            self.buffer = Some(span.clone());
            Ok(span)
        }
    }

    pub fn location(&self) -> Location {
        if let Some(span) = self.buffer.as_ref() {
            span.loc.clone()
        } else {
            self.inner.location()
        }
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        self.next_span().and_then(parse_i32)
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        self.next_span().and_then(parse_f32)
    }

    pub fn read_string(&mut self) -> Result<String> {
        self.next_span().and_then(parse_string)
    }

    pub fn read_any(&mut self) -> Result<Any> {
        self.next_span().and_then(parse_any)
    }

    pub fn read_list_start(&mut self) -> Result<()> {
        let span = self.next_span()?;
        match span.token {
            Token::ListStart => Ok(()),
            _ => Err(span.expected(TokenType::ListStart)),
        }
    }

    pub fn read_list_end(&mut self) -> Result<()> {
        let span = self.next_span()?;
        match span.token {
            Token::ListEnd => Ok(()),
            _ => Err(span.expected(TokenType::ListEnd)),
        }
    }

    pub fn read_list<F, V>(&mut self, f: F) -> Result<V>
    where
        F: FnOnce(&mut StrReader<'a>) -> Result<V>,
    {
        self.read_list_start()?;
        let v = f(self)?;
        self.read_list_end()?;
        Ok(v)
    }

    pub fn finish(mut self) -> Result<()> {
        let span = self.next_span()?;
        match span.token {
            Token::Eof => Ok(()),
            _ => Err(span.expected(TokenType::Eof)),
        }
    }
}

#[cfg(test)]
mod tests;
