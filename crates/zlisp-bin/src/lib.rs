//! Serialization and deserialization of Zipper-style, lisp-like data
//! structures (zlisp) to and from a Zipper-compatible binary data format.
#![warn(
    missing_docs,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unused
)]
mod ascii;
mod constants;
mod error;
mod reader;
mod writer;

pub use error::{Error, ErrorCode, Result, TokenType};
pub use reader::from_slice;
pub use writer::{to_vec, to_writer};
