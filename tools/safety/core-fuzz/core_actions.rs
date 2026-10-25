#![no_main]
mod driver;
libfuzzer_sys::fuzz_target!(|input: &[u8]| {
    driver::exercise(input);
});
