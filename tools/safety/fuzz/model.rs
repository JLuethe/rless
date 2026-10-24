// Test-only ownership/commit model. This is not the translated pager's buffer.
#![forbid(unsafe_code)]
pub fn exercise(input: &[u8], inject_bug: bool) {
    let mut state = Vec::<u8>::new();
    for (step, chunk) in input.chunks(8).enumerate() {
        let old = state.clone();
        let fail = chunk.first().copied().unwrap_or(0) % 3;
        let mut next = state.clone();
        if fail == 0 {
            assert_eq!(state, old);
            continue;
        }
        next.try_reserve(chunk.len()).unwrap();
        next.extend_from_slice(chunk);
        if fail == 1 {
            assert_eq!(state, old);
        } else {
            state = next;
            assert!(state.ends_with(chunk));
        }
        assert!(state.len() <= (step + 1) * 8);
    }
    if inject_bug && input.windows(5).any(|part| part == b"RLESS") {
        panic!("injected rollback violation: detector control");
    }
}
