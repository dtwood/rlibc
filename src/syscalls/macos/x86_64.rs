use types::*;

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

static CLASS_SHIFT: uint = 24;
static CLASS_MASK: int_t = 0xFF << CLASS_SHIFT;
static NUMBER_MASK: int_t = !CLASS_MASK;

macro_rules! syscall {
    ($class:expr, $id:expr, $name:ident) => {
        #[inline(always)]
        #[no_mangle]
        #[no_split_stack]
        pub unsafe extern fn $name() -> int_t {
            let mut ret: int_t = $class << CLASS_SHIFT | NUMBER_MASK & $id;
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
        #[no_split_stack]
        pub unsafe extern fn $name(a: $a) -> int_t {
            let mut ret: int_t = $class << CLASS_SHIFT | NUMBER_MASK & $id;
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
        #[no_split_stack]
        pub unsafe extern fn $name(a: $a, b: $b) -> int_t {
            let mut ret: int_t = $class << CLASS_SHIFT | NUMBER_MASK & $id;
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
        #[no_split_stack]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c) -> int_t {
            let mut ret: int_t = $class << CLASS_SHIFT | NUMBER_MASK & $id;
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
        #[no_split_stack]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c, d: $d) -> int_t {
            let mut ret: int_t = $class << CLASS_SHIFT | NUMBER_MASK & $id;
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
        #[no_split_stack]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c, d: $d, e: $e) -> int_t {
            let mut ret: int_t = $class << CLASS_SHIFT | NUMBER_MASK & $id;
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
        #[no_split_stack]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c, d: $d, e: $e, f:$f) -> int_t {
            let mut ret: int_t = $class << CLASS_SHIFT | NUMBER_MASK & $id;
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
syscall!(2, 000, sys_nosys)
syscall!(2, 001, sys_exit, int_t) // -> void
syscall!(2, 002, sys_fork)
syscall!(2, 003, sys_read, uint_t, *mut char_t, size_t) // sizeof(*mut) == 8
syscall!(2, 004, sys_write, uint_t, *const char_t, size_t) // uint_t/int_t ???
syscall!(2, 005, sys_open, *const char_t, int_t, int_t)
syscall!(2, 006, sys_close, uint_t)

syscall!(2, 010, sys_unlink, *const char_t)

syscall!(2, 128, sys_rename, *const char_t, *const char_t)

syscall!(2, 137, sys_rmdir, *const char_t)