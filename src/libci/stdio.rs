use libc::{c_int, c_char, size_t};
use libc::EISDIR;
use libci::string::strlen;
use libci::errno::errno;
use posix::unistd::{unlink, rmdir};
use syscalls::{sys_rename, sys_write};

static _IOFBF: c_int = 0;
static _IOLBF: c_int = 1;
static _IONBF: c_int = 2;

pub struct FILE {
    #[allow(dead_code)]
    fd: c_int,
}

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static mut __stdin: FILE = FILE { fd: 0 };
#[no_mangle]
#[allow(non_upper_case_globals)]
pub static mut __stdout: FILE = FILE { fd: 1 };
#[no_mangle]
#[allow(non_upper_case_globals)]
pub static mut __stderr: FILE = FILE { fd: 2 };

#[no_mangle]
pub unsafe extern "C" fn remove(file: *const c_char) -> c_int {
    if unlink(file) == -1 {
        match errno {
            EISDIR => rmdir(file),
            _ => -1,
        }
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn rename(old: *const c_char, new: *const c_char) -> c_int {
    match sys_rename(old, new) {
        n if n < 0 => {
            errno = -n;
            -1
        }
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn puts(s: *const c_char) -> c_int {
    let len = strlen(s);
    if sys_write(1, s, len) as size_t != len ||
       sys_write(1, b"\n\0".as_ptr() as *const ::libc::c_char, 1) != 1 {
        -1
    } else {
        0
    }
}
