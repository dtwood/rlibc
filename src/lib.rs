#![feature(asm, intrinsics, core_intrinsics, naked_functions, const_fn, unwind_attributes, lang_items)]
#![recursion_limit="128"]

extern crate libc as libc_types;
extern crate rlibc;
#[macro_use]
extern crate syscall;

#[macro_use]
mod macros;

pub mod startup;
pub mod libc;
pub mod posix;
pub mod math;
