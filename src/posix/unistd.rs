use libc_types::{c_char, c_int, c_void, size_t, ssize_t, c_uint, off_t, pid_t, uid_t, gid_t};
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use libc_types::c_ulong;

use libc::errno::errno;

// File and filesystem manipulation

#[no_mangle]
pub unsafe extern "C" fn unlink(file: *const c_char) -> c_int {
    forward!(UNLINK, file)
}

#[no_mangle]
pub unsafe extern "C" fn rmdir(file: *const c_char) -> c_int {
    forward!(RMDIR, file)
}

#[no_mangle]
pub unsafe extern "C" fn close(fd: c_int) -> c_int {
    forward!(CLOSE, fd as c_uint)
}

#[no_mangle]
pub unsafe extern "C" fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    forward!(READ, fd as c_uint, buf as *mut c_char, count) as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    forward!(WRITE, fd as c_uint, buf as *const c_char, count) as ssize_t
}

#[no_mangle]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub unsafe extern "C" fn pread(fd: c_int,
                               buf: *mut c_void,
                               count: size_t,
                               offset: off_t)
                               -> ssize_t {
    forward!(PREAD64,
             fd as c_ulong,
             buf as *mut c_char,
             count,
             offset) as ssize_t
}
#[no_mangle]
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub unsafe extern "C" fn pread(fd: c_int,
                               buf: *mut c_void,
                               count: size_t,
                               offset: off_t)
                               -> ssize_t {
    forward!(PREAD, fd, buf as *mut c_char, count, offset) as ssize_t
}

#[no_mangle]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub unsafe extern "C" fn pwrite(fd: c_int,
                                buf: *const c_void,
                                count: size_t,
                                offset: off_t)
                                -> ssize_t {
    forward!(PWRITE64,
             fd as c_uint,
             buf as *const c_char,
             count,
             offset) as ssize_t
}
#[no_mangle]
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub unsafe extern "C" fn pwrite(fd: c_int,
                                buf: *const c_void,
                                count: size_t,
                                offset: off_t)
                                -> ssize_t {
    forward!(PWRITE, fd, buf as *const c_char, count, offset) as ssize_t
}

#[no_mangle]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub unsafe extern "C" fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    forward!(LSEEK, fd as c_uint, offset, whence as c_uint) as off_t
}
#[no_mangle]
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub unsafe extern "C" fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    forward!(LSEEK, fd, offset, whence) as off_t
}


// Environment

#[no_mangle]
pub unsafe extern "C" fn getpid() -> pid_t {
    forward!(GETPID) as pid_t
}

#[no_mangle]
pub unsafe extern "C" fn getuid() -> uid_t {
    forward!(GETUID) as uid_t
}

#[no_mangle]
pub unsafe extern "C" fn geteuid() -> uid_t {
    forward!(GETEUID) as uid_t
}

#[no_mangle]
pub unsafe extern "C" fn setuid(uid: uid_t) -> c_int {
    forward!(SETUID, uid)
}

#[no_mangle]
pub unsafe extern "C" fn setgid(gid: gid_t) -> c_int {
    forward!(SETGID, gid)
}

#[no_mangle]
pub unsafe extern "C" fn setsid() -> pid_t {
    forward!(SETSID) as pid_t
}
