use crate::*;

pub unsafe fn MB_BYTE2LEN(b: u8) -> u8 {
    utf8len_tab[b as usize]
}

// For indirect
// for u8
/*
 * Return byte length of character that starts with byte "b".
 * Returns 1 for a single-byte character.
 * MB_BYTE2LEN_CHECK() can be used to count a special key as one byte.
 * Don't call MB_BYTE2LEN(b) with b < 0 or b > 255!
 */
// max length of an unicode char
/* properties used in enc_canon_table[] (first three mutually exclusive) */
/* Unicode: Big endian */
/* Unicode: Little endian */
/* Unicode: UCS-2 */
/* Unicode: UCS-4 */
/* Unicode: UTF-16 */
/* Latin1 */
/* Latin9 */
/* Mac Roman (not Macro Man! :-) */
// TODO(bfredl): eventually we should keep only one of the namings
// / Flags for vimconv_T
pub type C2RustUnnamed_1 = u32;
pub const CONV_ICONV: C2RustUnnamed_1 = 5;
pub const CONV_TO_LATIN9: C2RustUnnamed_1 = 4;
pub const CONV_TO_LATIN1: C2RustUnnamed_1 = 3;
pub const CONV_9_TO_UTF8: C2RustUnnamed_1 = 2;
pub const CONV_TO_UTF8: C2RustUnnamed_1 = 1;
pub const CONV_NONE: C2RustUnnamed_1 = 0;
// / Structure used for string conversions
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vimconv_T {
    pub vc_type: i32,
    pub vc_factor: i32,
    pub vc_fd: iconv_t,
    pub vc_fail: bool,
}
pub const ENC_8BIT: i32 = 0x1 as i32;
pub const ENC_DBCS: i32 = 0x2 as i32;
pub const ENC_MACROMAN: i32 = 0x800 as i32;
pub const ENC_4BYTE: i32 = 0x80 as i32;
pub const ENC_ENDIAN_L: i32 = 0x20 as i32;
pub const ENC_UNICODE: i32 = 0x4 as i32;
pub const ENC_ENDIAN_B: i32 = 0x10 as i32;
pub const ENC_2WORD: i32 = 0x100 as i32;
pub const ENC_2BYTE: i32 = 0x40 as i32;
pub const ENC_LATIN9: i32 = 0x400 as i32;
pub const ENC_LATIN1: i32 = 0x200 as i32;
pub const mb_ptr2len: unsafe extern "C" fn(_: *const u8) -> i32 = utfc_ptr2len;
// / Compare strings
// /
// / @param[in]  ic  True if case is to be ignored.
// /
// / @return 0 if s1 == s2, <0 if s1 < s2, >0 if s1 > s2.
#[inline]
pub unsafe extern "C" fn mb_strcmp_ic(mut ic: bool, mut s1: *const i8, mut s2: *const i8) -> i32 {
    return if ic as i32 != 0 { mb_stricmp(s1, s2) } else { strcmp(s1, s2) };
}
/*
 * Canonical encoding names and their properties.
 * "iso-8859-n" is handled by enc_canonize() directly.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub name: *const i8,
    pub prop: i32,
    pub codepage: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct interval {
    pub first: i64,
    pub last: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clinterval {
    pub first: u32,
    pub last: u32,
    pub class: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct convertStruct {
    pub rangeStart: i32,
    pub rangeEnd: i32,
    pub step: i32,
    pub offset: i32,
}
/*
 * Aliases for encoding names.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub name: *const i8,
    pub canon: i32,
}
extern "C" {
    pub static mut e_loadlib: [u8; 32];
    pub static mut e_loadfunc: [u8; 41];
    pub static mut utf8len_tab: [u8; 256];
    pub static mut utf8len_tab_zero: [u8; 256];
}
static mut enc_canon_table: [C2RustUnnamed_2; 59] = [
    {
        let mut init = C2RustUnnamed_2 {
            name: b"latin1\x00" as *const u8 as *const i8,
            prop: ENC_8BIT + ENC_LATIN1,
            codepage: 1252,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-2\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-3\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-4\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-5\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-6\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-7\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-8\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-9\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-10\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-11\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-13\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-14\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-15\x00" as *const u8 as *const i8,
            prop: ENC_8BIT + ENC_LATIN9,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"koi8-r\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"koi8-u\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"utf-8\x00" as *const u8 as *const i8,
            prop: ENC_UNICODE,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"ucs-2\x00" as *const u8 as *const i8,
            prop: ENC_UNICODE + ENC_ENDIAN_B + ENC_2BYTE,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"ucs-2le\x00" as *const u8 as *const i8,
            prop: ENC_UNICODE + ENC_ENDIAN_L + ENC_2BYTE,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"utf-16\x00" as *const u8 as *const i8,
            prop: ENC_UNICODE + ENC_ENDIAN_B + ENC_2WORD,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"utf-16le\x00" as *const u8 as *const i8,
            prop: ENC_UNICODE + ENC_ENDIAN_L + ENC_2WORD,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"ucs-4\x00" as *const u8 as *const i8,
            prop: ENC_UNICODE + ENC_ENDIAN_B + ENC_4BYTE,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"ucs-4le\x00" as *const u8 as *const i8,
            prop: ENC_UNICODE + ENC_ENDIAN_L + ENC_4BYTE,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"debug\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_DEBUG,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"euc-jp\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_JPNU,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"sjis\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_JPN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"euc-kr\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_KORU,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"euc-cn\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_CHSU,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"euc-tw\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_CHTU,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"big5\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_CHT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp437\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 437,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp737\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 737,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp775\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 775,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp850\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 850,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp852\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 852,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp855\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 855,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp857\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 857,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp860\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 860,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp861\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 861,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp862\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 862,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp863\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 863,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp865\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 865,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp866\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 866,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp869\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 869,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp874\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 874,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp932\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_JPN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp936\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_CHS,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp949\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_KOR,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp950\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_CHT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1250\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1250,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1251\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1251,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1253\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1253,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1254\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1254,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1255\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1255,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1256\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1256,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1257\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1257,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1258\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1258,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"macroman\x00" as *const u8 as *const i8,
            prop: ENC_8BIT + ENC_MACROMAN,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"hp-roman8\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
];
pub const IDX_LATIN_1: i32 = 0;
pub const IDX_ISO_2: i32 = 1;
pub const IDX_ISO_3: i32 = 2;
pub const IDX_ISO_4: i32 = 3;
pub const IDX_ISO_5: i32 = 4;
pub const IDX_ISO_6: i32 = 5;
pub const IDX_ISO_7: i32 = 6;
pub const IDX_ISO_8: i32 = 7;
pub const IDX_ISO_9: i32 = 8;
pub const IDX_ISO_10: i32 = 9;
pub const IDX_ISO_11: i32 = 10;
pub const IDX_ISO_13: i32 = 11;
pub const IDX_ISO_14: i32 = 12;
pub const IDX_ISO_15: i32 = 13;
pub const IDX_UTF8: i32 = 16;
pub const IDX_UCS2: i32 = 17;
pub const IDX_UCS2LE: i32 = 18;
pub const IDX_UTF16: i32 = 19;
pub const IDX_UTF16LE: i32 = 20;
pub const IDX_UCS4: i32 = 21;
pub const IDX_UCS4LE: i32 = 22;
pub const IDX_EUC_JP: i32 = 24;
pub const IDX_SJIS: i32 = 25;
pub const IDX_EUC_KR: i32 = 26;
pub const IDX_EUC_CN: i32 = 27;
pub const IDX_EUC_TW: i32 = 28;
pub const IDX_BIG5: i32 = 29;
pub const IDX_CP932: i32 = 45;
pub const IDX_CP936: i32 = 46;
pub const IDX_CP949: i32 = 47;
pub const IDX_CP950: i32 = 48;
pub const IDX_MACROMAN: i32 = 57;
pub const IDX_COUNT: i32 = 59;
static mut enc_alias_table: [C2RustUnnamed_13; 64] = [
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ansi\x00" as *const u8 as *const i8,
            canon: IDX_LATIN_1,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"iso-8859-1\x00" as *const u8 as *const i8,
            canon: IDX_LATIN_1,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin2\x00" as *const u8 as *const i8,
            canon: IDX_ISO_2,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin3\x00" as *const u8 as *const i8,
            canon: IDX_ISO_3,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin4\x00" as *const u8 as *const i8,
            canon: IDX_ISO_4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"cyrillic\x00" as *const u8 as *const i8,
            canon: IDX_ISO_5,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"arabic\x00" as *const u8 as *const i8,
            canon: IDX_ISO_6,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"greek\x00" as *const u8 as *const i8,
            canon: IDX_ISO_7,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"hebrew\x00" as *const u8 as *const i8,
            canon: IDX_ISO_8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin5\x00" as *const u8 as *const i8,
            canon: IDX_ISO_9,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"turkish\x00" as *const u8 as *const i8,
            canon: IDX_ISO_9,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin6\x00" as *const u8 as *const i8,
            canon: IDX_ISO_10,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"nordic\x00" as *const u8 as *const i8,
            canon: IDX_ISO_10,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"thai\x00" as *const u8 as *const i8,
            canon: IDX_ISO_11,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin7\x00" as *const u8 as *const i8,
            canon: IDX_ISO_13,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin8\x00" as *const u8 as *const i8,
            canon: IDX_ISO_14,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin9\x00" as *const u8 as *const i8,
            canon: IDX_ISO_15,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf8\x00" as *const u8 as *const i8,
            canon: IDX_UTF8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"unicode\x00" as *const u8 as *const i8,
            canon: IDX_UCS2,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs2\x00" as *const u8 as *const i8,
            canon: IDX_UCS2,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs2be\x00" as *const u8 as *const i8,
            canon: IDX_UCS2,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs-2be\x00" as *const u8 as *const i8,
            canon: IDX_UCS2,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs2le\x00" as *const u8 as *const i8,
            canon: IDX_UCS2LE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf16\x00" as *const u8 as *const i8,
            canon: IDX_UTF16,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf16be\x00" as *const u8 as *const i8,
            canon: IDX_UTF16,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf-16be\x00" as *const u8 as *const i8,
            canon: IDX_UTF16,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf16le\x00" as *const u8 as *const i8,
            canon: IDX_UTF16LE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs4\x00" as *const u8 as *const i8,
            canon: IDX_UCS4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs4be\x00" as *const u8 as *const i8,
            canon: IDX_UCS4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs-4be\x00" as *const u8 as *const i8,
            canon: IDX_UCS4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs4le\x00" as *const u8 as *const i8,
            canon: IDX_UCS4LE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf32\x00" as *const u8 as *const i8,
            canon: IDX_UCS4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf-32\x00" as *const u8 as *const i8,
            canon: IDX_UCS4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf32be\x00" as *const u8 as *const i8,
            canon: IDX_UCS4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf-32be\x00" as *const u8 as *const i8,
            canon: IDX_UCS4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf32le\x00" as *const u8 as *const i8,
            canon: IDX_UCS4LE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf-32le\x00" as *const u8 as *const i8,
            canon: IDX_UCS4LE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"932\x00" as *const u8 as *const i8,
            canon: IDX_CP932,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"949\x00" as *const u8 as *const i8,
            canon: IDX_CP949,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"936\x00" as *const u8 as *const i8,
            canon: IDX_CP936,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"gbk\x00" as *const u8 as *const i8,
            canon: IDX_CP936,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"950\x00" as *const u8 as *const i8,
            canon: IDX_CP950,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"eucjp\x00" as *const u8 as *const i8,
            canon: IDX_EUC_JP,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"unix-jis\x00" as *const u8 as *const i8,
            canon: IDX_EUC_JP,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ujis\x00" as *const u8 as *const i8,
            canon: IDX_EUC_JP,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"shift-jis\x00" as *const u8 as *const i8,
            canon: IDX_SJIS,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"pck\x00" as *const u8 as *const i8,
            canon: IDX_SJIS,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"euckr\x00" as *const u8 as *const i8,
            canon: IDX_EUC_KR,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"5601\x00" as *const u8 as *const i8,
            canon: IDX_EUC_KR,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"euccn\x00" as *const u8 as *const i8,
            canon: IDX_EUC_CN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"gb2312\x00" as *const u8 as *const i8,
            canon: IDX_EUC_CN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"euctw\x00" as *const u8 as *const i8,
            canon: IDX_EUC_TW,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"japan\x00" as *const u8 as *const i8,
            canon: IDX_EUC_JP,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"korea\x00" as *const u8 as *const i8,
            canon: IDX_EUC_KR,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"prc\x00" as *const u8 as *const i8,
            canon: IDX_EUC_CN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"zh-cn\x00" as *const u8 as *const i8,
            canon: IDX_EUC_CN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"chinese\x00" as *const u8 as *const i8,
            canon: IDX_EUC_CN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"zh-tw\x00" as *const u8 as *const i8,
            canon: IDX_EUC_TW,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"taiwan\x00" as *const u8 as *const i8,
            canon: IDX_EUC_TW,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"cp950\x00" as *const u8 as *const i8,
            canon: IDX_BIG5,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"950\x00" as *const u8 as *const i8,
            canon: IDX_BIG5,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"mac\x00" as *const u8 as *const i8,
            canon: IDX_MACROMAN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"mac-roman\x00" as *const u8 as *const i8,
            canon: IDX_MACROMAN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 { name: NULL_0 as *const i8, canon: 0 };
        init
    },
];
/*
 * Find encoding "name" in the list of canonical encoding names.
 * Returns -1 if not found.
 */
unsafe extern "C" fn enc_canon_search(mut name: *const u8) -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while i < IDX_COUNT {
        if strcmp(name as *mut i8, enc_canon_table[i as usize].name as *mut i8) == 0 {
            return i;
        }
        i += 1
    }
    return -(1);
}
/*
 * Find canonical encoding "name" in the list and return its properties.
 * Returns 0 if not found.
 */
#[no_mangle]
pub unsafe extern "C" fn enc_canon_props(mut name: *const u8) -> i32 {
    let mut i: i32 = 0;
    i = enc_canon_search(name);
    if i >= 0 {
        return enc_canon_table[i as usize].prop;
    }
    if strncmp(name as *mut i8, b"2byte-\x00" as *const u8 as *const i8 as *mut i8, 6) == 0 {
        return ENC_DBCS;
    }
    if strncmp(name as *mut i8, b"8bit-\x00" as *const u8 as *const i8 as *mut i8, 5) == 0 || strncmp(name as *mut i8, b"iso-8859-\x00" as *const u8 as *const i8 as *mut i8, 9) == 0 {
        return ENC_8BIT;
    }
    return 0;
}
/*
 * Return the size of the BOM for the current buffer:
 * 0 - no BOM
 * 2 - UCS-2 or UTF-16 BOM
 * 4 - UCS-4 BOM
 * 3 - UTF-8 BOM
 */
#[no_mangle]
pub unsafe extern "C" fn bomb_size() -> i32 {
    let mut n = 0;
    if (*curbuf).b_p_bomb != 0 && (*curbuf).b_p_bin == 0 {
        if *(*curbuf).b_p_fenc as i32 == NUL || strcmp((*curbuf).b_p_fenc as *mut i8, b"utf-8\x00" as *const u8 as *const i8 as *mut i8) == 0 {
            n = 3
        } else if strncmp((*curbuf).b_p_fenc as *mut i8, b"ucs-2\x00" as *const u8 as *const i8 as *mut i8, 5) == 0 || strncmp((*curbuf).b_p_fenc as *mut i8, b"utf-16\x00" as *const u8 as *const i8 as *mut i8, 6) == 0 {
            n = 2
        } else if strncmp((*curbuf).b_p_fenc as *mut i8, b"ucs-4\x00" as *const u8 as *const i8 as *mut i8, 5) == 0 {
            n = 4
        }
    }
    return n;
}
/*
 * Remove all BOM from "s" by moving remaining text.
 */
#[no_mangle]
pub unsafe extern "C" fn remove_bom(mut s: *mut u8) {
    let mut p = s as *mut i8;
    loop {
        p = strchr(p, 0xef as i32);
        if p.is_null() {
            break;
        }
        if *p.offset(1) as u8 as i32 == 0xbb as i32 && *p.offset(2) as u8 as i32 == 0xbf as i32 {
            memmove(p as *mut libc::c_void, p.offset(3) as *const libc::c_void, strlen(p.offset(3)).wrapping_add(1));
        } else {
            p = p.offset(1)
        }
    }
}
/*
 * Get class of pointer:
 * 0 for blank or NUL
 * 1 for punctuation
 * 2 for an (ASCII) word character
 * >2 for other word characters
 */
#[no_mangle]
pub unsafe extern "C" fn mb_get_class(mut p: *const u8) -> i32 {
    return mb_get_class_tab(p, (*curbuf).b_chartab.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mb_get_class_tab(mut p: *const u8, chartab: *const u64) -> i32 {
    if utf8len_tab[*p.offset(0) as usize] as i32 == 1 {
        if *p.offset(0) as i32 == NUL || ascii_iswhite(*p.offset(0) as char) as i32 != 0 {
            return 0;
        }
        if vim_iswordc_tab(*p.offset(0) as i32, chartab) {
            return 2;
        }
        return 1;
    }
    return utf_class_tab(utf_ptr2char(p), chartab);
}
/*
 * Return true if "c" is in "table".
 */
unsafe extern "C" fn intable(mut table: *const interval, mut n_items: size_t, mut c: i32) -> bool {
    let mut mid: i32 = 0;
    let mut bot: i32 = 0;
    let mut top: i32 = 0;
    /* first quick check for Latin1 etc. characters */
    if (c as i64) < (*table.offset(0)).first {
        return false;
    }
    /* binary search in table */
    bot = 0;
    top = n_items.wrapping_sub(1) as i32;
    while top >= bot {
        mid = (bot + top) / 2;
        if (*table.offset(mid as isize)).last < c as i64 {
            bot = mid + 1
        } else if (*table.offset(mid as isize)).first > c as i64 {
            top = mid - 1
        } else {
            return true;
        }
    }
    return false;
}
// / For UTF-8 character "c" return 2 for a double-width character, 1 for others.
// / Returns 4 or 6 for an unprintable character.
// / Is only correct for characters >= 0x80.
// / When p_ambw is "double", return 2 for a character with East Asian Width
// / class 'A'(mbiguous).
// /
// / @note Tables `doublewidth` and `ambiguous` are generated by
// /       gen_unicode_tables.lua, which must be manually invoked as needed.
#[no_mangle]
pub unsafe extern "C" fn utf_char2cells(mut c: i32) -> i32 {
    if c >= 0x100 as i32 {
        if !utf_printable(c) {
            return 6;
            // unprintable, displays <xxxx>
        }
        if intable(
            doublewidth.as_ptr(),
            (::std::mem::size_of::<[interval; 113]>() as u64)
                .wrapping_div(::std::mem::size_of::<interval>() as u64)
                .wrapping_div(((::std::mem::size_of::<[interval; 113]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as u64) as usize,
            c,
        ) {
            return 2;
        }
        if p_emoji != 0
            && intable(
                emoji_width.as_ptr(),
                (::std::mem::size_of::<[interval; 39]>() as u64)
                    .wrapping_div(::std::mem::size_of::<interval>() as u64)
                    .wrapping_div(((::std::mem::size_of::<[interval; 39]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as u64) as usize,
                c,
            ) as i32
                != 0
        {
            return 2;
        }
    } else if c >= 0x80 as i32 && !vim_isprintc(c) {
        // Characters below 0x100 are influenced by 'isprint' option.
        return 4;
        // unprintable, displays <xx>
    }
    if c >= 0x80 as i32
        && *p_ambw as i32 == 'd' as i32
        && intable(
            ambiguous.as_ptr(),
            (::std::mem::size_of::<[interval; 179]>() as u64)
                .wrapping_div(::std::mem::size_of::<interval>() as u64)
                .wrapping_div(((::std::mem::size_of::<[interval; 179]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as u64) as usize,
            c,
        ) as i32
            != 0
    {
        return 2;
    }
    return 1;
}
// / Return the number of display cells character at "*p" occupies.
// / This doesn't take care of unprintable characters, use ptr2cells() for that.
#[no_mangle]
pub unsafe extern "C" fn utf_ptr2cells(mut p: *const u8) -> i32 {
    let mut c: i32 = 0;
    /* Need to convert to a wide character. */
    if *p as i32 >= 0x80 as i32 {
        c = utf_ptr2char(p);
        /* An illegal byte is displayed as <xx>. */
        if utf_ptr2len(p) == 1 || c == NUL {
            return 4;
        }
        /* If the char is ASCII it must be an overlong sequence. */
        if c < 0x80 as i32 {
            return char2cells(c);
        }
        return utf_char2cells(c);
    }
    return 1;
}
// / Like utf_ptr2cells(), but limit string length to "size".
// / For an empty string or truncated character returns 1.
#[no_mangle]
pub unsafe extern "C" fn utf_ptr2cells_len(mut p: *const u8, mut size: i32) -> i32 {
    let mut c: i32 = 0;
    /* Need to convert to a wide character. */
    if size > 0 && *p as i32 >= 0x80 as i32 {
        if utf_ptr2len_len(p, size) < utf8len_tab[*p as usize] as i32 {
            return 1;
        } /* truncated */
        c = utf_ptr2char(p);
        /* An illegal byte is displayed as <xx>. */
        if utf_ptr2len(p) == 1 || c == NUL {
            return 4;
        }
        /* If the char is ASCII it must be an overlong sequence. */
        if c < 0x80 as i32 {
            return char2cells(c);
        }
        return utf_char2cells(c);
    }
    return 1;
}
// / Calculate the number of cells occupied by string `str`.
// /
// / @param str The source string, may not be NULL, must be a NUL-terminated
// /            string.
// / @return The number of cells occupied by string `str`
#[no_mangle]
pub unsafe extern "C" fn mb_string2cells(mut str: *const u8) -> size_t {
    let mut clen = 0;
    let mut p = str;
    while *p as i32 != NUL {
        clen = (clen as u64).wrapping_add(utf_ptr2cells(p) as u64) as size_t as size_t;
        p = p.offset(Some(Some(mb_ptr2len).expect("non-null function pointer")).expect("non-null function pointer")(p) as isize)
    }
    return clen;
}
// / Get the number of cells occupied by string `str` with maximum length `size`
// /
// / @param str The source string, may not be NULL, must be a NUL-terminated
// /            string.
// / @param size maximum length of string. It will terminate on earlier NUL.
// / @return The number of cells occupied by string `str`
#[no_mangle]
pub unsafe extern "C" fn mb_string2cells_len(mut str: *const u8, mut size: size_t) -> size_t {
    let mut clen = 0;
    let mut p = str;
    while *p as i32 != NUL && p < str.offset(size as isize) {
        clen = (clen as u64).wrapping_add(utf_ptr2cells(p) as u64) as size_t as size_t;
        p = p.offset(utf_ptr2len_len(p, size.wrapping_add(p.offset_from(str) as usize) as i32) as isize)
    }
    return clen;
}
// / Convert a UTF-8 byte sequence to a wide character
// /
// / If the sequence is illegal or truncated by a NUL then the first byte is
// / returned.
// / For an overlong sequence this may return zero.
// / Does not include composing characters for obvious reasons.
// /
// / @param[in]  p  String to convert.
// /
// / @return Unicode codepoint or byte value.
#[no_mangle]
pub unsafe extern "C" fn utf_ptr2char(p: *const u8) -> i32 {
    if (*p.offset(0) as i32) < 0x80 as i32 {
        // Be quick for ASCII.
        return *p.offset(0) as i32;
    }
    let len = utf8len_tab_zero[*p.offset(0) as usize];
    if len as i32 > 1 && *p.offset(1) as i32 & 0xc0 as i32 == 0x80 as i32 {
        if len as i32 == 2 {
            return ((*p.offset(0) as i32 & 0x1f as i32) << 6) + (*p.offset(1) as i32 & 0x3f as i32);
        }
        if *p.offset(2) as i32 & 0xc0 as i32 == 0x80 as i32 {
            if len as i32 == 3 {
                return ((*p.offset(0) as i32 & 0xf as i32) << 12) + ((*p.offset(1) as i32 & 0x3f as i32) << 6) + (*p.offset(2) as i32 & 0x3f as i32);
            }
            if *p.offset(3) as i32 & 0xc0 as i32 == 0x80 as i32 {
                if len as i32 == 4 {
                    return ((*p.offset(0) as i32 & 0x7 as i32) << 18) + ((*p.offset(1) as i32 & 0x3f as i32) << 12) + ((*p.offset(2) as i32 & 0x3f as i32) << 6) + (*p.offset(3) as i32 & 0x3f as i32);
                }
                if *p.offset(4) as i32 & 0xc0 as i32 == 0x80 as i32 {
                    if len as i32 == 5 {
                        return ((*p.offset(0) as i32 & 0x3 as i32) << 24) + ((*p.offset(1) as i32 & 0x3f as i32) << 18) + ((*p.offset(2) as i32 & 0x3f as i32) << 12) + ((*p.offset(3) as i32 & 0x3f as i32) << 6) + (*p.offset(4) as i32 & 0x3f as i32);
                    }
                    if *p.offset(5) as i32 & 0xc0 as i32 == 0x80 as i32 && len as i32 == 6 {
                        return ((*p.offset(0) as i32 & 0x1 as i32) << 30) + ((*p.offset(1) as i32 & 0x3f as i32) << 24) + ((*p.offset(2) as i32 & 0x3f as i32) << 18) + ((*p.offset(3) as i32 & 0x3f as i32) << 12) + ((*p.offset(4) as i32 & 0x3f as i32) << 6) + (*p.offset(5) as i32 & 0x3f as i32);
                    }
                }
            }
        }
    }
    // Illegal value: just return the first byte.
    return *p.offset(0) as i32;
}
/*
 * Convert a UTF-8 byte sequence to a wide character.
 * String is assumed to be terminated by NUL or after "n" bytes, whichever
 * comes first.
 * The function is safe in the sense that it never accesses memory beyond the
 * first "n" bytes of "s".
 *
 * On success, returns decoded codepoint, advances "s" to the beginning of
 * next character and decreases "n" accordingly.
 *
 * If end of string was reached, returns 0 and, if "n" > 0, advances "s" past
 * NUL byte.
 *
 * If byte sequence is illegal or incomplete, returns -1 and does not advance
 * "s".
 */
unsafe extern "C" fn utf_safe_read_char_adv(mut s: *mut *const u8, mut n: *mut size_t) -> i32 {
    let mut c: i32 = 0;
    if *n == 0 {
        /* end of buffer */
        return 0;
    }
    let mut k = utf8len_tab_zero[**s as usize];
    if k as i32 == 1 {
        /* ASCII character or NUL */
        *n = (*n).wrapping_sub(1);
        let fresh3 = *s;
        *s = (*s).offset(1);
        return *fresh3 as i32;
    }
    if k as u64 <= *n as u64 {
        /* We have a multibyte sequence and it isn't truncated by buffer
         * limits so utf_ptr2char() is safe to use. Or the first byte is
         * illegal (k=0), and it's also safe to use utf_ptr2char(). */
        c = utf_ptr2char(*s);
        /* On failure, utf_ptr2char() returns the first byte, so here we
         * check equality with the first byte. The only non-ASCII character
         * which equals the first byte of its own UTF-8 representation is
         * U+00C3 (UTF-8: 0xC3 0x83), so need to check that special case too.
         * It's safe even if n=1, else we would have k=2 > n. */
        if c != **s as i32 || c == 0xc3 as i32 && *(*s).offset(1) as i32 == 0x83 as i32 {
            /* byte sequence was successfully decoded */
            *s = (*s).offset(k as i32 as isize);
            *n = (*n as u64).wrapping_sub(k as u64) as size_t as size_t;
            return c;
        }
    }
    /* byte sequence is incomplete or illegal */
    return -(1);
}
/*
 * Get character at **pp and advance *pp to the next character.
 * Note: composing characters are skipped!
 */
#[no_mangle]
pub unsafe extern "C" fn mb_ptr2char_adv(pp: *mut *const u8) -> i32 {
    let mut c: i32 = 0;
    c = utf_ptr2char(*pp);
    *pp = (*pp).offset(Some(Some(mb_ptr2len).expect("non-null function pointer")).expect("non-null function pointer")(*pp) as isize);
    return c;
}
/*
 * Get character at **pp and advance *pp to the next character.
 * Note: composing characters are returned as separate characters.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_cptr2char_adv(mut pp: *mut *const u8) -> i32 {
    let mut c: i32 = 0;
    c = utf_ptr2char(*pp);
    *pp = (*pp).offset(utf_ptr2len(*pp) as isize);
    return c;
}
/*
 * Check if the character pointed to by "p2" is a composing character when it
 * comes after "p1".  For Arabic sometimes "ab" is replaced with "c", which
 * behaves like a composing character.
 */
#[no_mangle]
pub unsafe extern "C" fn utf_composinglike(mut p1: *const u8, mut p2: *const u8) -> bool {
    let mut c2: i32 = 0;
    c2 = utf_ptr2char(p2);
    if utf_iscomposing(c2) {
        return true;
    }
    if !arabic_maycombine(c2) {
        return false;
    }
    return arabic_combine(utf_ptr2char(p1), c2);
}
// / Convert a UTF-8 string to a wide character
// /
// / Also gets up to #MAX_MCO composing characters.
// /
// / @param[out]  pcc  Location where to store composing characters. Must have
// /                   space at least for #MAX_MCO + 1 elements.
// /
// / @return leading character.
#[no_mangle]
pub unsafe extern "C" fn utfc_ptr2char(mut p: *const u8, mut pcc: *mut i32) -> i32 {
    let mut len: i32 = 0;
    let mut c: i32 = 0;
    let mut cc: i32 = 0;
    let mut i = 0;
    c = utf_ptr2char(p);
    len = utf_ptr2len(p);
    /* Only accept a composing char when the first char isn't illegal. */
    if (len > 1 || (*p as i32) < 0x80 as i32) && *p.offset(len as isize) as i32 >= 0x80 as i32 && utf_composinglike(p, p.offset(len as isize)) as i32 != 0 {
        cc = utf_ptr2char(p.offset(len as isize));
        loop {
            let fresh4 = i;
            i = i + 1;
            *pcc.offset(fresh4 as isize) = cc;
            if i == MAX_MCO {
                break;
            }
            len += utf_ptr2len(p.offset(len as isize));
            if (*p.offset(len as isize) as i32) < 0x80 as i32 || {
                cc = utf_ptr2char(p.offset(len as isize));
                !utf_iscomposing(cc)
            } {
                break;
            }
        }
    }
    if i < MAX_MCO {
        /* last composing char must be 0 */
        *pcc.offset(i as isize) = 0
    }
    return c;
}
/*
 * Convert a UTF-8 byte string to a wide character.  Also get up to MAX_MCO
 * composing characters.  Use no more than p[maxlen].
 *
 * @param [out] pcc: composing chars, last one is 0
 */
#[no_mangle]
pub unsafe extern "C" fn utfc_ptr2char_len(mut p: *const u8, mut pcc: *mut i32, mut maxlen: i32) -> i32 {
    if maxlen > 0 {
    } else {
        assert!(false, "maxlen > 0");
    }
    let mut i = 0;
    let mut len = utf_ptr2len_len(p, maxlen);
    // Is it safe to use utf_ptr2char()?
    let mut safe = len > 1 && len <= maxlen;
    let mut c = if safe as i32 != 0 { utf_ptr2char(p) } else { *p as i32 };
    // Only accept a composing char when the first char isn't illegal.
    if (safe as i32 != 0 || c < 0x80 as i32) && len < maxlen && *p.offset(len as isize) as i32 >= 0x80 as i32 {
        while i < MAX_MCO {
            let mut len_cc = utf_ptr2len_len(p.offset(len as isize), maxlen - len);
            safe = len_cc > 1 && len_cc <= maxlen - len;
            if !safe
                || {
                    let ref mut fresh5 = *pcc.offset(i as isize);
                    *fresh5 = utf_ptr2char(p.offset(len as isize));
                    (*fresh5) < 0x80 as i32
                }
                || (if i == 0 { utf_composinglike(p, p.offset(len as isize)) as i32 } else { utf_iscomposing(*pcc.offset(i as isize)) as i32 }) == 0
            {
                break;
            }
            len += len_cc;
            i += 1
        }
    }
    if i < MAX_MCO {
        // last composing char must be 0
        *pcc.offset(i as isize) = 0
    }
    return c;
}
// / Get the length of a UTF-8 byte sequence representing a single codepoint
// /
// / @param[in]  p  UTF-8 string.
// /
// / @return Sequence length, 0 for empty string and 1 for non-UTF-8 byte
// /         sequence.
#[no_mangle]
pub unsafe extern "C" fn utf_ptr2len(p: *const u8) -> i32 {
    if *p as i32 == NUL {
        return 0;
    }
    let len = utf8len_tab[*p as usize] as i32;
    let mut i = 1;
    while i < len {
        if *p.offset(i as isize) as i32 & 0xc0 as i32 != 0x80 as i32 {
            return 1;
        }
        i += 1
    }
    return len;
}
/*
 * Return length of UTF-8 character, obtained from the first byte.
 * "b" must be between 0 and 255!
 * Returns 1 for an invalid first byte value.
 */
#[no_mangle]
pub unsafe extern "C" fn utf_byte2len(mut b: i32) -> i32 {
    return utf8len_tab[b as usize] as i32;
}
/*
 * Get the length of UTF-8 byte sequence "p[size]".  Does not include any
 * following composing characters.
 * Returns 1 for "".
 * Returns 1 for an illegal byte sequence (also in incomplete byte seq.).
 * Returns number > "size" for an incomplete byte sequence.
 * Never returns zero.
 */
#[no_mangle]
pub unsafe extern "C" fn utf_ptr2len_len(mut p: *const u8, mut size: i32) -> i32 {
    let mut len: i32 = 0; /* NUL, ascii or illegal lead byte */
    let mut i: i32 = 0; /* incomplete byte sequence. */
    let mut m: i32 = 0;
    len = utf8len_tab[*p as usize] as i32;
    if len == 1 {
        return 1;
    }
    if len > size {
        m = size
    } else {
        m = len
    }
    i = 1;
    while i < m {
        if *p.offset(i as isize) as i32 & 0xc0 as i32 != 0x80 as i32 {
            return 1;
        }
        i += 1
    }
    return len;
}
// / Return the number of bytes occupied by a UTF-8 character in a string
// /
// / This includes following composing characters.
#[no_mangle]
pub unsafe extern "C" fn utfc_ptr2len(p: *const u8) -> i32 {
    let mut b0 = *p;
    if b0 as i32 == NUL {
        return 0;
    }
    if (b0 as i32) < 0x80 as i32 && (*p.offset(1) as i32) < 0x80 as i32 {
        // be quick for ASCII
        return 1;
    }
    // Skip over first UTF-8 char, stopping at a NUL byte.
    let mut len = utf_ptr2len(p);
    // Check for illegal byte.
    if len == 1 && b0 as i32 >= 0x80 as i32 {
        return 1;
    }
    // Check for composing characters.  We can handle only the first six, but
    // skip all of them (otherwise the cursor would get stuck).
    let mut prevlen = 0;
    loop {
        if (*p.offset(len as isize) as i32) < 0x80 as i32 || !utf_composinglike(p.offset(prevlen as isize), p.offset(len as isize)) {
            return len;
        }
        // Skip over composing char.
        prevlen = len;
        len += utf_ptr2len(p.offset(len as isize))
    }
}
/*
 * Return the number of bytes the UTF-8 encoding of the character at "p[size]"
 * takes.  This includes following composing characters.
 * Returns 0 for an empty string.
 * Returns 1 for an illegal char or an incomplete byte sequence.
 */
#[no_mangle]
pub unsafe extern "C" fn utfc_ptr2len_len(mut p: *const u8, mut size: i32) -> i32 {
    let mut len: i32 = 0;
    let mut prevlen: i32 = 0;
    if size < 1 || *p as i32 == NUL {
        return 0;
    }
    if (*p.offset(0) as i32) < 0x80 as i32 && (size == 1 || (*p.offset(1) as i32) < 0x80 as i32) {
        /* be quick for ASCII */
        return 1;
    }
    /* Skip over first UTF-8 char, stopping at a NUL byte. */
    len = utf_ptr2len_len(p, size);
    /* Check for illegal byte and incomplete byte sequence. */
    if len == 1 && *p.offset(0) as i32 >= 0x80 as i32 || len > size {
        return 1;
    }
    /*
     * Check for composing characters.  We can handle only the first six, but
     * skip all of them (otherwise the cursor would get stuck).
     */
    prevlen = 0;
    while len < size {
        let mut len_next_char: i32 = 0;
        if (*p.offset(len as isize) as i32) < 0x80 as i32 {
            break;
        }
        /*
         * Next character length should not go beyond size to ensure that
         * UTF_COMPOSINGLIKE(...) does not read beyond size.
         */
        len_next_char = utf_ptr2len_len(p.offset(len as isize), size - len);
        if len_next_char > size - len {
            break;
        }
        if !utf_composinglike(p.offset(prevlen as isize), p.offset(len as isize)) {
            break;
        }
        /* Skip over composing char */
        prevlen = len;
        len += len_next_char
    }
    return len;
}
// / Determine how many bytes certain unicode codepoint will occupy
#[no_mangle]
pub unsafe extern "C" fn utf_char2len(c: i32) -> i32 {
    if c < 0x80 as i32 {
        return 1;
    } else if c < 0x800 as i32 {
        return 2;
    } else if c < 0x10000 as i32 {
        return 3;
    } else if c < 0x200000 as i32 {
        return 4;
    } else if c < 0x4000000 as i32 {
        return 5;
    } else {
        return 6;
    };
}
// / Convert Unicode character to UTF-8 string
// /
// / @param c character to convert to \p buf
// / @param[out] buf UTF-8 string generated from \p c, does not add \0
// / @return Number of bytes (1-6).
#[no_mangle]
pub unsafe extern "C" fn utf_char2bytes(c: i32, buf: *mut u8) -> i32 {
    if c < 0x80 as i32 {
        // 7 bits
        *buf.offset(0) = c as u8;
        return 1;
    } else if c < 0x800 as i32 {
        // 11 bits
        *buf.offset(0) = (0xc0 as i32 as u32).wrapping_add(c as u32 >> 6) as u8;
        *buf.offset(1) = (0x80 as i32 + (c & 0x3f as i32)) as u8;
        return 2;
    } else if c < 0x10000 as i32 {
        // 16 bits
        *buf.offset(0) = (0xe0 as i32 as u32).wrapping_add(c as u32 >> 12) as u8;
        *buf.offset(1) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 6 & 0x3f as i32 as u32) as u8;
        *buf.offset(2) = (0x80 as i32 + (c & 0x3f as i32)) as u8;
        return 3;
    } else if c < 0x200000 as i32 {
        // 21 bits
        *buf.offset(0) = (0xf0 as i32 as u32).wrapping_add(c as u32 >> 18) as u8; // 31 bits
        *buf.offset(1) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 12 & 0x3f as i32 as u32) as u8;
        *buf.offset(2) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 6 & 0x3f as i32 as u32) as u8;
        *buf.offset(3) = (0x80 as i32 + (c & 0x3f as i32)) as u8;
        return 4;
    } else if c < 0x4000000 as i32 {
        // 26 bits
        *buf.offset(0) = (0xf8 as i32 as u32).wrapping_add(c as u32 >> 24) as u8;
        *buf.offset(1) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 18 & 0x3f as i32 as u32) as u8;
        *buf.offset(2) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 12 & 0x3f as i32 as u32) as u8;
        *buf.offset(3) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 6 & 0x3f as i32 as u32) as u8;
        *buf.offset(4) = (0x80 as i32 + (c & 0x3f as i32)) as u8;
        return 5;
    } else {
        *buf.offset(0) = (0xfc as i32 as u32).wrapping_add(c as u32 >> 30) as u8;
        *buf.offset(1) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 24 & 0x3f as i32 as u32) as u8;
        *buf.offset(2) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 18 & 0x3f as i32 as u32) as u8;
        *buf.offset(3) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 12 & 0x3f as i32 as u32) as u8;
        *buf.offset(4) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 6 & 0x3f as i32 as u32) as u8;
        *buf.offset(5) = (0x80 as i32 + (c & 0x3f as i32)) as u8;
        return 6;
    };
}
/*
 * Return true if "c" is a composing UTF-8 character.  This means it will be
 * drawn on top of the preceding character.
 * Based on code from Markus Kuhn.
 */
#[no_mangle]
pub unsafe extern "C" fn utf_iscomposing(mut c: i32) -> bool {
    return intable(
        combining.as_ptr(),
        (::std::mem::size_of::<[interval; 280]>() as u64)
            .wrapping_div(::std::mem::size_of::<interval>() as u64)
            .wrapping_div(((::std::mem::size_of::<[interval; 280]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as u64) as usize,
        c,
    );
}
/*
 * Return true for characters that can be displayed in a normal way.
 * Only for characters of 0x100 and above!
 */
#[no_mangle]
pub unsafe extern "C" fn utf_printable(mut c: i32) -> bool {
    /* Sorted list of non-overlapping intervals.
     * 0xd800-0xdfff is reserved for UTF-16, actually illegal. */
    static mut nonprint: [interval; 9] = [
        {
            let mut init = interval { first: 0x70f as i32 as i64, last: 0x70f as i32 as i64 };
            init
        },
        {
            let mut init = interval {
                first: 0x180b as i32 as i64,
                last: 0x180e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x200b as i32 as i64,
                last: 0x200f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x202a as i32 as i64,
                last: 0x202e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x206a as i32 as i64,
                last: 0x206f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xd800 as i32 as i64,
                last: 0xdfff as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfeff as i32 as i64,
                last: 0xfeff as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfff9 as i32 as i64,
                last: 0xfffb as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfffe as i32 as i64,
                last: 0xffff as i32 as i64,
            };
            init
        },
    ];
    return !intable(
        nonprint.as_mut_ptr(),
        (::std::mem::size_of::<[interval; 9]>() as u64)
            .wrapping_div(::std::mem::size_of::<interval>() as u64)
            .wrapping_div(((::std::mem::size_of::<[interval; 9]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as u64) as usize,
        c,
    );
}
/*
 * Get class of a Unicode character.
 * 0: white space
 * 1: punctuation
 * 2 or bigger: some class of word character.
 */
#[no_mangle]
pub unsafe extern "C" fn utf_class(c: i32) -> i32 {
    return utf_class_tab(c, (*curbuf).b_chartab.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn utf_class_tab(c: i32, chartab: *const u64) -> i32 {
    /* sorted list of non-overlapping intervals */
    static mut classes: [clinterval; 71] = [
        {
            let mut init = clinterval {
                first: 0x37e as i32 as u32,
                last: 0x37e as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x387 as i32 as u32,
                last: 0x387 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x55a as i32 as u32,
                last: 0x55f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x589 as i32 as u32,
                last: 0x589 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x5be as i32 as u32,
                last: 0x5be as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x5c0 as i32 as u32,
                last: 0x5c0 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x5c3 as i32 as u32,
                last: 0x5c3 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x5f3 as i32 as u32,
                last: 0x5f4 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x60c as i32 as u32,
                last: 0x60c as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x61b as i32 as u32,
                last: 0x61b as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x61f as i32 as u32,
                last: 0x61f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x66a as i32 as u32,
                last: 0x66d as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x6d4 as i32 as u32,
                last: 0x6d4 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x700 as i32 as u32,
                last: 0x70d as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x964 as i32 as u32,
                last: 0x965 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x970 as i32 as u32,
                last: 0x970 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xdf4 as i32 as u32,
                last: 0xdf4 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xe4f as i32 as u32,
                last: 0xe4f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xe5a as i32 as u32,
                last: 0xe5b as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xf04 as i32 as u32,
                last: 0xf12 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xf3a as i32 as u32,
                last: 0xf3d as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xf85 as i32 as u32,
                last: 0xf85 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x104a as i32 as u32,
                last: 0x104f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x10fb as i32 as u32,
                last: 0x10fb as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1361 as i32 as u32,
                last: 0x1368 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x166d as i32 as u32,
                last: 0x166e as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1680 as i32 as u32,
                last: 0x1680 as i32 as u32,
                class: 0,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x169b as i32 as u32,
                last: 0x169c as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x16eb as i32 as u32,
                last: 0x16ed as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1735 as i32 as u32,
                last: 0x1736 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x17d4 as i32 as u32,
                last: 0x17dc as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1800 as i32 as u32,
                last: 0x180a as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2000 as i32 as u32,
                last: 0x200b as i32 as u32,
                class: 0,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x200c as i32 as u32,
                last: 0x2027 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2028 as i32 as u32,
                last: 0x2029 as i32 as u32,
                class: 0,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x202a as i32 as u32,
                last: 0x202e as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x202f as i32 as u32,
                last: 0x202f as i32 as u32,
                class: 0,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2030 as i32 as u32,
                last: 0x205e as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x205f as i32 as u32,
                last: 0x205f as i32 as u32,
                class: 0,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2060 as i32 as u32,
                last: 0x27ff as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2070 as i32 as u32,
                last: 0x207f as i32 as u32,
                class: 0x2070 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2080 as i32 as u32,
                last: 0x2094 as i32 as u32,
                class: 0x2080 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x20a0 as i32 as u32,
                last: 0x27ff as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2800 as i32 as u32,
                last: 0x28ff as i32 as u32,
                class: 0x2800 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2900 as i32 as u32,
                last: 0x2998 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x29d8 as i32 as u32,
                last: 0x29db as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x29fc as i32 as u32,
                last: 0x29fd as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2e00 as i32 as u32,
                last: 0x2e7f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x3000 as i32 as u32,
                last: 0x3000 as i32 as u32,
                class: 0,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x3001 as i32 as u32,
                last: 0x3020 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x3030 as i32 as u32,
                last: 0x3030 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x303d as i32 as u32,
                last: 0x303d as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x3040 as i32 as u32,
                last: 0x309f as i32 as u32,
                class: 0x3040 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x30a0 as i32 as u32,
                last: 0x30ff as i32 as u32,
                class: 0x30a0 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x3300 as i32 as u32,
                last: 0x9fff as i32 as u32,
                class: 0x4e00 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xac00 as i32 as u32,
                last: 0xd7a3 as i32 as u32,
                class: 0xac00 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xf900 as i32 as u32,
                last: 0xfaff as i32 as u32,
                class: 0x4e00 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xfd3e as i32 as u32,
                last: 0xfd3f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xfe30 as i32 as u32,
                last: 0xfe6b as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xff00 as i32 as u32,
                last: 0xff0f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xff1a as i32 as u32,
                last: 0xff20 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xff3b as i32 as u32,
                last: 0xff40 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xff5b as i32 as u32,
                last: 0xff65 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1d000 as i32 as u32,
                last: 0x1d24f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1d400 as i32 as u32,
                last: 0x1d7ff as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1f000 as i32 as u32,
                last: 0x1f2ff as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1f300 as i32 as u32,
                last: 0x1f9ff as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x20000 as i32 as u32,
                last: 0x2a6df as i32 as u32,
                class: 0x4e00 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2a700 as i32 as u32,
                last: 0x2b73f as i32 as u32,
                class: 0x4e00 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2b740 as i32 as u32,
                last: 0x2b81f as i32 as u32,
                class: 0x4e00 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2f800 as i32 as u32,
                last: 0x2fa1f as i32 as u32,
                class: 0x4e00 as i32 as u32,
            };
            init
        },
    ];
    let mut bot = 0;
    let mut top = (::std::mem::size_of::<[clinterval; 71]>() as u64)
        .wrapping_div(::std::mem::size_of::<clinterval>() as u64)
        .wrapping_div(((::std::mem::size_of::<[clinterval; 71]>() as u64).wrapping_rem(::std::mem::size_of::<clinterval>() as u64) == 0) as u64)
        .wrapping_sub(1) as i32;
    let mut mid: i32 = 0;
    /* First quick check for Latin1 characters, use 'iskeyword'. */
    if c < 0x100 as i32 {
        if c == ' ' as i32 || c == '\t' as i32 || c == NUL || c == 0xa0 as i32 {
            return 0;
            // blank
        }
        if vim_iswordc_tab(c, chartab) {
            return 2;
            // punctuation
            // word character
        }
        return 1;
    }
    /* binary search in table */
    while top >= bot {
        mid = (bot + top) / 2;
        if classes[mid as usize].last < c as u32 {
            bot = mid + 1
        } else if classes[mid as usize].first > c as u32 {
            top = mid - 1
        } else {
            return classes[mid as usize].class as i32;
        }
    }
    // emoji
    if intable(
        emoji_all.as_ptr(),
        (::std::mem::size_of::<[interval; 151]>() as u64)
            .wrapping_div(::std::mem::size_of::<interval>() as u64)
            .wrapping_div(((::std::mem::size_of::<[interval; 151]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as u64) as usize,
        c,
    ) {
        return 3;
    }
    /* most other characters are "word" characters */
    return 2;
}
#[no_mangle]
pub unsafe extern "C" fn utf_ambiguous_width(mut c: i32) -> bool {
    return c >= 0x80 as i32
        && (intable(
            ambiguous.as_ptr(),
            (::std::mem::size_of::<[interval; 179]>() as u64)
                .wrapping_div(::std::mem::size_of::<interval>() as u64)
                .wrapping_div(((::std::mem::size_of::<[interval; 179]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as u64) as usize,
            c,
        ) as i32
            != 0
            || intable(
                emoji_all.as_ptr(),
                (::std::mem::size_of::<[interval; 151]>() as u64)
                    .wrapping_div(::std::mem::size_of::<interval>() as u64)
                    .wrapping_div(((::std::mem::size_of::<[interval; 151]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as u64) as usize,
                c,
            ) as i32
                != 0);
}
/*
 * Generic conversion function for case operations.
 * Return the converted equivalent of "a", which is a UCS-4 character.  Use
 * the given conversion "table".  Uses binary search on "table".
 */
unsafe extern "C" fn utf_convert(mut a: i32, table: *const convertStruct, mut n_items: size_t) -> i32 {
    let mut start: size_t = 0; /* indices into table */
    let mut mid: size_t = 0;
    let mut end: size_t = 0;
    start = 0;
    end = n_items;
    while start < end {
        /* need to search further */
        mid = end.wrapping_add(start).wrapping_div(2);
        if (*table.offset(mid as isize)).rangeEnd < a {
            start = mid.wrapping_add(1)
        } else {
            end = mid
        }
    }
    if start < n_items && (*table.offset(start as isize)).rangeStart <= a && a <= (*table.offset(start as isize)).rangeEnd && (a - (*table.offset(start as isize)).rangeStart) % (*table.offset(start as isize)).step == 0 {
        return a + (*table.offset(start as isize)).offset;
    } else {
        return a;
    };
}
/*
 * Return the folded-case equivalent of "a", which is a UCS-4 character.  Uses
 * simple case folding.
 */
#[no_mangle]
pub unsafe extern "C" fn utf_fold(mut a: i32) -> i32 {
    if a < 0x80 as i32 {
        // be fast for ASCII
        return if a >= 0x41 as i32 && a <= 0x5a as i32 { (a) + 32 } else { a };
    }
    return utf_convert(
        a,
        foldCase.as_ptr(),
        (::std::mem::size_of::<[convertStruct; 192]>() as u64)
            .wrapping_div(::std::mem::size_of::<convertStruct>() as u64)
            .wrapping_div(((::std::mem::size_of::<[convertStruct; 192]>() as u64).wrapping_rem(::std::mem::size_of::<convertStruct>() as u64) == 0) as u64) as usize,
    );
}
// Vim's own character class functions.  These exist because many library
// islower()/toupper() etc. do not work properly: they crash when used with
// invalid values or can't handle latin1 when the locale is C.
// Speed is most important here.
// / Return the upper-case equivalent of "a", which is a UCS-4 character.  Use
// / simple case folding.
#[no_mangle]
pub unsafe extern "C" fn mb_toupper(mut a: i32) -> i32 {
    /* If 'casemap' contains "keepascii" use ASCII style toupper(). */
    if a < 128 && cmp_flags & CMP_KEEPASCII as u32 != 0 {
        return if a < 'a' as i32 || a > 'z' as i32 { a } else { (a) - ('a' as i32 - 'A' as i32) };
    }
    /* If towupper() is available and handles Unicode, use it. */
    if cmp_flags & CMP_INTERNAL as u32 == 0 {
        return towupper(a as wint_t) as i32;
    }
    /* For characters below 128 use locale sensitive toupper(). */
    if a < 128 {
        return toupper(a);
    }
    /* For any other characters use the above mapping table. */
    return utf_convert(
        a,
        toUpper.as_ptr(),
        (::std::mem::size_of::<[convertStruct; 187]>() as u64)
            .wrapping_div(::std::mem::size_of::<convertStruct>() as u64)
            .wrapping_div(((::std::mem::size_of::<[convertStruct; 187]>() as u64).wrapping_rem(::std::mem::size_of::<convertStruct>() as u64) == 0) as u64) as usize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mb_islower(mut a: i32) -> bool {
    // German sharp s is lower case but has no upper case equivalent.
    return mb_toupper(a) != a || a == 0xdf as i32;
}
// / Return the lower-case equivalent of "a", which is a UCS-4 character.  Use
// / simple case folding.
#[no_mangle]
pub unsafe extern "C" fn mb_tolower(mut a: i32) -> i32 {
    /* If 'casemap' contains "keepascii" use ASCII style tolower(). */
    if a < 128 && cmp_flags & CMP_KEEPASCII as u32 != 0 {
        return if a < 'A' as i32 || a > 'Z' as i32 { a } else { (a) + ('a' as i32 - 'A' as i32) };
    }
    /* If towlower() is available and handles Unicode, use it. */
    if cmp_flags & CMP_INTERNAL as u32 == 0 {
        return towlower(a as wint_t) as i32;
    }
    /* For characters below 128 use locale sensitive tolower(). */
    if a < 128 {
        return tolower(a);
    }
    /* For any other characters use the above mapping table. */
    return utf_convert(
        a,
        toLower.as_ptr(),
        (::std::mem::size_of::<[convertStruct; 172]>() as u64)
            .wrapping_div(::std::mem::size_of::<convertStruct>() as u64)
            .wrapping_div(((::std::mem::size_of::<[convertStruct; 172]>() as u64).wrapping_rem(::std::mem::size_of::<convertStruct>() as u64) == 0) as u64) as usize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mb_isupper(mut a: i32) -> bool {
    return mb_tolower(a) != a;
}
unsafe extern "C" fn utf_strnicmp(mut s1: *const u8, mut s2: *const u8, mut n1: size_t, mut n2: size_t) -> i32 {
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    let mut cdiff: i32 = 0;
    let mut buffer: [u8; 6] = [0; 6];
    loop {
        c1 = utf_safe_read_char_adv(&mut s1, &mut n1);
        c2 = utf_safe_read_char_adv(&mut s2, &mut n2);
        if c1 <= 0 || c2 <= 0 {
            break;
        }
        if c1 == c2 {
            continue;
        }
        cdiff = utf_fold(c1) - utf_fold(c2);
        if cdiff != 0 {
            return cdiff;
        }
    }
    /* some string ended or has an incomplete/illegal character sequence */
    if c1 == 0 || c2 == 0 {
        /* some string ended. shorter string is smaller */
        if c1 == 0 && c2 == 0 {
            return 0;
        }
        return if c1 == 0 { -(1) } else { 1 };
    }
    /* Continue with bytewise comparison to produce some result that
     * would make comparison operations involving this function transitive.
     *
     * If only one string had an error, comparison should be made with
     * folded version of the other string. In this case it is enough
     * to fold just one character to determine the result of comparison. */
    if c1 != -(1) && c2 == -(1) {
        n1 = utf_char2bytes(utf_fold(c1), buffer.as_mut_ptr()) as size_t;
        s1 = buffer.as_mut_ptr()
    } else if c2 != -(1) && c1 == -(1) {
        n2 = utf_char2bytes(utf_fold(c2), buffer.as_mut_ptr()) as size_t;
        s2 = buffer.as_mut_ptr()
    }
    while n1 > 0 && n2 > 0 && *s1 as i32 != NUL && *s2 as i32 != NUL {
        cdiff = *s1 as i32 - *s2 as i32;
        if cdiff != 0 {
            return cdiff;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
        n1 = n1.wrapping_sub(1);
        n2 = n2.wrapping_sub(1)
    }
    if n1 > 0 && *s1 as i32 == NUL {
        n1 = 0
    }
    if n2 > 0 && *s2 as i32 == NUL {
        n2 = 0
    }
    if n1 == 0 && n2 == 0 {
        return 0;
    }
    return if n1 == 0 { -(1) } else { 1 };
}
// / Measure the length of a string in corresponding UTF-32 and UTF-16 units.
// /
// / Invalid UTF-8 bytes, or embedded surrogates, count as one code point/unit
// / each.
// /
// / The out parameters are incremented. This is used to measure the size of
// / a buffer region consisting of multiple line segments.
// /
// / @param s the string
// / @param len maximum length (an earlier NUL terminates)
// / @param[out] codepoints incremented with UTF-32 code point size
// / @param[out] codeunits incremented with UTF-16 code unit size
#[no_mangle]
pub unsafe extern "C" fn mb_utflen(mut s: *const u8, mut len: size_t, mut codepoints: *mut size_t, mut codeunits: *mut size_t) {
    let mut count = 0u64;
    let mut extra = 0u64;
    let mut clen: size_t = 0;
    let mut i = 0;
    while i < len && *s.offset(i as isize) as i32 != NUL {
        clen = utf_ptr2len_len(s.offset(i as isize), len.wrapping_sub(i) as i32) as size_t;
        // NB: gets the byte value of invalid sequence bytes.
        // we only care whether the char fits in the BMP or not
        let mut c = if clen > 1 { utf_ptr2char(s.offset(i as isize)) } else { *s.offset(i as isize) as i32 };
        count = count.wrapping_add(1);
        if c > 0xffff as i32 {
            extra = extra.wrapping_add(1)
        }
        i = (i as u64).wrapping_add(clen as u64) as size_t as size_t
    }
    *codepoints = (*codepoints as u64).wrapping_add(count) as size_t as size_t;
    *codeunits = (*codeunits as u64).wrapping_add(count.wrapping_add(extra)) as size_t as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mb_utf_index_to_bytes(mut s: *const u8, mut len: size_t, mut index: size_t, mut use_utf16_units: bool) -> ssize_t {
    let mut count = 0u64;
    let mut clen: size_t = 0;
    let mut i: size_t = 0;
    if index == 0 {
        return 0;
    }
    i = 0;
    while i < len && *s.offset(i as isize) as i32 != NUL {
        clen = utf_ptr2len_len(s.offset(i as isize), len.wrapping_sub(i) as i32) as size_t;
        // NB: gets the byte value of invalid sequence bytes.
        // we only care whether the char fits in the BMP or not
        let mut c = if clen > 1 { utf_ptr2char(s.offset(i as isize)) } else { *s.offset(i as isize) as i32 };
        count = count.wrapping_add(1);
        if use_utf16_units as i32 != 0 && c > 0xffff as i32 {
            count = count.wrapping_add(1)
        }
        if count >= index as u64 {
            return i.wrapping_add(clen) as ssize_t;
        }
        i = (i as u64).wrapping_add(clen as u64) as size_t as size_t
    }
    return -(1) as ssize_t;
}
/*
 * Version of strnicmp() that handles multi-byte characters.
 * Needed for Big5, Shift-JIS and UTF-8 encoding.  Other DBCS encodings can
 * probably use strnicmp(), because there are no ASCII characters in the
 * second byte.
 * Returns zero if s1 and s2 are equal (ignoring case), the difference between
 * two characters otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_strnicmp(mut s1: *const u8, mut s2: *const u8, nn: size_t) -> i32 {
    return utf_strnicmp(s1, s2, nn, nn);
}
// / Compare strings case-insensitively
// /
// / @note We need to call mb_stricmp() even when we aren't dealing with
// /       a multi-byte encoding because mb_stricmp() takes care of all ASCII and
// /       non-ascii encodings, including characters with umlauts in latin1,
// /       etc., while STRICMP() only handles the system locale version, which
// /       often does not handle non-ascii properly.
// /
// / @param[in]  s1  First string to compare, not more then #MAXCOL characters.
// / @param[in]  s2  Second string to compare, not more then #MAXCOL characters.
// /
// / @return 0 if strings are equal, <0 if s1 < s2, >0 if s1 > s2.
#[no_mangle]
pub unsafe extern "C" fn mb_stricmp(mut s1: *const i8, mut s2: *const i8) -> i32 {
    return mb_strnicmp(s1 as *const u8, s2 as *const u8, MAXCOL as i32 as size_t);
}
/*
 * "g8": show bytes of the UTF-8 char under the cursor.  Doesn't matter what
 * 'encoding' has been set to.
 */
#[no_mangle]
pub unsafe extern "C" fn show_utf8() {
    let mut len: i32 = 0;
    let mut rlen = 0;
    let mut line = 0 as *mut u8;
    let mut clen: i32 = 0;
    let mut i: i32 = 0;
    /* Get the byte length of the char under the cursor, including composing
     * characters. */
    line = get_cursor_pos_ptr();
    len = utfc_ptr2len(line);
    if len == 0 {
        msg(b"NUL\x00" as *const u8 as *const i8 as *mut u8);
        return;
    }
    clen = 0;
    i = 0;
    while i < len {
        if clen == 0 {
            /* start of (composing) character, get its length */
            if i > 0 {
                strcpy(IObuff.as_mut_ptr().offset(rlen as isize) as *mut i8, b"+ \x00" as *const u8 as *const i8 as *mut i8); /* NUL is stored as NL */
                rlen += 2
            }
            clen = utf_ptr2len(line.offset(i as isize))
        }
        sprintf(
            (IObuff.as_mut_ptr() as *mut i8).offset(rlen as isize),
            b"%02x \x00" as *const u8 as *const i8,
            if *line.offset(i as isize) as i32 == NL as i32 { NUL } else { *line.offset(i as isize) as i32 },
        );
        clen -= 1;
        rlen += strlen(IObuff.as_mut_ptr().offset(rlen as isize) as *mut i8) as i32;
        if rlen > IOSIZE - 20 {
            break;
        }
        i += 1
    }
    msg(IObuff.as_mut_ptr());
}
// / Return offset from "p" to the first byte of the character it points into.
// / If "p" points to the NUL at the end of the string return 0.
// / Returns 0 when already at the first byte of a character.
#[no_mangle]
pub unsafe extern "C" fn utf_head_off(mut base: *const u8, mut p: *const u8) -> i32 {
    let mut c: i32 = 0;
    let mut len: i32 = 0;
    if (*p as i32) < 0x80 as i32 {
        /* be quick for ASCII */
        return 0;
    }
    /* Skip backwards over trailing bytes: 10xx.xxxx
     * Skip backwards again if on a composing char. */
    let mut q = 0 as *const u8;
    q = p;
    loop
    /* Move s to the last byte of this char. */
    {
        let mut s = 0 as *const u8;
        s = q;
        while *s.offset(1) as i32 & 0xc0 as i32 == 0x80 as i32 {
            s = s.offset(1)
        }
        /* Move q to the first byte of this char. */
        while q > base && *q as i32 & 0xc0 as i32 == 0x80 as i32 {
            q = q.offset(-1)
        }
        /* Check for illegal sequence. Do allow an illegal byte after where we
         * started. */
        len = utf8len_tab[*q as usize] as i32;
        if len != (s.offset_from(q) as i64 + 1) as i32 && len != (p.offset_from(q) as i64 + 1) as i32 {
            return 0;
        }
        if q <= base {
            break;
        }
        c = utf_ptr2char(q);
        if !utf_iscomposing(c) {
            if !arabic_maycombine(c) {
                break;
            }
            /* Advance to get a sneak-peak at the next char */
            let mut j = q;
            j = j.offset(-1);
            /* Move j to the first byte of this char. */
            while j > base && *j as i32 & 0xc0 as i32 == 0x80 as i32 {
                j = j.offset(-1)
            }
            if !arabic_combine(utf_ptr2char(j), c) {
                break;
            }
        }
        q = q.offset(-1)
    }
    return p.offset_from(q) as i64 as i32;
}
// / Copy a character, advancing the pointers
// /
// / @param[in,out]  fp  Source of the character to copy.
// / @param[in,out]  tp  Destination to copy to.
#[no_mangle]
pub unsafe extern "C" fn mb_copy_char(fp: *mut *const u8, tp: *mut *mut u8) {
    let l = utfc_ptr2len(*fp) as size_t;
    memmove(*tp as *mut libc::c_void, *fp as *const libc::c_void, l);
    *tp = (*tp).offset(l as isize);
    *fp = (*fp).offset(l as isize);
}
/*
 * Return the offset from "p" to the first byte of a character.  When "p" is
 * at the start of a character 0 is returned, otherwise the offset to the next
 * character.  Can start anywhere in a stream of bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_off_next(mut base: *mut u8, mut p: *mut u8) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if (*p as i32) < 0x80 as i32 {
        // be quick for ASCII
        return 0;
    }
    // Find the next character that isn't 10xx.xxxx
    i = 0;
    while *p.offset(i as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
        i += 1
    }
    if i > 0 {
        // Check for illegal sequence.
        j = 0;
        while p.offset(-(j as isize)) > base {
            if *p.offset(-j as isize) as i32 & 0xc0 as i32 != 0x80 as i32 {
                break;
            }
            j += 1
        }
        if utf8len_tab[*p.offset(-j as isize) as usize] as i32 != i + j {
            return 0;
        }
    }
    return i;
}
/*
 * Return the offset from "p" to the last byte of the character it points
 * into.  Can start anywhere in a stream of bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_tail_off(mut base: *mut u8, mut p: *mut u8) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if *p as i32 == NUL {
        return 0;
    }
    // Find the last character that is 10xx.xxxx
    i = 0;
    while *p.offset((i + 1) as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
        i += 1
    }
    // Check for illegal sequence.
    j = 0;
    while p.offset(-(j as isize)) > base {
        if *p.offset(-j as isize) as i32 & 0xc0 as i32 != 0x80 as i32 {
            break;
        }
        j += 1
    }
    if utf8len_tab[*p.offset(-j as isize) as usize] as i32 != i + j + 1 {
        return 0;
    }
    return i;
}
/*
 * Find the next illegal byte sequence.
 */
#[no_mangle]
pub unsafe extern "C" fn utf_find_illegal() {
    let mut current_block: u64;
    let mut pos = (*curwin).w_cursor;
    let mut p = 0 as *mut u8;
    let mut len: i32 = 0;
    let mut vimconv = vimconv_T {
        vc_type: 0,
        vc_factor: 0,
        vc_fd: 0 as *mut libc::c_void,
        vc_fail: false,
    };
    let mut tofree = NULL_0 as *mut u8;
    vimconv.vc_type = CONV_NONE as i32;
    if enc_canon_props((*curbuf).b_p_fenc) & ENC_8BIT != 0 {
        // 'encoding' is "utf-8" but we are editing a 8-bit encoded file,
        // possibly a utf-8 file with illegal bytes.  Setup for conversion
        // from utf-8 to 'fileencoding'.
        convert_setup(&mut vimconv, p_enc, (*curbuf).b_p_fenc);
    }
    (*curwin).w_cursor.coladd = 0;
    's_44: loop {
        p = get_cursor_pos_ptr();
        if vimconv.vc_type != CONV_NONE as i32 {
            xfree(tofree as *mut libc::c_void);
            tofree = string_convert(&mut vimconv, p, NULL_0 as *mut size_t);
            if tofree.is_null() {
                current_block = 3275366147856559585;
                break;
            }
            p = tofree
        }
        while *p as i32 != NUL {
            /* Illegal means that there are not enough trail bytes (checked by
             * utf_ptr2len()) or too many of them (overlong sequence). */
            len = utf_ptr2len(p);
            if *p as i32 >= 0x80 as i32 && (len == 1 || utf_char2len(utf_ptr2char(p)) != len) {
                if vimconv.vc_type == CONV_NONE as i32 {
                    (*curwin).w_cursor.col += p.offset_from(get_cursor_pos_ptr()) as i64 as colnr_T
                } else {
                    let mut l: i32 = 0;
                    len = p.offset_from(tofree) as i64 as i32;
                    p = get_cursor_pos_ptr();
                    while *p as i32 != NUL && {
                        let fresh6 = len;
                        len = len - 1;
                        (fresh6) > 0
                    } {
                        l = utf_ptr2len(p);
                        (*curwin).w_cursor.col += l;
                        p = p.offset(l as isize)
                    }
                }
                current_block = 11834598965984575227;
                break 's_44;
            } else {
                p = p.offset(len as isize)
            }
        }
        if (*curwin).w_cursor.lnum == (*curbuf).b_ml.ml_line_count {
            current_block = 3275366147856559585;
            break;
        }
        (*curwin).w_cursor.lnum += 1;
        (*curwin).w_cursor.col = 0
    }
    match current_block {
        3275366147856559585 => {
            /* didn't find it: don't move and beep */
            (*curwin).w_cursor = pos;
            beep_flush();
        }
        _ => {}
    }
    xfree(tofree as *mut libc::c_void);
    convert_setup(&mut vimconv, NULL_0 as *mut u8, NULL_0 as *mut u8);
}
/*
 * If the cursor moves on an trail byte, set the cursor on the lead byte.
 * Thus it moves left if necessary.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_adjust_cursor() {
    mark_mb_adjustpos(curbuf, &mut (*curwin).w_cursor);
}
// / Checks and adjusts cursor column. Not mode-dependent.
// / @see check_cursor_col_win
// /
// / @param  win_  Places cursor on a valid column for this window.
#[no_mangle]
pub unsafe extern "C" fn mb_check_adjust_col(mut win_: *mut libc::c_void) {
    let mut win = win_ as *mut win_T;
    let mut oldcol = (*win).w_cursor.col;
    // Column 0 is always valid.
    if oldcol != 0 {
        let mut p = ml_get_buf((*win).w_buffer, (*win).w_cursor.lnum, false);
        let mut len = strlen(p as *mut i8) as colnr_T;
        // Empty line or invalid column?
        if len == 0 || oldcol < 0 {
            (*win).w_cursor.col = 0
        } else {
            // Cursor column too big for line?
            if oldcol > len {
                (*win).w_cursor.col = len - 1
            }
            // Move the cursor to the head byte.
            (*win).w_cursor.col -= utf_head_off(p, p.offset((*win).w_cursor.col as isize))
        }
        // Reset `coladd` when the cursor would be on the right half of a
        // double-wide character.
        if (*win).w_cursor.coladd == 1 && *p.offset((*win).w_cursor.col as isize) as i32 != TAB && vim_isprintc(utf_ptr2char(p.offset((*win).w_cursor.col as isize))) as i32 != 0 && ptr2cells(p.offset((*win).w_cursor.col as isize)) > 1 {
            (*win).w_cursor.coladd = 0
        }
    };
}
/*
 * Return a pointer to the character before "*p", if there is one.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_prevptr(mut line: *mut u8, mut p: *mut u8) -> *mut u8 {
    if p > line {
        p = p.offset(-((utf_head_off(line, p.offset(-(1))) + 1) as isize))
    }
    return p;
}
/*
 * Return the character length of "str".  Each multi-byte character (with
 * following composing characters) counts as one.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_charlen(mut str: *mut u8) -> i32 {
    let mut p = str;
    let mut count: i32 = 0;
    if p.is_null() {
        return 0;
    }
    count = 0;
    while *p as i32 != NUL {
        p = p.offset(Some(Some(mb_ptr2len).expect("non-null function pointer")).expect("non-null function pointer")(p) as isize);
        count += 1
    }
    return count;
}
/*
 * Like mb_charlen() but for a string with specified length.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_charlen_len(mut str: *mut u8, mut len: i32) -> i32 {
    let mut p = str;
    let mut count: i32 = 0;
    count = 0;
    while *p as i32 != NUL && p < str.offset(len as isize) {
        p = p.offset(Some(Some(mb_ptr2len).expect("non-null function pointer")).expect("non-null function pointer")(p) as isize);
        count += 1
    }
    return count;
}
// / Try to unescape a multibyte character
// /
// / Used for the rhs and lhs of the mappings.
// /
// / @param[in,out]  pp  String to unescape. Is advanced to just after the bytes
// /                     that form a multibyte character.
// /
// / @return Unescaped string if it is a multibyte character, NULL if no
// /         multibyte character was found. Returns a static buffer, always one
// /         and the same.
#[no_mangle]
pub unsafe extern "C" fn mb_unescape(pp: *mut *const i8) -> *const i8 {
    static mut buf: [i8; 6] = [0; 6];
    let mut buf_idx = 0;
    let mut str = *pp as *mut u8;
    // Must translate K_SPECIAL KS_SPECIAL KE_FILLER to K_SPECIAL and CSI
    // KS_EXTRA KE_CSI to CSI.
    // Maximum length of a utf-8 character is 4 bytes.
    let mut str_idx = 0;
    while *str.offset(str_idx as isize) as i32 != NUL && buf_idx < 4 {
        if *str.offset(str_idx as isize) as i32 == K_SPECIAL && *str.offset(str_idx.wrapping_add(1) as isize) as i32 == KS_SPECIAL && *str.offset(str_idx.wrapping_add(2) as isize) as i32 == KE_FILLER {
            let fresh7 = buf_idx;
            buf_idx = buf_idx.wrapping_add(1);
            buf[fresh7 as usize] = K_SPECIAL as i8;
            str_idx = (str_idx as u64).wrapping_add(2) as size_t as size_t
        } else if *str.offset(str_idx as isize) as i32 == K_SPECIAL && *str.offset(str_idx.wrapping_add(1) as isize) as i32 == KS_EXTRA && *str.offset(str_idx.wrapping_add(2) as isize) as i32 == KE_CSI as i32 {
            let fresh8 = buf_idx;
            buf_idx = buf_idx.wrapping_add(1);
            buf[fresh8 as usize] = CSI as i8;
            str_idx = (str_idx as u64).wrapping_add(2) as size_t as size_t
        } else {
            if *str.offset(str_idx as isize) as i32 == K_SPECIAL {
                break;
            }
            let fresh9 = buf_idx;
            buf_idx = buf_idx.wrapping_add(1);
            buf[fresh9 as usize] = *str.offset(str_idx as isize) as i8
        }
        buf[buf_idx as usize] = NUL as i8;
        // Return a multi-byte character if it's found.  An illegal sequence
        // will result in a 1 here.
        if utf_ptr2len(buf.as_mut_ptr() as *const u8) > 1 {
            *pp = (str as *const i8).offset(str_idx as isize).offset(1);
            return buf.as_mut_ptr();
        }
        // Bail out quickly for ASCII.
        if (buf[0 as i32 as usize] as u8 as i32) < 128 {
            break;
        }
        str_idx = str_idx.wrapping_add(1)
    }
    return NULL_0 as *const i8;
}
/*
 * Skip the Vim specific head of a 'encoding' name.
 */
#[no_mangle]
pub unsafe extern "C" fn enc_skip(mut p: *mut u8) -> *mut u8 {
    if strncmp(p as *mut i8, b"2byte-\x00" as *const u8 as *const i8 as *mut i8, 6) == 0 {
        return p.offset(6);
    }
    if strncmp(p as *mut i8, b"8bit-\x00" as *const u8 as *const i8 as *mut i8, 5) == 0 {
        return p.offset(5);
    }
    return p;
}
/*
 * Find the canonical name for encoding "enc".
 * When the name isn't recognized, returns "enc" itself, but with all lower
 * case characters and '_' replaced with '-'.
 * Returns an allocated string.
 */
#[no_mangle]
pub unsafe extern "C" fn enc_canonize(mut enc: *mut u8) -> *mut u8 {
    let mut p = 0 as *mut u8;
    let mut s = 0 as *mut u8;
    let mut i: i32 = 0;
    if strcmp(enc as *mut i8, b"default\x00" as *const u8 as *const i8 as *mut i8) == 0 {
        // Use the default encoding as found by set_init_1().
        return vim_strsave(fenc_default);
    }
    /* copy "enc" to allocated memory, with room for two '-' */
    let mut r = xmalloc(strlen(enc as *mut i8).wrapping_add(3)) as *mut u8;
    /* Make it all lower case and replace '_' with '-'. */
    p = r;
    s = enc;
    while *s as i32 != NUL {
        if *s as i32 == '_' as i32 {
            let fresh10 = p;
            p = p.offset(1);
            *fresh10 = '-' as i32 as u8
        } else {
            let fresh11 = p;
            p = p.offset(1);
            *fresh11 = if (*s as i32) < 'A' as i32 || *s as i32 > 'Z' as i32 { *s as i32 } else { (*s as i32) + ('a' as i32 - 'A' as i32) } as u8
        }
        s = s.offset(1)
    }
    *p = NUL as u8;
    /* Skip "2byte-" and "8bit-". */
    p = enc_skip(r);
    /* Change "microsoft-cp" to "cp".  Used in some spell files. */
    if strncmp(p as *mut i8, b"microsoft-cp\x00" as *const u8 as *const i8 as *mut i8, 12) == 0 {
        memmove(p as *mut libc::c_void, p.offset(10) as *const libc::c_void, strlen(p.offset(10) as *mut i8).wrapping_add(1));
    }
    /* "iso8859" -> "iso-8859" */
    if strncmp(p as *mut i8, b"iso8859\x00" as *const u8 as *const i8 as *mut i8, 7) == 0 {
        memmove(p.offset(4) as *mut libc::c_void, p.offset(3) as *const libc::c_void, strlen(p.offset(3) as *mut i8).wrapping_add(1));
        *p.offset(3) = '-' as i32 as u8
    }
    /* "iso-8859n" -> "iso-8859-n" */
    if strncmp(p as *mut i8, b"iso-8859\x00" as *const u8 as *const i8 as *mut i8, 8) == 0 && *p.offset(8) as i32 != '-' as i32 {
        memmove(p.offset(9) as *mut libc::c_void, p.offset(8) as *const libc::c_void, strlen(p.offset(8) as *mut i8).wrapping_add(1));
        *p.offset(8) = '-' as i32 as u8
    }
    /* "latin-N" -> "latinN" */
    if strncmp(p as *mut i8, b"latin-\x00" as *const u8 as *const i8 as *mut i8, 6) == 0 {
        memmove(p.offset(5) as *mut libc::c_void, p.offset(6) as *const libc::c_void, strlen(p.offset(6) as *mut i8).wrapping_add(1));
    }
    if enc_canon_search(p) >= 0 {
        /* canonical name can be used unmodified */
        if p != r {
            memmove(r as *mut libc::c_void, p as *const libc::c_void, strlen(p as *mut i8).wrapping_add(1));
        }
    } else {
        i = enc_alias_search(p);
        if i >= 0 {
            /* alias recognized, get canonical name */
            xfree(r as *mut libc::c_void);
            r = vim_strsave(enc_canon_table[i as usize].name as *mut u8)
        }
    }
    return r;
}
/*
 * Search for an encoding alias of "name".
 * Returns -1 when not found.
 */
unsafe extern "C" fn enc_alias_search(mut name: *mut u8) -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while !enc_alias_table[i as usize].name.is_null() {
        if strcmp(name as *mut i8, enc_alias_table[i as usize].name as *mut i8) == 0 {
            return enc_alias_table[i as usize].canon;
        }
        i += 1
    }
    return -(1);
}
/*
 * Get the canonicalized encoding of the current locale.
 * Returns an allocated string when successful, NULL when not.
 */
#[no_mangle]
pub unsafe extern "C" fn enc_locale() -> *mut u8 {
    let mut i: i32 = 0;
    let mut buf: [i8; 50] = [0; 50];
    let mut s = 0 as *const i8;
    s = nl_langinfo(CODESET_0);
    if s.is_null() || *s as i32 == NUL {
        s = setlocale(LC_CTYPE, NULL_0 as *const i8);
        if s.is_null() || *s as i32 == NUL {
            s = os_getenv(b"LC_ALL\x00" as *const u8 as *const i8);
            if !s.is_null() {
                s = os_getenv(b"LC_CTYPE\x00" as *const u8 as *const i8);
                if !s.is_null() {
                    s = os_getenv(b"LANG\x00" as *const u8 as *const i8)
                }
            }
        }
    }
    if s.is_null() {
        return NULL_0 as *mut u8;
    }
    // The most generic locale format is:
    // language[_territory][.codeset][@modifier][+special][,[sponsor][_revision]]
    // If there is a '.' remove the part before it.
    // if there is something after the codeset, remove it.
    // Make the name lowercase and replace '_' with '-'.
    // Exception: "ja_JP.EUC" == "euc-jp", "zh_CN.EUC" = "euc-cn",
    // "ko_KR.EUC" == "euc-kr"
    let mut p: *const i8 = vim_strchr(s as *mut u8, '.' as i32) as *mut i8;
    let mut current_block_24: u64;
    if !p.is_null() {
        if p > s.offset(2)
            && strncasecmp(p.offset(1) as *mut i8, b"EUC\x00" as *const u8 as *const i8 as *mut i8, 3) == 0
            && *(*__ctype_b_loc()).offset(*p.offset(4) as i32 as isize) as i32 & _ISalnum as i32 as u16 as i32 == 0
            && *p.offset(4) as i32 != '-' as i32
            && *p.offset(-(3) as isize) as i32 == '_' as i32
        {
            // Copy "XY.EUC" to "euc-XY" to buf[10].
            memmove(buf.as_mut_ptr() as *mut libc::c_void, b"euc-\x00" as *const u8 as *const i8 as *const libc::c_void, 4);
            buf[4 as i32 as usize] = if *p.offset(-(2) as isize) as u32 >= 'A' as i32 as u32 && *p.offset(-(2) as isize) as u32 <= 'Z' as i32 as u32
                || *p.offset(-(2) as isize) as u32 >= 'a' as i32 as u32 && *p.offset(-(2) as isize) as u32 <= 'z' as i32 as u32
                || ascii_isdigit(*p.offset(-(2) as isize) as i32) as i32 != 0
            {
                if (*p.offset(-(2) as isize) as i32) < 'A' as i32 || *p.offset(-(2) as isize) as i32 > 'Z' as i32 {
                    *p.offset(-(2) as isize) as i32
                } else {
                    (*p.offset(-(2) as isize) as i32) + ('a' as i32 - 'A' as i32)
                }
            } else {
                0
            } as i8;
            buf[5 as i32 as usize] = if *p.offset(-(1) as isize) as u32 >= 'A' as i32 as u32 && *p.offset(-(1) as isize) as u32 <= 'Z' as i32 as u32
                || *p.offset(-(1) as isize) as u32 >= 'a' as i32 as u32 && *p.offset(-(1) as isize) as u32 <= 'z' as i32 as u32
                || ascii_isdigit(*p.offset(-(1) as isize) as i32) as i32 != 0
            {
                if (*p.offset(-(1) as isize) as i32) < 'A' as i32 || *p.offset(-(1) as isize) as i32 > 'Z' as i32 {
                    *p.offset(-(1) as isize) as i32
                } else {
                    (*p.offset(-(1) as isize) as i32) + ('a' as i32 - 'A' as i32)
                }
            } else {
                0
            } as i8;
            buf[6 as i32 as usize] = NUL as i8;
            current_block_24 = 1538046216550696469;
        } else {
            s = p.offset(1);
            current_block_24 = 16296608149235199289;
        }
    } else {
        current_block_24 = 16296608149235199289;
    }
    match current_block_24 {
        16296608149235199289 => {
            i = 0;
            while i < ::std::mem::size_of::<[i8; 50]>() as u64 as i32 - 1 && *s.offset(i as isize) as i32 != NUL {
                if *s.offset(i as isize) as i32 == '_' as i32 || *s.offset(i as isize) as i32 == '-' as i32 {
                    buf[i as usize] = '-' as i32 as i8
                } else {
                    if !(*s.offset(i as isize) as u8 as u32 >= 'A' as i32 as u32 && *s.offset(i as isize) as u8 as u32 <= 'Z' as i32 as u32
                        || *s.offset(i as isize) as u8 as u32 >= 'a' as i32 as u32 && *s.offset(i as isize) as u8 as u32 <= 'z' as i32 as u32
                        || ascii_isdigit(*s.offset(i as isize) as u8 as i32) as i32 != 0)
                    {
                        break;
                    }
                    buf[i as usize] = if (*s.offset(i as isize) as i32) < 'A' as i32 || *s.offset(i as isize) as i32 > 'Z' as i32 {
                        *s.offset(i as isize) as i32
                    } else {
                        (*s.offset(i as isize) as i32) + ('a' as i32 - 'A' as i32)
                    } as i8
                }
                i += 1
            }
            buf[i as usize] = NUL as i8
        }
        _ => {}
    }
    return enc_canonize(buf.as_mut_ptr() as *mut u8);
}
/*
 * Call iconv_open() with a check if iconv() works properly (there are broken
 * versions).
 * Returns (void *)-1 if failed.
 * (should return iconv_t, but that causes problems with prototypes).
 */
#[no_mangle]
pub unsafe extern "C" fn my_iconv_open(mut to: *mut u8, mut from: *mut u8) -> *mut libc::c_void {
    let mut fd = 0 as *mut libc::c_void; /* detected a broken iconv() previously */
    let mut tobuf: [u8; 400] = [0; 400];
    let mut p = 0 as *mut i8;
    let mut tolen: size_t = 0;
    static mut iconv_working: WorkingStatus = kUnknown;
    if iconv_working as u32 == kBroken as i32 as u32 {
        return -(1) as *mut libc::c_void;
    }
    fd = iconv_open(enc_skip(to) as *mut i8, enc_skip(from) as *mut i8);
    if fd != -(1) as iconv_t && iconv_working as u32 == kUnknown as i32 as u32 {
        /*
         * Do a dummy iconv() call to check if it actually works.  There is a
         * version of iconv() on Linux that is broken.  We can't ignore it,
         * because it's wide-spread.  The symptoms are that after outputting
         * the initial shift state the "to" pointer is NULL and conversion
         * stops for no apparent reason after about 8160 characters.
         */
        p = tobuf.as_mut_ptr() as *mut i8;
        tolen = ICONV_TESTLEN as size_t;
        iconv(fd, NULL_0 as *mut *mut i8, NULL_0 as *mut size_t, &mut p, &mut tolen);
        if p.is_null() {
            iconv_working = kBroken;
            iconv_close(fd);
            fd = -(1) as iconv_t
        } else {
            iconv_working = kWorking
        }
    }
    return fd;
}
pub const ICONV_TESTLEN: i32 = 400;
/*
 * Convert the string "str[slen]" with iconv().
 * If "unconvlenp" is not NULL handle the string ending in an incomplete
 * sequence and set "*unconvlenp" to the length of it.
 * Returns the converted string in allocated memory.  NULL for an error.
 * If resultlenp is not NULL, sets it to the result length in bytes.
 */
unsafe extern "C" fn iconv_string(vcp: *const vimconv_T, mut str: *mut u8, mut slen: size_t, mut unconvlenp: *mut size_t, mut resultlenp: *mut size_t) -> *mut u8 {
    let mut from = 0 as *const i8;
    let mut fromlen: size_t = 0;
    let mut to = 0 as *mut i8;
    let mut tolen: size_t = 0;
    let mut len = 0;
    let mut done = 0;
    let mut result = NULL_0 as *mut u8;
    let mut p = 0 as *mut u8;
    let mut l: i32 = 0;
    from = str as *mut i8;
    fromlen = slen;
    loop {
        if len == 0 || ICONV_ERRNO == ICONV_E2BIG {
            /* Allocate enough room for most conversions.  When re-allocating
             * increase the buffer size. */
            len = len.wrapping_add(fromlen.wrapping_mul(2)).wrapping_add(40);
            p = xmalloc(len) as *mut u8;
            if done > 0 {
                memmove(p as *mut libc::c_void, result as *const libc::c_void, done);
            }
            xfree(result as *mut libc::c_void);
            result = p
        }
        to = (result as *mut i8).offset(done as isize);
        tolen = len.wrapping_sub(done).wrapping_sub(2);
        // Avoid a warning for systems with a wrong iconv() prototype by
        // casting the second argument to void *.
        if iconv((*vcp).vc_fd, &mut from as *mut *const i8 as *mut libc::c_void as *mut *mut i8, &mut fromlen, &mut to, &mut tolen) != SIZE_MAX {
            // Finished, append a NUL.
            *to = NUL as i8;
            break;
        } else if !(*vcp).vc_fail && !unconvlenp.is_null() && (ICONV_ERRNO == ICONV_EINVAL || ICONV_ERRNO == EINVAL) {
            // Check both ICONV_EINVAL and EINVAL, because the dynamically loaded
            // iconv library may use one of them.
            // Handle an incomplete sequence at the end.
            *to = NUL as i8;
            *unconvlenp = fromlen;
            break;
        } else {
            if !(*vcp).vc_fail && (ICONV_ERRNO == ICONV_EILSEQ || ICONV_ERRNO == EILSEQ || ICONV_ERRNO == ICONV_EINVAL || ICONV_ERRNO == EINVAL) {
                // Check both ICONV_EILSEQ and EILSEQ, because the dynamically loaded
                // iconv library may use one of them.
                // Can't convert: insert a '?' and skip a character.  This assumes
                // conversion from 'encoding' to something else.  In other
                // situations we don't know what to skip anyway.
                let fresh12 = to;
                to = to.offset(1);
                *fresh12 = '?' as i32 as i8;
                if utf_ptr2cells(from as *mut u8) > 1 {
                    let fresh13 = to;
                    to = to.offset(1);
                    *fresh13 = '?' as i32 as i8
                }
                l = utfc_ptr2len_len(from as *const u8, fromlen as i32);
                from = from.offset(l as isize);
                fromlen = (fromlen as u64).wrapping_sub(l as u64) as size_t as size_t
            } else if ICONV_ERRNO != ICONV_E2BIG {
                // conversion failed
                let mut ptr_ = &mut result as *mut *mut u8 as *mut *mut libc::c_void;
                xfree(*ptr_);
                *ptr_ = NULL_0 as *mut libc::c_void;
                break;
            }
            // Not enough room or skipping illegal sequence.
            done = to.offset_from(result as *mut i8) as i64 as size_t
        }
    }
    if !resultlenp.is_null() && !result.is_null() {
        *resultlenp = to.offset_from(result as *mut i8) as i64 as size_t
    }
    return result;
}
// HAVE_ICONV
/*
 * Setup "vcp" for conversion from "from" to "to".
 * The names must have been made canonical with enc_canonize().
 * vcp->vc_type must have been initialized to CONV_NONE.
 * Note: cannot be used for conversion from/to ucs-2 and ucs-4 (will use utf-8
 * instead).
 * Afterwards invoke with "from" and "to" equal to NULL to cleanup.
 * Return FAIL when conversion is not supported, OK otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn convert_setup(mut vcp: *mut vimconv_T, mut from: *mut u8, mut to: *mut u8) -> i32 {
    return convert_setup_ext(vcp, from, true, to, true);
}
/*
 * As convert_setup(), but only when from_unicode_is_utf8 is TRUE will all
 * "from" unicode charsets be considered utf-8.  Same for "to".
 */
#[no_mangle]
pub unsafe extern "C" fn convert_setup_ext(mut vcp: *mut vimconv_T, mut from: *mut u8, mut from_unicode_is_utf8: bool, mut to: *mut u8, mut to_unicode_is_utf8: bool) -> i32 {
    let mut from_prop: i32 = 0;
    let mut to_prop: i32 = 0;
    let mut from_is_utf8: i32 = 0;
    let mut to_is_utf8: i32 = 0;
    // Reset to no conversion.
    if (*vcp).vc_type == CONV_ICONV as i32 && (*vcp).vc_fd != -(1) as iconv_t {
        iconv_close((*vcp).vc_fd);
    }
    *vcp = {
        let mut init = vimconv_T {
            vc_type: CONV_NONE as i32,
            vc_factor: 1,
            vc_fd: 0 as *mut libc::c_void,
            vc_fail: false,
        };
        init
    };
    /* No conversion when one of the names is empty or they are equal. */
    if from.is_null() || *from as i32 == NUL || to.is_null() || *to as i32 == NUL || strcmp(from as *mut i8, to as *mut i8) == 0 {
        return OK;
    }
    from_prop = enc_canon_props(from);
    to_prop = enc_canon_props(to);
    if from_unicode_is_utf8 {
        from_is_utf8 = from_prop & ENC_UNICODE
    } else {
        from_is_utf8 = (from_prop == ENC_UNICODE) as i32
    }
    if to_unicode_is_utf8 {
        to_is_utf8 = to_prop & ENC_UNICODE
    } else {
        to_is_utf8 = (to_prop == ENC_UNICODE) as i32
    }
    if from_prop & ENC_LATIN1 != 0 && to_is_utf8 != 0 {
        /* Internal latin1 -> utf-8 conversion. */
        (*vcp).vc_type = CONV_TO_UTF8 as i32;
        (*vcp).vc_factor = 2
    /* up to twice as long */
    } else if from_prop & ENC_LATIN9 != 0 && to_is_utf8 != 0 {
        /* Internal latin9 -> utf-8 conversion. */
        (*vcp).vc_type = CONV_9_TO_UTF8 as i32;
        (*vcp).vc_factor = 3
    /* up to three as long (euro sign) */
    } else if from_is_utf8 != 0 && to_prop & ENC_LATIN1 != 0 {
        /* Internal utf-8 -> latin1 conversion. */
        (*vcp).vc_type = CONV_TO_LATIN1 as i32
    } else if from_is_utf8 != 0 && to_prop & ENC_LATIN9 != 0 {
        /* Internal utf-8 -> latin9 conversion. */
        (*vcp).vc_type = CONV_TO_LATIN9 as i32
    } else {
        // NOLINT(readability/braces)
        // Use iconv() for conversion.
        (*vcp).vc_fd = my_iconv_open(if to_is_utf8 != 0 { b"utf-8\x00" as *const u8 as *const i8 as *mut u8 } else { to }, if from_is_utf8 != 0 { b"utf-8\x00" as *const u8 as *const i8 as *mut u8 } else { from });
        if (*vcp).vc_fd != -(1) as iconv_t {
            (*vcp).vc_type = CONV_ICONV as i32;
            (*vcp).vc_factor = 4
            /* could be longer too... */
        }
    }
    if (*vcp).vc_type == CONV_NONE as i32 {
        return FAIL;
    }
    return OK;
}
/*
 * Convert text "ptr[*lenp]" according to "vcp".
 * Returns the result in allocated memory and sets "*lenp".
 * When "lenp" is NULL, use NUL terminated strings.
 * Illegal chars are often changed to "?", unless vcp->vc_fail is set.
 * When something goes wrong, NULL is returned and "*lenp" is unchanged.
 */
#[no_mangle]
pub unsafe extern "C" fn string_convert(vcp: *const vimconv_T, mut ptr: *mut u8, mut lenp: *mut size_t) -> *mut u8 {
    return string_convert_ext(vcp, ptr, lenp, NULL_0 as *mut size_t);
}
/*
 * Like string_convert(), but when "unconvlenp" is not NULL and there are is
 * an incomplete sequence at the end it is not converted and "*unconvlenp" is
 * set to the number of remaining bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn string_convert_ext(vcp: *const vimconv_T, mut ptr: *mut u8, mut lenp: *mut size_t, mut unconvlenp: *mut size_t) -> *mut u8 {
    let mut retval = NULL_0 as *mut u8;
    let mut d = 0 as *mut u8;
    let mut l: i32 = 0;
    let mut c: i32 = 0;
    let mut len: size_t = 0;
    if lenp.is_null() {
        len = strlen(ptr as *mut i8)
    } else {
        len = *lenp
    }
    if len == 0 {
        return vim_strsave(b"\x00" as *const u8 as *const i8 as *mut u8);
    }
    match (*vcp).vc_type {
        1 => {
            /* latin1 to utf-8 conversion */
            retval = xmalloc(len.wrapping_mul(2).wrapping_add(1)) as *mut u8;
            d = retval;
            let mut i = 0;
            while i < len {
                c = *ptr.offset(i as isize) as i32;
                if c < 0x80 as i32 {
                    let fresh14 = d;
                    d = d.offset(1);
                    *fresh14 = c as u8
                } else {
                    let fresh15 = d;
                    d = d.offset(1);
                    *fresh15 = (0xc0 as i32 as u32).wrapping_add(c as u32 >> 6) as u8;
                    let fresh16 = d;
                    d = d.offset(1);
                    *fresh16 = (0x80 as i32 + (c & 0x3f as i32)) as u8
                }
                i = i.wrapping_add(1)
            }
            *d = NUL as u8;
            if !lenp.is_null() {
                *lenp = d.offset_from(retval) as i64 as size_t
            }
        }
        2 => {
            /* latin9 to utf-8 conversion */
            retval = xmalloc(len.wrapping_mul(3).wrapping_add(1)) as *mut u8;
            d = retval;
            let mut i_0 = 0;
            while i_0 < len {
                c = *ptr.offset(i_0 as isize) as i32;
                match c {
                    164 => {
                        c = 0x20ac as i32
                        /* Y */
                    }
                    166 => c = 0x160 as i32,
                    168 => c = 0x161 as i32,
                    180 => c = 0x17d as i32,
                    184 => c = 0x17e as i32,
                    188 => c = 0x152 as i32,
                    189 => c = 0x153 as i32,
                    190 => c = 0x178 as i32,
                    _ => {}
                }
                d = d.offset(utf_char2bytes(c, d) as isize);
                i_0 = i_0.wrapping_add(1)
            }
            *d = NUL as u8;
            if !lenp.is_null() {
                *lenp = d.offset_from(retval) as i64 as size_t
            }
        }
        3 | 4 => {
            /* utf-8 to latin1 conversion */
            /* utf-8 to latin9 conversion */
            retval = xmalloc(len.wrapping_add(1)) as *mut u8;
            d = retval;
            let mut i_1 = 0;
            while i_1 < len {
                l = utf_ptr2len_len(ptr.offset(i_1 as isize), len.wrapping_sub(i_1) as i32);
                if l == 0 {
                    let fresh17 = d;
                    d = d.offset(1);
                    *fresh17 = NUL as u8
                } else if l == 1 {
                    let mut l_w = utf8len_tab_zero[*ptr.offset(i_1 as isize) as usize];
                    if l_w as i32 == 0 {
                        /* Illegal utf-8 byte cannot be converted */
                        xfree(retval as *mut libc::c_void);
                        return NULL_0 as *mut u8;
                    }
                    if !unconvlenp.is_null() && l_w as u64 > len.wrapping_sub(i_1) {
                        /* Incomplete sequence at the end. */
                        *unconvlenp = len.wrapping_sub(i_1);
                        break;
                    } else {
                        let fresh18 = d;
                        d = d.offset(1);
                        *fresh18 = *ptr.offset(i_1 as isize)
                    }
                } else {
                    c = utf_ptr2char(ptr.offset(i_1 as isize));
                    if (*vcp).vc_type == CONV_TO_LATIN9 as i32 {
                        match c {
                            8364 => {
                                c = 0xa4 as i32
                                /* not in latin9 */
                            }
                            352 => c = 0xa6 as i32,
                            353 => c = 0xa8 as i32,
                            381 => c = 0xb4 as i32,
                            382 => c = 0xb8 as i32,
                            338 => c = 0xbc as i32,
                            339 => c = 0xbd as i32,
                            376 => c = 0xbe as i32,
                            164 | 166 | 168 | 180 | 184 | 188 | 189 | 190 => c = 0x100 as i32,
                            _ => {}
                        }
                    }
                    if !utf_iscomposing(c) {
                        /* skip composing chars */
                        if c < 0x100 as i32 {
                            let fresh19 = d;
                            d = d.offset(1);
                            *fresh19 = c as u8
                        } else if (*vcp).vc_fail {
                            xfree(retval as *mut libc::c_void);
                            return NULL_0 as *mut u8;
                        } else {
                            let fresh20 = d;
                            d = d.offset(1);
                            *fresh20 = 0xbf as i32 as u8;
                            if utf_char2cells(c) > 1 {
                                let fresh21 = d;
                                d = d.offset(1);
                                *fresh21 = '?' as i32 as u8
                            }
                        }
                    }
                    i_1 = (i_1 as u64).wrapping_add((l - 1) as u64) as size_t as size_t
                }
                i_1 = i_1.wrapping_add(1)
            }
            *d = NUL as u8;
            if !lenp.is_null() {
                *lenp = d.offset_from(retval) as i64 as size_t
            }
        }
        5 => {
            // conversion with vcp->vc_fd
            retval = iconv_string(vcp, ptr, len, unconvlenp, lenp)
        }
        _ => {}
    }
    return retval;
}
