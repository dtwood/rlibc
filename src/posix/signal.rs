use libc_types::{c_int, c_void, pid_t};
use libc::errno::errno;
use posix::unistd::getpid;

#[allow(non_camel_case_types)]
pub type sighandler_t = fn(c_int);

#[allow(non_camel_case_types)]
type __sa_handler_t = fn(c_int);
#[allow(non_camel_case_types)]
type __sa_sigaction = fn(c_int, *mut c_void, *mut c_void);
#[allow(non_camel_case_types)]
type __sigaction_u_t = *mut c_void;


/// Generates a signal
/// Sends signal `sig` to the current executing program.
/// The signal is handled as specified by function `signal`.
#[no_mangle]
pub unsafe extern "C" fn raise(sig: c_int) -> c_int {
    kill(getpid(), sig)
}

/// Set function to handle signal.
/// Specifies a way to handle the signals with the signal number specified by sig.
#[no_mangle]
pub unsafe extern "C" fn signal(_sig: c_int, _func: sighandler_t) -> sighandler_t {
    ::std::mem::transmute(0usize)
}

/// Send a signal to a process or a group of processes.
#[no_mangle]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub unsafe extern "C" fn kill(pid: pid_t, sig: c_int) -> c_int {
    forward!(KILL, pid, sig)
}
#[no_mangle]
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub unsafe extern "C" fn kill(pid: pid_t, sig: c_int) -> c_int {
    forward!(KILL, pid as c_int, sig, 0)
}
