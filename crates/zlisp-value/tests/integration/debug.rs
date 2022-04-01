use zlisp_value::Value;

macro_rules! assert_debug {
    ($value:expr, $debug:expr) => {
        let v: Value = $value.into();
        assert_eq!(format!("{:?}", v), $debug);
    };
}

#[test]
fn debug_tests() {
    assert_debug!(i32::MIN, format!("Int({:?})", i32::MIN));
    assert_debug!(0, "Int(0)");
    assert_debug!(i32::MAX, format!("Int({:?})", i32::MAX));

    assert_debug!(f32::MIN, format!("Float({:?})", f32::MIN));
    assert_debug!(0.0, "Float(0.0)");
    assert_debug!(f32::MAX, format!("Float({:?})", f32::MAX));

    assert_debug!("foo", r#"String("foo")"#);

    assert_debug!(vec![], "[]");
    assert_debug!(&[Value::Int(0)], "[Int(0)]");
    assert_debug!(
        &[
            Value::from(0),
            Value::from(0.0),
            Value::from("foo"),
            Value::from(&[])
        ],
        r#"[Int(0), Float(0.0), String("foo"), []]"#
    );
}
