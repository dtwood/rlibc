//! Memory management

use libc::{c_int, c_void, size_t};
use libc::off_t;
use libc::rlimit;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use libc::{c_ulong, intptr_t};
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use libci::errno::errno;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use libc::ENOMEM;

/// Increases the data break to the given address, returning 0 on success
/// or -1 on failure, setting errno to ENOMEM.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern "C" fn brk(addr: *const c_void) -> c_int {
    let oldbrk = syscall!(BRK, 0);
    match syscall!(BRK, addr) != oldbrk {
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
    let oldbrk = syscall!(BRK, 0) as *const u8;
    if increment != 0 {
        let newbrk = oldbrk.offset(increment as isize);
        if syscall!(BRK, newbrk) as *const u8 != newbrk {
            errno = ENOMEM;
            !0 as *const c_void
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
    syscall!(GETRLIMIT, resource, rlim) as c_int
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
    syscall!(MMAP,
             addr,
             length,
             prot,
             flags,
             fd,
             offset) as *const c_void
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
    syscall!(MMAP, addr, length, prot, flags, fd, offset) as *const c_void
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern "C" fn munmap(addr: *const c_void, length: size_t) -> c_int {
    syscall!(MUNMAP, addr, length) as c_int
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern "C" fn munmap(addr: *const c_void, length: size_t) -> c_int {
    syscall!(MUNMAP, addr, length) as c_int
}
