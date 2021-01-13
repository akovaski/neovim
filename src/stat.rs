pub const __S_IFMT: libc::c_int = libc::S_IFMT as i32;
pub fn S_ISFIFO(perm: i32) -> bool {
    perm & __S_IFMT == libc::S_IFIFO as i32
}
pub fn S_ISSOCK(perm: i32) -> bool {
    perm & __S_IFMT == libc::S_IFSOCK as i32
}
