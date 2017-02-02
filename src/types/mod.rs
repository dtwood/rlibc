#[cfg(any(target_os = "linux", target_os = "android"))]
#[path = "linux/mod.rs"]
mod linux;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use self::linux::*;
