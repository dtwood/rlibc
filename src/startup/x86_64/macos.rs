use libc_types::{c_int, c_char};
use posix::pm::exit;
use posix::stdlib::{ARGV, ARGC, ENVP, ENVC, APPLE};

extern "C" {
    fn main(argc: c_int,
            argv: *const *const c_char,
            envp: *const *const c_char,
            apple: *const *const c_char)
            -> c_int;
}

#[naked]
#[export_name="\x01start"]
pub unsafe extern "C" fn start(argc: usize, argv: *const *const c_char) {
    asm!("xor %rbp, %rbp");
    asm!("pop %rdi");
    asm!("mov %rsp, %rsi");
    asm!("andq $$~0xff, %rsp");

    ARGC = argc;
    ARGV = argv;
    ENVP = ARGV.offset(ARGC as isize + 1);

    let mut apple: *const *const c_char = ENVP;
    while *apple as usize != 0 {
        apple = apple.offset(1); // increases by one pointer size
    }
    ENVC = apple as usize - ENVP as usize - 1;
    apple = apple.offset(1); // one NULL pointer separates apple[] from env[]
    APPLE = apple;

    let status = main(ARGC as c_int, ARGV, ENVP, apple);
    exit(status);
}
