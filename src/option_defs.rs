use crate::eval::typval::sctx_T;
extern "C" {
    pub static mut dy_flags: libc::c_uint;
    pub static mut p_isf: *mut libc::c_uchar;
    pub static mut p_isi: *mut libc::c_uchar;
    pub static mut p_isp: *mut libc::c_uchar;
    pub static mut p_sel: *mut libc::c_uchar;
    pub static mut p_sbr: *mut libc::c_uchar;
}

pub const CMP_INTERNAL: i32 = 0x1 as i32;
pub const CMP_KEEPASCII: i32 = 0x2 as i32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LastSet {
    pub script_ctx: sctx_T,
    pub channel_id: u64,
}
// end-of-line style
pub const EOL_UNKNOWN: i32 = -1; // not defined yet
pub const EOL_UNIX: i32 = 0; // NL
pub const EOL_DOS: i32 = 1; // CR NL
pub const EOL_MAC: i32 = 2; // CR

pub const DY_LASTLINE: u32 = 0x001;
pub const DY_TRUNCATE: u32 = 0x002;
pub const DY_UHEX: u32 = 0x004;
pub type p_shm_option = i32;
pub const SHM_SEARCHCOUNT: p_shm_option = 83;
pub const SHM_FILEINFO: p_shm_option = 70;
pub const SHM_RECORDING: p_shm_option = 113;
pub const SHM_COMPLETIONMENU: p_shm_option = 99;
pub const SHM_INTRO: p_shm_option = 73;
pub const SHM_ATTENTION: p_shm_option = 65;
pub const SHM_SEARCH: p_shm_option = 115;
pub const SHM_OVERALL: p_shm_option = 79;
pub const SHM_OVER: p_shm_option = 111;
pub const SHM_TRUNCALL: p_shm_option = 84;
pub const SHM_TRUNC: p_shm_option = 116;
pub const SHM_WRITE: p_shm_option = 87;
pub const SHM_ABBREVIATIONS: p_shm_option = 97;
pub const SHM_WRI: p_shm_option = 119;
pub const SHM_NEW: p_shm_option = 110;
pub const SHM_LINES: p_shm_option = 108;
pub const SHM_TEXT: p_shm_option = 120;
pub const SHM_LAST: p_shm_option = 105;
pub const SHM_FILE: p_shm_option = 102;
pub const SHM_MOD: p_shm_option = 109;
pub const SHM_RO: p_shm_option = 114;
pub type StatusLineOptionFlag = u8;
pub const STL_CLICK_FUNC: StatusLineOptionFlag = 64;
pub const STL_TABCLOSENR: StatusLineOptionFlag = 88;
pub const STL_TABPAGENR: StatusLineOptionFlag = 84;
pub const STL_HIGHLIGHT: StatusLineOptionFlag = 35;
pub const STL_USER_HL: StatusLineOptionFlag = 42;
pub const STL_TRUNCMARK: StatusLineOptionFlag = 60;
pub const STL_SEPARATE: StatusLineOptionFlag = 61;
pub const STL_VIM_EXPR: StatusLineOptionFlag = 123;
pub const STL_PAGENUM: StatusLineOptionFlag = 78;
pub const STL_ARGLISTSTAT: StatusLineOptionFlag = 97;
pub const STL_ALTPERCENT: StatusLineOptionFlag = 80;
pub const STL_PERCENTAGE: StatusLineOptionFlag = 112;
pub const STL_QUICKFIX: StatusLineOptionFlag = 113;
pub const STL_MODIFIED_ALT: StatusLineOptionFlag = 77;
pub const STL_MODIFIED: StatusLineOptionFlag = 109;
pub const STL_PREVIEWFLAG_ALT: StatusLineOptionFlag = 87;
pub const STL_PREVIEWFLAG: StatusLineOptionFlag = 119;
pub const STL_FILETYPE_ALT: StatusLineOptionFlag = 89;
pub const STL_FILETYPE: StatusLineOptionFlag = 121;
pub const STL_HELPFLAG_ALT: StatusLineOptionFlag = 72;
pub const STL_HELPFLAG: StatusLineOptionFlag = 104;
pub const STL_ROFLAG_ALT: StatusLineOptionFlag = 82;
pub const STL_ROFLAG: StatusLineOptionFlag = 114;
pub const STL_BYTEVAL_X: StatusLineOptionFlag = 66;
pub const STL_BYTEVAL: StatusLineOptionFlag = 98;
pub const STL_OFFSET_X: StatusLineOptionFlag = 79;
pub const STL_OFFSET: StatusLineOptionFlag = 111;
pub const STL_KEYMAP: StatusLineOptionFlag = 107;
pub const STL_BUFNO: StatusLineOptionFlag = 110;
pub const STL_NUMLINES: StatusLineOptionFlag = 76;
pub const STL_LINE: StatusLineOptionFlag = 108;
pub const STL_VIRTCOL_ALT: StatusLineOptionFlag = 86;
pub const STL_VIRTCOL: StatusLineOptionFlag = 118;
pub const STL_COLUMN: StatusLineOptionFlag = 99;
pub const STL_FILENAME: StatusLineOptionFlag = 116;
pub const STL_FULLPATH: StatusLineOptionFlag = 70;
pub const STL_FILEPATH: StatusLineOptionFlag = 102;
pub const SWB_USEOPEN: libc::c_uint = 0x001;
pub const SWB_USETAB: libc::c_uint = 0x002;
pub const SWB_SPLIT: libc::c_uint = 0x004;
pub const SWB_NEWTAB: libc::c_uint = 0x008;
pub const SWB_VSPLIT: libc::c_uint = 0x010;
pub const SWB_USELAST: libc::c_uint = 0x020;
pub const CPO_INTMOD: libc::c_int = 'i' as i32;
pub const STL_ALL: [u8; 38] = [
    STL_FILEPATH as libc::c_int as u8,
    STL_FULLPATH as libc::c_int as u8,
    STL_FILENAME as libc::c_int as u8,
    STL_COLUMN as libc::c_int as u8,
    STL_VIRTCOL as libc::c_int as u8,
    STL_VIRTCOL_ALT as libc::c_int as u8,
    STL_LINE as libc::c_int as u8,
    STL_NUMLINES as libc::c_int as u8,
    STL_BUFNO as libc::c_int as u8,
    STL_KEYMAP as libc::c_int as u8,
    STL_OFFSET as libc::c_int as u8,
    STL_OFFSET_X as libc::c_int as u8,
    STL_BYTEVAL as libc::c_int as u8,
    STL_BYTEVAL_X as libc::c_int as u8,
    STL_ROFLAG as libc::c_int as u8,
    STL_ROFLAG_ALT as libc::c_int as u8,
    STL_HELPFLAG as libc::c_int as u8,
    STL_HELPFLAG_ALT as libc::c_int as u8,
    STL_FILETYPE as libc::c_int as u8,
    STL_FILETYPE_ALT as libc::c_int as u8,
    STL_PREVIEWFLAG as libc::c_int as u8,
    STL_PREVIEWFLAG_ALT as libc::c_int as u8,
    STL_MODIFIED as libc::c_int as u8,
    STL_MODIFIED_ALT as libc::c_int as u8,
    STL_QUICKFIX as libc::c_int as u8,
    STL_PERCENTAGE as libc::c_int as u8,
    STL_ALTPERCENT as libc::c_int as u8,
    STL_ARGLISTSTAT as libc::c_int as u8,
    STL_PAGENUM as libc::c_int as u8,
    STL_VIM_EXPR as libc::c_int as u8,
    STL_SEPARATE as libc::c_int as u8,
    STL_TRUNCMARK as libc::c_int as u8,
    STL_USER_HL as libc::c_int as u8,
    STL_HIGHLIGHT as libc::c_int as u8,
    STL_TABPAGENR as libc::c_int as u8,
    STL_TABCLOSENR as libc::c_int as u8,
    STL_CLICK_FUNC as libc::c_int as u8,
    0 as libc::c_int as u8,
];
pub const NO_LOCAL_UNDOLEVEL: i64 = -123456;
extern "C" {
    pub static mut p_ambw: *mut u8;
    pub static mut p_emoji: i32;
    pub static mut cmp_flags: u32;
    pub static mut p_enc: *mut u8;
    pub static mut p_acd: libc::c_int;
    pub static mut breakat_flags: [libc::c_char; 256];
    pub static mut p_ch: libc::c_long;
    pub static mut p_confirm: libc::c_int;
    pub static mut p_cpo: *mut u8;
    pub static mut p_ea: libc::c_int;
    pub static mut p_fic: libc::c_int;
    pub static mut p_fdls: libc::c_long;
    pub static mut p_hid: libc::c_int;
    pub static mut p_icon: libc::c_int;
    pub static mut p_iconstring: *mut u8;
    pub static mut p_magic: libc::c_int;
    pub static mut p_mls: libc::c_long;
    pub static mut p_report: libc::c_long;
    pub static mut p_ru: libc::c_int;
    pub static mut p_tpm: libc::c_long;
    pub static mut p_sol: libc::c_int;
    pub static mut swb_flags: libc::c_uint;
    pub static mut p_title: libc::c_int;
    pub static mut p_titlelen: libc::c_long;
    pub static mut p_titlestring: *mut u8;
    pub static mut p_wic: libc::c_int;
    pub static mut p_write: libc::c_int;
}
pub enum BV {
    AI = 0,
    AR,
    BH,
    BKC,
    BT,
    EFM,
    GP,
    MP,
    BIN,
    BL,
    BOMB,
    CHANNEL,
    CI,
    CIN,
    CINK,
    CINO,
    CINW,
    CM,
    CMS,
    COM,
    CPT,
    DICT,
    TSR,
    CFU,
    DEF,
    INC,
    EOL,
    FIXEOL,
    EP,
    ET,
    FENC,
    FP,
    BEXPR,
    FEX,
    FF,
    FLP,
    FO,
    FT,
    IMI,
    IMS,
    INDE,
    INDK,
    INEX,
    INF,
    ISK,
    KMAP,
    KP,
    LISP,
    LW,
    MENC,
    MA,
    ML,
    MOD,
    MPS,
    NF,
    OFU,
    PATH,
    PI,
    QE,
    RO,
    SCBK,
    SI,
    SMC,
    SYN,
    SPC,
    SPF,
    SPL,
    STS,
    SUA,
    SW,
    SWF,
    TFU,
    TAGS,
    TC,
    TS,
    TW,
    TX,
    UDF,
    UL,
    WM,
    COUNT, // must be the last one
}
