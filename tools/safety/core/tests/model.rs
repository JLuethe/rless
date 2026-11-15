#[cfg(feature = "fault-testing")]
#[path = "../../core-fuzz/driver.rs"]
mod driver;

#[test]
#[cfg(feature = "fault-testing")]
fn bounded_action_model_has_success_failure_and_recovery_paths() {
    let mut attempts = 0;
    let mut failures = 0;
    for seed in [0u64, 1, 712, 20261023] {
        let mut state = seed.wrapping_add(1);
        for _ in 0..64 {
            let bytes: Vec<_> = (0..256)
                .map(|_| {
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    state as u8
                })
                .collect();
            let stats = driver::exercise(&bytes);
            attempts += stats.reserve_attempts;
            failures += stats.reserve_failures;
        }
    }
    assert!(attempts > 0 && failures > 0 && failures < attempts);
}
