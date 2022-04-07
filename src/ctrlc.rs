use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(not(unix))]
compile_error! {"Windows is not supported at the moment"}

static CTRL_C: AtomicBool = AtomicBool::new(false);

extern "C" fn callback(_signum: i32) {
    CTRL_C.store(true, Ordering::Relaxed);
}

pub fn init() {
    unsafe {
        if libc::signal(libc::SIGINT, callback as libc::sighandler_t) == libc::SIG_ERR {
            unreachable!()
        }
    }
}

pub fn poll() -> bool {
    CTRL_C.swap(false, Ordering::Relaxed)
}
