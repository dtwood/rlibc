use core::slice::from_raw_parts;
use libc::{c_char, c_int, size_t};
#[cfg(target_os = "macos")]
use libci::string::{strlen, strncpy};
use libci::string::{strnlen, strncmp};
use posix::pm::_exit;

pub static mut ARGV: *const *const c_char = 0 as *const *const c_char;
pub static mut ARGC: usize = 0;
pub static mut ENVP: *const *const c_char = 0 as *const *const c_char;
pub static mut ENVC: usize = 0;

#[cfg(target_os = "macos")]
pub static mut APPLE: *const *const c_char = 0 as *const *const c_char;

const K_ENV_MAXKEYLEN: size_t = 512;

#[no_mangle]
pub unsafe extern "C" fn get_argv() -> &'static [*const c_char] {
    from_raw_parts(ARGV, ARGC)
}

#[no_mangle]
pub unsafe extern "C" fn get_envp() -> &'static [*const c_char] {
    from_raw_parts(ENVP, ENVC)
}

#[no_mangle]
#[cfg(target_os = "macos")]
#[allow(non_snake_case)]
pub unsafe extern "C" fn _NSGetArgc() -> *const c_int {
    (&ARGC) as *const usize as *const c_int
}

#[no_mangle]
#[cfg(target_os = "macos")]
#[allow(non_snake_case)]
pub unsafe extern "C" fn _NSGetArgv() -> *const *const *const c_char {
    (&ARGV) as *const *const *const c_char
}

#[no_mangle]
#[cfg(target_os = "macos")]
#[allow(non_snake_case)]
pub unsafe extern "C" fn _NSGetEnviron() -> *const *const *const c_char {
    (&ENVP) as *const *const *const c_char
}

#[no_mangle]
#[cfg(target_os = "macos")]
#[allow(non_snake_case)]
pub unsafe extern "C" fn _NSGetProgname() -> *const *const c_char {
    APPLE // apple[0] should point to the binary's path
}

#[no_mangle]
#[cfg(target_os = "macos")]
#[allow(non_snake_case)]
pub unsafe extern "C" fn _NSGetExecutablePath(buf: *mut c_char, size: *mut u32) -> c_int {
    let len = strlen(*APPLE);
    if len < *size as size_t {
        strncpy(buf, *APPLE, len);
        0
    } else {
        *size = len as u32;
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn getenv(key: *const c_char) -> *const c_char {
    let len = strnlen(key, K_ENV_MAXKEYLEN);
    for &env in get_envp().iter() {
        if strncmp(key, env, len) == 0 && *env.offset(len as isize) == '=' as i8 {
            return env.offset(len as isize + 1);
        }
    }
    0 as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn setenv(_key: *const c_char,
                                _val: *const c_char,
                                _overwrite: c_int)
                                -> c_int {
    _exit(1); // TODO implement mutable environment
}

#[no_mangle]
pub unsafe extern "C" fn unsetenv(_key: *const c_char) -> c_int {
    _exit(1); // TODO implement mutable environment
}

/*
#[no_mangle]
pub unsafe extern fn mkstemp(tplt: *mut c_char) -> c_int {
    let slc = tplt.to_mut_slice(strlen(tplt as *_) as usize);
    if slc.len() < 6 || slc.lastn(6).iter().any(|c| *c != cc!('X')) {
        errno = EINVAL;
        return -1;
    }
    let rand = os_rand();
    let mut buf: [c_char, ..6] = uninit();
    loop {
        rand.fill(buf);
        for (i, c) in slc.lastn(6).mut_iter().enumerate() {
            match buf[i] & 15 {
                0..9   => *c = buf[i] + cc!('0'),
                10..15 => *c = buf[i] + cc!('a') - 10,
            }
        }
        match open(tplt, ) {
            i if i >= 0          => return i,
            _ if errno != EEXIST => return -1,
            _ => { }
        }
    }
}
*/
