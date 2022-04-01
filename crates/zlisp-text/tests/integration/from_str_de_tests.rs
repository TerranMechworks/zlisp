use super::map;
use super::structs::*;
use assert_matches::assert_matches;
use std::collections::HashMap;
use zlisp_text::{from_str, ErrorCode, Location, TokenType};

macro_rules! assert_ok {
    ($type:ty, $input:expr, $value:expr) => {
        let v = from_str::<$type>($input).unwrap();
        assert_eq!(v, $value);
    };
}

macro_rules! assert_err {
    ($type:ty, $input:expr, $line:expr, $col:expr, $code:pat) => {
        let err = from_str::<$type>($input).unwrap_err();
        assert_matches!(err.code(), $code);
        let loc = Location::new($line, $col);
        assert_eq!(err.location().unwrap(), &loc);
    };
}

macro_rules! assert_unsupported {
    ($type:ty) => {
        let err = from_str::<$type>("").unwrap_err();
        assert_matches!(err.code(), ErrorCode::UnsupportedType);
    };
}

macro_rules! unwrap_err {
    ($type:ty, $input:expr, $line:expr, $col:expr) => {{
        let err = from_str::<$type>($input).unwrap_err();
        let loc = Location::new($line, $col);
        assert_eq!(err.location().unwrap(), &loc);
        err
    }};
}

#[test]
fn bool_tests() {
    assert_unsupported!(bool);
}

#[test]
fn signed_tests() {
    assert_unsupported!(i8);
    assert_unsupported!(i16);
    assert_unsupported!(i64);

    assert_ok!(i32, "0", 0);
}

#[test]
fn unsigned_tests() {
    assert_unsupported!(u8);
    assert_unsupported!(u16);
    assert_unsupported!(u32);
    assert_unsupported!(u64);
}

#[test]
fn float_tests() {
    assert_unsupported!(f64);

    assert_ok!(f32, "0.0", 0.0);
}

#[test]
fn char_tests() {
    assert_unsupported!(char);
}

#[test]
fn string_tests() {
    assert_unsupported!(&str);
    assert_ok!(String, "foo", "foo");
}

#[test]
fn bytes_tests() {
    assert_unsupported!(&[u8]);
}

#[test]
fn option_tests() {
    assert_ok!(Option<()>, "()", None);
    assert_ok!(Option<()>, "(())", Some(()));

    type Value = Option<i32>;

    assert_ok!(Value, "()", None);
    assert_ok!(Value, "(-1)", Some(-1));

    assert_err!(
        Value,
        "(-1 -2)",
        1,
        "(-1 ".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::ListEnd,
            found: TokenType::Text,
        }
    );
}

#[test]
fn unit_type_tests() {
    type Value = ();

    assert_ok!(Value, "()", ());

    assert_err!(
        Value,
        "(-1)",
        1,
        "(".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::ListEnd,
            found: TokenType::Text,
        }
    );
}

#[test]
fn unit_struct_tests() {
    type Value = UnitStruct;

    assert_ok!(Value, "()", UnitStruct);

    assert_err!(
        Value,
        "(-1)",
        1,
        "(".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::ListEnd,
            found: TokenType::Text,
        }
    );
}

#[test]
fn newtype_struct_tests() {
    type Value = NewTypeStruct;

    // a newtype struct is always deserialized as the inner type
    assert_ok!(Value, "-1", NewTypeStruct(-1));
}

#[test]
fn seq_tests() {
    type Value = Vec<i32>;

    assert_ok!(Value, "()", vec![]);
    assert_ok!(Value, "(-1)", vec![-1]);
    assert_ok!(Value, "(-1 -2)", vec![-1, -2]);
}

#[test]
fn tuple_tests() {
    assert_ok!(((),), "(())", ((),));

    type Value = (i32,);

    assert_ok!(Value, "(-1)", (-1,));

    assert_err!(
        Value,
        "()",
        1,
        "(".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::Text,
            found: TokenType::ListEnd,
        }
    );
    assert_err!(
        Value,
        "(-1 -2)",
        1,
        "(-1 ".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::ListEnd,
            found: TokenType::Text,
        }
    );
}

#[test]
fn tuple_struct_tests() {
    type Value = TupleStruct;

    assert_ok!(Value, "(-1 -2)", TupleStruct(-1, -2));

    assert_err!(
        Value,
        "()",
        1,
        "(".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::Text,
            found: TokenType::ListEnd,
        }
    );
    assert_err!(
        Value,
        "(-1)",
        1,
        "(-1".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::Text,
            found: TokenType::ListEnd,
        }
    );
    assert_err!(
        Value,
        "(-1 -2 -3)",
        1,
        "(-1 -2 ".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::ListEnd,
            found: TokenType::Text,
        }
    );
}

#[test]
fn map_tests() {
    type Value = HashMap<i32, i32>;

    assert_ok!(Value, "()", map![]);
    assert_ok!(Value, "(-1 -2)", map![-1 => -2]);

    assert_err!(
        Value,
        "(-1)",
        1,
        "(-1".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::Text,
            found: TokenType::ListEnd,
        }
    );
    assert_err!(
        Value,
        "(-1 -2 -3)",
        1,
        "(-1 -2 -3".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::Text,
            found: TokenType::ListEnd,
        }
    );
}

#[test]
fn struct_tests() {
    type Value = Struct;

    assert_ok!(Value, "(a -1 b -2)", Struct { a: -1, b: -2 });
    assert_ok!(Value, "(b -2 a -1)", Struct { a: -1, b: -2 });

    assert_err!(
        Value,
        "(a)",
        1,
        "(a".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::Text,
            found: TokenType::ListEnd,
        }
    );
    assert_err!(
        Value,
        "(a -1 b)",
        1,
        "(a -1 b".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::Text,
            found: TokenType::ListEnd,
        }
    );
}

#[test]
fn struct_optional_tests() {
    type Value = OptStruct;

    assert_ok!(Value, "(a -1 b -2)", OptStruct { a: -1, b: -2 });
    assert_ok!(Value, "(b -2 a -1)", OptStruct { a: -1, b: -2 });
    assert_ok!(Value, "(a -1)", OptStruct { a: -1, b: 0 });
    assert_ok!(Value, "(b -2)", OptStruct { a: 0, b: -2 });

    assert_err!(
        Value,
        "(a)",
        1,
        "(a".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::Text,
            found: TokenType::ListEnd,
        }
    );
    assert_err!(
        Value,
        "(a -1 b)",
        1,
        "(a -1 b".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::Text,
            found: TokenType::ListEnd,
        }
    );
}

#[test]
fn enum_unit_variant_tests() {
    type Value = UnitVariant;

    assert_ok!(Value, "V", UnitVariant::V);

    let err = unwrap_err!(Value, "!", 1, 0);
    assert_matches!(err.code(), ErrorCode::Custom(s) if s.contains("unknown variant"))
}

#[test]
fn enum_newtype_variant_tests() {
    type Value = NewTypeVariant;

    assert_ok!(Value, "V(-1)", NewTypeVariant::V(-1));

    let err = unwrap_err!(Value, "!", 1, 0);
    assert_matches!(err.code(), ErrorCode::Custom(s) if s.contains("unknown variant"));

    assert_err!(
        Value,
        "V()",
        1,
        "V(".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::Text,
            found: TokenType::ListEnd,
        }
    );
    assert_err!(
        Value,
        "V(-1 -2)",
        1,
        "V(-1 ".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::ListEnd,
            found: TokenType::Text,
        }
    );
}

#[test]
fn enum_tuple_variant_tests() {
    type Value = TupleVariant;

    assert_ok!(Value, "V(-1 -2)", TupleVariant::V(-1, -2));

    let err = unwrap_err!(Value, "!", 1, 0);
    assert_matches!(err.code(), ErrorCode::Custom(s) if s.contains("unknown variant"));

    assert_err!(
        Value,
        "V()",
        1,
        "V(".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::Text,
            found: TokenType::ListEnd,
        }
    );
    assert_err!(
        Value,
        "V(-1)",
        1,
        "V(-1".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::Text,
            found: TokenType::ListEnd,
        }
    );
    assert_err!(
        Value,
        "V(-1 -2 -3)",
        1,
        "V(-1 -2 ".len(),
        ErrorCode::ExpectedToken {
            expected: TokenType::ListEnd,
            found: TokenType::Text,
        }
    );
}

#[test]
fn enum_struct_variant_tests() {
    type Value = StructVariant;

    assert_ok!(Value, "V(a -1 b -2)", StructVariant::V { a: -1, b: -2 });
    assert_ok!(Value, "V(b -2 a -1)", StructVariant::V { a: -1, b: -2 });

    let err = unwrap_err!(Value, "!", 1, 0);
    assert_matches!(err.code(), ErrorCode::Custom(s) if s.contains("unknown variant"));
}

#[test]
fn enum_struct_variant_optional_tests() {
    type Value = OptStructVariant;

    assert_ok!(Value, "V(a -1 b -2)", OptStructVariant::V { a: -1, b: -2 });
    assert_ok!(Value, "V(b -2 a -1)", OptStructVariant::V { a: -1, b: -2 });

    assert_ok!(Value, "V(a -1)", OptStructVariant::V { a: -1, b: 0 });
    assert_ok!(Value, "V(b -2)", OptStructVariant::V { a: 0, b: -2 });
}
