use assert_matches::assert_matches;
use zlisp_bin::{to_vec, ErrorCode};

macro_rules! assert_unsupported {
    ($type:ty, $value:expr) => {
        let v: $type = $value;
        let err = to_vec(&v).unwrap_err();
        assert_matches!(err.code(), ErrorCode::UnsupportedType);
        assert_matches!(err.offset(), None);
    };
}

macro_rules! assert_err {
    ($type:ty, $value:expr, $code:pat) => {{
        let v: $type = $value;
        let err = to_vec(&v).unwrap_err();
        assert_matches!(err.code(), $code);
        assert_matches!(err.offset(), None);
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
    assert_err!(&str, "🎅", ErrorCode::StringContainsInvalidByte);

    let max_len = " ".repeat(255);
    let _ = to_vec(&max_len).unwrap();

    let over_len = " ".repeat(256);
    assert_err!(&str, &over_len, ErrorCode::StringTooLong);
}

#[test]
fn bytes_tests() {
    // normal byte arrays don't work: https://github.com/serde-rs/serde/issues/518
    // this will instead be serialized as a sequence.
    // assert_unsupported!(&[u8], b"");

    pub struct Bytes<'a>(&'a [u8]);

    impl<'a> serde::ser::Serialize for Bytes<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            serializer.serialize_bytes(self.0)
        }
    }

    assert_unsupported!(Bytes, Bytes(b""));
}
