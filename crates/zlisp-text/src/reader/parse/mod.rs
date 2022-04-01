use super::tokenizer::{Span, Text, Token};
use crate::error::{Error, ErrorCode, Location, Result, TokenType};
use std::num::ParseFloatError;

#[derive(Debug, Clone, PartialEq)]
pub enum Any {
    Int(i32),
    Float(f32),
    String(String),
    ListStart,
}

fn parse_i32_inner<'a>(s: &'a str, loc: Location) -> Result<i32> {
    // i32::from_str_radix does exactly what we want. it allows signs (- or +),
    // it does not allow empty strings, or just the sign. and it only allows
    // digits other than the sign.
    i32::from_str_radix(s, 10).map_err(|e| {
        let code = ErrorCode::ParseIntError {
            e,
            s: s.to_string(),
        };
        Error::new(code, Some(loc))
    })
}

/// hack to construct a new ParseFloatError

fn pfe_invalid() -> ParseFloatError {
    "-".parse::<f32>().unwrap_err()
}

fn float_invalid<'a>(e: ParseFloatError, s: &'a str, loc: Location) -> Error {
    let code = ErrorCode::ParseFloatError { e, s: s.to_owned() };
    Error::new(code, Some(loc))
}

fn parse_f32_inner<'a>(s: &'a str, loc: Location) -> Result<f32> {
    // first, parsing floats is hard, see the core `dec2flt` module.
    // unfortunately, Rust's float parsing allows for exponent forms (e.g.
    // '2.5e10'), and non-finite values (e.g. 'inf', '-inf', '+infinity',
    // and 'NaN'). worse still, out of range values are converted to infinity.
    // and finally, ParseFloatError (PFE) cannot easily be constructed. there
    // exist only two kinds of PFE, empty and invalid. the only way to
    // construct a PFE i know of is to have a hacky helper method trying to
    // parse a known, invalid string. this is acceptable, since the method will
    // only be called in the bad/error path.
    //
    // the strategy then is to first validate the input, before using Rust's
    // built-in parsing, and finally verifying the parsing.

    // validate the input. this ensures we reject exponent and non-finite forms
    let mut v = s.as_bytes();
    match v.first() {
        // skip the sign
        Some(b'-') | Some(b'+') => v = &v[1..],
        // don't validate digits yet
        Some(_) => (),
        // don't care about an empty input, the float parsing handles this
        None => (),
    }
    let mut seen_point = false;
    for c in v.iter() {
        match c {
            // '.' can only appear once
            b'.' if !seen_point => seen_point = true,
            // digits can appear wherever
            b'0'..=b'9' => (),
            _ => return Err(float_invalid(pfe_invalid(), s, loc)),
        }
    }

    str::parse(s)
        .and_then(|f: f32| {
            // annoyingly, parsing a float allows +inf, -inf, and NaN, which can happen
            // if the float is too big for f32
            if f.is_finite() {
                Ok(f)
            } else {
                Err(pfe_invalid())
            }
        })
        .map_err(|e| float_invalid(e, s, loc))
}

fn parse_any_inner<'a>(s: &'a str, loc: Location) -> Result<Any> {
    if let Ok(v) = parse_i32_inner(s, loc.clone()) {
        return Ok(Any::Int(v));
    }
    if let Ok(v) = parse_f32_inner(s, loc) {
        return Ok(Any::Float(v));
    }
    Ok(Any::String(s.to_owned()))
}

pub fn parse_i32<'a>(span: Span<'a>) -> Result<i32> {
    match span.token {
        Token::Text(text) => match text {
            Text::Quoted(_) => {
                let code = ErrorCode::QuotedString;
                Err(Error::new(code, Some(span.loc)))
            }
            Text::Unquoted(s) => parse_i32_inner(s, span.loc),
        },
        _ => Err(span.expected(TokenType::Text)),
    }
}

pub fn parse_f32<'a>(span: Span<'a>) -> Result<f32> {
    match span.token {
        Token::Text(text) => match text {
            Text::Quoted(_) => {
                let code = ErrorCode::QuotedString;
                Err(Error::new(code, Some(span.loc)))
            }
            Text::Unquoted(s) => parse_f32_inner(s, span.loc),
        },
        _ => Err(span.expected(TokenType::Text)),
    }
}

pub fn parse_string<'a>(span: Span<'a>) -> Result<String> {
    match span.token {
        Token::Text(text) => match text {
            Text::Quoted(s) => Ok(s),
            Text::Unquoted(s) => Ok(s.to_owned()),
        },
        _ => Err(span.expected(TokenType::Text)),
    }
}

pub fn parse_any<'a>(span: Span<'a>) -> Result<Any> {
    match span.token {
        Token::Text(text) => match text {
            Text::Quoted(s) => Ok(Any::String(s)),
            Text::Unquoted(s) => parse_any_inner(s, span.loc),
        },
        Token::ListStart => Ok(Any::ListStart),
        _ => Err(span.expected(TokenType::TextOrListStart)),
    }
}

#[cfg(test)]
mod tests;
