use super::bin_builder::BinBuilder;
use super::map;
use assert_matches::assert_matches;
use serde_derive::Deserialize;
use std::collections::HashMap;
use zlisp_bin::{from_slice, ErrorCode};

macro_rules! assert_ok {
    ($type:ty, $input:expr, $value:expr) => {
        let v = from_slice::<$type>($input).unwrap();
        assert_eq!(v, $value);
    };
}

macro_rules! assert_err {
    ($type:ty, $input:expr, $offset:expr, $code:pat) => {
        let err = from_slice::<$type>($input).unwrap_err();
        assert_matches!(err.code(), $code);
        assert_eq!(err.offset(), Some($offset));
    };
}

macro_rules! assert_unsupported {
    ($type:ty) => {
        let input = BinBuilder::new().build();
        let err = from_slice::<$type>(&input).unwrap_err();
        assert_matches!(err.code(), ErrorCode::UnsupportedType);
    };
}

macro_rules! unwrap_err {
    ($type:ty, $input:expr, $offset:expr) => {{
        let err = from_slice::<$type>($input).unwrap_err();
        assert_eq!(err.offset(), Some($offset));
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

    let input = BinBuilder::new().int(0).build();
    assert_ok!(i32, &input, 0);
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

    let input = BinBuilder::new().float(0.0).build();
    assert_ok!(f32, &input, 0.0);
}

#[test]
fn char_tests() {
    assert_unsupported!(char);
}

#[test]
fn string_tests() {
    let input = BinBuilder::new().str("foo").build();
    assert_ok!(&str, &input, "foo");
    assert_ok!(String, &input, "foo");
}

#[test]
fn bytes_tests() {
    assert_unsupported!(&[u8]);
}

#[test]
fn option_tests() {
    let input = BinBuilder::new().list(0).build();
    assert_ok!(Option<i32>, &input, None);
    let input = BinBuilder::new().list(1).int(-1).build();
    assert_ok!(Option<i32>, &input, Some(-1));

    type Value = Option<()>;

    let input = BinBuilder::new().list(0).build();
    assert_ok!(Value, &input, None);
    let input = BinBuilder::new().list(1).list(0).build();
    assert_ok!(Value, &input, Some(()));

    let input = BinBuilder::new().list(2).build();
    assert_err!(
        Value,
        &input,
        4,
        ErrorCode::ExpectedListOfLength {
            expected_min: 0,
            expected_max: 1,
            found: 2,
        }
    );
}

#[test]
fn unit_type_tests() {
    type Value = ();

    let input = BinBuilder::new().list(0).build();
    assert_ok!(Value, &input, ());

    let input = BinBuilder::new().list(1).build();
    assert_err!(
        Value,
        &input,
        4,
        ErrorCode::ExpectedListOfLength {
            expected_min: 0,
            expected_max: 0,
            found: 1,
        }
    );
}

#[test]
fn unit_struct_tests() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct UnitStruct;
    type Value = UnitStruct;

    let input = BinBuilder::new().list(0).build();
    assert_ok!(Value, &input, UnitStruct);

    let input = BinBuilder::new().list(1).build();
    assert_err!(
        Value,
        &input,
        4,
        ErrorCode::ExpectedListOfLength {
            expected_min: 0,
            expected_max: 0,
            found: 1,
        }
    );
}

#[test]
fn newtype_struct_tests() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct NewTypeStruct(i32);
    type Value = NewTypeStruct;

    // a newtype struct is always deserialized as the inner type
    let input = BinBuilder::new().int(-1).build();
    assert_ok!(Value, &input, NewTypeStruct(-1));
}

#[test]
fn seq_tests() {
    type Value = Vec<i32>;

    let input = BinBuilder::new().list(0).build();
    assert_ok!(Value, &input, vec![]);
    let input = BinBuilder::new().list(1).int(-1).build();
    assert_ok!(Value, &input, vec![-1]);
    let input = BinBuilder::new().list(2).int(-1).int(-2).build();
    assert_ok!(Value, &input, vec![-1, -2]);
}

#[test]
fn tuple_tests() {
    let input = BinBuilder::new().list(1).int(-1).build();
    assert_ok!((i32,), &input, (-1,));

    type Value = ((),);

    let input = BinBuilder::new().list(1).list(0).build();
    assert_ok!(Value, &input, ((),));

    let input = BinBuilder::new().list(0).build();
    assert_err!(
        Value,
        &input,
        4,
        ErrorCode::ExpectedListOfLength {
            expected_min: 1,
            expected_max: 1,
            found: 0,
        }
    );
    let input = BinBuilder::new().list(2).build();
    assert_err!(
        Value,
        &input,
        4,
        ErrorCode::ExpectedListOfLength {
            expected_min: 1,
            expected_max: 1,
            found: 2,
        }
    );
}

#[test]
fn tuple_struct_tests() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct TupleStruct(i32, i32);
    type Value = TupleStruct;

    let input = BinBuilder::new().list(2).int(-1).int(-2).build();
    assert_ok!(Value, &input, TupleStruct(-1, -2));

    let input = BinBuilder::new().list(1).build();
    assert_err!(
        Value,
        &input,
        4,
        ErrorCode::ExpectedListOfLength {
            expected_min: 2,
            expected_max: 2,
            found: 1,
        }
    );
    let input = BinBuilder::new().list(3).build();
    assert_err!(
        Value,
        &input,
        4,
        ErrorCode::ExpectedListOfLength {
            expected_min: 2,
            expected_max: 2,
            found: 3,
        }
    );
}

#[test]
fn map_tests() {
    type Value = HashMap<i32, i32>;

    let input = BinBuilder::new().list(0).build();
    assert_ok!(Value, &input, map![]);

    let input = BinBuilder::new().list(2).int(-1).int(-2).build();
    assert_ok!(Value, &input, map![-1 => -2]);

    let input = BinBuilder::new().list(1).int(-1).build();
    assert_err!(Value, &input, 8, ErrorCode::ExpectedKeyValuePair);
}

#[test]
fn struct_tests() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct Struct {
        a: i32,
        b: i32,
    }
    type Value = Struct;

    let input = BinBuilder::new()
        .list(4)
        .str("a")
        .int(-1)
        .str("b")
        .int(-2)
        .build();
    assert_ok!(Value, &input, Struct { a: -1, b: -2 });
    let input = BinBuilder::new()
        .list(4)
        .str("b")
        .int(-2)
        .str("a")
        .int(-1)
        .build();
    assert_ok!(Value, &input, Struct { a: -1, b: -2 });

    let input = BinBuilder::new().list(1).build();
    assert_err!(Value, &input, 8, ErrorCode::ExpectedKeyValuePair);
    let input = BinBuilder::new().list(3).str("a").int(-1).build();
    assert_err!(Value, &input, 25, ErrorCode::ExpectedKeyValuePair);
}

#[test]
fn struct_optional_tests() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct OptStruct {
        #[serde(default)]
        a: i32,
        #[serde(default)]
        b: i32,
    }
    type Value = OptStruct;

    let input = BinBuilder::new().list(2).str("a").int(-1).build();
    assert_ok!(Value, &input, OptStruct { a: -1, b: 0 });
    let input = BinBuilder::new().list(2).str("b").int(-2).build();
    assert_ok!(Value, &input, OptStruct { a: 0, b: -2 });

    let input = BinBuilder::new().list(1).build();
    assert_err!(Value, &input, 8, ErrorCode::ExpectedKeyValuePair);
    let input = BinBuilder::new().list(3).str("a").int(-1).build();
    assert_err!(Value, &input, 25, ErrorCode::ExpectedKeyValuePair);
}

#[test]
fn enum_unit_variant_tests() {
    #[derive(Debug, PartialEq, Deserialize)]
    enum UnitVariant {
        V,
    }
    type Value = UnitVariant;

    let input = BinBuilder::new().str("V").build();
    assert_ok!(Value, &input, UnitVariant::V);

    let input = BinBuilder::new().str("!").build();
    let err = unwrap_err!(Value, &input, 0);
    assert_matches!(err.code(), ErrorCode::Custom(s) if s.contains("unknown variant"))
}

#[test]
fn enum_newtype_variant_tests() {
    #[derive(Debug, PartialEq, Deserialize)]
    enum NewTypeVariant {
        V(i32),
    }
    type Value = NewTypeVariant;

    let input = BinBuilder::new().str("V").list(1).int(-1).build();
    assert_ok!(Value, &input, NewTypeVariant::V(-1));

    let input = BinBuilder::new().str("!").build();
    let err = unwrap_err!(Value, &input, 0);
    assert_matches!(err.code(), ErrorCode::Custom(s) if s.contains("unknown variant"));

    let input = BinBuilder::new().str("V").list(0).build();
    assert_err!(
        Value,
        &input,
        13,
        ErrorCode::ExpectedListOfLength {
            expected_min: 1,
            expected_max: 1,
            found: 0,
        }
    );
    let input = BinBuilder::new().str("V").list(2).build();
    assert_err!(
        Value,
        &input,
        13,
        ErrorCode::ExpectedListOfLength {
            expected_min: 1,
            expected_max: 1,
            found: 2,
        }
    );
}

#[test]
fn enum_tuple_variant_tests() {
    #[derive(Debug, PartialEq, Deserialize)]
    enum TupleVariant {
        V(i32, i32),
    }
    type Value = TupleVariant;

    let input = BinBuilder::new().str("V").list(2).int(-1).int(-2).build();
    assert_ok!(Value, &input, TupleVariant::V(-1, -2));

    let input = BinBuilder::new().str("!").build();
    let err = unwrap_err!(Value, &input, 0);
    assert_matches!(err.code(), ErrorCode::Custom(s) if s.contains("unknown variant"));

    let input = BinBuilder::new().str("V").list(1).build();
    assert_err!(
        Value,
        &input,
        13,
        ErrorCode::ExpectedListOfLength {
            expected_min: 2,
            expected_max: 2,
            found: 1,
        }
    );
    let input = BinBuilder::new().str("V").list(3).build();
    assert_err!(
        Value,
        &input,
        13,
        ErrorCode::ExpectedListOfLength {
            expected_min: 2,
            expected_max: 2,
            found: 3,
        }
    );
}

#[test]
fn enum_struct_variant_tests() {
    #[derive(Debug, PartialEq, Deserialize)]
    enum StructVariant {
        V { a: i32, b: i32 },
    }
    type Value = StructVariant;

    let input = BinBuilder::new()
        .str("V")
        .list(4)
        .str("a")
        .int(-1)
        .str("b")
        .int(-2)
        .build();
    assert_ok!(Value, &input, StructVariant::V { a: -1, b: -2 });
    let input = BinBuilder::new()
        .str("V")
        .list(4)
        .str("b")
        .int(-2)
        .str("a")
        .int(-1)
        .build();
    assert_ok!(Value, &input, StructVariant::V { a: -1, b: -2 });

    let input = BinBuilder::new().str("!").build();
    let err = unwrap_err!(Value, &input, 0);
    assert_matches!(err.code(), ErrorCode::Custom(s) if s.contains("unknown variant"));

    let input = BinBuilder::new().str("V").list(1).build();
    assert_err!(Value, &input, 17, ErrorCode::ExpectedKeyValuePair);
    let input = BinBuilder::new().str("V").list(3).str("a").int(-1).build();
    assert_err!(Value, &input, 34, ErrorCode::ExpectedKeyValuePair);
}

#[test]
fn enum_struct_variant_optional_tests() {
    #[derive(Debug, PartialEq, Deserialize)]
    enum OptStructVariant {
        V {
            #[serde(default)]
            a: i32,
            #[serde(default)]
            b: i32,
        },
    }

    type Value = OptStructVariant;

    let input = BinBuilder::new()
        .str("V")
        .list(4)
        .str("a")
        .int(-1)
        .str("b")
        .int(-2)
        .build();
    assert_ok!(Value, &input, OptStructVariant::V { a: -1, b: -2 });
    let input = BinBuilder::new()
        .str("V")
        .list(4)
        .str("b")
        .int(-2)
        .str("a")
        .int(-1)
        .build();
    assert_ok!(Value, &input, OptStructVariant::V { a: -1, b: -2 });

    let input = BinBuilder::new().str("V").list(2).str("a").int(-1).build();
    assert_ok!(Value, &input, OptStructVariant::V { a: -1, b: 0 });
    let input = BinBuilder::new().str("V").list(2).str("b").int(-2).build();
    assert_ok!(Value, &input, OptStructVariant::V { a: 0, b: -2 });

    let input = BinBuilder::new().str("!").build();
    let err = unwrap_err!(Value, &input, 0);
    assert_matches!(err.code(), ErrorCode::Custom(s) if s.contains("unknown variant"));

    let input = BinBuilder::new().str("V").list(1).build();
    assert_err!(Value, &input, 17, ErrorCode::ExpectedKeyValuePair);
    let input = BinBuilder::new().str("V").list(3).str("a").int(-1).build();
    assert_err!(Value, &input, 34, ErrorCode::ExpectedKeyValuePair);
}
