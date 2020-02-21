pub const NUL: char = 0o00 as char;
pub const BELL: char = 0o07 as char;
pub const BS: char = 0o10 as char;
pub const TAB: char = 0o11 as char;
pub const NL: char = 0o12 as char;
pub const FF: char = 0o14 as char;
pub const CAR: char = 0o15 as char; /* CR is used by Mac OS X */
pub const ESC: char = 0o33 as char;
pub const DEL: char = 0x7f as char;
pub const CSI: char = 0x9b as char; // Control Sequence Introducer
pub const DCS: char = 0x90 as char; /* Device Control String */
pub const STERM: char = 0x9c as char; /* String Terminator */

pub const POUND: char = 0xA3 as char;

pub const Ctrl_AT: char = 0 as char; /* @ */
pub const Ctrl_A: char = 1 as char;
pub const Ctrl_B: char = 2 as char;
pub const Ctrl_C: char = 3 as char;
pub const Ctrl_D: char = 4 as char;
pub const Ctrl_E: char = 5 as char;
pub const Ctrl_F: char = 6 as char;
pub const Ctrl_G: char = 7 as char;
pub const Ctrl_H: char = 8 as char;
pub const Ctrl_I: char = 9 as char;
pub const Ctrl_J: char = 10 as char;
pub const Ctrl_K: char = 11 as char;
pub const Ctrl_L: char = 12 as char;
pub const Ctrl_M: char = 13 as char;
pub const Ctrl_N: char = 14 as char;
pub const Ctrl_O: char = 15 as char;
pub const Ctrl_P: char = 16 as char;
pub const Ctrl_Q: char = 17 as char;
pub const Ctrl_R: char = 18 as char;
pub const Ctrl_S: char = 19 as char;
pub const Ctrl_T: char = 20 as char;
pub const Ctrl_U: char = 21 as char;
pub const Ctrl_V: char = 22 as char;
pub const Ctrl_W: char = 23 as char;
pub const Ctrl_X: char = 24 as char;
pub const Ctrl_Y: char = 25 as char;
pub const Ctrl_Z: char = 26 as char;
/* CTRL- [ Left Square Bracket == ESC*/
pub const Ctrl_BSL: char = 28 as char; /* \ BackSLash */
pub const Ctrl_RSB: char = 29 as char; /* ] Right Square Bracket */
pub const Ctrl_HAT: char = 30 as char; /* ^ */
pub const Ctrl__: char = 31 as char;

/// Checks if `c` is a space or tab character.
///
/// @see {ascii_isdigit}
#[inline(always)]
pub fn ascii_iswhite(c: char) -> bool {
    return c == ' ' || c == '\t';
}

/// Check whether character is a decimal digit.
///
/// Library isdigit() function is officially locale-dependent and, for
/// example, returns true for superscript 1 (¹) in locales where encoding
/// contains it in lower 8 bits. Also avoids crashes in case c is below
/// 0 or above 255: library functions are officially defined as accepting
/// only EOF and unsigned char values (otherwise it is undefined behaviour)
/// what may be used for some optimizations (e.g. simple `return
/// isdigit_table[c];`).
#[inline(always)]
pub fn ascii_isdigit<C: Into<char>>(c: C) -> bool {
    let c = c.into();
    return c >= '0' && c <= '9';
}

/// Checks if `c` is a hexadecimal digit, that is, one of 0-9, a-f, A-F.
///
/// @see {ascii_isdigit}
#[inline(always)]
pub fn ascii_isxdigit(c: char) -> bool {
    return c >= '0' && c <= '9' || c >= 'a' && c <= 'f' || c >= 'A' && c <= 'F';
}

/// Checks if `c` is an “identifier” character
///
/// That is, whether it is alphanumeric character or underscore.
#[inline(always)]
pub fn ascii_isident(c: char) -> bool {
    ASCII_ISALNUM(c) || c == '_'
}

/// Checks if `c` is a binary digit, that is, 0-1.
///
/// @see {ascii_isdigit}
#[inline(always)]
pub unsafe extern "C" fn ascii_isbdigit(c: char) -> bool {
    return c == '0' || c == '1';
}

pub fn ASCII_ISALNUM(c: char) -> bool {
    ASCII_ISALPHA(c) || ascii_isdigit(c)
}
pub fn ASCII_ISALPHA(c: char) -> bool {
    c >= 'A' && c <= 'Z' || c >= 'a' && c <= 'z'
}
