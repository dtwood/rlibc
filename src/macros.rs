#[macro_export]
macro_rules! forward {
    ($sys: ident) => { forward!($sys,) };
    ($sys: ident, $($p: expr),*) => {
        match syscall!($sys $(, $p)*) as i32 {
            n if n < 0 => {
                errno = -n;
                -1
            },
            n => n,
        }
    };
    ($sys: ident, $($p: expr,)*) => { forward!($sys, $($p),*) };
}

macro_rules! undef_helper {
    ($name: ident) => {
        const ERROR_TEXT: &'static str = "\n\nUnimplemented stdlib function: `";
        let function_name: &'static str = stringify!($name);
        const ERROR_TEXT_END: &'static str = "`, exiting.\n";

        ::posix::unistd::write(2, ERROR_TEXT.as_ptr() as _, ERROR_TEXT.len());
        ::posix::unistd::write(2, function_name.as_ptr() as _, function_name.len());
        ::posix::unistd::write(2, ERROR_TEXT_END.as_ptr() as _, ERROR_TEXT_END.len());

        ::posix::signal::raise(::libc_types::SIGTRAP);
        ::posix::pm::exit(1);
    }
}

#[macro_export]
macro_rules! undef {
    ($name: ident) => {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn $name() {
            undef_helper!($name);
        }
    };

    ($name: ident = $mangled_name: expr) => {
        #[export_name=$mangled_name]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn $name() {
            undef_helper!($name);
        }
    };
}
