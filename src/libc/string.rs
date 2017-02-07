use libc_types::{c_uchar, c_char, c_int, size_t};

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char {
    for i in (0..n as isize).rev() {
        *dest.offset(i) = *src.offset(i);
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char {
    if (dest as usize) > (src as usize) {
        return memcpy(dest, src, n);
    }
    for i in 0..n as isize {
        *dest.offset(i) = *src.offset(i);
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char {
    let mut i = 0;
    while *src.offset(i) != 0 {
        *dest.offset(i) = *src.offset(i);
        i += 1;
    }
    *dest.offset(i) = 0;
    dest
}

#[no_mangle]
pub unsafe extern "C" fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char {
    let n = n as isize;
    let mut i = 0;
    while i < n && *src.offset(i) != 0 {
        *dest.offset(i) = *src.offset(i);
        i += 1;
    }
    while i < n {
        *dest.offset(i) = 0;
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char {
    let base = strlen(dest as *const _) as isize;
    let mut i = 0;
    while *src.offset(i) != 0 {
        *dest.offset(base + i) = *src.offset(i);
        i += 1;
    }
    *dest.offset(base + i) = 0;
    dest
}

#[no_mangle]
pub unsafe extern "C" fn strncat(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char {
    let base = strlen(dest as *const _) as isize;
    for i in 0..n as isize {
        *dest.offset(base + i) = *src.offset(i);
        if *src.offset(i) == 0 {
            break;
        }
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(m1: *const c_char, m2: *const c_char, n: size_t) -> c_int {
    let m1 = m1 as *const c_uchar;
    let m2 = m2 as *const c_uchar;
    for i in 0..n as isize {
        let v1 = *m1.offset(i) as isize;
        let v2 = *m2.offset(i) as isize;
        match v1 - v2 {
            j if j < 0 => return -1,
            j if j > 0 => return 1,
            _ => {}
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn strcmp(m1: *const c_char, m2: *const c_char) -> c_int {
    let mut m1 = m1; // as *const c_uchar;
    let mut m2 = m2; // as *const c_uchar;
    loop {
        let v1 = *m1;
        let v2 = *m2;

        let diff = v1 - v2;
        if diff != 0 {
            return diff.into();
        }
        if v1 == 0 && v2 == 0 {
            return 0;
        }

        m1 = m1.offset(1);
        m2 = m2.offset(2);
    }
}

#[no_mangle]
pub unsafe extern "C" fn strcoll(m1: *const c_char, m2: *const c_char) -> c_int {
    strcmp(m1, m2)
}

#[no_mangle]
pub unsafe extern "C" fn strncmp(m1: *const c_char, m2: *const c_char, n: size_t) -> c_int {
    let m1 = m1 as *const c_uchar;
    let m2 = m2 as *const c_uchar;
    for i in 0..n as isize {
        let v1 = *m1.offset(i) as isize;
        let v2 = *m2.offset(i) as isize;
        match v1 - v2 {
            j if j < 0 => return -1,
            j if j > 0 => return 1,
            _ => {}
        }
        if v1 == 0 {
            break;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn strxfrm(dest: *mut c_char, src: *const c_char, n: size_t) -> size_t {
    let len = strlen(src);
    if len < n {
        memcpy(dest, src, len + 1);
    }
    len
}

#[no_mangle]
pub unsafe extern "C" fn memchr(s: *const c_char, c: c_int, n: size_t) -> *const c_char {
    let c = c as c_char;
    for i in 0..n as isize {
        if *s.offset(i) == c {
            return s.offset(i);
        }
    }
    0 as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn strchr(s: *const c_char, c: c_int) -> *const c_char {
    if c == 0 {
        return s.offset(strlen(s) as isize);
    }
    let c = c as c_char;
    let mut i = 0;
    while *s.offset(i) != 0 {
        if *s.offset(i) == c {
            return s.offset(i);
        }
        i += 1;
    }
    0 as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn strcspn(s1: *const c_char, s2: *const c_char) -> size_t {
    let len = strlen(s2);
    let mut i = 0;
    while *s1.offset(i) != 0 {
        if memchr(s2, *s1.offset(i) as c_int, len) as usize != 0 {
            break;
        }
        i += 1;
    }
    i as size_t
}

#[no_mangle]
pub unsafe extern "C" fn strpbrk(s1: *const c_char, s2: *const c_char) -> *const c_char {
    let len = strlen(s2);
    let mut i = 0;
    while *s1.offset(i) != 0 {
        if memchr(s2, *s1.offset(i) as c_int, len) as usize == 0 {
            return s1.offset(i);
        }
        i += 1;
    }
    0 as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn strrchr(s: *const c_char, c: c_int) -> *const c_char {
    let mut last = -1;
    let mut i = 0;
    while *s.offset(i) != 0 {
        if *s.offset(i) as c_int == c {
            last = i;
        }
        i += 1;
    }
    match last {
        -1 => 0 as *const c_char,
        _ => s.offset(last),
    }
}

#[no_mangle]
pub unsafe extern "C" fn strspn(s1: *const c_char, s2: *const c_char) -> size_t {
    let len = strlen(s2);
    let mut i = 0;
    while *s1.offset(i) != 0 {
        if memchr(s2, *s1.offset(i) as c_int, len) as usize == 0 {
            break;
        }
        i += 1;
    }
    i as size_t
}

#[no_mangle]
pub unsafe extern "C" fn strstr(s1: *const c_char, s2: *const c_char) -> *const c_char {
    let len1 = strlen(s1) as isize;
    let len2 = strlen(s2) as isize;
    for i in 0..len1 - len2 {
        if memcmp(s1.offset(i), s2, len2 as size_t) == 0 {
            return s1.offset(i);
        }
    }
    0 as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn strtok(s1: *mut c_char, s2: *const c_char) -> *const c_char {
    static mut SS: *mut c_char = 0 as *mut c_char;
    static mut LEN: isize = 0;
    if s1 as usize != 0 {
        SS = s1;
        LEN = strlen(SS as *const _) as isize;
    }
    if SS as usize == 0 {
        return 0 as *const c_char;
    }
    let len2 = strlen(s2) as isize;
    let mut i = 0;
    while i < LEN {
        if memchr(s2, *SS.offset(i) as c_int, len2 as size_t) as usize != 0 {
            break;
        }
        i += 1;
    }
    SS = SS.offset(i);
    LEN -= i;
    if LEN == 0 {
        SS = 0 as *mut c_char;
        return 0 as *const c_char;
    }
    let mut i = 0;
    while i < LEN {
        if memchr(s2, *SS.offset(i) as c_int, len2 as size_t) as usize == 0 {
            break;
        }
        i += 1;
    }
    if i == LEN {
        LEN = 0;
        let tmp = SS;
        SS = 0 as *mut c_char;
        return tmp as *const _;
    }
    *SS.offset(i) = 0;
    let tmp = SS;
    SS = SS.offset(i + 1);
    LEN -= i + 1;
    tmp as *const _
}

#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut c_char, c: c_int, n: size_t) -> *mut c_char {
    let c = c as c_char;
    for i in (0..n as isize).rev() {
        *dest.offset(i) = c;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn strerror(_: c_int) -> *const c_char {
    panic!();
}

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const c_char) -> size_t {
    let mut len = 0;
    while *s.offset(len) != 0 {
        len += 1;
    }
    len as size_t
}

#[no_mangle]
pub unsafe extern "C" fn strnlen(s: *const c_char, n: size_t) -> size_t {
    let mut len: usize = 0;
    while *s.offset(len as isize) != 0 && len < (n as usize) {
        len += 1;
    }
    len as size_t
}
