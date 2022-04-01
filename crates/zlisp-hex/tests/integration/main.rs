use serde_test::{assert_tokens, Configure as _, Token};
use zlisp_hex::{Hex, HexConversionError};

macro_rules! conv_i32_ok {
    ($input:expr) => {
        let input: i32 = $input;
        let hex: Hex = input.try_into().unwrap();
        let output: i32 = hex.into();
        assert_eq!(output, input);
    };
}
macro_rules! conv_i32_err {
    ($input:expr) => {
        let input: i32 = $input;
        let res: Result<Hex, ()> = input.try_into();
        res.unwrap_err();
    };
}

macro_rules! conv_str_ok {
    ($input:expr) => {
        let input: &str = $input;
        let hex: Hex = input.try_into().unwrap();
        let output: String = hex.into();
        assert_eq!(&output, input);
    };
}
macro_rules! conv_str_err {
    ($input:expr, $expected:expr) => {
        let expected: HexConversionError = $expected;
        let input: &str = $input;
        let res: Result<Hex, HexConversionError> = input.try_into();
        let err = res.unwrap_err();
        assert_eq!(err, expected);
    };
}

#[test]
fn i32_conv() {
    conv_i32_ok!(0);
    conv_i32_ok!(1);
    conv_i32_ok!(i32::MAX);

    conv_i32_err!(-1);
    conv_i32_err!(i32::MIN);
}

#[test]
fn str_conv() {
    conv_str_ok!("0x0");
    conv_str_ok!("0x0");
    conv_str_ok!(&format!("{:#x}", i32::MAX));

    conv_str_err!("", HexConversionError::MissingPrefix);
    conv_str_err!("0", HexConversionError::MissingPrefix);
    conv_str_err!("1x", HexConversionError::MissingPrefix);
    conv_str_err!("0xz", HexConversionError::Invalid);
    conv_str_err!(
        &format!("{:#x}", (i32::MAX as i64) + 1),
        HexConversionError::Invalid
    );
    // this is a bit weird...
    conv_str_err!("0x-1", HexConversionError::NegativeValue);
}

#[test]
fn serde_conv() {
    let value: Hex = 1.try_into().unwrap();
    assert_tokens(&value.compact(), &[Token::I32(1)]);
    assert_tokens(&value.readable(), &[Token::Str("0x1")]);
    assert_tokens(&value.readable(), &[Token::String("0x1")]);
}
