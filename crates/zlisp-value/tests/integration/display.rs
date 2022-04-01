use zlisp_value::Value;

macro_rules! assert_display {
    ($value:expr, $display:expr) => {
        let v: Value = $value.into();
        assert_eq!(format!("{}", v), $display);
    };
}

#[test]
fn display_tests() {
    assert_display!(i32::MIN, format!("{}", i32::MIN));
    assert_display!(0, "0");
    assert_display!(i32::MAX, format!("{}", i32::MAX));

    assert_display!(f32::MIN, format!("{:.6}", f32::MIN));
    assert_display!(0.0, "0.000000");
    assert_display!(f32::MAX, format!("{:.6}", f32::MAX));

    assert_display!("foo", "foo");

    assert_display!(vec![], "()");
    assert_display!(&[Value::Int(0)], "(0)");
    assert_display!(
        &[
            Value::from(0),
            Value::from(0.0),
            Value::from("foo"),
            Value::from(&[])
        ],
        "(0 0.000000 foo ())"
    );
}

macro_rules! assert_pretty {
    ($value:expr, $display:expr) => {
        let v: Value = $value.into();
        assert_eq!(format!("{:#}", v), $display);
    };
}

#[test]
fn pretty_tests() {
    assert_pretty!(i32::MIN, format!("{}", i32::MIN));
    assert_pretty!(0, "0");
    assert_pretty!(i32::MAX, format!("{}", i32::MAX));

    assert_pretty!(f32::MIN, format!("{:.6}", f32::MIN));
    assert_pretty!(0.0, "0.000000");
    assert_pretty!(f32::MAX, format!("{:.6}", f32::MAX));

    assert_pretty!("foo", "foo");

    assert_pretty!(vec![], "()");
    assert_pretty!(&[Value::Int(0)], "(0)");
    assert_pretty!(
        &[Value::from(0), Value::from(0.0), Value::from("foo")],
        "(0\t0.000000\tfoo)"
    );

    assert_pretty!(
        &[Value::from(&[])],
        "(
\t()
)"
    );
    assert_pretty!(
        &[Value::from(&[Value::from(0)])],
        "(
\t(0)
)"
    );

    assert_pretty!(
        &[
            Value::from(0),
            Value::from(0.0),
            Value::from("foo"),
            Value::from(&[])
        ],
        "(
\t0
\t0.000000
\tfoo
\t()
)"
    );
}
