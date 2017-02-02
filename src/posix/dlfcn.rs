//! Dynamic linking or loading is evil and thus not supported.

use libc::{c_char, c_int, c_void};
use posix::pm::exit;

#[no_mangle]
pub unsafe fn dlopen(_filename: *const c_char, _flag: c_int) -> *const c_void {
    exit(1);
}

#[no_mangle]
pub unsafe fn dlerror() -> *const c_char {
    exit(1);
}

#[no_mangle]
pub unsafe fn dlsym(_handle: *const c_void, _symbol: *const c_char) -> *const c_void {
    exit(1);
}

#[no_mangle]
pub unsafe fn dlclose(_handle: *const c_void) -> c_int {
    exit(1);
}

#[no_mangle]
pub unsafe fn dladdr(_addr: *const c_void, _info: *const c_void) -> c_int {
    exit(1);
}
