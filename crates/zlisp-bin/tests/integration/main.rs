mod any;
mod bin_builder;
mod from_slice_de_tests;
mod from_slice_parse_tests;
mod round_trip_tests;
mod to_vec_ser_tests;

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
