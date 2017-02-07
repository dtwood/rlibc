use libc_types::c_int;

#[no_mangle]
pub extern "C" fn isalnum(c: c_int) -> c_int {
    match c as u8 as char {
        'a'...'z' |
        'A'...'Z' |
        '0'...'9' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn isalpha(c: c_int) -> c_int {
    match c as u8 as char {
        'a'...'z' |
        'A'...'Z' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn isblank(c: c_int) -> c_int {
    match c as u8 as char {
        ' ' | '\t' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn iscntrl(c: c_int) -> c_int {
    match c as u8 as char {
        '\x00'...'\x19' |
        '\x7f' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn isdigit(c: c_int) -> c_int {
    match c as u8 as char {
        '0'...'9' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn isgraph(c: c_int) -> c_int {
    match c {
        0x21...0x7e => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn islower(c: c_int) -> c_int {
    match c as u8 as char {
        'a'...'z' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn isprint(c: c_int) -> c_int {
    match c {
        0x20...0x7e => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn ispunct(c: c_int) -> c_int {
    ((isgraph(c) != 0) && (isalpha(c) == 0)) as c_int
}

#[no_mangle]
pub extern "C" fn isspace(c: c_int) -> c_int {
    match c as u8 as char {
        ' ' | '\t' | '\n' | '\x0b' | '\x0c' | '\r' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn isupper(c: c_int) -> c_int {
    match c as u8 as char {
        'A'...'Z' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn isxdigit(c: c_int) -> c_int {
    match c as u8 as char {
        '0'...'9' |
        'A'...'F' |
        'a'...'f' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn tolower(c: c_int) -> c_int {
    match c as u8 as char {
        'A'...'Z' => c + 0x20,
        _ => c,
    }
}

#[no_mangle]
pub extern "C" fn toupper(c: c_int) -> c_int {
    match c as u8 as char {
        'a'...'z' => c - 0x20,
        _ => c,
    }
}
