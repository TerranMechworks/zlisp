use super::*;
use assert_matches::assert_matches;

#[test]
fn pfe_invalid_test() {
    pfe_invalid();
}

macro_rules! assert_i32_ok {
    ($s:expr, $expected:expr) => {
        let actual = parse_i32_inner($s, Location::new(1, 1)).unwrap();
        assert_eq!(actual, $expected);
    };
}

macro_rules! assert_i32_err {
    ($s:expr) => {
        let loc = Location::new(1, 1);
        let err = parse_i32_inner($s, loc.clone()).unwrap_err();
        assert_eq!(err.location(), Some(loc).as_ref());
        assert_matches!(err.code(), ErrorCode::ParseIntError {
            e: _,
            s,
        } if s == $s);
    };
}

macro_rules! assert_f32_ok {
    ($s:expr, $expected:expr) => {
        let actual = parse_f32_inner($s, Location::new(1, 1)).unwrap();
        assert_eq!(actual, $expected);
    };
}

macro_rules! assert_f32_err {
    ($s:expr) => {
        let loc = Location::new(1, 1);
        let err = parse_f32_inner($s, loc.clone()).unwrap_err();
        assert_eq!(err.location(), Some(loc).as_ref());
        assert_matches!(err.code(), ErrorCode::ParseFloatError {
            e: _,
            s,
        } if s == $s);
    };
}

#[test]
fn i32_tests() {
    assert_i32_ok!("0", 0);
    assert_i32_ok!("-0", 0);
    assert_i32_ok!("+0", 0);
    assert_i32_ok!("1", 1);
    assert_i32_ok!("-1", -1);
    assert_i32_ok!("+1", 1);

    let max_s = format!("{}", i32::MAX);
    assert_i32_ok!(&max_s, i32::MAX);
    let min_s = format!("{}", i32::MIN);
    assert_i32_ok!(&min_s, i32::MIN);

    // empty
    assert_i32_err!("");
    assert_i32_err!("-");
    assert_i32_err!("+");

    // invalid
    assert_i32_err!("0x0");
    assert_i32_err!("a");

    // overflow
    let over_s = format!("{}", (i32::MAX as i64) + 1);
    assert_i32_err!(&over_s);
    let under_s = format!("{}", (i32::MIN as i64) - 1);
    assert_i32_err!(&under_s);
}

#[test]
fn f32_tests() {
    assert_f32_ok!("0", 0.0);
    assert_f32_ok!("-0", 0.0);
    assert_f32_ok!("+0", 0.0);
    assert_f32_ok!("0.", 0.0);
    assert_f32_ok!("-0.", 0.0);
    assert_f32_ok!("+0.", 0.0);
    assert_f32_ok!(".0", 0.0);
    assert_f32_ok!("-.0", 0.0);
    assert_f32_ok!("+.0", 0.0);
    assert_f32_ok!("0.0", 0.0);
    assert_f32_ok!("-0.0", 0.0);
    assert_f32_ok!("+0.0", 0.0);

    assert_f32_ok!("1", 1.0);
    assert_f32_ok!("-1", -1.0);
    assert_f32_ok!("+1", 1.0);
    assert_f32_ok!("1.", 1.0);
    assert_f32_ok!("-1.", -1.0);
    assert_f32_ok!("+1.", 1.0);
    assert_f32_ok!(".1", 0.1);
    assert_f32_ok!("-.1", -0.1);
    assert_f32_ok!("+.1", 0.1);
    assert_f32_ok!("1.0", 1.0);
    assert_f32_ok!("-1.0", -1.0);
    assert_f32_ok!("+1.0", 1.0);

    let max_s = format!("{:.1}", f32::MAX);
    assert_f32_ok!(&max_s, f32::MAX);
    let min_s = format!("{:.1}", f32::MIN);
    assert_f32_ok!(&min_s, f32::MIN);

    // empty
    assert_f32_err!("");
    assert_f32_err!("-");
    assert_f32_err!("+");

    // invalid
    assert_f32_err!("a");
    assert_f32_err!("-a");
    assert_f32_err!("+a");

    // non-finite
    assert_f32_err!("inf");
    assert_f32_err!("-inf");
    assert_f32_err!("+inf");
    assert_f32_err!("infinity");
    assert_f32_err!("-infinity");
    assert_f32_err!("+infinity");
    assert_f32_err!("NaN");

    // double-period
    assert_f32_err!("0..0");
    assert_f32_err!("-0..0");
    assert_f32_err!("+0..0");
    assert_f32_err!("0..");
    assert_f32_err!("-0..");
    assert_f32_err!("+0..");
    assert_f32_err!("..0");
    assert_f32_err!("-..0");
    assert_f32_err!("+..0");

    // overflow / non-finite
    let over_s = format!("{:.1}", f64::MAX);
    assert_f32_err!(&over_s);
    let under_s = format!("{:.1}", f64::MIN);
    assert_f32_err!(&under_s);
}
