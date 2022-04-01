mod from_str_de_tests;
mod round_trip_tests;
mod string_quoting_tests;
mod structs;
mod to_pretty_fmt_tests;
mod to_pretty_ser_tests;
mod to_string_ser_tests;

#[macro_export]
macro_rules! map {
    () => { HashMap::new() };
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut m = HashMap::new();
            $(
            m.insert($key, $value);
            )+
            m
        }
    };
}
