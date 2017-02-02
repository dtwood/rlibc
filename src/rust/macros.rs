#[macro_export]
macro_rules! cc {
    ($e:expr) => {
        $e as char_t
    }
}

macro_rules! forward {
    ($sys:ident, $($p:expr),*) => {
        match $sys($($p),*) {
            n if n < 0 => {
                use libci::errno::errno;
                errno = -n;
                -1
            },
            n => n,
        }
    };
}
