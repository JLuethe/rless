#![no_main]
// Intentional test-only oracle defect; excluded from the product workspace.
mod model;
libfuzzer_sys::fuzz_target!(|input: &[u8]| model::exercise(input, true));
