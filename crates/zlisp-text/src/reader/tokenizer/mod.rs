use crate::ascii::from_raw;
use crate::error::{Error, ErrorCode, Location, Result, TokenType};

/// A tokenizer for text zlisp data.
///
/// The tokenizer keeps track of the location in the text data. The tokenizer
/// does not try to parse scalars.
#[derive(Debug, Clone)]
pub struct Tokenizer<'a> {
    input: &'a str,
    line: usize,
    col: usize,
}

#[derive(Debug, Clone)]
pub enum Text<'a> {
    Quoted(String),
    Unquoted(&'a str),
}

#[derive(Debug, Clone)]
pub enum Token<'a> {
    Text(Text<'a>),
    ListStart,
    ListEnd,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Span<'a> {
    pub token: Token<'a>,
    pub loc: Location,
}

impl<'a> Span<'a> {
    pub const fn new(token: Token<'a>, loc: Location) -> Self {
        Self { token, loc }
    }

    pub fn expected(self, expected: TokenType) -> Error {
        let found = match &self.token {
            Token::Text(_) => TokenType::Text,
            Token::ListStart => TokenType::ListStart,
            Token::ListEnd => TokenType::ListEnd,
            Token::Eof => TokenType::Eof,
        };
        let code = ErrorCode::ExpectedToken { expected, found };
        Error::new(code, Some(self.loc))
    }
}

impl<'a> Tokenizer<'a> {
    pub const fn new(input: &'a str) -> Self {
        Self {
            input,
            line: 1,
            col: 0,
        }
    }

    pub fn location(&self) -> Location {
        Location::new(self.line, self.col)
    }

    fn read_quoted_text(&mut self, start: &'a str) -> Result<(Text<'a>, &'a str)> {
        let str_loc = self.location();
        let mut buffer = String::new();
        let mut iter = start.char_indices();
        while let Some((o, c)) = iter.next() {
            match c {
                '"' => {
                    self.col += 1;
                    // inside a quote
                    loop {
                        let (_o, c) = iter.next().ok_or_else(|| {
                            Error::new(ErrorCode::EofWhileParsingQuote, Some(self.location()))
                        })?;
                        match c {
                            // another quote is the only delimiter for the
                            // quoted section. however, this is not a delimiter
                            // for the value itself.
                            '"' => {
                                self.col += 1;
                                break;
                            }
                            '\0' => {
                                return Err(Error::new(
                                    ErrorCode::StringContainsNull,
                                    Some(self.location()),
                                ))
                            }
                            // a newline is a possibility inside a quote
                            '\n' => {
                                self.line += 1;
                                self.col = 0;
                            }
                            _ if c.is_ascii() => self.col += 1,
                            _ => {
                                return Err(Error::new(
                                    ErrorCode::StringContainsInvalidChar,
                                    Some(self.location()),
                                ))
                            }
                        }

                        buffer.push(c);
                    }
                }
                // found a delimiter
                ' ' | '\t' | '\r' | '\n' | '(' | ')' => {
                    let (_value, remaining) = start.split_at(o);
                    return from_raw(&buffer, str_loc).map(|()| (Text::Quoted(buffer), remaining));
                }
                '\0' => {
                    return Err(Error::new(
                        ErrorCode::StringContainsNull,
                        Some(self.location()),
                    ))
                }
                _ if c.is_ascii() => {
                    buffer.push(c);
                    self.col += 1;
                }
                _ => {
                    return Err(Error::new(
                        ErrorCode::StringContainsInvalidChar,
                        Some(self.location()),
                    ))
                }
            }
        }
        // consumed all of the input
        from_raw(&buffer, str_loc).map(|()| (Text::Quoted(buffer), ""))
    }

    fn read_text(&mut self, start: &'a str) -> Result<(Text<'a>, &'a str)> {
        let str_loc = self.location();
        for (o, c) in start.char_indices() {
            match c {
                // found a quote. the value can't be borrowed. quoting is rare,
                // so a performance hit of starting over/backtracking is
                // acceptable.
                '"' => return self.read_quoted_text(start),
                // found a delimiter
                ' ' | '\t' | '\r' | '\n' | '(' | ')' => {
                    let (value, remaining) = start.split_at(o);
                    return from_raw(&value, str_loc).map(|()| (Text::Unquoted(value), remaining));
                }
                '\0' => {
                    return Err(Error::new(
                        ErrorCode::StringContainsNull,
                        Some(self.location()),
                    ))
                }
                _ if c.is_ascii() => self.col += 1,
                _ => {
                    return Err(Error::new(
                        ErrorCode::StringContainsInvalidChar,
                        Some(self.location()),
                    ))
                }
            }
        }
        // consumed all of the input
        from_raw(start, str_loc).map(|()| (Text::Unquoted(start), ""))
    }

    pub fn read_token(&mut self) -> Result<Span<'a>> {
        for (o, c) in self.input.char_indices() {
            match c {
                '(' => {
                    // PANIC/SAFETY: '(' is one byte in UTF-8, so o + 1 is okay.
                    let (_discard, input) = self.input.split_at(o + 1);
                    self.input = input;
                    let span = Span::new(Token::ListStart, self.location());
                    self.col += 1;
                    return Ok(span);
                }
                ')' => {
                    // PANIC/SAFETY: ')' is one byte in UTF-8, so o + 1 is okay.
                    let (_discard, input) = self.input.split_at(o + 1);
                    self.input = input;
                    let span = Span::new(Token::ListEnd, self.location());
                    self.col += 1;
                    return Ok(span);
                }
                '\n' => {
                    self.line += 1;
                    self.col = 0;
                }
                ' ' | '\t' | '\r' => {
                    self.col += 1;
                }
                _ => {
                    let (_discard, start) = self.input.split_at(o);
                    let loc = self.location();
                    let (scalar, end) = self.read_text(start)?;
                    self.input = end;
                    return Ok(Span::new(Token::Text(scalar), loc));
                }
            }
        }
        Ok(Span::new(Token::Eof, self.location()))
    }
}
