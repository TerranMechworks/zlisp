use serde_test::{assert_tokens, Token};
use zlisp_value::Value;

macro_rules! assert_int_tokens {
    ($value:expr) => {
        let value: Value = ($value).into();
        assert_tokens(&value, &[Token::I32($value)]);
    };
}

#[test]
fn int_tests() {
    assert_int_tokens!(0);
    assert_int_tokens!(1);
    assert_int_tokens!(-1);
    assert_int_tokens!(i32::MAX);
    assert_int_tokens!(i32::MIN);
}

macro_rules! assert_float_tokens {
    ($value:expr) => {
        let value: Value = ($value).into();
        assert_tokens(&value, &[Token::F32($value)]);
    };
}

#[test]
fn float_tests() {
    assert_float_tokens!(0.0);
    assert_float_tokens!(1.0);
    assert_float_tokens!(-1.0);
    assert_float_tokens!(f32::MAX);
    assert_float_tokens!(f32::MIN);
}

#[test]
fn string_tests() {
    let value: Value = "foo".into();
    assert_tokens(&value, &[Token::Str("foo")]);
}

#[test]
fn list_tests() {
    let value: Value = vec![].into();
    assert_tokens(&value, &[Token::Seq { len: Some(0) }, Token::SeqEnd]);
    let value: Value = vec![1.into()].into();
    assert_tokens(
        &value,
        &[Token::Seq { len: Some(1) }, Token::I32(1), Token::SeqEnd],
    );
}
