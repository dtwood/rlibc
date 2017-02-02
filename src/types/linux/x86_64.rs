#![allow(non_camel_case_types)]

use libc::*;

pub type __kernel_size_t    = c_ulong;
pub type __kernel_ssize_t   = c_long;
pub type __kernel_ptrdiff_t = c_long;

pub struct iocb {
    pub aio_data: u64,
    pub aio_key: u32,
    pub aio_reserved1: u32,
    pub aio_lio_opcode: u16,
    pub aio_reqprio: i16,
    pub aio_fildes: u32,
    pub aio_buf: u64,
    pub aio_nbytes: u64,
    pub aio_offset: i64,
    pub aio_reserved2: u64,
    pub aio_flags: u32,
    pub aio_resfd: u32,
}

pub struct pt_regs {
        pub r15: c_ulong,
        pub r14: c_ulong,
        pub r13: c_ulong,
        pub r12: c_ulong,
        pub bp: c_ulong,
        pub bx: c_ulong,
        pub r11: c_ulong,
        pub r10: c_ulong,
        pub r9: c_ulong,
        pub r8: c_ulong,
        pub ax: c_ulong,
        pub cx: c_ulong,
        pub dx: c_ulong,
        pub si: c_ulong,
        pub di: c_ulong,
        pub orig_ax: c_ulong,
        pub ip: c_ulong,
        pub cs: c_ulong,
        pub flags: c_ulong,
        pub sp: c_ulong,
        pub ss: c_ulong,
}

pub struct cap_user_data_t {
    pub effective: u32,
    pub permitted: u32,
    pub inheritable: u32,
}

pub struct cap_user_header_t {
    pub version: u32,
    pub pid: c_int,
}

pub type __statfs_word = c_long;

pub struct __kernel_fd_set {
    // XXX size_of
    pub fds_bits: [c_ulong; (FD_SETSIZE / (8 * 8))],
}

pub struct getcpu_cache {
    // XXX size_of
    pub blob: [c_ulong; 128/8],
}
