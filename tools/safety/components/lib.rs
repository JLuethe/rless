//! Compile the actual application module, without the native ELF constructor.
//! Native signal disposition/constructor behavior has separate process tests.
#![allow(dead_code)]
#[path = "../../../src/runtime.rs"]
mod runtime;
