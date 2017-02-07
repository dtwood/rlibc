#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
use libc::c_void;

pub mod ctype;
pub mod errno;
pub mod stdio;
pub mod string;
pub mod time;
pub mod undef;

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
#[allow(non_camel_case_types)]
pub type caddr_t = *mut c_void;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use libc::{c_int, c_uchar, timeval};

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[repr(C)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[repr(C)]
pub struct file_handle {
    pub handle_bytes: u32,
    pub handle_type: c_int,
    pub f_handle: [c_uchar; 0],
}
