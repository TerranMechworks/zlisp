use super::structs::Bytes;
use assert_matches::assert_matches;
use zlisp_text::{to_string, ErrorCode, WhitespaceConfig};

macro_rules! assert_unsupported {
    ($type:ty, $value:expr) => {
        let v: $type = $value;
        let err = to_string(&v, WhitespaceConfig::default()).unwrap_err();
        assert_matches!(err.code(), ErrorCode::UnsupportedType);
        assert_matches!(err.location(), None);
    };
}

macro_rules! assert_err {
    ($type:ty, $value:expr, $code:pat) => {{
        let v: $type = $value;
        let err = to_string(&v, WhitespaceConfig::default()).unwrap_err();
        assert_matches!(err.code(), $code);
        assert_matches!(err.location(), None);
    }};
}

#[test]
fn bool_tests() {
    assert_unsupported!(bool, true);
    assert_unsupported!(bool, false);
}

#[test]
fn signed_tests() {
    assert_unsupported!(i8, 0);
    assert_unsupported!(i16, 0);
    assert_unsupported!(i64, 0);
}

#[test]
fn unsigned_tests() {
    assert_unsupported!(u8, 0);
    assert_unsupported!(u16, 0);
    assert_unsupported!(u32, 0);
    assert_unsupported!(u64, 0);
}

#[test]
fn float_tests() {
    assert_unsupported!(f64, 0.0);
}

#[test]
fn char_tests() {
    assert_unsupported!(char, ' ');
}

#[test]
fn string_tests() {
    assert_err!(&str, "\0", ErrorCode::StringContainsNull);
    assert_err!(&str, "\"", ErrorCode::StringContainsQuote);
    assert_err!(&str, "🎅", ErrorCode::StringContainsInvalidChar);

    let max_len = " ".repeat(255);
    let _ = to_string(&max_len, WhitespaceConfig::default()).unwrap();

    let over_len = " ".repeat(256);
    assert_err!(&str, &over_len, ErrorCode::StringTooLong);
}

#[test]
fn bytes_tests() {
    // normal byte arrays don't work: https://github.com/serde-rs/serde/issues/518
    // this will instead be serialized as a sequence.
    // assert_unsupported!(&[u8], b"");
    assert_unsupported!(Bytes, Bytes(b""));
}
