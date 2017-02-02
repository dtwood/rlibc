//! Memory management

use libc::{c_int, c_uint, c_void, size_t};
use libc::off_t;
use libc::rlimit;
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
use libci::caddr_t;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use libc::intptr_t;


use syscalls::{sys_getrlimit, sys_mmap, sys_munmap};
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use syscalls::sys_brk;

/// Increases the data break to the given address, returning 0 on success
/// or -1 on failure, setting errno to ENOMEM.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern "C" fn brk(addr: *const c_void) -> c_int {
    let oldbrk = sys_brk(0) as usize;
    match sys_brk(addr as c_ulong) as usize != oldbrk {
        true => 0,
        false => {
            errno = ENOMEM;
            -1
        }
    }
}

/// Increments the data break by `increment`, returning either the previous
/// break or `((void*)-1)` on failure, setting errno to ENOMEM.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern "C" fn sbrk(increment: intptr_t) -> *const c_void {
    let oldbrk = sys_brk(0) as *const u8;
    if increment != 0 {
        let newbrk = offset(oldbrk, increment as isize);
        if sys_brk(newbrk as c_ulong) as *const u8 != newbrk {
            errno = ENOMEM;
            -1 as *const c_void
        } else {
            oldbrk as *const c_void
        }
    } else {
        oldbrk as *const c_void
    }
}

/// Get resource limits. For RLIMIT_DATA, this is the maximum size of the
/// process's data segment. This limit affects calls to brk(2) and sbrk(2).
#[no_mangle]
pub unsafe extern "C" fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int {
    (forward!(sys_getrlimit, resource as c_uint, rlim))
}

/// Map or unmap files or devices into memory.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern "C" fn mmap(addr: *const c_void,
                              length: size_t,
                              prot: c_int,
                              flags: c_int,
                              fd: c_int,
                              offset: off_t)
                              -> *const c_void {
    forward!(sys_mmap,
             addr as c_ulong,
             length as c_ulong,
             prot as c_ulong,
             flags as c_ulong,
             fd as c_ulong,
             offset as c_ulong) as *const c_void
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern "C" fn mmap(addr: *const c_void,
                              length: size_t,
                              prot: c_int,
                              flags: c_int,
                              fd: c_int,
                              offset: off_t)
                              -> *const c_void {
    forward!(sys_mmap, addr as *mut _, length, prot, flags, fd, offset) as *const c_void
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern "C" fn munmap(addr: *const c_void, length: size_t) -> c_int {
    forward!(sys_munmap, addr as c_ulong, length)
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern "C" fn munmap(addr: *const c_void, length: size_t) -> c_int {
    forward!(sys_munmap, addr as caddr_t, length)
}
