//! Process lifetime management

use libc::c_int;
use syscalls::*;
use posix::signal::raise;
use libc::SIGABRT;

static mut ATEXIT_FNS: [Option<extern "C" fn()>; 32] =
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
     None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
     None, None];

/// Terminates the process normally, performing the regular cleanup.
/// All C streams are closed, and all files created with tmpfile are removed.
/// Status can be zero or EXIT_SUCCESS, or EXIT_FAILURE.
#[no_mangle]
pub unsafe extern "C" fn exit(x: c_int) -> ! {
    for func in ATEXIT_FNS.iter().rev() {
        if let &Some(func) = func {
            func();
        }
    }
    _exit(x);
}

/// _Exit is a synonym for _exit
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn _Exit(x: c_int) -> ! {
    _exit(x);
}

#[no_mangle]
pub extern "C" fn _exit(x: c_int) -> ! {
    unsafe {
        sys_exit(x);
    }
    loop {} // for divergence check
}

#[no_mangle]
pub unsafe extern "C" fn abort() {
    raise(SIGABRT);
}


#[no_mangle]
/// Note: this doesn't check for a null argument, sparing a branch.
pub unsafe extern "C" fn atexit(func: Option<extern "C" fn()>) -> c_int {
    for i in &mut ATEXIT_FNS {
        if i.is_none() {
            *i = func;
            return 0;
        }
    }
    return 1;
}
