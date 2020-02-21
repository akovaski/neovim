use crate::buffer_defs::*;
use crate::pos::pos_T;

extern "C" {
    pub static mut curbuf: *mut buf_T;
    pub static mut got_int: libc::c_int;
    pub static mut curwin: *mut win_T;
    pub static mut VIsual: pos_T;
    pub static mut VIsual_active: libc::c_int;
    pub static mut State: libc::c_int;
}
