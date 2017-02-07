//! File access and modification

use libc_types::{c_char, c_int, timeval, utimbuf};

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
        syscall!(UTIMES, path, tv.as_mut_ptr()) as c_int
    } else {
        syscall!(UTIMES, path, 0 as *mut timeval) as c_int
    }
}
