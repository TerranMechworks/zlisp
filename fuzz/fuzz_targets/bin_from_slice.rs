#![no_main]
use libfuzzer_sys::fuzz_target;
use zlisp_bin::from_slice;
use zlisp_value::Value;

fuzz_target!(|data: &[u8]| {
    let _ = from_slice::<Value>(data);
});
