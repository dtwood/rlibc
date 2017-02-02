use libc::{c_int, c_char};
use posix::stdlib::{ARGV, ARGC, ENVP, ENVC};
use posix::pm::exit;

/// This function is called by start().
/// It stores the addresses of the stack arguments, invokes main(), and passes
/// the return status to exit().
#[no_mangle]
pub unsafe extern "C" fn __libc_start_main(main: fn(c_int, *const *const c_char, *const *const c_char) -> c_int, argc: usize, argv: *const *const c_char) {
    ARGC = argc;
    ARGV = argv;
    ENVP = ARGV.offset(ARGC as isize + 1);

    let mut envc: *const *const c_char = ENVP;
    while *envc as usize != 0 {
        envc = envc.offset(1); // increases by one pointer size
    }
    ENVC = (envc as usize - ENVP as usize) / ::core::mem::size_of::<*const *const c_char>() - 1;

    exit(main(ARGC as c_int, ARGV, ENVP));
}
