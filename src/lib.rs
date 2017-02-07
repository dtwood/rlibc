#![feature(asm, intrinsics, core_intrinsics, naked_functions, const_fn, unwind_attributes, lang_items)]
#![recursion_limit="128"]

extern crate libc;
extern crate rlibc;
#[macro_use]
extern crate syscall;
extern crate spin;

mod types;
#[macro_use]
mod macros;

pub mod startup;
pub mod libci;
pub mod posix;
pub mod math;

pub use libci::ctype::*;
pub use libci::errno::*;
pub use libci::stdio::*;
pub use libci::string::*;
pub use libci::time::*;
