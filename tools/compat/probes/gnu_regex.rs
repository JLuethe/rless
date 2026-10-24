// Feasibility probe; no pager code or C regex struct layout in Rust.
unsafe extern "C" {
    fn rless_probe_match(pattern: *const u8, plen: usize,
                         text: *const u8, len: usize, end: *mut i32) -> i32;
}
fn main() {
    let cases: &[(&[u8], &[u8], i32, i32)] = &[
        (b"a|aa", b"aa", 0, 2),
        (b"([a-z])\\1", b"book", 1, 3),
        (b"b", b"a\0b", 2, 3),
        (b"(", b"any", -2, -1),
    ];
    for (pattern, text, expected_start, expected_end) in cases {
        let mut end = -1;
        let start = unsafe { rless_probe_match(pattern.as_ptr(), pattern.len(),
                                               text.as_ptr(), text.len(), &mut end) };
        assert_eq!((start, end), (*expected_start, *expected_end));
    }
    println!("4 GNU regex ABI/semantics probes passed");
}
