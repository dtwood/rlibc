//! File access and modification

use libc::{c_char, c_int, timeval, utimbuf};
use syscalls::sys_utimes;

/// Change file last access and modification times.
#[no_mangle]
pub unsafe extern "C" fn utime(path: *const c_char, times: *const utimbuf) -> c_int {
    if times.is_null() {
        let mut tv = [timeval {
                          tv_sec: (*times).actime,
                          tv_usec: 0,
                      },
                      timeval {
                          tv_sec: (*times).modtime,
                          tv_usec: 0,
                      }];
        forward!(sys_utimes, path, tv.as_mut_ptr())
    } else {
        forward!(sys_utimes, path, 0 as *mut timeval)
    }
}
