use crate::*;
use std::cmp::min;
use std::ptr;
pub type Timestamp = u64;
extern "C" {
    static mut got_int: libc::c_int;
    fn os_char_avail() -> bool;
//pub fn os_hrtime() -> u64;
}

static mut delay_mutex: Option<uv_mutex_t> = None;
static mut delay_cond: Option<uv_cond_t> = None;

/// Initializes the time module
#[no_mangle]
pub unsafe extern "C" fn time_init() {
    let mut some_delay_mutex = Default::default();
    let mut some_delay_cond = Default::default();
    uv_mutex_init(&mut some_delay_mutex);
    uv_cond_init(&mut some_delay_cond);
    delay_mutex = Some(some_delay_mutex);
    delay_cond = Some(some_delay_cond);
}

/// Gets a high-resolution (nanosecond), monotonically-increasing time relative
/// to an arbitrary time in the past.
///
/// Not related to the time of day and therefore not subject to clock drift.
///
/// @return Relative time value with nanosecond precision.
#[no_mangle]
pub unsafe extern "C" fn os_hrtime() -> u64 {
    return uv_hrtime();
}

/// Gets a millisecond-resolution, monotonically-increasing time relative to an
/// arbitrary time in the past.
///
/// Not related to the time of day and therefore not subject to clock drift.
/// The value is cached by the loop, it will not change until the next
/// loop-tick (unless uv_update_time is called).
///
/// @return Relative time value with millisecond precision.
#[no_mangle]
pub unsafe extern "C" fn os_now() -> u64 {
    return uv_now(&mut main_loop.uv);
}

/// Sleeps for `ms` milliseconds.
///
/// @param ms          Number of milliseconds to sleep
/// @param ignoreinput If true, only SIGINT (CTRL-C) can interrupt.
#[no_mangle]
pub unsafe extern "C" fn os_delay(mut ms: u64, ignoreinput: bool) {
    if ignoreinput {
        if ms > libc::c_int::max_value() as u64 {
            ms = libc::c_int::max_value() as u64;
        }
        LOOP_PROCESS_EVENTS_UNTIL(&mut main_loop, &mut None, ms as libc::c_int, || {
            got_int != 0
        });
    } else {
        os_microdelay(ms * 1000, ignoreinput);
    }
}

/// Sleeps for `us` microseconds.
///
/// @param us          Number of microseconds to sleep.
/// @param ignoreinput If true, ignore all input (including SIGINT/CTRL-C).
///                    If false, waiting is aborted on any input.
#[no_mangle]
pub unsafe extern "C" fn os_microdelay(us: u64, ignoreinput: bool) {
    let mut elapsed: u64 = 0;
    let mut base: u64 = uv_hrtime();
    // Convert microseconds to nanoseconds, or UINT64_MAX on overflow.
    let ns: u64 = us.saturating_mul(1000);

    uv_mutex_lock(&mut delay_mutex.unwrap());

    while elapsed < ns {
        // If ignoring input, we simply wait the full delay.
        // Else we check for input in ~100ms intervals.
        let ns_delta: u64 = if ignoreinput {
            ns - elapsed
        } else {
            min(ns - elapsed, 100000000)
        }; // 100ms
        let rv = uv_cond_timedwait(
            &mut delay_cond.unwrap(),
            &mut delay_mutex.unwrap(),
            ns_delta,
        );
        if 0 != rv && UV_ETIMEDOUT != rv {
            c_assert!(false);
            break;
        } // Else: Timeout proceeded normally.

        if !ignoreinput && os_char_avail() {
            break;
        }

        let now: u64 = uv_hrtime();
        elapsed += now - base;
        base = now
    }
    uv_mutex_unlock(&mut delay_mutex.unwrap());
}

/// Portable version of POSIX localtime_r()
///
/// @return NULL in case of error
#[no_mangle]
pub unsafe extern "C" fn os_localtime_r(
    clock: *const libc::time_t,
    result: *mut libc::tm,
) -> *mut libc::tm {
    // POSIX provides localtime_r() as a thread-safe version of localtime().
    #[cfg(unix)]
    return libc::localtime_r(clock, result);

    #[cfg(not(unix))]
    assert!("This function only supported on unix-like systems")
    // TODO: Windows
}

/// Gets the current Unix timestamp and adjusts it to local time.
///
/// @param result Pointer to a 'struct tm' where the result should be placed
/// @return A pointer to a 'struct tm' in the current time zone (the 'result'
///         argument) or NULL in case of error
#[no_mangle]
pub unsafe extern "C" fn os_localtime(result: *mut libc::tm) -> *mut libc::tm {
    let mut rawtime = libc::time(ptr::null_mut());
    return os_localtime_r(&mut rawtime, result);
}

/// Obtains the current Unix timestamp.
///
/// @return Seconds since epoch.
#[no_mangle]
pub unsafe extern "C" fn os_time() -> Timestamp {
    return libc::time(ptr::null_mut()) as Timestamp;
}
