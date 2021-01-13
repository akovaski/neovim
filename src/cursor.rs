extern "C" {
    pub fn get_cursor_line_ptr() -> *mut libc::c_uchar;
    pub fn check_cursor_lnum();
    pub fn check_cursor_col();
}
