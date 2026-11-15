//! Atomic notification state shared by the installed handlers and dispatcher.
//! This module has no libc calls, so its production operations can run under Miri.
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};

pub(crate) struct SignalEvents {
    pub(crate) pending: AtomicU32,
    pub(crate) commands: AtomicUsize,
    pub(crate) overflow: AtomicBool,
}

impl SignalEvents {
    pub(crate) const fn new() -> Self {
        Self {
            pending: AtomicU32::new(0),
            commands: AtomicUsize::new(0),
            overflow: AtomicBool::new(false),
        }
    }

    #[inline]
    pub(crate) fn publish(&self, bit: u32) {
        self.pending.fetch_or(bit, Ordering::SeqCst);
    }

    #[inline]
    pub(crate) fn publish_command(&self, bit: u32) {
        if self.commands.fetch_add(1, Ordering::SeqCst) == usize::MAX {
            self.overflow.store(true, Ordering::SeqCst);
        }
        self.publish(bit);
    }

    pub(crate) fn has_pending(&self) -> bool {
        self.pending.load(Ordering::SeqCst) != 0
    }

    pub(crate) fn take_pending(&self) -> u32 {
        self.pending.swap(0, Ordering::SeqCst)
    }

    pub(crate) fn clear_pending(&self, bits: u32) -> u32 {
        self.pending.fetch_and(!bits, Ordering::SeqCst) & bits
    }

    pub(crate) fn take_commands(&self) -> usize {
        self.commands.swap(0, Ordering::SeqCst)
    }

    pub(crate) fn take_overflow(&self) -> bool {
        self.overflow.swap(false, Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier};

    #[test]
    fn state_bits_coalesce_but_commands_keep_their_count() {
        let events = SignalEvents::new();
        const RESIZE: u32 = 1 << 3;
        const COMMAND: u32 = 1 << 5;
        const TERMINATE: u32 = 1 << 6;
        events.publish(RESIZE);
        events.publish(RESIZE);
        events.publish_command(COMMAND);
        events.publish_command(COMMAND);
        events.publish(TERMINATE);
        assert_eq!(events.take_pending(), RESIZE | COMMAND | TERMINATE);
        assert_eq!(events.take_commands(), 2);
        assert!(!events.has_pending());
        assert!(!events.take_overflow());
    }

    #[test]
    fn command_counter_overflow_remains_visible_after_dispatch() {
        let events = SignalEvents::new();
        events.commands.store(usize::MAX, Ordering::SeqCst);
        events.publish_command(1 << 5);
        assert_eq!(events.take_pending(), 1 << 5);
        assert_eq!(events.take_commands(), 0);
        assert!(events.take_overflow());
        assert!(!events.take_overflow());
    }

    #[test]
    fn publication_racing_with_dispatch_is_seen_in_one_of_two_snapshots() {
        for _ in 0..16 {
            let events = Arc::new(SignalEvents::new());
            let gate = Arc::new(Barrier::new(2));
            let other_events = Arc::clone(&events);
            let other_gate = Arc::clone(&gate);
            let publisher = std::thread::spawn(move || {
                other_gate.wait();
                other_events.publish(1 << 3);
            });
            gate.wait();
            let before = events.take_pending();
            publisher.join().unwrap();
            let after = events.take_pending();
            assert_eq!(before | after, 1 << 3);
            assert!(!events.has_pending());
        }
    }
}
