use super::map;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use zlisp_bin::{from_slice, to_vec};

macro_rules! round_trip {
    ($type:ty, $value:expr) => {
        let expected: $type = $value;
        let bin = to_vec(&expected).unwrap();
        let actual: $type = from_slice(&bin).unwrap();
        assert_eq!(actual, expected);
    };
}

#[test]
fn signed_tests() {
    round_trip!(i32, 0);
    round_trip!(i32, 1);
    round_trip!(i32, -1);
    round_trip!(i32, i32::MIN);
    round_trip!(i32, i32::MAX);
}

#[test]
fn float_tests() {
    round_trip!(f32, 0.0);
    round_trip!(f32, 1.0);
    round_trip!(f32, -1.0);
    round_trip!(f32, f32::MIN);
    round_trip!(f32, f32::MAX);
}

#[test]
fn string_tests() {
    round_trip!(&str, "foo");
    round_trip!(String, String::from("foo"));
}

#[test]
fn option_tests() {
    round_trip!(Option<i32>, None);
    round_trip!(Option<i32>, Some(-1));
    round_trip!(Option<()>, None);
    round_trip!(Option<()>, Some(()));
}

#[test]
fn unit_type_tests() {
    round_trip!((), ());
}

#[test]
fn unit_struct_tests() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct UnitStruct;

    round_trip!(UnitStruct, UnitStruct);
}

#[test]
fn newtype_struct_tests() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct NewTypeStruct(i32);

    round_trip!(NewTypeStruct, NewTypeStruct(1));
}

#[test]
fn seq_tests() {
    round_trip!(Vec<i32>, vec![]);
    round_trip!(Vec<i32>, vec![-1]);
    round_trip!(Vec<i32>, vec![-1, -2]);
}

#[test]
fn tuple_tests() {
    round_trip!((i32,), (-1,));
    round_trip!(((),), ((),));
}

#[test]
fn tuple_struct_tests() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TupleStruct(i32, i32);

    round_trip!(TupleStruct, TupleStruct(-1, -2));
}

#[test]
fn map_tests() {
    round_trip!(HashMap<i32, i32>, map![]);
    round_trip!(HashMap<i32, i32>, map![-1 => -2]);
    round_trip!(HashMap<&str, i32>, map!["a" => -1, "b" => -2]);
}

#[test]
fn struct_tests() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Struct {
        a: i32,
        b: i32,
    }

    round_trip!(Struct, Struct { a: -1, b: -2 });
}

#[test]
fn struct_optional_tests() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct OptStruct {
        #[serde(default)]
        a: i32,
        #[serde(default)]
        b: i32,
    }

    round_trip!(OptStruct, OptStruct { a: -1, b: -2 });
}

#[test]
fn enum_unit_variant_tests() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum UnitVariant {
        V,
    }

    round_trip!(UnitVariant, UnitVariant::V);
}

#[test]
fn enum_newtype_variant_tests() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum NewTypeVariant {
        V(i32),
    }

    round_trip!(NewTypeVariant, NewTypeVariant::V(-1));
}

#[test]
fn enum_tuple_variant_tests() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum TupleVariant {
        V(i32, i32),
    }

    round_trip!(TupleVariant, TupleVariant::V(-1, -2));
}

#[test]
fn enum_struct_variant_tests() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum StructVariant {
        V { a: i32, b: i32 },
    }

    round_trip!(StructVariant, StructVariant::V { a: -1, b: -2 });
}

#[test]
fn enum_struct_variant_optional_tests() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum OptStructVariant {
        V {
            #[serde(default)]
            a: i32,
            #[serde(default)]
            b: i32,
        },
    }

    round_trip!(OptStructVariant, OptStructVariant::V { a: -1, b: -2 });
}
