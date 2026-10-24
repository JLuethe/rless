//! Preserve the inherited Unix SIGPIPE disposition across Rust's runtime startup.
use std::sync::atomic::{AtomicUsize, Ordering};

static INHERITED_SIGPIPE: AtomicUsize = AtomicUsize::new(libc::SIG_DFL);

unsafe extern "C" fn remember_sigpipe() {
    let mut action: libc::sigaction = std::mem::zeroed();
    if libc::sigaction(libc::SIGPIPE, std::ptr::null(), &mut action) == 0 {
        INHERITED_SIGPIPE.store(action.sa_sigaction, Ordering::Relaxed);
    }
}

// ELF constructors run before std initializes the Rust main function. Across
// exec, SIGPIPE can only be default or ignored; custom handlers are reset.
#[used]
#[link_section = ".init_array"]
static REMEMBER_SIGPIPE: unsafe extern "C" fn() = remember_sigpipe;

pub fn init() {
    unsafe {
        libc::signal(libc::SIGPIPE, INHERITED_SIGPIPE.load(Ordering::Relaxed));
    }
}
