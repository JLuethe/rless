// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

#![allow(unused_unsafe, unused_variables, unused_imports)]
#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod brac;
pub mod ch;
pub mod charset;
pub mod cmdbuf;
pub mod command;
pub mod cvt;
pub mod decode;
pub mod edit;
pub mod evar;
pub mod filename;
pub mod forwback;
pub mod help;
pub mod ifile;
pub mod input;
pub mod jump;
pub mod lesskey_parse;
pub mod line;
pub mod linenum;
pub mod lmsg;
pub mod lsystem;
pub mod mark;
pub mod optfunc;
pub mod option;
pub mod opttbl;
pub mod os;
pub mod output;
pub mod pager;
pub mod pattern;
pub mod position;
pub mod process;
pub mod prompt;
pub mod screen;
pub mod search;
pub mod signal;
pub mod tags;
pub mod ttyin;
pub mod version;
pub mod xbuf;

mod runtime;
