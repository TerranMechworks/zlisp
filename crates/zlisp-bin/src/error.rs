use serde::{de, ser};
use std::fmt;

/// A high-level description of a token.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    /// An integer.
    Int,
    /// A float.
    Float,
    /// A string.
    String,
    /// A list.
    List,
    /// The end of the file/data.
    Eof,
    /// Any token.
    Any,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Int => f.write_str("integer"),
            TokenType::Float => f.write_str("float"),
            TokenType::String => f.write_str("string"),
            TokenType::List => f.write_str("list"),
            TokenType::Eof => f.write_str("end of file"),
            TokenType::Any => f.write_str("anything"),
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
    /// An error occurred during an I/O operation.
    IO(std::io::Error),
    /// The data type is not supported by the serializer or deserializer.
    UnsupportedType,

    // --- Deserializers ---
    /// The deserialization finished, but some data remained.
    TrailingData,
    /// A token was expected, but an incompatible token was found.
    ExpectedToken {
        /// The expected token type.
        expected: TokenType,
        /// The actual token type.
        found: TokenType,
    },
    /// A list of a certain length was expected.
    ExpectedListOfLength {
        /// The minimum expected list length.
        expected_min: usize,
        /// The maximum expected list length.
        expected_max: usize,
        /// The actual list length.
        found: usize,
    },
    /// A key-value pair was expected, but only a key was found.
    ExpectedKeyValuePair,

    // --- Readers ---
    /// Based on previous data, a certain number of bytes was expected, but
    /// fewer bytes were available.
    InsufficientData {
        /// The expected number of bytes.
        expected: usize,
        /// The available number of bytes.
        available: usize,
    },
    /// The data contained an invalid token type.
    InvalidTokenType,
    /// The data contained an invalid list length.
    InvalidListLength,
    /// The data contained an invalid string length.
    InvalidStringLength,

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
    /// A string contains a null byte/character.
    StringContainsNull,
    /// A string contains a quote byte/character.
    ///
    /// In the text format, there is no way to escape quotes.
    StringContainsQuote,
    /// A string contains an invalid byte/character.
    StringContainsInvalidByte,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // General
            ErrorCode::Custom(s) => write!(f, "{}", s),
            ErrorCode::IO(e) => fmt::Display::fmt(e, f),
            ErrorCode::UnsupportedType => f.write_str("unsupported type"),
            // Deserializers
            ErrorCode::TrailingData => f.write_str("trailing data"),
            ErrorCode::ExpectedToken { expected, found } => {
                write!(f, "expected {}, found {}", expected, found)
            }
            ErrorCode::ExpectedListOfLength {
                expected_min,
                expected_max,
                found,
            } => {
                write!(
                    f,
                    "expected list length {}-{}, found {}",
                    expected_min, expected_max, found
                )
            }
            ErrorCode::ExpectedKeyValuePair => f.write_str("expected key-value pair"),
            // Readers
            ErrorCode::InsufficientData {
                expected,
                available,
            } => {
                write!(
                    f,
                    "expected: {} bytes, available: {} bytes",
                    expected, available
                )
            }
            ErrorCode::InvalidTokenType => f.write_str("invalid token type"),
            ErrorCode::InvalidListLength => f.write_str("invalid list length"),
            ErrorCode::InvalidStringLength => f.write_str("invalid string length"),
            // Writers
            ErrorCode::SequenceTooLong => f.write_str("sequence is too long"),
            ErrorCode::SequenceMustHaveLength => f.write_str("sequence must have a known length"),
            // Strings
            ErrorCode::StringTooLong => f.write_str("string is too long"),
            ErrorCode::StringContainsNull => f.write_str("string contains a null"),
            ErrorCode::StringContainsQuote => f.write_str("string contains a quote"),
            ErrorCode::StringContainsInvalidByte => f.write_str("string contains a non-ASCII byte"),
        }
    }
}

#[derive(Debug)]
struct ErrorContext {
    code: ErrorCode,
    offset: Option<usize>,
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.offset {
            Some(offset) => write!(f, "{} (at offset: {})", self.code, offset),
            None => fmt::Display::fmt(&self.code, f),
        }
    }
}

/// This type represents all possible errors that can occur when serializing or
/// deserializing binary zlisp data.
#[derive(Debug)]
pub struct Error(Box<ErrorContext>);

/// A specialized [Result](std::result::Result) type for serialization or
/// deserialization operations.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Construct a new error.
    #[cold]
    pub fn new(code: ErrorCode, offset: Option<usize>) -> Self {
        Self(Box::new(ErrorContext { code, offset }))
    }

    /// The error code.

    pub const fn code(&self) -> &ErrorCode {
        &self.0.code
    }

    /// The error location.
    ///
    /// For deserialization, this is the offset in the data. For serialization,
    /// likely `None`.

    pub const fn offset(&self) -> Option<usize> {
        self.0.offset
    }

    pub(crate) fn attach_offset(mut self, offset: usize) -> Self {
        if self.0.offset.is_none() {
            self.0.offset = Some(offset)
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
            ErrorCode::IO(e) => Some(e),
            _ => None,
        }
    }
}
