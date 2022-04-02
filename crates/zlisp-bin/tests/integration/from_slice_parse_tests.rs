use super::any::Any;
use super::bin_builder::{BinBuilder, FLOAT, INT, INVALID_TYPE, LIST, MAX_LIST_LEN, STRING};
use assert_matches::assert_matches;
use zlisp_bin::{from_slice, ErrorCode, TokenType};

macro_rules! assert_err {
    ($type:ty, $input:expr, $offset:expr, $code:pat) => {
        let err = from_slice::<$type>($input).unwrap_err();
        assert_matches!(err.code(), $code);
        assert_eq!(err.offset(), Some($offset));
    };
}

macro_rules! assert_ok {
    ($type:ty, $input:expr, $value:expr) => {
        let v = from_slice::<$type>($input).unwrap();
        assert_eq!(v, $value);
    };
}

#[test]
fn must_consume_all_data() {
    let input = BinBuilder::new().int(1).slice(&[0u8]).build();
    assert_err!(i32, &input, 8, ErrorCode::TrailingData);
}

#[test]
fn parse_int() {
    let input = BinBuilder::new().int(0).build();
    assert_ok!(i32, &input, 0);
    let input = BinBuilder::new().int(1).build();
    assert_ok!(i32, &input, 1);
    let input = BinBuilder::new().int(-1).build();
    assert_ok!(i32, &input, -1);
    let input = BinBuilder::new().int(i32::MIN).build();
    assert_ok!(i32, &input, i32::MIN);
    let input = BinBuilder::new().int(i32::MAX).build();
    assert_ok!(i32, &input, i32::MAX);

    let input = BinBuilder::new().build();
    assert_err!(
        i32,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::Int,
            found: TokenType::Eof,
        }
    );

    let input = BinBuilder::new().slice(&[0u8]).build();
    assert_err!(
        i32,
        &input,
        0,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 1,
        }
    );
    let input = BinBuilder::new().slice(&[0u8, 0u8]).build();
    assert_err!(
        i32,
        &input,
        0,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 2,
        }
    );
    let input = BinBuilder::new().slice(&[0u8, 0u8, 0u8]).build();
    assert_err!(
        i32,
        &input,
        0,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 3,
        }
    );

    let input = BinBuilder::new().i32(INVALID_TYPE).build();
    assert_err!(i32, &input, 0, ErrorCode::InvalidTokenType);

    let input = BinBuilder::new().i32(FLOAT).build();
    assert_err!(
        i32,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::Int,
            found: TokenType::Float,
        }
    );
    let input = BinBuilder::new().i32(STRING).build();
    assert_err!(
        i32,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::Int,
            found: TokenType::String,
        }
    );
    let input = BinBuilder::new().i32(LIST).build();
    assert_err!(
        i32,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::Int,
            found: TokenType::List,
        }
    );
}

#[test]
fn parse_float() {
    let input = BinBuilder::new().float(0.0).build();
    assert_ok!(f32, &input, 0.0);
    let input = BinBuilder::new().float(1.0).build();
    assert_ok!(f32, &input, 1.0);
    let input = BinBuilder::new().float(-1.0).build();
    assert_ok!(f32, &input, -1.0);
    let input = BinBuilder::new().float(f32::MIN).build();
    assert_ok!(f32, &input, f32::MIN);
    let input = BinBuilder::new().float(f32::MAX).build();
    assert_ok!(f32, &input, f32::MAX);

    let input = BinBuilder::new().build();
    assert_err!(
        f32,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::Float,
            found: TokenType::Eof,
        }
    );

    let input = BinBuilder::new().slice(&[0u8]).build();
    assert_err!(
        f32,
        &input,
        0,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 1,
        }
    );
    let input = BinBuilder::new().slice(&[0u8, 0u8]).build();
    assert_err!(
        f32,
        &input,
        0,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 2,
        }
    );
    let input = BinBuilder::new().slice(&[0u8, 0u8, 0u8]).build();
    assert_err!(
        f32,
        &input,
        0,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 3,
        }
    );

    let input = BinBuilder::new().i32(INVALID_TYPE).build();
    assert_err!(f32, &input, 0, ErrorCode::InvalidTokenType);

    let input = BinBuilder::new().i32(INT).build();
    assert_err!(
        f32,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::Float,
            found: TokenType::Int,
        }
    );
    let input = BinBuilder::new().i32(STRING).build();
    assert_err!(
        f32,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::Float,
            found: TokenType::String,
        }
    );
    let input = BinBuilder::new().i32(LIST).build();
    assert_err!(
        f32,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::Float,
            found: TokenType::List,
        }
    );
}

#[test]
fn parse_str() {
    let input = BinBuilder::new().str("foo").build();
    assert_ok!(&str, &input, "foo");
    let input = BinBuilder::new().str("").build();
    assert_ok!(&str, &input, "");

    let input = BinBuilder::new().build();
    assert_err!(
        &str,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::String,
            found: TokenType::Eof,
        }
    );

    let input = BinBuilder::new().slice(&[0u8]).build();
    assert_err!(
        &str,
        &input,
        0,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 1,
        }
    );
    let input = BinBuilder::new().slice(&[0u8, 0u8]).build();
    assert_err!(
        &str,
        &input,
        0,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 2,
        }
    );
    let input = BinBuilder::new().slice(&[0u8, 0u8, 0u8]).build();
    assert_err!(
        &str,
        &input,
        0,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 3,
        }
    );

    let input = BinBuilder::new().i32(INVALID_TYPE).build();
    assert_err!(&str, &input, 0, ErrorCode::InvalidTokenType);

    let input = BinBuilder::new().i32(INT).build();
    assert_err!(
        &str,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::String,
            found: TokenType::Int,
        }
    );
    let input = BinBuilder::new().i32(FLOAT).build();
    assert_err!(
        &str,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::String,
            found: TokenType::Float,
        }
    );
    let input = BinBuilder::new().i32(LIST).build();
    assert_err!(
        &str,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::String,
            found: TokenType::List,
        }
    );

    let input = BinBuilder::new().i32(STRING).build();
    assert_err!(
        &str,
        &input,
        4,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 0,
        }
    );
    let input = BinBuilder::new().i32(STRING).slice(&[0u8]).build();
    assert_err!(
        &str,
        &input,
        4,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 1,
        }
    );
    let input = BinBuilder::new().i32(STRING).slice(&[0u8, 0u8]).build();
    assert_err!(
        &str,
        &input,
        4,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 2,
        }
    );
    let input = BinBuilder::new()
        .i32(STRING)
        .slice(&[0u8, 0u8, 0u8])
        .build();
    assert_err!(
        &str,
        &input,
        4,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 3,
        }
    );

    let input = BinBuilder::new().i32(STRING).i32(0).build();
    assert_ok!(&str, &input, "");
    let input = BinBuilder::new().i32(STRING).i32(-1).build();
    assert_err!(&str, &input, 4, ErrorCode::InvalidStringLength);
    let input = BinBuilder::new().i32(STRING).i32(i32::MIN).build();
    assert_err!(&str, &input, 4, ErrorCode::InvalidStringLength);

    let input = BinBuilder::new().i32(STRING).i32(1).build();
    assert_err!(
        &str,
        &input,
        8,
        ErrorCode::InsufficientData {
            expected: 1,
            available: 0,
        }
    );

    let input = BinBuilder::new().i32(STRING).i32(2).slice(&[0u8]).build();
    assert_err!(
        &str,
        &input,
        8,
        ErrorCode::InsufficientData {
            expected: 2,
            available: 1,
        }
    );

    let input = BinBuilder::new().i32(STRING).i32(255).build();
    let len = 255;
    let err = from_slice::<&str>(&input).unwrap_err();
    assert_matches!(err.code(), ErrorCode::InsufficientData {
        expected,
        available: 0,
    } if expected == &len);
    assert_eq!(err.offset(), Some(8));
}

#[test]
fn parse_str_content() {
    let input = BinBuilder::new().str("\0").build();
    assert_err!(&str, &input, 8, ErrorCode::StringContainsNull);
    let input = BinBuilder::new().str("\"").build();
    assert_err!(&str, &input, 8, ErrorCode::StringContainsQuote);
    let input = BinBuilder::new().str("🎅").build();
    assert_err!(&str, &input, 8, ErrorCode::StringContainsInvalidByte);

    let max_len = " ".repeat(255);
    let input = BinBuilder::new().str(&max_len).build();
    assert_ok!(&str, &input, &max_len);

    let over_len = " ".repeat(256);
    let input = BinBuilder::new().str(&over_len).build();
    assert_err!(&str, &input, 4, ErrorCode::StringTooLong);
}

#[test]
fn parse_list() {
    let input = BinBuilder::new().list(0).build();
    assert_ok!(Vec<i32>, &input, &[]);
    let input = BinBuilder::new().list(1).int(2).build();
    assert_ok!(Vec<i32>, &input, &[2]);

    let input = BinBuilder::new().build();
    assert_err!(
        Vec<i32>,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::List,
            found: TokenType::Eof,
        }
    );

    let input = BinBuilder::new().slice(&[0u8]).build();
    assert_err!(
        Vec<i32>,
        &input,
        0,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 1,
        }
    );
    let input = BinBuilder::new().slice(&[0u8, 0u8]).build();
    assert_err!(
        Vec<i32>,
        &input,
        0,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 2,
        }
    );
    let input = BinBuilder::new().slice(&[0u8, 0u8, 0u8]).build();
    assert_err!(
        Vec<i32>,
        &input,
        0,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 3,
        }
    );

    let input = BinBuilder::new().i32(INVALID_TYPE).build();
    assert_err!(Vec<i32>, &input, 0, ErrorCode::InvalidTokenType);

    let input = BinBuilder::new().i32(INT).build();
    assert_err!(
        Vec<i32>,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::List,
            found: TokenType::Int,
        }
    );
    let input = BinBuilder::new().i32(FLOAT).build();
    assert_err!(
        Vec<i32>,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::List,
            found: TokenType::Float,
        }
    );
    let input = BinBuilder::new().i32(STRING).build();
    assert_err!(
        Vec<i32>,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::List,
            found: TokenType::String,
        }
    );

    let input = BinBuilder::new().i32(LIST).build();
    assert_err!(
        Vec<i32>,
        &input,
        4,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 0,
        }
    );
    let input = BinBuilder::new().i32(LIST).slice(&[0u8]).build();
    assert_err!(
        Vec<i32>,
        &input,
        4,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 1,
        }
    );
    let input = BinBuilder::new().i32(LIST).slice(&[0u8, 0u8]).build();
    assert_err!(
        Vec<i32>,
        &input,
        4,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 2,
        }
    );
    let input = BinBuilder::new().i32(LIST).slice(&[0u8, 0u8, 0u8]).build();
    assert_err!(
        Vec<i32>,
        &input,
        4,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 3,
        }
    );

    let input = BinBuilder::new().i32(LIST).i32(1).build();
    assert_ok!(Vec<i32>, &input, &[]);
    let mut builder = BinBuilder::new().i32(LIST).i32(MAX_LIST_LEN + 1);
    let mut expected = Vec::with_capacity(MAX_LIST_LEN as usize);
    for i in 0..MAX_LIST_LEN {
        builder = builder.int(i);
        expected.push(i);
    }
    let input = builder.build();
    assert_ok!(Vec<i32>, &input, &expected[..]);

    // under length
    let input = BinBuilder::new().i32(LIST).i32(0).build();
    assert_err!(Vec<i32>, &input, 4, ErrorCode::InvalidListLength);
    let input = BinBuilder::new().i32(LIST).i32(-1).build();
    assert_err!(Vec<i32>, &input, 4, ErrorCode::InvalidListLength);
    let input = BinBuilder::new().i32(LIST).i32(i32::MIN).build();
    assert_err!(Vec<i32>, &input, 4, ErrorCode::InvalidListLength);

    // over length
    let input = BinBuilder::new().i32(LIST).i32(MAX_LIST_LEN + 2).build();
    assert_err!(Vec<i32>, &input, 4, ErrorCode::SequenceTooLong);
    let input = BinBuilder::new().i32(LIST).i32(i32::MAX).build();
    assert_err!(Vec<i32>, &input, 4, ErrorCode::SequenceTooLong);
}

#[test]
fn parse_any() {
    let input = BinBuilder::new().int(0).build();
    assert_ok!(Any, &input, Any::Int(0));
    let input = BinBuilder::new().float(0.0).build();
    assert_ok!(Any, &input, Any::Float(0.0));
    let input = BinBuilder::new().str("foo").build();
    assert_ok!(Any, &input, Any::str("foo"));
    let input = BinBuilder::new().list(0).build();
    assert_ok!(Any, &input, Any::List(vec![]));

    let input = BinBuilder::new().build();
    assert_err!(
        Any,
        &input,
        0,
        ErrorCode::ExpectedToken {
            expected: TokenType::Any,
            found: TokenType::Eof,
        }
    );

    let input = BinBuilder::new().slice(&[0u8]).build();
    assert_err!(
        Any,
        &input,
        0,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 1,
        }
    );
    let input = BinBuilder::new().slice(&[0u8, 0u8]).build();
    assert_err!(
        Any,
        &input,
        0,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 2,
        }
    );
    let input = BinBuilder::new().slice(&[0u8, 0u8, 0u8]).build();
    assert_err!(
        Any,
        &input,
        0,
        ErrorCode::InsufficientData {
            expected: 4,
            available: 3,
        }
    );

    let input = BinBuilder::new().i32(INVALID_TYPE).build();
    assert_err!(Any, &input, 0, ErrorCode::InvalidTokenType);
}
