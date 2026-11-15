//! Isolated pure Rust preparation for M2. Not linked into either pager binary.
//! Production caller migration remains gated on the original M0/M1 prerequisites.
#![forbid(unsafe_code)]

pub mod arena;
pub mod bytebuf;
pub mod coordinate;
mod reserve;

#[cfg(any(test, feature = "fault-testing"))]
pub use reserve::ReserveFault;

/// Errors are small values; reporting them does not allocate a context string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageError {
    CapacityOverflow,
    AllocationFailed,
    InvalidRange,
    InteriorNul { offset: usize },
    InvalidFaultOrdinal,
}
