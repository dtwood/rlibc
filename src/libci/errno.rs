use libc::c_int;

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static mut errno: c_int = 0;
