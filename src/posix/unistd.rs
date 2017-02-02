use libc::{c_char, c_int, c_void, size_t, ssize_t, c_uint};
use libc::off_t;
use libc::{pid_t, uid_t, gid_t};
use syscalls::{sys_unlink, sys_rmdir, sys_read, sys_write, sys_close, sys_lseek};
use syscalls::{sys_getpid, sys_getuid, sys_geteuid, sys_setuid, sys_setgid, sys_setsid};

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use syscalls::{sys_pread64, sys_pwrite64};

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
use syscalls::{sys_pread, sys_pwrite};

use libci::errno::errno;

macro_rules! forward {
    ($sys:ident, $($p:expr),*) => {
        match $sys($($p),*) {
            n if n < 0 => {
                errno = -n;
                -1
            },
            n => n,
        }
    };
}

// File and filesystem manipulation

#[no_mangle]
pub unsafe extern "C" fn unlink(file: *const c_char) -> c_int {
    forward!(sys_unlink, file)
}

#[no_mangle]
pub unsafe extern "C" fn rmdir(file: *const c_char) -> c_int {
    forward!(sys_rmdir, file)
}

#[no_mangle]
pub unsafe extern "C" fn close(fd: c_int) -> c_int {
    forward!(sys_close, fd as c_uint)
}

#[no_mangle]
pub unsafe extern "C" fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    (forward!(sys_read, fd as c_uint, buf as *mut c_char, count)) as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    (forward!(sys_write, fd as c_uint, buf as *const c_char, count)) as ssize_t
}

#[no_mangle]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub unsafe extern "C" fn pread(fd: c_int,
                               buf: *mut c_void,
                               count: size_t,
                               offset: off_t)
                               -> ssize_t {
    (forward!(sys_pread64,
              fd as ulong_t,
              buf as *mut c_char,
              count,
              offset) as ssize_t)
}
#[no_mangle]
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub unsafe extern "C" fn pread(fd: c_int,
                               buf: *mut c_void,
                               count: size_t,
                               offset: off_t)
                               -> ssize_t {
    (forward!(sys_pread, fd, buf as *mut c_char, count, offset) as ssize_t)
}

#[no_mangle]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub unsafe extern "C" fn pwrite(fd: c_int,
                                buf: *const c_void,
                                count: size_t,
                                offset: off_t)
                                -> ssize_t {
    (forward!(sys_pwrite64,
              fd as c_uint,
              buf as *const c_char,
              count,
              offset) as ssize_t)
}
#[no_mangle]
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub unsafe extern "C" fn pwrite(fd: c_int,
                                buf: *const c_void,
                                count: size_t,
                                offset: off_t)
                                -> ssize_t {
    (forward!(sys_pwrite, fd, buf as *const c_char, count, offset) as ssize_t)
}

#[no_mangle]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub unsafe extern "C" fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    (forward!(sys_lseek, fd as c_uint, offset, whence as c_uint) as off_t)
}
#[no_mangle]
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub unsafe extern "C" fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    (forward!(sys_lseek, fd, offset, whence) as off_t)
}


// Environment

#[no_mangle]
pub unsafe extern "C" fn getpid() -> pid_t {
    (forward!(sys_getpid,) as pid_t)
}

#[no_mangle]
pub unsafe extern "C" fn getuid() -> uid_t {
    (forward!(sys_getuid,) as uid_t)
}

#[no_mangle]
pub unsafe extern "C" fn geteuid() -> uid_t {
    (forward!(sys_geteuid,) as uid_t)
}

#[no_mangle]
pub unsafe extern "C" fn setuid(uid: uid_t) -> c_int {
    forward!(sys_setuid, uid)
}

#[no_mangle]
pub unsafe extern "C" fn setgid(gid: gid_t) -> c_int {
    forward!(sys_setgid, gid)
}

#[no_mangle]
pub unsafe extern "C" fn setsid() -> pid_t {
    (forward!(sys_setsid,) as pid_t)
}
