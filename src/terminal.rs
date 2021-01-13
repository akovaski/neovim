extern "C" {
    pub type Terminal;
    pub fn terminal_close(term: *mut Terminal, msg_0: *mut libc::c_char);
    pub fn terminal_check_size(term: *mut Terminal);
}
