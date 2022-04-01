use super::map;
use super::structs::*;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use zlisp_text::{to_pretty, WhitespaceConfig};

macro_rules! assert_fmt {
    ($type:ty, $value:expr, $expected:expr) => {
        // not only is the default config horrible to test (windows newlines),
        // but the delimiter and indent are the same.
        let config = WhitespaceConfig::builder()
            .indent("    ")
            .delimiter(" ")
            .newline("\n")
            .build();
        let expected = concat!($expected, "\n");
        let v: $type = $value;
        let actual = to_pretty(&v, &config).unwrap();
        assert_eq!(&actual, &expected);
    };
}

#[test]
fn fmt_scalar_tests() {
    assert_fmt!(i32, 0, "0");
    assert_fmt!(f32, 0.0, "0.000000");
    assert_fmt!(String, String::from("foo"), "foo");
    assert_fmt!(NewTypeStruct, NewTypeStruct(0), "0");
    // no expanded or nested (well, newtype, but it just delegates)
}

#[test]
fn fmt_unit_tests() {
    assert_fmt!((), (), "()");
    assert_fmt!(UnitStruct, UnitStruct, "()");
    // no expanded or nested
}

#[test]
fn fmt_option_tests() {
    // compact
    assert_fmt!(Option<i32>, None, "()");
    assert_fmt!(Option<i32>, Some(-1), "(-1)");
    assert_fmt!(Option<()>, None, "()");
    assert_fmt!(Option<()>, Some(()), "(())");
    // expanded
    assert_fmt!(
        Option<Vec<i32>>,
        Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]),
        "((
    0
    1
    2
    3
    4
    5
    6
    7
    8
    9
    10
    11
))"
    );
    // nested compact?
    assert_fmt!(Option<Option<i32>>, Some(None), "(())");
    assert_fmt!(Option<Option<i32>>, Some(Some(-1)), "((-1))");
    // nested expanded?
    assert_fmt!(
        Option<Option<Vec<i32>>>,
        Some(Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])),
        "(((
    0
    1
    2
    3
    4
    5
    6
    7
    8
    9
    10
    11
)))"
    );
}

#[test]
fn fmt_seq_tests() {
    // compact
    assert_fmt!(Vec<i32>, vec![], "()");
    assert_fmt!(Vec<i32>, vec![-1], "(-1)");
    assert_fmt!(Vec<i32>, vec![-1, -2], "(-1 -2)");
    // expanded
    assert_fmt!(
        Vec<i32>,
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        "(
    0
    1
    2
    3
    4
    5
    6
    7
    8
    9
    10
    11
)"
    );
    // nested compact
    assert_fmt!(Vec<Vec<i32>>, vec![vec![0, 1, 2]], "((0 1 2))");
    // nested expanded
    assert_fmt!(
        Vec<Vec<i32>>,
        vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]],
        "(
    (
        0
        1
        2
        3
        4
        5
        6
        7
        8
        9
        10
        11
    )
)"
    );
}

#[test]
fn fmt_tuple_tests() {
    // compact
    assert_fmt!(((),), ((),), "(())");
    assert_fmt!((i32,), (-1,), "(-1)");
    assert_fmt!((i32, i32), (-1, -2), "(-1 -2)");
    // expanded
    assert_fmt!(
        (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32),
        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11),
        "(
    0
    1
    2
    3
    4
    5
    6
    7
    8
    9
    10
    11
)"
    );
    // nested compact
    assert_fmt!((Vec<i32>,), (vec![-1, -2],), "((-1 -2))");
    // nested expanded
    assert_fmt!(
        ((i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32),),
        ((0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11),),
        "(
    (
        0
        1
        2
        3
        4
        5
        6
        7
        8
        9
        10
        11
    )
)"
    );
}

#[test]
fn fmt_tuple_struct_tests() {
    // tuple structs are expected to work like tuples
    assert_fmt!(TupleStruct, TupleStruct(-1, -2), "(-1 -2)");
}

#[test]
fn fmt_map_tests() {
    // compact
    assert_fmt!(HashMap<i32, i32>, map![], "(\n)");
    assert_fmt!(HashMap<i32, i32>, map![-1 => -2], "(
    -1 -2
)");
    // expanded
    assert_fmt!(
        HashMap<(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32), i32>,
        map![(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11) => -1],
        "(
    (
        0
        1
        2
        3
        4
        5
        6
        7
        8
        9
        10
        11
    ) -1
)"
    );
    // nested compact
    assert_fmt!(HashMap<(i32, i32), (i32, i32)>, map![(-1, -2) => (-3, -4)], "(
    (-1 -2) (-3 -4)
)");
    // nested expanded
    assert_fmt!(
        HashMap<String, (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32)>,
        map![String::from("key") => (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11)],
        "(
    key (
        0
        1
        2
        3
        4
        5
        6
        7
        8
        9
        10
        11
    )
)"
    );
}

#[test]
fn fmt_struct_tests() {
    // compact
    assert_fmt!(Struct, Struct { a: -1, b: -2 }, "(a -1 b -2)");
    assert_fmt!(OptStruct, OptStruct { a: -1, b: -2 }, "(a -1 b -2)");
    // expanded
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    pub struct AltStruct {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
    }
    assert_fmt!(
        AltStruct,
        AltStruct {
            a: -1,
            b: -2,
            c: -3,
            d: -4,
            e: -5,
            f: -6,
        },
        "(
    a -1
    b -2
    c -3
    d -4
    e -5
    f -6
)"
    );
    // nested compact
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    pub struct NestedStruct {
        a: Vec<i32>,
    }
    assert_fmt!(
        NestedStruct,
        NestedStruct { a: vec![-1, -2] },
        "(a (-1 -2))"
    );
    // nested expanded
    assert_fmt!(
        NestedStruct,
        NestedStruct {
            a: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        },
        "(
    a (
        0
        1
        2
        3
        4
        5
        6
        7
        8
        9
        10
        11
    )
)"
    );
}

#[test]
fn fmt_enum_unit_variant_tests() {
    // compact
    assert_fmt!(UnitVariant, UnitVariant::V, "V");
    // no expanded or nested
}

#[test]
fn fmt_enum_newtype_variant_tests() {
    // compact
    assert_fmt!(NewTypeVariant, NewTypeVariant::V(-1), "V(-1)");

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    pub enum AltNewTypeVariant {
        V(Vec<i32>),
    }
    assert_fmt!(AltNewTypeVariant, AltNewTypeVariant::V(vec![-1]), "V((-1))");

    // expanded
    assert_fmt!(
        HashMap<(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32), i32>,
        map![(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11) => -1],
        "(
    (
        0
        1
        2
        3
        4
        5
        6
        7
        8
        9
        10
        11
    ) -1
)"
    );
}

#[test]
fn fmt_enum_tuple_variant_tests() {
    // tuple variants are expected to work like tuples
    assert_fmt!(TupleVariant, TupleVariant::V(-1, -2), "V(-1 -2)");
}

#[test]
fn fmt_enum_struct_variant_tests() {
    // struct variants are expected to work like structs
    assert_fmt!(
        StructVariant,
        StructVariant::V { a: -1, b: -2 },
        "V(a -1 b -2)"
    );
    assert_fmt!(
        OptStructVariant,
        OptStructVariant::V { a: -1, b: -2 },
        "V(a -1 b -2)"
    );
}
