use core::mem::transmute;
use core::ptr;
use libc::{c_int, c_char, c_long, time_t, tm, timeval, timezone};
use syscalls::sys_gettimeofday;

#[no_mangle]
pub unsafe extern "C" fn time(time: *mut time_t) -> time_t {
    let mut now: timeval = timeval {
        tv_sec: 0xabcd,
        tv_usec: 0xabcd,
    };
    if forward!(sys_gettimeofday, &mut now as *mut timeval, ptr::null_mut()) >= 0 {
        if time != ptr::null_mut() {
            *time = now.tv_sec;
        }
        now.tv_sec
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn gettimeofday(tv: *mut timeval, tz: *mut timezone) -> c_int {
    forward!(sys_gettimeofday, tv, tz)
}

#[allow(non_upper_case_globals)]
pub static mut gmtime_tm: tm = tm {
    tm_sec: 0,
    tm_min: 0,
    tm_hour: 0,
    tm_mday: 0,
    tm_mon: 0,
    tm_year: 0,
    tm_wday: 0,
    tm_yday: 0,
    tm_isdst: 0,
    tm_gmtoff: 0,
    tm_zone: 0 as *mut c_char,
};

static TM_ZONE_GMT: [i8; 4] = ['G' as i8, 'M' as i8, 'T' as i8, 0];

const EPOCH_YR: c_int = 1970;
const YR_1900: c_int = 1900;
const SECS_DAY: u64 = 86400;

/// Length of the passed gregorian year (e.g. 1970).
fn yearsize(year: c_int) -> u64 {
    match year {
        // leap years on non-centennials
        y if (y % 4 == 0 && y % 100 != 0) => 366,
        // leap years on centennials not multiples of 400
        y if (y % 4 == 0 && y % 100 == 0 && y % 400 != 0) => 365,
        // leap years on multiples of 400
        y if (y % 4 == 0 && y % 100 == 0 && y % 400 == 0) => 366,
        // normal non-leap years. This doesn't exhaust for some reason:
        // y if (y % 4 != 0) => 365,
        _ => 365,
    }
}

fn leapyear(year: c_int) -> bool {
    yearsize(year) == 366
}

fn monthlen(ly: bool, mon: c_int) -> u64 {
    let _ytab = &[[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
                  [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]];
    _ytab[ly as usize][mon as usize]
}

/// TODO negative times
#[no_mangle]
pub unsafe extern "C" fn gmtime(timer: *const time_t) -> *const tm {
    let time: time_t = *timer;
    let dayclock: u64 = time as u64 % SECS_DAY;
    let mut dayno: u64 = time as u64 / SECS_DAY;
    let mut year: c_int = EPOCH_YR;

    gmtime_tm = tm {
        tm_sec: (dayclock % 60) as c_int,
        tm_min: ((dayclock % 3600) / 60) as c_int,
        tm_hour: (dayclock / 3600) as c_int,
        tm_wday: ((dayno + 4) % 7) as c_int, // day 0 was a thursday

        tm_year: {
            while dayno >= yearsize(year) {
                dayno -= yearsize(year);
                year += 1;
            }
            year - YR_1900
        },

        tm_yday: dayno as c_int,
        tm_mon: {
            let mut mon = 0;
            while dayno >= monthlen(leapyear(year), mon) {
                dayno -= monthlen(leapyear(year), mon);
                mon += 1;
            }
            mon
        },

        tm_mday: dayno as c_int + 1,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: TM_ZONE_GMT.as_ptr() as *mut c_char,
    };

    &gmtime_tm as *const tm
}

/// TODO negative times
/// TODO thread-local storage
#[no_mangle]
pub unsafe extern "C" fn gmtime_r(timer: *const time_t) -> *const tm {
    gmtime(timer)
}

/// TODO time localization
#[no_mangle]
pub unsafe extern "C" fn localtime(timer: *const time_t) -> *const tm {
    gmtime(timer)
}

/// TODO time localization
/// TODO thread-local storage
#[no_mangle]
pub unsafe extern "C" fn localtime_r(timer: *const time_t) -> *const tm {
    gmtime_r(timer)
}

/// Convert a GMT `struct tm` to a time_t.
#[no_mangle]
pub unsafe extern "C" fn timegm(timer_ptr: *const tm) -> time_t {
    let timer: &tm = transmute(timer_ptr);
    let yr = timer.tm_year + EPOCH_YR;

    let mut t = (yr as time_t - 1970) * (yearsize(yr) * SECS_DAY) as time_t;
    t += timer.tm_yday as time_t * SECS_DAY as time_t;
    t += (timer.tm_hour * 3600 + timer.tm_min * 60 + timer.tm_sec) as time_t;
    t
}

/// TODO time localization
#[no_mangle]
pub unsafe extern "C" fn mktime(timer_ptr: *const tm) -> time_t {
    timegm(timer_ptr)
}

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static mut tzname: [*mut c_char; 2] = [0 as *mut c_char, 0 as *mut c_char];
#[no_mangle]
#[allow(non_upper_case_globals)]
pub static mut timezone: c_long = 0;
#[no_mangle]
#[allow(non_upper_case_globals)]
pub static mut daylight: c_int = 0;

/// TODO time localization
#[no_mangle]
pub unsafe extern "C" fn tzset() {
    tzname[0] = TM_ZONE_GMT.as_ptr() as *mut c_char;
    tzname[1] = TM_ZONE_GMT.as_ptr() as *mut c_char;
}
