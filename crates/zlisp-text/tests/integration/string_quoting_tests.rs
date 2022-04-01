use zlisp_text::{from_str, to_pretty, to_string, WhitespaceConfig};

macro_rules! assert_quoted {
    ($input:expr, $value:expr, $output:expr) => {
        let o = concat!($output, "\r\n");
        let v: String = from_str($input).unwrap();
        assert_eq!(&v, $value);
        let s = to_string(&v, WhitespaceConfig::default()).expect("to_string");
        assert_eq!(&s, o, "to_string");
        let s = to_pretty(&v, WhitespaceConfig::default()).expect("to_pretty");
        assert_eq!(&s, o, "to_pretty");
    };
}

#[test]
fn string_tests() {
    assert_quoted!("foo", "foo", "foo");
    assert_quoted!("\"f\"oo", "foo", "foo");
    assert_quoted!("f\"o\"o", "foo", "foo");
    assert_quoted!("fo\"o\"", "foo", "foo");
    assert_quoted!("\"fo\"o", "foo", "foo");
    assert_quoted!("fo\"o\"", "foo", "foo");
    assert_quoted!("\"foo\"", "foo", "foo");
    assert_quoted!("\"f\"o\"o\"", "foo", "foo");
    assert_quoted!("\"f\"\"o\"\"o\"", "foo", "foo");
    assert_quoted!("\" \t\r\n\"", " \t\r\n", "\" \t\r\n\"");
}
