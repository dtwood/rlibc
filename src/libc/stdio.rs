use libc_types::{c_int, c_char, size_t};
use libc_types::EISDIR;
use libc::string::strlen;
use libc::errno::errno;

static _IOFBF: c_int = 0;
static _IOLBF: c_int = 1;
static _IONBF: c_int = 2;

pub struct FILE {
    #[allow(dead_code)]
    fd: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn remove(file: *const c_char) -> c_int {
    if forward!(UNLINK, file) == -1 {
        match errno {
            EISDIR => forward!(RMDIR, file),
            _ => -1,
        }
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn rename(old: *const c_char, new: *const c_char) -> c_int {
    forward!(RENAME, old, new)
}

#[no_mangle]
pub unsafe extern "C" fn puts(s: *const c_char) -> c_int {
    let len = strlen(s);
    if syscall!(WRITE, 1, s, len) as size_t != len ||
       syscall!(WRITE, 1, b"\n\0".as_ptr() as *const c_char, 1) != 1 {
        -1
    } else {
        0
    }
}
