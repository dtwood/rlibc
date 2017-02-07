use libc_types::{c_int, c_char};
use posix::pm::exit;
use posix::stdlib::{ARGV, ARGC, ENVP, ENVC};

extern "C" {
    fn main(argc: c_int,
            argv: *const *const c_char,
            envp: *const *const c_char)
            -> c_int;
}

#[no_mangle]
#[export_name="_start"]
pub unsafe extern "C" fn start(argc: usize, argv: *const *const c_char) {
    asm!("xor %rbp, %rbp");
    asm!("pop %rdi");
    asm!("mov %rsp, %rsi");
    asm!("andq $$~0xff, %rsp");

    ARGC = argc;
    ARGV = argv;
    ENVP = ARGV.offset(ARGC as isize + 1);

    let mut envc: *const *const c_char = ENVP;
    while *envc as usize != 0 {
        envc = envc.offset(1); // increases by one pointer size
    }
    ENVC = (envc as usize - ENVP as usize) / ::std::mem::size_of::<*const *const c_char>() - 1;

    exit(main(ARGC as c_int, ARGV, ENVP));
}
