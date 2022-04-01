//! Hexadecimal integer support for zlisp serialization and deserialization
//!
//! The [`Hex`] newtype supports positive 32-bit signed integer serialization.
//! For binary formats, the value is serialized/deserialized as an `i32`, which
//! may not be negative. For text formats, the value is serialized/deserialized
//! as a string in hexadecimal format.
#![warn(
    missing_docs,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unused
)]
use serde::{de, ser};
use std::fmt;

/// Represents a hexadecimal zlisp value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Hex(i32);

impl Hex {
    /// Get the underlying value.
    pub const fn get(&self) -> i32 {
        self.0
    }
}

impl From<Hex> for i32 {
    fn from(value: Hex) -> Self {
        value.0
    }
}

impl From<&Hex> for i32 {
    fn from(value: &Hex) -> Self {
        value.0
    }
}

impl From<Hex> for String {
    fn from(value: Hex) -> Self {
        format!("{:#x}", value.0)
    }
}

impl From<&Hex> for String {
    fn from(value: &Hex) -> Self {
        format!("{:#x}", value.0)
    }
}

impl TryFrom<i32> for Hex {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

/// This type represents all possible errors that can occur when converting a
/// string to [`Hex`].
#[derive(Debug, Clone, PartialEq)]
pub enum HexConversionError {
    /// The value is missing the prefix `0x`.
    MissingPrefix,
    /// The value could not be parsed.
    Invalid,
    /// The value could be parsed, but is negative.
    NegativeValue,
}

impl TryFrom<&str> for Hex {
    type Error = HexConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pat = "0x";
        if !value.starts_with(pat) {
            return Err(HexConversionError::MissingPrefix);
        };
        let src = &value[pat.len()..];
        let v = i32::from_str_radix(src, 16).map_err(|_e| HexConversionError::Invalid)?;
        v.try_into().map_err(|()| HexConversionError::NegativeValue)
    }
}

struct BinHexVisitor;

impl<'de> de::Visitor<'de> for BinHexVisitor {
    type Value = Hex;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a non-negative, 32-bit signed integer")
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value
            .try_into()
            .map_err(|()| E::custom(format!("negative value: {}", value)))
    }
}

struct TextHexVisitor;

impl<'de> de::Visitor<'de> for TextHexVisitor {
    type Value = Hex;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a non-negative, hexadecimal string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.try_into().map_err(|e| match e {
            HexConversionError::MissingPrefix => E::custom(format!("missing prefix: {}", value)),
            HexConversionError::Invalid => E::custom(format!("invalid: {}", value)),
            HexConversionError::NegativeValue => E::custom(format!("negative value: {}", value)),
        })
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&value)
    }
}

impl<'de> de::Deserialize<'de> for Hex {
    fn deserialize<D>(deserializer: D) -> Result<Hex, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(TextHexVisitor)
        } else {
            deserializer.deserialize_i32(BinHexVisitor)
        }
    }
}

impl ser::Serialize for Hex {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if serializer.is_human_readable() {
            let repr: String = self.into();
            serializer.serialize_str(&repr)
        } else {
            serializer.serialize_i32(self.0)
        }
    }
}
