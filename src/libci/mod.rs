use libc::c_void;

pub mod ctype;
pub mod errno;
pub mod stdio;
pub mod string;
pub mod time;

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
#[allow(non_camel_case_types)]
pub type caddr_t = *mut c_void;
