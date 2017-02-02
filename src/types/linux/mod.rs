#![allow(non_camel_case_types)]

use libc::*;

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

pub type __kernel_key_t       = c_int;
pub type __kernel_c_long      = c_long;
pub type __kernel_c_ulong     = c_ulong;
pub type __kernel_ino_t       = __kernel_c_ulong;
pub type __kernel_mode_t      = c_uint;
pub type __kernel_pid_t       = c_int;
pub type __kernel_ipc_pid_t   = c_int;
pub type __kernel_uid_t       = c_uint;
pub type __kernel_gid_t       = c_uint;
pub type __kernel_suseconds_t = __kernel_c_long;
pub type __kernel_daddr_t     = c_int;
pub type __kernel_uid32_t     = c_uint;
pub type __kernel_gid32_t     = c_uint;
pub type __kernel_old_uid32_t = __kernel_uid_t;
pub type __kernel_old_gid32_t = __kernel_gid_t;
pub type __kernel_old_dev_t   = c_uint;
pub struct __kernel_fsid_t {
    pub val: [c_int; 2],
}
pub type __kernel_off_t       = __kernel_c_long;
pub type __kernel_loff_t      = c_longlong;
pub type __kernel_time_t      = __kernel_c_long;
pub type __kernel_clock_t     = __kernel_c_long;
pub type __kernel_timer_t     = c_int;
pub type __kernel_clockid_t   = c_int;
pub type __kernel_caddr_t     = *mut c_char;
pub type __kernel_uid16_t     = c_ushort;
pub type __kernel_gid16_t     = c_ushort;

pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: __kernel_size_t,
}

pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: c_int,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: __kernel_size_t,
    pub msg_control: *mut c_void,
    pub msg_controllen: __kernel_size_t,
    pub msg_flags: c_uint,
}

pub struct mmsghdr {
    pub msg_hdr: msghdr,
    pub msg_len: c_uint,
}

pub struct timex {
    pub modes: c_uint, /* mode selector */
    pub offest: __kernel_c_long, /* time offset (usec) */
    pub freq: __kernel_c_long,  /* frequency offset (scaled ppm) */
    pub maxerror: __kernel_c_long, /* maximum error (usec) */
    pub esterror: __kernel_c_long, /* estimated error (usec) */
    pub status: c_int, /* clock command/status */
    pub constant: __kernel_c_long, /* pll time constant */
    pub precision: __kernel_c_long, /* clock precision (usec) (read only) */
    pub tolerance: __kernel_c_long, /* clock frequency tolerance (ppm)
    pub                            * (read only)
    pub                            */
    pub time: timeval, /* (read only, except for ADJ_SETOFFSET) */
    pub tick: __kernel_c_long, /* (modified) usecs between clock ticks */

    pub ppsfreq: __kernel_c_long, /* pps frequency (scaled ppm) (ro) */
    pub jitter: __kernel_c_long, /* pps jitter (us) (ro) */
    pub shift: c_int, /* interval duration (s) (shift) (ro) */
    pub stabil: __kernel_c_long, /* pps stability (scaled ppm) (ro) */
    pub jitcnt: __kernel_c_long, /* jitter limit exceeded (ro) */
    pub calcnt: __kernel_c_long, /* calibration intervals (ro) */
    pub errcnt: __kernel_c_long, /* calibration errors (ro) */
    pub stbcnt: __kernel_c_long, /* stability limit exceeded (ro) */

    pub tai: c_int, /* TAI offset (ro) */

    pub _____padding: [i32; 11],
}

pub struct timeval {
    pub tv_sec: __kernel_time_t,
    pub tv_usec: __kernel_suseconds_t,
}

pub struct file_handle {
    pub handle_bytes: u32,
    pub handle_type: c_int,
    pub f_handle: [c_uchar; 0],
}

pub struct timespec {
    pub tv_sec: __kernel_time_t,
    pub tv_nsec: c_long,
}

pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}

pub struct robust_list_head {
    pub list: robust_list,
    pub futex_offset: c_long,
    pub list_op_pending: *mut robust_list,
}

pub struct robust_list {
    pub next: *mut robust_list,
}

pub type key_serial_t = i32;

pub struct kexec_segment {
    pub buf: *const c_void,
    pub bufsz: size_t,
    pub mem: *const c_void,
    pub memsz: size_t,
}

pub type timer_t = __kernel_timer_t;

pub struct sembuf {
    #[allow(dead_code)]
    sem_num: c_ushort,
    #[allow(dead_code)]
    sem_op: c_short,
    #[allow(dead_code)]
    sem_flg: c_short,
}

pub struct linux_dirent64 {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: c_ushort,
    pub d_type: c_uchar,
    pub d_name: [c_char; 0],
}

pub struct io_event {
    pub data: u64,
    pub obj: u64,
    pub res: i64,
    pub res2: i64,
}

pub type aio_context_t = __kernel_c_ulong;
pub type qid_t = __kernel_uid32_t;

pub struct __sysctl_args {
    pub name: *mut c_int,
    pub nlen: c_int,
    pub oldval: *mut c_void,
    pub oldlenp: *mut size_t,
    pub newval: *mut c_void,
    pub newlen: size_t,
    pub __unused: [c_ulong; 4],
}

pub struct ustat {
    pub f_tfree: __kernel_daddr_t,
    pub f_tinode: __kernel_ino_t,
    pub f_fname: [c_char; 6],
    pub f_fpack: [c_char; 6],
}

pub struct linux_dirent {
    pub d_ino: c_ulong,
    pub d_off: c_ulong,
    pub d_reclen: c_ushort,
    pub d_name: [c_char; 1],
}

pub struct msgbuf {
    pub mtype: __kernel_c_long,
    pub mtext: [c_char; 1],
}

pub struct old_utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
}

pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}

pub type __sigrestore_t = *mut __restorefn_t;
pub type __restorefn_t = fn();

pub type __sighandler_t = *mut __signalfn_t;
pub type __signalfn_t = fn(c_int);


pub type __kernel_mqd_t = c_int;
pub type __kernel_sa_family_t = c_ushort;
