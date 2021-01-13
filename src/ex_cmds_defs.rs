use crate::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct exarg {
    pub arg: *mut u8,
    pub nextcmd: *mut u8,
    pub cmd: *mut u8,
    pub cmdlinep: *mut *mut u8,
    pub cmdidx: cmdidx_T,
    pub argt: u32,
    pub skip: libc::c_int,
    pub forceit: libc::c_int,
    pub addr_count: libc::c_int,
    pub line1: linenr_T,
    pub line2: linenr_T,
    pub addr_type: libc::c_int,
    pub flags: libc::c_int,
    pub do_ecmd_cmd: *mut u8,
    pub do_ecmd_lnum: linenr_T,
    pub append: libc::c_int,
    pub usefilter: libc::c_int,
    pub amount: libc::c_int,
    pub regname: libc::c_int,
    pub force_bin: libc::c_int,
    pub read_edit: libc::c_int,
    pub force_ff: libc::c_int,
    pub force_enc: libc::c_int,
    pub bad_char: libc::c_int,
    pub useridx: libc::c_int,
    pub errmsg: *mut u8,
    pub getline: LineGetter,
    pub cookie: *mut libc::c_void,
    pub cstack: *mut cstack_T,
}
pub type eslist_T = eslist_elem;
impl Default for exarg_T {
    fn default() -> Self {
        exarg_T {
            arg: ptr::null_mut(),
            nextcmd: ptr::null_mut(),
            cmd: ptr::null_mut(),
            cmdlinep: ptr::null_mut(),
            cmdidx: CMD_append,
            argt: 0,
            skip: 0,
            forceit: 0,
            addr_count: 0,
            line1: 0,
            line2: 0,
            addr_type: 0,
            flags: 0,
            do_ecmd_cmd: ptr::null_mut(),
            do_ecmd_lnum: 0,
            append: 0,
            usefilter: 0,
            amount: 0,
            regname: 0,
            force_bin: 0,
            read_edit: 0,
            force_ff: 0,
            force_enc: 0,
            bad_char: 0,
            useridx: 0,
            errmsg: ptr::null_mut(),
            getline: None,
            cookie: ptr::null_mut(),
            cstack: ptr::null_mut(),
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cstack_T {
    pub cs_flags: [libc::c_int; 50],
    pub cs_pending: [libc::c_char; 50],
    pub cs_pend: C2RustUnnamed_17,
    pub cs_forinfo: [*mut libc::c_void; 50],
    pub cs_line: [libc::c_int; 50],
    pub cs_idx: libc::c_int,
    pub cs_looplevel: libc::c_int,
    pub cs_trylevel: libc::c_int,
    pub cs_emsg_silent_list: *mut eslist_T,
    pub cs_lflags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eslist_elem {
    pub saved_emsg_silent: libc::c_int,
    pub next: *mut eslist_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub csp_rv: [*mut libc::c_void; 50],
    pub csp_ex: [*mut libc::c_void; 50],
}
pub type LineGetter = Option<
    unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_void, _: libc::c_int, _: bool) -> *mut u8,
>;
pub type exarg_T = exarg;
// cursor position in line
// values for xp_backslash
// nothing special for backslashes
// uses one backslash before a space
// uses three backslashes before a space
// / Command modifiers ":vertical", ":browse", ":confirm", ":hide", etc. set a
// / flag.  This needs to be saved for recursive commands, put them in a
// / structure for easy manipulation.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdmod_T {
    pub split: libc::c_int,
    pub tab: libc::c_int,
    pub browse: bool,
    pub confirm: bool,
    pub hide: bool,
    pub keepalt: bool,
    pub keepjumps: bool,
    pub keepmarks: bool,
    pub keeppatterns: bool,
    pub lockmarks: bool,
    pub noswapfile: bool,
    pub save_ei: *mut u8,
    pub filter_regmatch: regmatch_T,
    pub filter_force: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expand {
    pub xp_context: libc::c_int,
    pub xp_pattern: *mut u8,
    pub xp_pattern_len: size_t,
    pub xp_arg: *mut u8,
    pub xp_script_ctx: sctx_T,
    pub xp_backslash: libc::c_int,
    pub xp_shell: libc::c_int,
    pub xp_numfiles: libc::c_int,
    pub xp_files: *mut *mut u8,
    pub xp_line: *mut u8,
    pub xp_col: libc::c_int,
}
pub const XP_BS_ONE: libc::c_int = 1 as libc::c_int;
pub const XP_BS_THREE: libc::c_int = 2 as libc::c_int;
pub const XP_BS_NONE: libc::c_int = 0 as libc::c_int;
