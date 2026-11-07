//! Preserve the inherited Unix SIGPIPE disposition across Rust's runtime startup.
use std::sync::atomic::{AtomicUsize, Ordering};

static INHERITED_SIGPIPE: AtomicUsize = AtomicUsize::new(libc::SIG_DFL);

unsafe extern "C" fn remember_sigpipe() {
    let mut action: libc::sigaction = std::mem::zeroed();
    if libc::sigaction(libc::SIGPIPE, std::ptr::null(), &mut action) != 0 {
        // No pager state exists yet. Continuing would replace an unknown
        // inherited disposition with the default one in init().
        libc::_exit(1);
    }
    INHERITED_SIGPIPE.store(action.sa_sigaction, Ordering::Relaxed);
}

// ELF constructors run before std initializes the Rust main function. Across
// exec, SIGPIPE can only be default or ignored; custom handlers are reset.
#[used]
#[link_section = ".init_array"]
static REMEMBER_SIGPIPE: unsafe extern "C" fn() = remember_sigpipe;

pub fn init() {
    unsafe {
        if libc::signal(libc::SIGPIPE, INHERITED_SIGPIPE.load(Ordering::Relaxed)) == libc::SIG_ERR {
            // A failed restoration must not leave std's SIGPIPE disposition
            // in place while the pager starts processing untrusted input.
            libc::_exit(1);
        }
    }
}
