#![allow(unused_assignments)]

use libc::{c_int, c_char, c_uint, size_t, uid_t, gid_t, rlimit, off_t};
use libc::{timeval, timezone};
use libci::caddr_t;

/*
 * From opensource.apple.com/source/xnu/xnu-1699.26.8/osfmk/mach/i386/syscall_sw.h:
 * Syscall classes for 64-bit system call entry.
 * For 64-bit users, the 32-bit syscall number is partitioned
 * with the high-order bits representing the class and low-order
 * bits being the syscall number within that class.
 * The high-order 32-bits of the 64-bit syscall number are unused.
 * All system classes enter the kernel via the syscall instruction.
 *
 * These are not #ifdef'd for x86-64 because they might be used for
 * 32-bit someday and so the 64-bit comm page in a 32-bit kernel
 * can use them.
 *
 * shift = 24
 * none: 0
 * mach: 1
 * unix: 2
 * mdep: 3
 * diag: 4
 * ipc: 5
 * class_mask = 0xFF << 24
 * call = class << 24 & class_mask | id & !class_mask
 */

const CLASS_SHIFT: usize = 24;
const CLASS_MASK: c_int = 0xFF << CLASS_SHIFT;
const NUMBER_MASK: c_int = !CLASS_MASK;

macro_rules! syscall {
    ($class:expr, $id:expr, $name:ident) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name() -> c_int {
            let mut ret: c_int = $class << CLASS_SHIFT | NUMBER_MASK & $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($class:expr, $id:expr, $name:ident, $a:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a) -> c_int {
            let mut ret: c_int = $class << CLASS_SHIFT | NUMBER_MASK & $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($class:expr, $id:expr, $name:ident, $a:ty, $b:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a, b: $b) -> c_int {
            let mut ret: c_int = $class << CLASS_SHIFT | NUMBER_MASK & $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a), "{rsi}"(b) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($class:expr, $id:expr, $name:ident, $a:ty, $b:ty, $c:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c) -> c_int {
            let mut ret: c_int = $class << CLASS_SHIFT | NUMBER_MASK & $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a), "{rsi}"(b), "{rdx}"(c) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($class:expr, $id:expr, $name:ident, $a:ty, $b:ty, $c:ty, $d:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c, d: $d) -> c_int {
            let mut ret: c_int = $class << CLASS_SHIFT | NUMBER_MASK & $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a), "{rsi}"(b), "{rdx}"(c), "{r10}"(d) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($class:expr, $id:expr, $name:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c, d: $d, e: $e) -> c_int {
            let mut ret: c_int = $class << CLASS_SHIFT | NUMBER_MASK & $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a), "{rsi}"(b), "{rdx}"(c), "{r10}"(d) "{r8}"(e) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($class:expr, $id:expr, $name:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c, d: $d, e: $e, f:$f) -> c_int {
            let mut ret: c_int = $class << CLASS_SHIFT | NUMBER_MASK & $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a), "{rsi}"(b), "{rdx}"(c), "{r10}"(d) "{r8}"(e), "{r9}"(f) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
}

// UNIX / BSD system calls:
// assert sizeof(*mut) == user_addr_t
syscall!(2, 000, sys_nosys);
syscall!(2, 001, sys_exit, c_int); // -> void
syscall!(2, 002, sys_fork);
syscall!(2, 003, sys_read, c_uint, *mut c_char, size_t);
syscall!(2, 004, sys_write, c_uint, *const c_char, size_t); // c_uint/c_int ???
syscall!(2, 005, sys_open, *const c_char, c_int, c_int);
syscall!(2, 006, sys_close, c_uint);

syscall!(2, 010, sys_unlink, *const c_char);

syscall!(2, 020, sys_getpid);

syscall!(2, 023, sys_setuid, uid_t);
syscall!(2, 024, sys_getuid);
syscall!(2, 025, sys_geteuid);

syscall!(2, 037, sys_kill, c_int, c_int, c_int);

syscall!(2, 073, sys_munmap, caddr_t, size_t);

syscall!(2, 116, sys_gettimeofday, *mut timeval, *mut timezone);

syscall!(2, 128, sys_rename, *const c_char, *const c_char);

syscall!(2, 137, sys_rmdir, *const c_char);
syscall!(2, 138, sys_utimes, *const c_char, *mut timeval); // WARNING *mut c_char

syscall!(2, 147, sys_setsid);

syscall!(2, 153, sys_pread, c_int, *mut c_char, size_t, off_t);
syscall!(2, 154, sys_pwrite, c_int, *const c_char, size_t, off_t);

syscall!(2, 181, sys_setgid, gid_t);

syscall!(2, 193, sys_getrlimit, c_uint, *mut rlimit);
syscall!(2, 194, sys_setrlimit, c_uint, *mut rlimit);

syscall!(2,
         197,
         sys_mmap,
         caddr_t,
         size_t,
         c_int,
         c_int,
         c_int,
         off_t);

syscall!(2, 199, sys_lseek, c_int, off_t, c_int);
