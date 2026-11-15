//! Compile the production atomic event state without the native ELF constructor.
//! Signal disposition and wakeup behavior have separate process tests.
#![allow(dead_code)]
#[path = "../../../src/signal_events.rs"]
mod signal_events;
