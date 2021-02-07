//
// Log module
//
// How Linux printk() handles recursion, buffering, etc:
// https://lwn.net/Articles/780556/
//
use crate::*;
use std::ptr;

pub const DEBUG_LOG_LEVEL: libc::c_int = 0;
#[allow(dead_code)]
pub const INFO_LOG_LEVEL: libc::c_int = 1;
pub const WARN_LOG_LEVEL: libc::c_int = 2;
pub const ERROR_LOG_LEVEL: libc::c_int = 3;

macro_rules! WLOG {
    ($s:expr $(, $x:expr)* $(,)?) => {
        logmsg!(WARN_LOG_LEVEL,
                $s,
                $($x),*
                );
    }
}

macro_rules! ELOG {
    ($s:expr $(, $x:expr)* $(,)?) => {
        logmsg!(
            ERROR_LOG_LEVEL,
            $s,
            $($x),*
            );
    }
}

macro_rules! DLOG {
    ($s:expr $(, $x:expr)* $(,)?) => {
        logmsg!(
            DEBUG_LOG_LEVEL,
            $s,
            $($x),*
            );
    }
}

macro_rules! ILOG {
    ($s:expr $(, $x:expr)* $(,)?) => {
        logmsg!(
            INFO_LOG_LEVEL,
            $s,
            $($x),*
            );
    }
}

macro_rules! logmsg {
    ($level:expr, $s:expr $(, $x:expr)* $(,)?) => {
        logmsg(
            $level,
            std::ptr::null(),
            std::ffi::CString::new(file!())
                .expect("CString::new failed")
                .as_ptr() as *const libc::c_char,
            line!() as libc::c_int,
            true,
            std::ffi::CString::new($s)
                .expect("CString::new failed")
                .as_ptr() as *const libc::c_char,
            $($x),*
            );
    }
}

//const LOG_FILE_ENV: *const i8 = S!("NVIM_LOG_FILE");
macro_rules! LOG_FILE_ENV {
    () => {
        "NVIM_LOG_FILE"
    };
}

/// Cached location of the expanded log file path decided by log_path_init().
static mut log_file_path: [i8; MAXPATHL + 1] = [0; MAXPATHL + 1];

static mut mutex: uv_mutex_t = uv_mutex_t_zero();

unsafe fn log_try_create(fname: &[i8]) -> bool {
    if fname.len() == 0 || fname[0] == 0 {
        return false;
    }
    let log_file = fopen(fname.as_ptr(), S!("a"));
    if log_file.is_null() {
        return false;
    }
    fclose(log_file);
    return true;
}

/// Initializes path to log file. Sets $NVIM_LOG_FILE if empty.
///
/// Tries $NVIM_LOG_FILE, or falls back to $XDG_DATA_HOME/nvim/log. Path to log
/// file is cached, so only the first call has effect, unless first call was not
/// successful. Failed initialization indicates either a bug in expand_env()
/// or both $NVIM_LOG_FILE and $HOME environment variables are undefined.
///
/// @return true if path was initialized, false otherwise.
unsafe fn log_path_init() -> bool {
    if log_file_path[0] != 0 {
        return true;
    }
    let size = var_size!(log_file_path);
    expand_env(
        S!("$" LOG_FILE_ENV!()),
        log_file_path.as_mut_ptr() as *mut u8,
        size as i32 - 1,
    );
    if strequal(S!("$" LOG_FILE_ENV!()), log_file_path.as_mut_ptr())
        || log_file_path[0] == 0
        || os_isdir(log_file_path.as_mut_ptr() as *mut u8)
        || !log_try_create(&log_file_path)
    {
        // Invalid $NVIM_LOG_FILE or failed to expand; fall back to default.
        let defaultpath = stdpaths_user_data_subpath(S!("log"), 0, true);
        let mut len = xstrlcpy(log_file_path.as_mut_ptr(), defaultpath, size);
        xfree(defaultpath);
        // Fall back to .nvimlog
        if len >= size || !log_try_create(&log_file_path) {
            len = xstrlcpy(log_file_path.as_mut_ptr(), S!(".nvimlog"), size);
        }
        // Fall back to stderr
        if len >= size || !log_try_create(&log_file_path) {
            log_file_path[0] = 0;
            return false;
        }
        os_setenv(S!(LOG_FILE_ENV!()), log_file_path.as_mut_ptr(), true_0);
    }
    return true;
}

#[no_mangle]
pub unsafe extern "C" fn log_init() {
    uv_mutex_init(&mut mutex);
    log_path_init();
}

#[no_mangle]
pub unsafe extern "C" fn log_lock() {
    uv_mutex_lock(&mut mutex);
}

#[no_mangle]
pub unsafe extern "C" fn log_unlock() {
    uv_mutex_unlock(&mut mutex);
}

/// Logs a message to $NVIM_LOG_FILE.
///
/// @param log_level  Log level (see log.h)
/// @param context    Description of a shared context or subsystem
/// @param func_name  Function name, or NULL
/// @param line_num   Source line number, or -1
/// @param eol        Append linefeed "\n"
/// @param fmt        printf-style format string
#[no_mangle]
pub unsafe extern "C" fn logmsg(
    log_level: i32,
    context: *const i8,
    func_name: *const i8,
    line_num: i32,
    eol: bool,
    fmt: *const i8,
    mut args: ...
) -> bool {
    if log_level < MIN_LOG_LEVEL {
        return false;
    }

    //TODO: EXITFREE

    log_lock();
    let mut ret = false;
    let log_file = open_log_file();

    if !log_file.is_null() {
        ret = v_do_log_to_file(
            log_file,
            log_level,
            context,
            func_name,
            line_num,
            eol,
            fmt,
            args.as_va_list(),
        );

        if log_file != stderr && log_file != stdout {
            fclose(log_file);
        }
    }

    log_unlock();
    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn log_uv_handles(loop_0: *mut libc::c_void) {
    let l = loop_0 as *mut uv_loop_t;
    log_lock();
    let log_file = open_log_file();

    if !log_file.is_null() {
        uv_print_all_handles(l, log_file);

        if log_file != stderr && log_file != stdout {
            fclose(log_file);
        }
    }

    log_unlock();
}

/// Open the log file for appending.
///
/// @return FILE* decided by log_path_init() or stderr in case of error
#[no_mangle]
pub unsafe extern "C" fn open_log_file() -> *mut FILE {
    static mut opening_log_file: bool = false;
    // Disallow recursion. (This only matters for log_path_init; for logmsg and
    // friends we use a mutex: log_lock).
    if opening_log_file {
        do_log_to_file(
            stderr,
            ERROR_LOG_LEVEL,
            ptr::null_mut(),
            S!("open_log_file"),
            line!() as i32,
            true,
            S!("Cannot LOG() recursively."),
        );
        return stderr;
    }

    let mut log_file = ptr::null_mut();
    opening_log_file = true;
    if log_path_init() {
        log_file = fopen(log_file_path.as_mut_ptr(), S!("a"));
    }
    opening_log_file = false;

    if !log_file.is_null() {
        return log_file;
    }

    // May happen if:
    //  - LOG() is called before early_init()
    //  - Directory does not exist
    //  - File is not writable
    do_log_to_file(
        stderr,
        ERROR_LOG_LEVEL,
        ptr::null_mut(),
        S!("open_log_file"),
        line!() as i32,
        true,
        S!("Logging to stderr, failed to open $" LOG_FILE_ENV!() ": %s"),
        log_file_path.as_mut_ptr(),
    );
    return stderr;
}

#[no_mangle]
pub unsafe extern "C" fn log_callstack_to_file(
    log_file: *mut FILE,
    func_name: *const i8,
    line_num: i32,
) {
    const trace_len: usize = 100;
    let mut trace: [*mut libc::c_void; trace_len] = [ptr::null_mut(); trace_len];
    let trace_size = backtrace(trace.as_mut_ptr(), trace.len() as i32);

    let mut exepath: [i8; MAXPATHL] = [0; MAXPATHL];
    let mut exepathlen = MAXPATHL as size_t;
    if os_exepath(exepath.as_mut_ptr(), &mut exepathlen) != 0 {
        abort();
    }
    assert!(24 + exepathlen < IOSIZE as usize); // Must fit in `cmdbuf` below.

    const cmdbuf_size: usize = IOSIZE as usize + 20 * trace_len + MAXPATHL;
    let mut cmdbuf: [i8; cmdbuf_size] = [0; cmdbuf_size];
    snprintf(
        cmdbuf.as_mut_ptr(),
        var_size!(cmdbuf) as u64,
        S!("addr2line -e %s -f -p"),
        exepath.as_mut_ptr(),
    );
    for i in 1..trace_size as usize {
        let mut buf: [i8; 20] = [0; 20]; // 64-bit pointer 0xNNNNNNNNNNNNNNNN with leading space.
        snprintf(buf.as_mut_ptr(), var_size!(buf) as u64, S!(" %p"), trace[i]);
        xstrlcat(cmdbuf.as_mut_ptr(), buf.as_mut_ptr(), var_size!(cmdbuf));
    }
    // Now we have a command string like:
    //    addr2line -e /path/to/exe -f -p 0x123 0x456 ...

    do_log_to_file(
        log_file,
        DEBUG_LOG_LEVEL,
        ptr::null_mut(),
        func_name,
        line_num,
        true,
        S!("trace:"),
    );
    let fp = popen(cmdbuf.as_mut_ptr(), S!("r"));
    let mut linebuf: [i8; IOSIZE as usize] = [0; IOSIZE as usize];
    while !fgets(linebuf.as_mut_ptr(), var_size!(linebuf) as i32, fp).is_null() {
        fprintf(log_file, S!("  %s"), linebuf.as_mut_ptr());
    }
    pclose(fp);

    if log_file != stderr && log_file != stdout {
        fclose(log_file);
    }
}

#[no_mangle]
pub unsafe extern "C" fn log_callstack(func_name: *const i8, line_num: i32) {
    log_lock();
    let log_file = open_log_file();
    if !log_file.is_null() {
        log_callstack_to_file(log_file, func_name, line_num);
    }
    log_unlock();
}

unsafe extern "C" fn do_log_to_file(
    log_file: *mut FILE,
    log_level: i32,
    context: *const i8,
    func_name: *const i8,
    line_num: i32,
    eol: bool,
    fmt: *const i8,
    mut args: ...
) -> bool {
    v_do_log_to_file(
        log_file,
        log_level,
        context,
        func_name,
        line_num,
        eol,
        fmt,
        args.as_va_list(),
    )
}

unsafe fn v_do_log_to_file(
    log_file: *mut FILE,
    log_level: i32,
    context: *const i8,
    func_name: *const i8,
    line_num: i32,
    eol: bool,
    fmt: *const i8,
    mut args: ::std::ffi::VaList,
) -> bool {
    static mut log_levels: [*const i8; 4] = [S!("DEBUG"), S!("INFO "), S!("WARN "), S!("ERROR")];
    assert!(log_level >= DEBUG_LOG_LEVEL && log_level <= ERROR_LOG_LEVEL);

    // Format the timestamp.
    let mut local_time = tm_zero();
    if os_localtime(&mut local_time).is_null() {
        return false;
    }
    let mut date_time: [i8; 20] = [0; 20];
    if strftime(
        date_time.as_mut_ptr(),
        var_size!(date_time),
        S!("%Y-%m-%dT%H:%M:%S"),
        &mut local_time,
    ) == 0
    {
        return false;
    }

    let mut millis = 0;
    if !cfg!(windows) {
        let mut curtime = timeval_zero();
        if gettimeofday(&mut curtime, ptr::null_mut()) == 0 {
            millis = curtime.tv_usec as i32 / 1000;
        }
    }

    // Print the log message.
    let pid = os_get_pid();
    let rv = if line_num == -1 || func_name.is_null() {
        fprintf(
            log_file,
            S!("%s %s.%03d %-5ld %s"),
            log_levels[log_level as usize],
            date_time.as_mut_ptr(),
            millis,
            pid,
            if context.is_null() { S!("?:") } else { context },
        )
    } else {
        fprintf(
            log_file,
            S!("%s %s.%03d %-5ld %s%s:%d: "),
            log_levels[log_level as usize],
            date_time.as_mut_ptr(),
            millis,
            pid,
            if context.is_null() { S!("") } else { context },
            func_name,
            line_num,
        )
    };
    if rv < 0 {
        return false;
    }
    if vfprintf(log_file, fmt, args.as_va_list()) < 0 {
        return false;
    }
    if eol {
        fputc('\n' as i32, log_file);
    }
    if fflush(log_file) == EOF {
        return false;
    }

    return true;
}
