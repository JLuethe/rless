#![no_main]
mod model;
libfuzzer_sys::fuzz_target!(|input: &[u8]| model::exercise(input, false));
