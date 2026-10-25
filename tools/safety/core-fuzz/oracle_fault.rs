#![no_main]
mod driver;
libfuzzer_sys::fuzz_target!(|input: &[u8]| {
    driver::exercise(input);
    // Dedicated test-only oracle defect; excluded from production binaries.
    if input.windows(5).any(|v| v == b"RLESS") {
        panic!("injected component oracle violation: detector control");
    }
});
