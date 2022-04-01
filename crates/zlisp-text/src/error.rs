use serde::{de, ser};
use std::fmt;

/// A high-level description of a token.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    /// Text representing a scalar (int, float, unquoted, or quoted string).
    Text,
    /// The start of a list.
    ListStart,
    /// The end of a list.
    ListEnd,
    /// The enf of the file.
    Eof,
    /// Text or the start of a list.
    TextOrListStart,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Text => f.write_str("text"),
            TokenType::ListStart => f.write_str("start of list"),
            TokenType::ListEnd => f.write_str("end of list"),
            TokenType::Eof => f.write_str("end of file"),
            TokenType::TextOrListStart => f.write_str("text or start of list"),
        }
    }
}

/// The detailed cause of an error.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorCode {
    // --- General ---
    /// A custom error message.
    ///
    /// This is how serde errors are reported.
    Custom(String),
    /// The data type is not supported by the serializer or deserializer.
    UnsupportedType,
    // --- Tokenizer ---
    /// An opening quote was found, but no closing quote.
    EofWhileParsingQuote,
    // --- Parser ---
    /// A token was expected, but an incompatible token was found.
    ExpectedToken {
        /// The expected token type.
        expected: TokenType,
        /// The actual token type.
        found: TokenType,
    },
    /// An integer could not be parsed from a text token.
    ParseIntError {
        /// The parsing error.
        e: std::num::ParseIntError,
        /// The entire text token.
        s: String,
    },
    /// A float could not be parsed from a text token.
    ParseFloatError {
        /// The parsing error.
        e: std::num::ParseFloatError,
        /// The entire text token.
        s: String,
    },
    /// A quoted string may not be converted to an int or float.
    QuotedString,

    // --- Writers ---
    /// A sequence is too long to serialize.
    SequenceTooLong,
    /// A sequence must have a length to be serialized.
    SequenceMustHaveLength,

    // --- Strings ---
    /// A string is too long.
    ///
    /// Strings may not be longer than 255 bytes.
    StringTooLong,
    /// A string contains a null character.
    StringContainsNull,
    /// A string contains a quote character.
    StringContainsQuote,
    /// A string contains an invalid character.
    StringContainsInvalidChar,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // General
            ErrorCode::Custom(s) => write!(f, "{}", s),
            ErrorCode::UnsupportedType => f.write_str("unsupported type"),
            // Tokenizer
            ErrorCode::EofWhileParsingQuote => {
                f.write_str("end of file while parsing a quoted string")
            }
            // Parser
            ErrorCode::ExpectedToken { expected, found } => {
                write!(f, "expected {}, found {}", expected, found)
            }
            ErrorCode::ParseIntError { e, s } => {
                // PIE Empty: "cannot parse integer from empty string"
                // PIE Invalid: "invalid digit found in string"
                // PIE Overflow: "number too large to fit in target type"
                write!(f, "{}: `{}`", e, s)
            }
            ErrorCode::ParseFloatError { e, s } => {
                // PFE Empty: "cannot parse float from empty string"
                // PFE Invalid: "invalid float literal"
                write!(f, "{}: `{}`", e, s)
            }
            ErrorCode::QuotedString => f.write_str("a quoted string may not be converted"),
            // Writers
            ErrorCode::SequenceTooLong => f.write_str("sequence is too long"),
            ErrorCode::SequenceMustHaveLength => f.write_str("sequence must have a known length"),
            // Strings
            ErrorCode::StringTooLong => f.write_str("string is too long"),
            ErrorCode::StringContainsNull => f.write_str("string contains a null"),
            ErrorCode::StringContainsQuote => f.write_str("string contains a quote"),
            ErrorCode::StringContainsInvalidChar => {
                f.write_str("string contains a non-ASCII character")
            }
        }
    }
}

/// A location in text data.
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub(crate) line: usize,
    pub(crate) col: usize,
}

impl Location {
    /// Construct a new location.

    pub const fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    /// The line in the text data.
    ///
    /// The first line in the text is `1`.

    pub fn line(&self) -> usize {
        self.line
    }

    /// The column in the text data.
    ///
    /// The first character in a line is `1`, although the column may be `0`
    /// immediately after a line break.

    pub fn column(&self) -> usize {
        self.col
    }
}

#[derive(Debug)]
struct ErrorContext {
    code: ErrorCode,
    location: Option<Location>,
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.location {
            Some(loc) => write!(
                f,
                "{} (at line: {}, column: {})",
                self.code, loc.line, loc.col
            ),
            None => fmt::Display::fmt(&self.code, f),
        }
    }
}

/// This type represents all possible errors that can occur when serializing or
/// deserializing text zlisp data.
#[derive(Debug)]
pub struct Error(Box<ErrorContext>);

/// A specialized [Result](std::result::Result) type for serialization or
/// deserialization operations.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Construct a new error.
    #[cold]
    pub fn new(code: ErrorCode, location: Option<Location>) -> Self {
        Self(Box::new(ErrorContext { code, location }))
    }

    /// The error code.

    pub const fn code(&self) -> &ErrorCode {
        &self.0.code
    }

    /// The error location.
    ///
    /// For deserialization, this is the line and column in the data. For
    /// serialization, likely `None`.

    pub const fn location(&self) -> Option<&Location> {
        self.0.location.as_ref()
    }

    pub(crate) fn attach_location(mut self, loc: Location) -> Self {
        if self.0.location.is_none() {
            self.0.location = Some(loc)
        }
        self
    }

    fn custom_ser<T: fmt::Display>(msg: T) -> Self {
        Self::new(ErrorCode::Custom(msg.to_string()), None)
    }

    fn custom_de<T: fmt::Display>(msg: T) -> Self {
        Self::new(ErrorCode::Custom(msg.to_string()), None)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::custom_ser(msg)
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::custom_de(msg)
    }
}

impl de::StdError for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.0.code {
            ErrorCode::ParseIntError { e, s: _ } => Some(e),
            ErrorCode::ParseFloatError { e, s: _ } => Some(e),
            _ => None,
        }
    }
}
