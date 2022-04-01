//! Serialization and deserialization of Zipper-style, lisp-like data
//! structures (zlisp) to and from a Zipper-compatible text data format.
#![warn(
    missing_docs,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unused
)]
mod ascii;
mod error;
mod reader;
mod writer;

pub use error::{Error, ErrorCode, Location, Result, TokenType};
pub use reader::from_str;
pub use writer::{to_pretty, to_string, WhitespaceConfig, WhitespaceConfigBuilder};
