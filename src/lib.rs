#![crate_name="c"]
#![crate_type="staticlib"]
#![feature(asm, lang_items, intrinsics, core_intrinsics)]
#![no_std]

extern crate libc;
extern crate rlibc;

#[cfg(any(all(target_os = "linux", target_arch = "x86_64"),
		  all(target_os = "android", target_arch = "x86_64")))]
pub use rust::x86_64::linux::start::__libc_start_main;

#[cfg(any(all(target_os = "macos", target_arch = "x86_64"),
		  all(target_os = "ios", target_arch = "x86_64")))]
pub use rust::x86_64::macos::start::_libc_start_main;

#[macro_use]
mod rust;

mod types;
pub mod libci;
pub mod posix;
pub mod math;
pub mod syscalls;
