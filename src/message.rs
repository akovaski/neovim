extern "C" {
    pub fn msg(s: *const u8) -> libc::c_int;
    pub fn trunc_string(s: *mut u8, buf: *mut u8, room_in: libc::c_int, buflen: libc::c_int);
    pub fn smsg(s: *const libc::c_char, _: ...) -> libc::c_int;
    pub fn emsg(s: *const u8) -> bool;
    pub fn emsgf(fmt: *const libc::c_char, _: ...) -> bool;
    pub fn msg_trunc_attr(s: *mut u8, force: libc::c_int, attr: libc::c_int) -> *mut u8;
    pub fn set_keep_msg(s: *mut u8, attr: libc::c_int);
    pub fn msg_start();
    pub fn msg_putchar(c: libc::c_int);
    pub fn msg_outtrans(str: *mut u8) -> libc::c_int;
    pub fn msg_puts(s: *const libc::c_char);
    pub fn message_filtered(msg_0: *mut u8) -> bool;
}
