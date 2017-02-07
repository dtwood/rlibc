use libc::{c_int, c_char, size_t};
use libc::EISDIR;
use libci::string::strlen;
use libci::errno::errno;
use core::fmt::{self, Write};
use spin::Mutex;

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
       syscall!(WRITE, 1, b"\n\0".as_ptr() as *const ::libc::c_char, 1) != 1 {
        -1
    } else {
        0
    }
}

struct FileWrapper(usize);

static STDIO: Mutex<FileWrapper> = Mutex::new(FileWrapper(1));

impl Write for FileWrapper {
    fn write_str(&mut self, _s: &str) -> fmt::Result {
        match unsafe { syscall!(WRITE, self.0, _s.as_ptr(), _s.len()) } {
            0 => Ok(()),
            _ => panic!(),
        }
    }
}

macro_rules! print (
    ($($x: expr),*) => { write!(STDIO.lock(), $($x),*).unwrap(); }
);

#[no_mangle]
pub extern "C" fn printf(_format: *const c_char) -> c_int {
    print!("{} {}", "asdf", 1);
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fprintf(_f: *mut FILE, _format: *const c_char)
 -> c_int { unimplemented!() }

#[no_mangle]
pub extern "C" fn sprintf(_s: *mut c_char,
               _format: *const c_char)
 -> c_int { unimplemented!() }

#[no_mangle]
pub extern "C" fn snprintf(_s: *mut c_char, _n: size_t,
                _format: *const c_char)
 -> c_int { unimplemented!() }
