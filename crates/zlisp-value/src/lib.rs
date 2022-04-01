//! Arbitrary value serialization and deserialization of Zipper-style,
//! lisp-like data structures (zlisp).
//!
//! This crate only provides the [`Value`] type. The crates `zlisp-bin` and
//! `zlisp-text` provide serde capabilities for binary and text data formats,
//! respectively. In combination with either of these crates, [`Value`] can be
//! used to deserialize or serialize any valid zlisp data.
//!
//! Since values have general serde support, other data formats can also be
//! used. This is more useful for serialization, since the supported data types
//! are fairly limited. For example, this can be used to serialize zlisp to
//! JSON, using the [`serde_json`](https://crates.io/crates/serde_json) crate.
//!
//! Apart from serde support, [`Value`] has several [`From`] implementations
//! for easy constructing, as well as [`Debug`](std::fmt::Debug) and
//! [`Display`](std::fmt::Display) implementations.
#![warn(
    missing_docs,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unused
)]
mod value;

pub use value::Value;
