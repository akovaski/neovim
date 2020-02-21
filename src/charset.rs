/// Code related to character sets.
use crate::*;
use std::cmp::min;
use std::convert::TryInto;
use std::mem;
use std::ptr;

// Flags for vim_str2nr()
pub type ChStr2NrFlags = i32;
pub const STR2NR_DEC: ChStr2NrFlags = 0;
pub const STR2NR_BIN: ChStr2NrFlags = 1 << 0; // Allow binary numbers
pub const STR2NR_OCT: ChStr2NrFlags = 1 << 1; // Allow octal numbers
pub const STR2NR_HEX: ChStr2NrFlags = 1 << 2; // Allow hexadecimal numbers

// Force one of the above variants.
//
// STR2NR_FORCE|STR2NR_DEC is actually not different from supplying zero
// as flags, but still present for completeness.
pub const STR2NR_FORCE: ChStr2NrFlags = 1 << 3;
// Recognize all formats vim_str2nr() can recognize.
pub const STR2NR_ALL: ChStr2NrFlags = STR2NR_BIN | STR2NR_OCT | STR2NR_HEX;

/// Check if `c` is one of the characters in 'breakat'.
/// Used very often if 'linebreak' is set
#[inline(always)]
pub unsafe fn vim_isbreak(c: libc::c_int) -> bool {
    return breakat_flags[c as usize] != 0;
}

static mut chartab_initialized: bool = false;

// b_chartab[] is an array with 256 bits, each bit representing one of the
// characters 0-255.
unsafe fn SET_CHARTAB(buf: *mut buf_T, c: usize) {
    (*buf).b_chartab[c >> 6] |= 1 << (c & 0x3f)
}
unsafe fn RESET_CHARTAB(buf: *mut buf_T, c: usize) {
    (*buf).b_chartab[c >> 6] &= !(1 << (c & 0x3f));
}
unsafe fn GET_CHARTAB_TAB(chartab: *const u64, c: i32) -> u64 {
    *chartab.offset((c as u32 >> 6) as isize) & 1u64 << (c & 0x3f)
}

// Table used below, see init_chartab() for an explanation
static mut g_chartab: [libc::c_uchar; 256] = [0; 256];

// Flags for g_chartab[].
const CT_CELL_MASK: u8 = 0x07; //< mask: nr of display cells (1, 2 or 4)
const CT_PRINT_CHAR: u8 = 0x10; //< flag: set for printable chars
const CT_ID_CHAR: u8 = 0x20; //< flag: set for ID chars
const CT_FNAME_CHAR: u8 = 0x40; //< flag: set for file name chars

/// Fill g_chartab[].  Also fills curbuf->b_chartab[] with flags for keyword
/// characters for current buffer.
///
/// Depends on the option settings 'iskeyword', 'isident', 'isfname',
/// 'isprint' and 'encoding'.
///
/// The index in g_chartab[] is the character when first byte is up to 0x80,
/// if the first byte is 0x80 and above it depends on further bytes.
///
/// The contents of g_chartab[]:
/// - The lower two bits, masked by CT_CELL_MASK, give the number of display
///   cells the character occupies (1 or 2).  Not valid for UTF-8 above 0x80.
/// - CT_PRINT_CHAR bit is set when the character is printable (no need to
///   translate the character before displaying it).  Note that only DBCS
///   characters can have 2 display cells and still be printable.
/// - CT_FNAME_CHAR bit is set when the character can be in a file name.
/// - CT_ID_CHAR bit is set when the character can be in an identifier.
///
/// @return FAIL if 'iskeyword', 'isident', 'isfname' or 'isprint' option has
/// an error, OK otherwise.
#[no_mangle]
pub unsafe extern "C" fn init_chartab() -> libc::c_int {
    return buf_init_chartab(curbuf, 1);
}

/// Helper for init_chartab
///
/// @param global false: only set buf->b_chartab[]
///
/// @return FAIL if 'iskeyword', 'isident', 'isfname' or 'isprint' option has
/// an error, OK otherwise.
#[no_mangle]
pub unsafe extern "C" fn buf_init_chartab(buf: *mut buf_T, global: libc::c_int) -> libc::c_int {
    if global != 0 {
        for c in 0..256 {
            // Set the default size for printable characters:
            // From <Space> to '~' is 1 (printable), others are 2 (not printable).
            // This also inits all 'isident' and 'isfname' flags to false.
            g_chartab[c] = if c < ' ' as usize {
                if dy_flags & DY_UHEX != 0 {
                    4
                } else {
                    2
                }
            } else if c <= '~' as usize {
                1 | CT_PRINT_CHAR
            } else if c >= 0xa0 {
                // UTF-8: bytes 0xa0 - 0xff are printable (latin1)
                1 | CT_PRINT_CHAR
                    // Assume that every multi-byte char is a filename character.
                    | CT_FNAME_CHAR
            } else {
                // the rest is unprintable by default
                if dy_flags & DY_UHEX != 0 {
                    4
                } else {
                    2
                }
            };
        }
    }

    // Init word char flags all to false
    memset((*buf).b_chartab.as_mut_ptr(), 0, 32);

    // In lisp mode the '-' character is included in keywords.
    if (*buf).b_p_lisp != 0 {
        SET_CHARTAB(buf, '-' as usize);
    }

    // Walk through the 'isident', 'iskeyword', 'isfname' and 'isprint'
    // options Each option is a list of characters, character numbers or
    // ranges, separated by commas, e.g.: "200-210,x,#-178,-"
    let start = if global != 0 { 0 } else { 3 };
    for i in start..=3 {
        let mut p: *const u8 = match i {
            0 => p_isi,          // first round: 'isident'
            1 => p_isp,          // second round: 'isprint'
            2 => p_isf,          // third round: 'isfname'
            3 => (*buf).b_p_isk, // fourth round: 'iskeyword'
            _ => unreachable!(),
        };

        while *p != 0 {
            let mut tilde = false;
            let mut do_isalpha = false;

            if *p == '^' as u8 && *p.offset(1) != 0 {
                tilde = true;
                p = p.offset(1)
            }

            let c = if ascii_isdigit(*p) {
                getdigits_int(&mut p, true, 0)
            } else {
                mb_ptr2char_adv(&mut p)
            };

            let c2 = if *p == '-' as u8 && *p.offset(1) != 0 {
                p = p.offset(1);
                if ascii_isdigit(*p) {
                    Some(getdigits_int(&mut p, true, 0))
                } else {
                    Some(mb_ptr2char_adv(&mut p))
                }
            } else {
                None
            };

            if c <= 0
                || c >= 256
                || c2.is_some() && c2.unwrap() < c
                || c2.is_some() && c2.unwrap() >= 256
                || !(*p == 0 || *p == ',' as u8)
            {
                return FAIL;
            }

            let c_range = if let Some(c2) = c2 {
                c..=c2
            } else {
                // not a range
                // A single '@' (not "@-@"):
                // Decide on letters being ID/printable/keyword chars with
                // standard function isalpha(). This takes care of locale for
                // single-byte characters).
                if c == '@' as i32 {
                    do_isalpha = true;
                    1..=255
                } else {
                    c..=c
                }
            };

            for c in c_range {
                // Use the MB_ functions here, because isalpha() doesn't
                // work properly when 'encoding' is "latin1" and the locale is
                // "C".
                if !do_isalpha || mb_islower(c) || mb_isupper(c) {
                    let c = c as usize;
                    if i == 0 {
                        // (re)set ID flag
                        if tilde {
                            g_chartab[c] &= !CT_ID_CHAR;
                        } else {
                            g_chartab[c] |= CT_ID_CHAR;
                        }
                    } else if i == 1 {
                        // (re)set printable
                        // For double-byte we keep the cell width, so
                        // that we can detect it from the first byte.
                        if c < ' ' as usize || c > '~' as usize {
                            if tilde {
                                g_chartab[c] = (g_chartab[c] & !CT_CELL_MASK)
                                    | if dy_flags & DY_UHEX != 0 { 4 } else { 2 };
                                g_chartab[c] &= !CT_PRINT_CHAR;
                            } else {
                                g_chartab[c] = (g_chartab[c] & !CT_CELL_MASK) | 1;
                                g_chartab[c] |= CT_PRINT_CHAR;
                            }
                        }
                    } else if i == 2 {
                        // (re)set fname flag
                        if tilde {
                            g_chartab[c] &= !CT_FNAME_CHAR;
                        } else {
                            g_chartab[c] |= CT_FNAME_CHAR;
                        }
                    } else if i == 3 {
                        // (re)set keyword flag
                        if tilde {
                            RESET_CHARTAB(buf, c);
                        } else {
                            SET_CHARTAB(buf, c);
                        }
                    } else {
                        unreachable!();
                    }
                }
            }

            let c = *p as libc::c_int;
            p = skip_to_option_part(p);

            if c == ',' as i32 && *p == 0 {
                // Trailing comma is not allowed.
                return FAIL;
            }
        }
    }

    chartab_initialized = true;
    return OK;
}

/// Translate any special characters in buf[bufsize] in-place.
///
/// The result is a string with only printable characters, but if there is not
/// enough room, not all characters will be translated.
#[no_mangle]
pub unsafe extern "C" fn trans_characters(mut buf: *mut libc::c_uchar, bufsize: libc::c_int) {
    // length of string needing translation
    let mut len = strlen(buf as *mut libc::c_char) as libc::c_int;

    // room in buffer after string
    let mut room = bufsize - len;

    while *buf != 0 {
        // length of trs[]
        let trs_len = utfc_ptr2len(buf);

        // Assume a multi-byte character doesn't need translation.
        if trs_len > 1 {
            len -= trs_len
        } else {
            // translated character
            let trs = transchar_byte(*buf as libc::c_int);
            let trs_len = strlen(trs as *mut libc::c_char) as libc::c_int;

            if trs_len > 1 {
                room -= trs_len - 1;
                if room <= 0 {
                    return;
                }
                memmove(buf.offset(trs_len as isize), buf.offset(1), len);
            }
            memmove(buf, trs, trs_len);
            len -= 1
        }
        buf = buf.offset(trs_len as isize)
    }
}

/// Find length of a string capable of holding s with all specials replaced
///
/// Assumes replacing special characters with printable ones just like
/// strtrans() does.
///
/// @param[in]  s  String to check.
///
/// @return number of bytes needed to hold a translation of `s`, NUL byte not
///         included.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn transstr_len(s: *const libc::c_char) -> libc::size_t {
    let mut p = s as *const u8;
    let mut len = 0;
    while *p != 0 {
        let l = utfc_ptr2len(p) as libc::size_t;
        if l > 1 {
            let mut pcc: [libc::c_int; MAX_MCO + 1] = [0; MAX_MCO + 1];
            pcc[0] = utfc_ptr2char(p, &mut pcc[1]);

            if vim_isprintc(pcc[0]) {
                len += l;
            } else {
                for &c in pcc.iter() {
                    if c == 0 {
                        break;
                    }
                    let mut hexbuf: [libc::c_char; 9] = [0; 9];
                    len += transchar_hex(hexbuf.as_mut_ptr(), c);
                }
            }
            p = p.offset(l as isize)
        } else {
            let b2c_l = byte2cells(*p as libc::c_int) as libc::size_t;
            p = p.offset(1);
            // Illegal byte sequence may occupy up to 4 characters.
            len += if b2c_l > 0 { b2c_l } else { 4 };
        }
    }
    return len;
}

/// Replace special characters with printable ones
///
/// @param[in]  s  String to replace characters from.
/// @param[out]  buf  Buffer to which result should be saved.
/// @param[in]  len  Buffer length. Resulting string may not occupy more then
///                  len - 1 bytes (one for trailing NUL byte).
///
/// @return length of the resulting string, without the NUL byte.
#[no_mangle]
pub unsafe extern "C" fn transstr_buf(
    s: *const libc::c_char,
    buf: *mut libc::c_char,
    len: libc::size_t,
) -> libc::size_t {
    let mut p = s as *const u8;
    let mut buf_p = buf;
    let buf_e = buf_p.offset(len as isize - 1);

    while *p != 0 && buf_p < buf_e {
        let l = utfc_ptr2len(p) as isize;
        if l > 1 {
            if buf_p.offset(l) > buf_e {
                break; // Exceeded `buf` size.
            }
            let mut pcc: [libc::c_int; 7] = [0; 7];
            pcc[0] = utfc_ptr2char(p, pcc.as_mut_ptr().offset(1));

            if vim_isprintc(pcc[0]) {
                memmove(buf_p, p as *const i8, l);
                buf_p = buf_p.offset(l);
            } else {
                for i in 0..pcc.len() {
                    if pcc[i] == 0 {
                        break;
                    }
                    let mut hexbuf: [libc::c_char; 9] = [0; 9]; // <up to 6 bytes>NUL
                    let hexlen = transchar_hex(hexbuf.as_mut_ptr(), pcc[i]) as isize;
                    if buf_p.offset(hexlen) > buf_e {
                        break;
                    }
                    memmove(buf_p, hexbuf.as_mut_ptr(), hexlen);
                    buf_p = buf_p.offset(hexlen);
                }
            }
            p = p.offset(l)
        } else {
            let tb = transchar_byte(*p as libc::c_int) as *const libc::c_char;
            p = p.offset(1);
            let tb_len = strlen(tb) as isize;
            if buf_p.offset(tb_len) > buf_e {
                break; // Exceeded `buf` size.
            }
            memmove(buf_p, tb, tb_len);
            buf_p = buf_p.offset(tb_len)
        }
    }
    *buf_p = 0;
    assert!(buf_p <= buf_e);
    return buf_p.offset_from(buf) as libc::size_t;
}

/// Copy string and replace special characters with printable characters
///
/// Works like `strtrans()` does, used for that and in some other places.
///
/// @param[in]  s  String to replace characters from.
///
/// @return [allocated] translated string
#[no_mangle]
pub unsafe extern "C" fn transstr(s: *const libc::c_char) -> *mut libc::c_char {
    // Compute the length of the result, taking account of unprintable
    // multi-byte characters.
    let len = transstr_len(s) + 1;
    let buf: *mut libc::c_char = xmalloc(len);
    transstr_buf(s, buf, len);
    return buf;
}

/// Convert the string "str[orglen]" to do ignore-case comparing.
/// Use the current locale.
///
/// When "buf" is NULL, return an allocated string.
/// Otherwise, put the result in buf, limited by buflen, and return buf.
#[no_mangle]
pub unsafe extern "C" fn str_foldcase(
    str: *mut libc::c_uchar,
    orglen: libc::c_int,
    in_buf: *mut libc::c_uchar,
    buflen: libc::c_int,
) -> *mut libc::c_uchar {
    enum OutputContainer {
        Buf(*mut u8, i32),
        GA(garray_T),
    }
    impl OutputContainer {
        unsafe fn offset<I: TryInto<isize>>(&mut self, i: I) -> *mut u8
        where
            <I as TryInto<isize>>::Error: std::fmt::Debug,
        {
            let i = i.try_into().unwrap();
            match self {
                GA(ga) => (ga.ga_data as *mut u8).offset(i),
                Buf(buf, _) => buf.offset(i),
            }
        }
        fn len(&self) -> i32 {
            match self {
                GA(ga) => ga.ga_len,
                Buf(_, len) => *len,
            }
        }
    }
    use OutputContainer::{Buf, GA};

    let mut out = if in_buf.is_null() {
        let mut ga = mem::zeroed();
        ga_init(&mut ga, 1, 10);
        GA(ga)
    } else {
        Buf(in_buf, orglen)
    };

    // Copy "str" into "buf" or allocated memory, unmodified.
    match &mut out {
        GA(ga) => {
            ga_grow(ga, orglen + 1);
            ga.ga_len = orglen
        }
        Buf(_, len) => {
            if *len >= buflen {
                // Ugly!
                *len = buflen - 1
            }
        }
    }

    memmove(out.offset(0), str, out.len());
    *out.offset(out.len()) = 0;

    // Make each character lower case.
    let mut i = 0;
    while *out.offset(i) != 0 {
        let c = utf_ptr2char(out.offset(i));
        let olen = utf_ptr2len(out.offset(i));
        let mut lc = mb_tolower(c);

        // Only replace the character when it is not an invalid
        // sequence (ASCII character or more than one byte) and
        // mb_tolower() doesn't return the original character.
        if (c < 0x80 || olen > 1) && c != lc {
            let mut nlen = utf_char2len(lc);

            // If the byte length changes need to shift the following
            // characters forward or backward.
            if olen != nlen {
                if nlen > olen {
                    match &mut out {
                        GA(ga) => ga_grow(ga, nlen - olen + 1),
                        Buf(_, len) if *len + nlen - olen >= buflen => {
                            // out of memory, keep old char
                            lc = c;
                            nlen = olen
                        }
                        Buf(_, _) => {}
                    }
                }

                if olen != nlen {
                    STRMOVE(out.offset(i + nlen), out.offset(i + olen));
                    match &mut out {
                        GA(ga) => ga.ga_len += nlen - olen,
                        Buf(_, len) => *len += nlen - olen,
                    }
                }
            }
            utf_char2bytes(lc, out.offset(i));
        }

        // skip to next multi-byte char
        i += utfc_ptr2len(out.offset(i));
    }

    out.offset(0)
}

// Catch 22: g_chartab[] can't be initialized before the options are
// initialized, and initializing options may cause transchar() to be called!
// When chartab_initialized == false don't use g_chartab[].
// Does NOT work for multi-byte characters, c must be <= 255.
// Also doesn't work for the first byte of a multi-byte, "c" must be a
// character!
static mut transchar_buf: [libc::c_uchar; 11] = [0; 11];

/// Translate a character into a printable one, leaving printable ASCII intact
///
/// All unicode characters are considered non-printable in this function.
///
/// @param[in]  c  Character to translate.
///
/// @return translated character into a static buffer.
#[no_mangle]
pub unsafe extern "C" fn transchar(mut c: libc::c_int) -> *mut libc::c_uchar {
    let mut i = 0;
    if IS_SPECIAL(c) {
        // special key code, display as ~@ char
        transchar_buf[0] = '~' as u8;
        transchar_buf[1] = '@' as u8;
        i = 2;
        c = K_SECOND(c);
    }

    if !chartab_initialized && (c >= ' ' as i32 && c <= '~' as i32)
        || c <= 0xff && vim_isprintc_strict(c)
    {
        // printable character
        transchar_buf[i] = c as u8;
        transchar_buf[i + 1] = 0;
    } else if c <= 0xff {
        transchar_nonprint(transchar_buf.as_mut_ptr().offset(i as isize), c);
    } else {
        transchar_hex(transchar_buf.as_mut_ptr().offset(i as isize) as *mut i8, c);
    }
    return transchar_buf.as_mut_ptr();
}

/// Like transchar(), but called with a byte instead of a character
///
/// Checks for an illegal UTF-8 byte.
///
/// @param[in]  c  Byte to translate.
///
/// @return pointer to translated character in transchar_buf.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn transchar_byte(c: libc::c_int) -> *mut libc::c_uchar {
    if c >= 0x80 {
        transchar_nonprint(transchar_buf.as_mut_ptr(), c);
        return transchar_buf.as_mut_ptr();
    }
    return transchar(c);
}

/// Convert non-printable characters to 2..4 printable ones
///
/// @warning Does not work for multi-byte characters, c must be <= 255.
///
/// @param[out]  buf  Buffer to store result in, must be able to hold at least
///                   5 bytes (conversion result + NUL).
/// @param[in]  c  Character to convert. NUL is assumed to be NL according to
///                `:h NL-used-for-NUL`.
#[no_mangle]
pub unsafe extern "C" fn transchar_nonprint(buf: *mut libc::c_uchar, mut c: libc::c_int) {
    if c == '\n' as i32 {
        // we use newline in place of a NUL
        c = 0;
    } else if c == '\r' as i32 && get_fileformat(curbuf) == EOL_MAC {
        // we use CR in place of  NL in this case
        c = '\n' as i32;
    }
    assert!(c <= 0xff);

    if dy_flags & DY_UHEX != 0 || c > 0x7f {
        // 'display' has "uhex"
        transchar_hex(buf as *mut i8, c);
    } else {
        // 0x00 - 0x1f and 0x7f
        *buf.offset(0) = '^' as u8;
        // DEL displayed as ^?
        *buf.offset(1) = (c ^ 0x40) as u8;

        *buf.offset(2) = 0;
    }
}

/// Convert a non-printable character to hex C string like "<FFFF>"
///
/// @param[out]  buf  Buffer to store result in.
/// @param[in]  c  Character to convert.
///
/// @return Number of bytes stored in buffer, excluding trailing NUL byte.
#[no_mangle]
pub unsafe extern "C" fn transchar_hex(buf: *mut libc::c_char, c: libc::c_int) -> libc::size_t {
    let mut i = 0;
    macro_rules! ass_buf {
        ($val: expr) => {{
            *buf.offset(i) = $val as i8;
            i += 1;
        }};
    }

    ass_buf!('<');
    if c > 255 {
        if c > 255 * 256 {
            ass_buf!(nr2hex(c as u32 >> 20));
            ass_buf!(nr2hex(c as u32 >> 16));
        }
        ass_buf!(nr2hex(c as u32 >> 12));
        ass_buf!(nr2hex(c as u32 >> 8));
    }
    ass_buf!(nr2hex(c as u32 >> 4));
    ass_buf!(nr2hex(c as u32));
    ass_buf!('>');
    *buf.offset(i) = 0;
    return i as usize;
}

/// Convert the lower 4 bits of byte "c" to its hex character
///
/// Lower case letters are used to avoid the confusion of <F1> being 0xf1 or
/// function key 1.
///
/// @param[in]  n  Number to convert.
///
/// @return the hex character.
#[inline]
#[must_use]
fn nr2hex(n: u32) -> u32 {
    if n & 0xf <= 9 {
        (n & 0xf) + '0' as u32
    } else {
        (n & 0xf) - 10 + 'a' as u32
    }
}

/// Return number of display cells occupied by byte "b".
///
/// Caller must make sure 0 <= b <= 255.
/// For multi-byte mode "b" must be the first byte of a character.
/// A TAB is counted as two cells: "^I".
/// This will return 0 for bytes >= 0x80, because the number of
/// cells depends on further bytes in UTF-8.
///
/// @reeturn Number of display cells.
#[no_mangle]
pub unsafe extern "C" fn byte2cells(b: libc::c_int) -> libc::c_int {
    if b >= 0x80 {
        0
    } else {
        (g_chartab[b as usize] & CT_CELL_MASK) as i32
    }
}

/// Return number of display cells occupied by character "c".
///
/// "c" can be a special key (negative number) in which case 3 or 4 is returned.
/// A TAB is counted as two cells: "^I" or four: "<09>".
///
/// @return Number of display cells.
#[no_mangle]
pub unsafe extern "C" fn char2cells(c: libc::c_int) -> libc::c_int {
    if IS_SPECIAL(c) {
        char2cells(K_SECOND(c)) + 2
    } else if c >= 0x80 {
        // UTF-8: above 0x80 need to check the value
        utf_char2cells(c)
    } else {
        (g_chartab[c as usize & 0xff] & CT_CELL_MASK) as i32
    }
}

/// Return number of display cells occupied by character at "*p".
/// A TAB is counted as two cells: "^I" or four: "<09>".
///
/// @return number of display cells.
#[no_mangle]
pub unsafe extern "C" fn ptr2cells(p: *const libc::c_uchar) -> libc::c_int {
    if *p >= 0x80 {
        // For UTF-8 we need to look at more bytes if the first byte is >= 0x80.
        utf_ptr2cells(p)
    } else {
        // For DBCS we can tell the cell count from the first byte.
        (g_chartab[*p as usize] & CT_CELL_MASK) as i32
    }
}

/// Return the number of character cells string "s" will take on the screen,
/// counting TABs as two characters: "^I".
///
/// 's' must be non-null.
///
/// @return number of character cells.
#[no_mangle]
pub unsafe extern "C" fn vim_strsize(s: *mut libc::c_uchar) -> libc::c_int {
    vim_strnsize(s, MAXCOL)
}

/// Return the number of character cells string "s[len]" will take on the
/// screen, counting TABs as two characters: "^I".
///
/// 's' must be non-null.
///
/// @return Number of character cells.
#[no_mangle]
pub unsafe extern "C" fn vim_strnsize(
    mut s: *mut libc::c_uchar,
    mut len: libc::c_int,
) -> libc::c_int {
    assert!(!s.is_null());
    let mut size = 0;
    while *s != 0 && len > 0 {
        let l = utfc_ptr2len(s);
        size += ptr2cells(s);
        s = s.offset(l as isize);
        len -= l;
    }
    return size;
}

/// Return the number of characters 'c' will take on the screen, taking
/// into account the size of a tab.
/// Use a define to make it fast, this is used very often!!!
/// Also see getvcol() below.
///
/// @return Number of characters.
/// RET_WIN_BUF_CHARTABSIZE
unsafe fn WIN_BUF_CHARTABSIZE(wp: &win_T, buf: &buf_T, p: *mut u8, col: colnr_T) -> i32 {
    if *p == '\t' as u8 && (wp.w_onebuf_opt.wo_list == 0 || wp.w_p_lcs_chars.tab1 != 0) {
        let ts = buf.b_p_ts as i32;
        ts - col % ts
    } else {
        ptr2cells(p)
    }
}

#[no_mangle]
pub unsafe extern "C" fn chartabsize(p: *mut libc::c_uchar, col: colnr_T) -> libc::c_int {
    WIN_BUF_CHARTABSIZE(curwin.as_ref().unwrap(), curbuf.as_ref().unwrap(), p, col)
}

unsafe fn win_chartabsize(wp: &win_T, p: *mut u8, col: colnr_T) -> i32 {
    WIN_BUF_CHARTABSIZE(wp, wp.w_buffer.as_ref().unwrap(), p, col)
}

/// Return the number of characters the string 's' will take on the screen,
/// taking into account the size of a tab.
///
/// @return Number of characters the string will take on the screen.
#[no_mangle]
pub unsafe extern "C" fn linetabsize(s: *mut libc::c_uchar) -> libc::c_int {
    linetabsize_col(0, s)
}

/// Like linetabsize(), but starting at column "startcol".
///
/// @return Number of characters the string will take on the screen.
#[no_mangle]
pub unsafe extern "C" fn linetabsize_col(
    startcol: libc::c_int,
    mut s: *mut libc::c_uchar,
) -> libc::c_int {
    let mut col = startcol;
    let line = s; /* pointer to start of line, for breakindent */
    while *s != 0 {
        col += lbr_chartabsize_adv(line, &mut s, col)
    }
    col
}

/// Like linetabsize(), but for a given window instead of the current one.
///
/// @return Number of characters the string will take on the screen.
#[no_mangle]
pub unsafe extern "C" fn win_linetabsize(
    wp: &mut win_T,
    line: *mut libc::c_uchar,
    len: colnr_T,
) -> libc::c_uint {
    let mut col = 0;

    let mut s = line;
    while *s != 0 && (len == MAXCOL || s < line.offset(len as isize)) {
        col += win_lbr_chartabsize(wp, line, s, col, ptr::null_mut());
        MB_PTR_ADV!(s);
    }
    col as u32
}

/// Check that "c" is a normal identifier character:
/// Letters and characters from the 'isident' option.
///
/// @param  c  character to check
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn vim_isIDc(c: libc::c_int) -> bool {
    c > 0 && c < 0x100 && g_chartab[c as usize] & CT_ID_CHAR != 0
}

/// Check that "c" is a keyword character:
/// Letters and characters from 'iskeyword' option for the current buffer.
/// For multi-byte characters mb_get_class() is used (builtin rules).
///
/// @param  c  character to check
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn vim_iswordc(c: libc::c_int) -> bool {
    vim_iswordc_buf(c, curbuf.as_ref().unwrap())
}

/// Check that "c" is a keyword character
/// Letters and characters from 'iskeyword' option for given buffer.
/// For multi-byte characters mb_get_class() is used (builtin rules).
///
/// @param[in]  c  Character to check.
/// @param[in]  chartab  Buffer chartab.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn vim_iswordc_tab(c: libc::c_int, chartab: *const u64) -> bool {
    if c >= 0x100 {
        utf_class_tab(c, chartab) >= 2
    } else {
        c > 0 && GET_CHARTAB_TAB(chartab, c) != 0
    }
}

/// Check that "c" is a keyword character:
/// Letters and characters from 'iskeyword' option for given buffer.
/// For multi-byte characters mb_get_class() is used (builtin rules).
///
/// @param  c    character to check
/// @param  buf  buffer whose keywords to use
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn vim_iswordc_buf(c: libc::c_int, buf: &buf_T) -> bool {
    vim_iswordc_tab(c, buf.b_chartab.as_ptr())
}

/// Just like vim_iswordc() but uses a pointer to the (multi-byte) character.
///
/// @param  p  pointer to the multi-byte character
///
/// @return true if "p" points to a keyword character.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn vim_iswordp(p: *const libc::c_uchar) -> bool {
    vim_iswordp_buf(p, curbuf.as_ref().unwrap())
}

/// Just like vim_iswordc_buf() but uses a pointer to the (multi-byte)
/// character.
///
/// @param  p    pointer to the multi-byte character
/// @param  buf  buffer whose keywords to use
///
/// @return true if "p" points to a keyword character.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn vim_iswordp_buf(p: *const libc::c_uchar, buf: &buf_T) -> bool {
    let mut c = *p as i32;

    if MB_BYTE2LEN(c as u8) > 1 {
        c = utf_ptr2char(p)
    }
    vim_iswordc_buf(c, buf)
}

/// Check that "c" is a valid file-name character.
/// Assume characters above 0x100 are valid (multi-byte).
///
/// @param  c  character to check
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn vim_isfilec(c: libc::c_int) -> bool {
    c >= 0x100 || c > 0 && g_chartab[c as usize] & CT_FNAME_CHAR != 0
}

/// Check that "c" is a valid file-name character or a wildcard character
/// Assume characters above 0x100 are valid (multi-byte).
/// Explicitly interpret ']' as a wildcard character as path_has_wildcard("]")
/// returns false.
///
/// @param  c  character to check
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn vim_isfilec_or_wc(c: libc::c_int) -> bool {
    let buf = [c as u8, 0];
    vim_isfilec(c) || c == ']' as i32 || path_has_wildcard(buf.as_ptr())
}

/// Check that "c" is a printable character.
/// Assume characters above 0x100 are printable for double-byte encodings.
///
/// @param  c  character to check
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn vim_isprintc(c: libc::c_int) -> bool {
    if c >= 0x100 {
        utf_printable(c)
    } else {
        c > 0 && g_chartab[c as usize] & CT_PRINT_CHAR != 0
    }
}

/// Strict version of vim_isprintc(c), don't return true if "c" is the head
/// byte of a double-byte character.
///
/// @param  c  character to check
///
/// @return true if "c" is a printable character.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn vim_isprintc_strict(c: libc::c_int) -> bool {
    if c >= 0x100 {
        utf_printable(c)
    } else {
        c > 0 && g_chartab[c as usize] & CT_PRINT_CHAR != 0
    }
}

/// like chartabsize(), but also check for line breaks on the screen
///
/// @return The number of characters taken up on the screen.
#[no_mangle]
pub unsafe extern "C" fn lbr_chartabsize(
    line: *mut libc::c_uchar,
    s: *mut libc::c_uchar,
    col: colnr_T,
) -> libc::c_int {
    if (*curwin).w_onebuf_opt.wo_lbr == 0 && *p_sbr == 0 && (*curwin).w_onebuf_opt.wo_bri == 0 {
        if (*curwin).w_onebuf_opt.wo_wrap != 0 {
            win_nolbr_chartabsize(curwin.as_mut().unwrap(), s, col, ptr::null_mut())
        } else {
            WIN_BUF_CHARTABSIZE(curwin.as_ref().unwrap(), curbuf.as_ref().unwrap(), s, col)
        }
    } else {
        win_lbr_chartabsize(
            curwin.as_mut().unwrap(),
            if line.is_null() { s } else { line },
            s,
            col,
            ptr::null_mut(),
        )
    }
}

/// Call lbr_chartabsize() and advance the pointer.
///
/// @return The number of characters take up on the screen.
#[no_mangle]
pub unsafe extern "C" fn lbr_chartabsize_adv(
    line: *mut libc::c_uchar,
    s: *mut *mut libc::c_uchar,
    col: colnr_T,
) -> libc::c_int {
    let retval = lbr_chartabsize(line, *s, col);
    MB_PTR_ADV!(*s);
    return retval;
}

/// This function is used very often, keep it fast!!!!
///
/// If "headp" not NULL, set *headp to the size of what we for 'showbreak'
/// string at start of line.  Warning: *headp is only set if it's a non-zero
/// value, init to 0 before calling.
///
/// @return The number of characters taken up on the screen.
#[no_mangle]
pub unsafe extern "C" fn win_lbr_chartabsize(
    wp: &mut win_T,
    line: *mut libc::c_uchar,
    mut s: *mut libc::c_uchar,
    mut col: colnr_T,
    headp: *mut libc::c_int,
) -> libc::c_int {
    let mut col2: colnr_T;
    let mut col_adj = 0; /* col + screen size of tab */
    let mut colmax: colnr_T;
    let mut added: i32;
    let mut mb_added: i32 = 0;
    let mut numberextra: i32;
    let mut ps;
    let n: i32;

    // No 'linebreak', 'showbreak' and 'breakindent': return quickly.
    if wp.w_onebuf_opt.wo_lbr == 0 && wp.w_onebuf_opt.wo_bri == 0 && *p_sbr == 0 {
        if wp.w_onebuf_opt.wo_wrap != 0 {
            return win_nolbr_chartabsize(wp, s, col, headp);
        }
        return WIN_BUF_CHARTABSIZE(wp, wp.w_buffer.as_mut().unwrap(), s, col);
    }

    // First get normal size, without 'linebreak'
    let mut size = win_chartabsize(wp, s, col);
    let mut c = *s as i32;
    if *s == '\t' as u8 {
        col_adj = size - 1
    }

    // If 'linebreak' set check at a blank before a non-blank if the line
    // needs a break here
    if wp.w_onebuf_opt.wo_lbr != 0
        && vim_isbreak(c)
        && !vim_isbreak(*s.offset(1) as i32)
        && wp.w_onebuf_opt.wo_wrap != 0
        && wp.w_width_inner != 0
    {
        // Count all characters from first non-blank after a blank up to next
        // non-blank after a blank.
        numberextra = win_col_off(wp);
        col2 = col;
        colmax = wp.w_width_inner - numberextra - col_adj;

        if col >= colmax {
            colmax += col_adj;
            n = colmax + win_col_off2(wp);

            if n > 0 {
                colmax += ((col - colmax) / n + 1) * n - col_adj
            }
        }

        loop {
            ps = s;
            MB_PTR_ADV!(s);
            c = *s as i32;

            if !(c != 0 && (vim_isbreak(c) || col2 == col || !vim_isbreak(*ps as i32))) {
                break;
            }

            col2 += win_chartabsize(wp, s, col2);

            if col2 >= colmax {
                /* doesn't fit */
                size = colmax - col + col_adj;
                break;
            }
        }
    } else if size == 2
        && MB_BYTE2LEN(*s) > 1
        && wp.w_onebuf_opt.wo_wrap != 0
        && in_win_border(wp, col)
    {
        // Count the ">" in the last column.
        size += 1;
        mb_added = 1
    }

    // May have to add something for 'breakindent' and/or 'showbreak'
    // string at start of line.
    // Set *headp to the size of what we add.
    added = 0;

    if (*p_sbr != 0 || wp.w_onebuf_opt.wo_bri != 0) && wp.w_onebuf_opt.wo_wrap != 0 && col != 0 {
        let mut sbrlen = 0;
        let mut numberwidth = win_col_off(wp);

        numberextra = numberwidth;
        col += numberextra + mb_added;

        if col >= wp.w_width_inner {
            col -= wp.w_width_inner;
            numberextra = wp.w_width_inner - (numberextra - win_col_off2(wp));
            if col >= numberextra && numberextra > 0 {
                col %= numberextra
            }
            if *p_sbr != 0 {
                sbrlen = mb_charlen(p_sbr);
                if col >= sbrlen {
                    col -= sbrlen
                }
            }
            if col >= numberextra && numberextra > 0 {
                col %= numberextra
            } else if col > 0 && numberextra > 0 {
                col += numberwidth - win_col_off2(wp)
            }

            numberwidth -= win_col_off2(wp)
        }

        if col == 0 || col + size + sbrlen > wp.w_width_inner {
            if *p_sbr != 0 {
                if size + sbrlen + numberwidth > wp.w_width_inner {
                    // Calculate effective window width.
                    let mut width = wp.w_width_inner - sbrlen - numberwidth;
                    let prev_width = if col != 0 {
                        (wp.w_width_inner) - (sbrlen + col)
                    } else {
                        0
                    };

                    if width <= 0 {
                        width = 1
                    }
                    added += (size - prev_width) / width * vim_strsize(p_sbr);
                    if (size - prev_width) % width != 0 {
                        // Wrapped, add another length of 'sbr'.
                        added += vim_strsize(p_sbr)
                    }
                } else {
                    added += vim_strsize(p_sbr)
                }
            }

            if wp.w_onebuf_opt.wo_bri != 0 {
                added += get_breakindent_win(wp, line)
            }

            size += added;
            if col != 0 {
                added = 0
            }
        }
    }

    if !headp.is_null() {
        *headp = added + mb_added
    }
    return size;
}

/// Like win_lbr_chartabsize(), except that we know 'linebreak' is off and
/// 'wrap' is on.  This means we need to check for a double-byte character that
/// doesn't fit at the end of the screen line.
///
/// @return The number of characters take up on the screen.
unsafe extern "C" fn win_nolbr_chartabsize(
    wp: &mut win_T,
    s: *mut libc::c_uchar,
    col: colnr_T,
    headp: *mut libc::c_int,
) -> libc::c_int {
    let n: i32;

    if *s == '\t' as u8 && (wp.w_onebuf_opt.wo_list == 0 || wp.w_p_lcs_chars.tab1 != 0) {
        n = (*wp.w_buffer).b_p_ts as i32;
        return n - col % n;
    }
    n = ptr2cells(s);

    // Add one cell for a double-width character in the last column of the
    // window, displayed with a ">".
    if n == 2 && MB_BYTE2LEN(*s) > 1 && in_win_border(wp, col) {
        if !headp.is_null() {
            *headp = 1
        }
        return 3;
    }
    return n;
}

/// Check that virtual column "vcol" is in the rightmost column of window "wp".
///
/// @param  wp    window
/// @param  vcol  column number
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn in_win_border(wp: &mut win_T, vcol: colnr_T) -> bool {
    let width1: i32; // width of first line (after line number)
    let width2: i32; // width of further lines

    if wp.w_width_inner == 0 {
        // there is no border
        return false;
    }
    width1 = wp.w_width_inner - win_col_off(wp);

    if vcol < width1 - 1 {
        return false;
    }

    if vcol == width1 - 1 {
        return true;
    }
    width2 = width1 + win_col_off2(wp);

    if width2 <= 0 {
        return false;
    }
    return (vcol - width1) % width2 == width2 - 1;
}

/// Get virtual column number of pos.
///  start: on the first position of this character (TAB, ctrl)
/// cursor: where the cursor is on this character (first char, except for TAB)
///    end: on the last position of this character (TAB, ctrl)
///
/// This is used very often, keep it fast!
#[no_mangle]
pub unsafe extern "C" fn getvcol(
    wp: &mut win_T,
    pos: &mut pos_T,
    start: Option<&mut colnr_T>,
    cursor: Option<&mut colnr_T>,
    end: Option<&mut colnr_T>,
) {
    let mut vcol: colnr_T;
    let mut ptr; // points to current char
    let mut posptr; // points to char at pos->col
    let line; // start of the line
    let mut incr: i32;
    let mut head: i32;
    let ts = (*wp.w_buffer).b_p_ts as i32;
    let mut c: i32;

    vcol = 0;
    ptr = ml_get_buf(wp.w_buffer, pos.lnum, false);

    line = ptr;
    if pos.col == MAXCOL {
        // continue until the NUL
        posptr = ptr::null_mut()
    } else {
        // Special check for an empty line, which can happen on exit, when
        // ml_get_buf() always returns an empty string.
        if *ptr == 0 {
            pos.col = 0
        }
        posptr = ptr.offset(pos.col as isize);
        posptr = posptr.offset(-(utf_head_off(line, posptr) as isize))
    }

    // This function is used very often, do some speed optimizations.
    // When 'list', 'linebreak', 'showbreak' and 'breakindent' are not set
    // use a simple loop.
    // Also use this when 'list' is set but tabs take their normal size.
    if (wp.w_onebuf_opt.wo_list == 0 || wp.w_p_lcs_chars.tab1 != 0)
        && wp.w_onebuf_opt.wo_lbr == 0
        && *p_sbr == 0
        && wp.w_onebuf_opt.wo_bri == 0
    {
        loop {
            head = 0;
            c = *ptr as i32;

            // make sure we don't go past the end of the line
            if c == 0 {
                // NUL at end of line only takes one column
                incr = 1;
                break;
            }

            // A tab gets expanded, depending on the current column
            if c == '\t' as i32 {
                incr = ts - vcol % ts;
            } else {
                // For utf-8, if the byte is >= 0x80, need to look at
                // further bytes to find the cell width.
                if c >= 0x80 as libc::c_int {
                    incr = utf_ptr2cells(ptr);
                } else {
                    incr = (g_chartab[c as usize] & CT_CELL_MASK) as i32;
                }

                // If a double-cell char doesn't fit at the end of a line
                // it wraps to the next line, it's like this char is three
                // cells wide.
                if incr == 2
                    && wp.w_onebuf_opt.wo_wrap != 0
                    && MB_BYTE2LEN(*ptr) > 1
                    && in_win_border(wp, vcol)
                {
                    incr += 1;
                    head = 1
                }
            }

            if !posptr.is_null() && ptr >= posptr {
                // character at pos->col
                break;
            }

            vcol += incr;
            MB_PTR_ADV!(ptr);
        }
    } else {
        loop {
            // A tab gets expanded, depending on the current column
            head = 0;
            incr = win_lbr_chartabsize(wp, line, ptr, vcol, &mut head);

            // make sure we don't go past the end of the line
            if *ptr == 0 {
                // NUL at end of line only takes one column
                incr = 1;
                break;
            }

            if !posptr.is_null() && ptr >= posptr {
                // character at pos->col
                break;
            }

            vcol += incr;
            MB_PTR_ADV!(ptr);
        }
    }

    if let Some(start) = start {
        *start = vcol + head;
    }

    if let Some(end) = end {
        *end = vcol + incr - 1;
    }

    if let Some(cursor) = cursor {
        if *ptr == '\t' as u8
            && State & NORMAL != 0
            && wp.w_onebuf_opt.wo_list == 0
            && !virtual_active()
            && !(VIsual_active != 0 && (*p_sel == 'e' as u8 || ltoreq(*pos, VIsual)))
        {
            // cursor at end
            *cursor = vcol + incr - 1;
        } else {
            // cursor at start
            *cursor = vcol + head;
        }
    }
}

/// Get virtual cursor column in the current window, pretending 'list' is off.
///
/// @return The virtual cursor column.
#[no_mangle]
pub unsafe extern "C" fn getvcol_nolist(posp: &mut pos_T) -> colnr_T {
    let cw = curwin.as_mut().unwrap();
    let list_save = cw.w_onebuf_opt.wo_list;
    let mut vcol: colnr_T = 0;

    cw.w_onebuf_opt.wo_list = 0;
    if posp.coladd != 0 {
        getvvcol(cw, posp, None, Some(&mut vcol), None);
    } else {
        getvcol(cw, posp, None, Some(&mut vcol), None);
    }
    cw.w_onebuf_opt.wo_list = list_save;
    return vcol;
}

/// Get virtual column in virtual mode.
#[no_mangle]
pub unsafe extern "C" fn getvvcol(
    wp: &mut win_T,
    pos: &mut pos_T,
    start: Option<&mut colnr_T>,
    cursor: Option<&mut colnr_T>,
    end: Option<&mut colnr_T>,
) {
    let mut col: colnr_T = 0;
    let mut coladd: colnr_T;
    let mut endadd: colnr_T;
    let ptr;

    if virtual_active() {
        // For virtual mode, only want one value
        getvcol(wp, pos, Some(&mut col), None, None);

        coladd = pos.coladd;
        endadd = 0;

        // Cannot put the cursor on part of a wide character.
        ptr = ml_get_buf(wp.w_buffer, pos.lnum, false);

        if pos.col < strlen(ptr as *mut i8) as colnr_T {
            let c = utf_ptr2char(ptr.offset(pos.col as isize));
            if c != '\t' as i32 && vim_isprintc(c) {
                endadd = char2cells(c) - 1;
                if coladd > endadd {
                    // past end of line
                    endadd = 0
                } else {
                    coladd = 0
                }
            }
        }
        col += coladd;

        if let Some(start) = start {
            *start = col
        }
        if let Some(cursor) = cursor {
            *cursor = col
        }
        if let Some(end) = end {
            *end = col + endadd
        }
    } else {
        getvcol(wp, pos, start, cursor, end);
    };
}

/// Get the leftmost and rightmost virtual column of pos1 and pos2.
/// Used for Visual block mode.
#[no_mangle]
pub unsafe extern "C" fn getvcols(
    wp: &mut win_T,
    pos1: &mut pos_T,
    pos2: &mut pos_T,
    left: &mut colnr_T,
    right: &mut colnr_T,
) {
    let mut from1: colnr_T = 0;
    let mut from2: colnr_T = 0;
    let mut to1: colnr_T = 0;
    let mut to2: colnr_T = 0;

    if lt(*pos1, *pos2) {
        getvvcol(wp, pos1, Some(&mut from1), None, Some(&mut to1));
        getvvcol(wp, pos2, Some(&mut from2), None, Some(&mut to2));
    } else {
        getvvcol(wp, pos2, Some(&mut from1), None, Some(&mut to1));
        getvvcol(wp, pos1, Some(&mut from2), None, Some(&mut to2));
    }

    *left = min(from1, from2);

    *right = if to2 > to1 {
        if *p_sel == 'e' as u8 && from2 - 1 >= to1 {
            from2 - 1
        } else {
            to2
        }
    } else {
        to1
    };
}

/// skipwhite: skip over ' ' and '\t'.
///
/// @param[in]  q  String to skip in.
///
/// @return Pointer to character after the skipped whitespace.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn skipwhite(q: *const libc::c_uchar) -> *const libc::c_uchar {
    let mut p = q;
    while ascii_iswhite(*p as char) {
        p = p.offset(1);
    }
    return p;
}

/// getwhitecols: return the number of whitespace
/// columns (bytes) at the start of a given line
#[no_mangle]
pub unsafe extern "C" fn getwhitecols_curline() -> libc::intptr_t {
    return getwhitecols(get_cursor_line_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn getwhitecols(p: *const libc::c_uchar) -> libc::intptr_t {
    return skipwhite(p).offset_from(p);
}

/// Skip over digits
///
/// @param[in]  q  String to skip digits in.
///
/// @return Pointer to the character after the skipped digits.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn skipdigits(q: *const libc::c_uchar) -> *const libc::c_uchar {
    let mut p = q;
    while ascii_isdigit(*p as char) {
        // skip to next non-digit
        p = p.offset(1);
    }
    return p;
}

/// skip over binary digits
///
/// @param q pointer to string
///
/// @return Pointer to the character after the skipped digits.
#[no_mangle]
pub unsafe extern "C" fn skipbin(q: *const libc::c_char) -> *const libc::c_char {
    let mut p = q;
    while ascii_isbdigit(*p as u8 as char) {
        // skip to next non-digit
        p = p.offset(1);
    }
    return p;
}

/// skip over digits and hex characters
///
/// @return Pointer to the character after the skipped digits and hex
///         characters.
#[no_mangle]
pub unsafe extern "C" fn skiphex(q: *mut libc::c_uchar) -> *mut libc::c_uchar {
    let mut p = q;
    while ascii_isxdigit(*p as char) {
        // skip to next non-digit
        p = p.offset(1)
    }
    return p;
}

/// skip to digit (or NUL after the string)
///
/// @return Pointer to the digit or (NUL after the string).
#[no_mangle]
pub unsafe extern "C" fn skiptodigit(q: *mut libc::c_uchar) -> *mut libc::c_uchar {
    let mut p = q;
    while *p != 0 && !ascii_isdigit(*p as char) {
        // skip to next digit
        p = p.offset(1);
    }
    return p;
}

/// skip to binary character (or NUL after the string)
///
/// @param q pointer to string
///
/// @return Pointer to the binary character or (NUL after the string).
#[no_mangle]
pub unsafe extern "C" fn skiptobin(q: *const libc::c_char) -> *const libc::c_char {
    let mut p = q;
    while *p != 0 && !ascii_isbdigit(*p as u8 as char) {
        // skip to next digit
        p = p.offset(1);
    }
    return p;
}

/// skip to hex character (or NUL after the string)
///
/// @return Pointer to the hex character or (NUL after the string).
#[no_mangle]
pub unsafe extern "C" fn skiptohex(q: *mut libc::c_uchar) -> *mut libc::c_uchar {
    let mut p = q;
    while *p != 0 && !ascii_isxdigit(*p as char) {
        // skip to next digit
        p = p.offset(1);
    }
    return p;
}

/// Skip over text until ' ' or '\t' or NUL
///
/// @param[in]  p  Text to skip over.
///
/// @return Pointer to the next whitespace or NUL character.
#[no_mangle]
pub unsafe extern "C" fn skiptowhite(mut p: *mut libc::c_uchar) -> *mut libc::c_uchar {
    while *p != ' ' as u8 && *p != '\t' as u8 && *p != 0 {
        p = p.offset(1);
    }
    return p;
}

/// skiptowhite_esc: Like skiptowhite(), but also skip escaped chars
///
/// @param p
///
/// @return Pointer to the next whitespace character.
#[no_mangle]
pub unsafe extern "C" fn skiptowhite_esc(mut p: *mut libc::c_uchar) -> *mut libc::c_uchar {
    while *p != ' ' as u8 && *p != '\t' as u8 && *p != 0 {
        if (*p == '\\' as u8 || *p == Ctrl_V as u8) && *p.offset(1) != 0 {
            p = p.offset(1);
        }
        p = p.offset(1);
    }
    return p;
}

/// Gets a number from a string and skips over it, signalling overflow.
///
/// @param[out]  pp  A pointer to a pointer to libc::c_uchar.
///                  It will be advanced past the read number.
/// @param[out]  nr  Number read from the string.
///
/// @return true on success, false on error/overflow
#[no_mangle]
pub unsafe extern "C" fn try_getdigits(pp: *mut *const libc::c_uchar, nr: *mut intmax_t) -> bool {
    *__errno_location() = 0;
    *nr = strtoimax(*pp as *const i8, pp as *mut *const i8, 10);
    if *__errno_location() == libc::ERANGE
        && (*nr == intmax_t::min_value() || *nr == intmax_t::max_value())
    {
        return false;
    }
    return true;
}

/// Gets a number from a string and skips over it.
///
/// @param[out]  pp  Pointer to a pointer to libc::c_uchar.
///                  It will be advanced past the read number.
/// @param strict    Abort on overflow.
/// @param def       Default value, if parsing fails or overflow occurs.
///
/// @return Number read from the string, or `def` on parse failure or overflow.
#[no_mangle]
pub unsafe extern "C" fn getdigits(
    pp: *mut *const libc::c_uchar,
    strict: bool,
    def: intmax_t,
) -> intmax_t {
    let mut number: intmax_t = 0;
    let ok = try_getdigits(pp, &mut number);
    if strict && !ok {
        abort();
    }
    return if ok { number } else { def };
}

/// Gets an int number from a string.
///
/// @see getdigits
#[no_mangle]
pub unsafe extern "C" fn getdigits_int(
    pp: *mut *const u8,
    strict: bool,
    def: libc::c_int,
) -> libc::c_int {
    getdigits(pp, strict, def as intmax_t).try_into().unwrap()
}

/// Gets a long number from a string.
///
/// @see getdigits
#[no_mangle]
pub unsafe extern "C" fn getdigits_long(
    pp: *mut *const libc::c_uchar,
    strict: bool,
    def: libc::c_long,
) -> libc::c_long {
    getdigits(pp, strict, def)
}

/// Check that "lbuf" is empty or only contains blanks.
///
/// @param  lbuf  line buffer to check
#[no_mangle]
pub unsafe extern "C" fn vim_isblankline(lbuf: *mut libc::c_uchar) -> bool {
    let p = skipwhite(lbuf);
    return *p == 0 || *p == '\r' as u8 || *p == '\n' as u8;
}

/// Convert a string into a long and/or unsigned long, taking care of
/// hexadecimal, octal and binary numbers.  Accepts a '-' sign.
/// If "prep" is not NULL, returns a flag to indicate the type of the number:
///   0      decimal
///   '0'    octal
///   'B'    bin
///   'b'    bin
///   'X'    hex
///   'x'    hex
/// If "len" is not NULL, the length of the number in characters is returned.
/// If "nptr" is not NULL, the signed result is returned in it.
/// If "unptr" is not NULL, the unsigned result is returned in it.
/// If "what" contains STR2NR_BIN recognize binary numbers.
/// If "what" contains STR2NR_OCT recognize octal numbers.
/// If "what" contains STR2NR_HEX recognize hex numbers.
/// If "what" contains STR2NR_FORCE always assume bin/oct/hex.
/// If maxlen > 0, check at a maximum maxlen chars.
///
/// @param start
/// @param prep Returns guessed type of number 0 = decimal, 'x' or 'X' is
///             hexadecimal, '0' = octal, 'b' or 'B' is binary. When using
///             STR2NR_FORCE is always zero.
/// @param len Returns the detected length of number.
/// @param what Recognizes what number passed, @see ChStr2NrFlags.
/// @param nptr Returns the signed result.
/// @param unptr Returns the unsigned result.
/// @param maxlen Max length of string to check.
#[no_mangle]
pub unsafe extern "C" fn vim_str2nr(
    start: *const libc::c_uchar,
    prep: Option<&mut libc::c_int>,
    len: Option<&mut libc::c_int>,
    what: libc::c_int,
    nptr: Option<&mut varnumber_T>,
    unptr: Option<&mut uvarnumber_T>,
    maxlen: libc::c_int,
) {
    enum NumberType {
        Bin,
        Oct,
        Dec,
        Hex,
    }
    let number_type: NumberType;
    let mut ptr = start as *const u8;
    let STRING_ENDED =
        |ptr: *const u8| -> bool { !(maxlen == 0 || (ptr.offset_from(start) as i32) < maxlen) };
    let mut pre = 0; // default is decimal
    let negative = *ptr.offset(0) == '-' as u8;
    let mut un = 0;

    if negative {
        ptr = ptr.offset(1);
    }

    if what & STR2NR_FORCE != 0 {
        // When forcing main consideration is skipping the prefix. Octal and decimal
        // numbers have no prefixes to skip. pre is not set.
        match what & !STR2NR_FORCE {
            STR2NR_HEX => {
                if !STRING_ENDED(ptr.offset(2))
                    && *ptr.offset(0) == '0' as u8
                    && (*ptr.offset(1) == 'x' as u8 || *ptr.offset(1) == 'X' as u8)
                    && ascii_isxdigit(*ptr.offset(2) as char)
                {
                    ptr = ptr.offset(2);
                }
                number_type = NumberType::Hex;
            }
            STR2NR_BIN => {
                if !STRING_ENDED(ptr.offset(2))
                    && *ptr.offset(0) == '0' as u8
                    && (*ptr.offset(1) == 'b' as u8 || *ptr.offset(1) == 'B' as u8)
                    && ascii_isbdigit(*ptr.offset(2) as char)
                {
                    ptr = ptr.offset(2);
                }
                number_type = NumberType::Bin;
            }
            STR2NR_OCT => {
                number_type = NumberType::Oct;
            }
            STR2NR_DEC => {
                number_type = NumberType::Dec;
            }
            _ => {
                unreachable!();
            }
        }
    } else if what & (STR2NR_HEX | STR2NR_OCT | STR2NR_BIN) != 0
        && !STRING_ENDED(ptr.offset(1))
        && *ptr.offset(0) == '0' as u8
        && *ptr.offset(1) != '8' as u8
        && *ptr.offset(1) != '9' as u8
    {
        pre = *ptr.offset(1);
        if what & STR2NR_HEX != 0
            && !STRING_ENDED(ptr.offset(2))
            && (pre == 'X' as u8 || pre == 'x' as u8)
            && ascii_isxdigit(*ptr.offset(2) as char)
        {
            // Detect hexadecimal: 0x or 0X followed by hex digit.
            ptr = ptr.offset(2);
            number_type = NumberType::Hex;
        } else if what & STR2NR_BIN != 0
            && !STRING_ENDED(ptr.offset(2))
            && (pre == 'B' as u8 || pre == 'b' as u8)
            && ascii_isbdigit(*ptr.offset(2) as char)
        {
            // Detect binary: 0b or 0B followed by 0 or 1.
            ptr = ptr.offset(2);
            number_type = NumberType::Bin;
        } else {
            // Detect octal number: zero followed by octal digits without '8' or '9'.
            pre = 0;
            if what & STR2NR_OCT == 0
                || !('0' as u8 <= *ptr.offset(1) && *ptr.offset(1) <= '7' as u8)
            {
                number_type = NumberType::Dec;
            } else {
                let mut decimal = false;
                for i in 2.. {
                    if STRING_ENDED(ptr.offset(i)) || !ascii_isdigit(*ptr.offset(i) as char) {
                        break;
                    }
                    if *ptr.offset(i) > '7' as u8 {
                        decimal = true;
                        break;
                    }
                }
                if decimal {
                    number_type = NumberType::Dec;
                } else {
                    pre = '0' as u8;
                    number_type = NumberType::Oct;
                }
            }
        }
    } else {
        number_type = NumberType::Dec;
    }

    // Do the string-to-numeric conversion "manually" to avoid sscanf quirks.
    let base = match number_type {
        NumberType::Bin => 2,
        NumberType::Oct => 8,
        NumberType::Dec => 10,
        NumberType::Hex => 16,
    };
    let cond = |c| match number_type {
        NumberType::Bin => c == '0' as u8 || c == '1' as u8,
        NumberType::Oct => '0' as u8 <= c && c <= '7' as u8,
        NumberType::Dec => ascii_isdigit(c as char),
        NumberType::Hex => ascii_isxdigit(c as char),
    };
    let conv = |c| match number_type {
        NumberType::Bin | NumberType::Oct | NumberType::Dec => c - '0' as u8,
        NumberType::Hex => hex2nr(c as i32) as u8,
    };
    while !STRING_ENDED(ptr) && cond(*ptr) {
        let digit = conv(*ptr) as uvarnumber_T;
        // avoid ubsan error for overflow
        if un < uvarnumber_T::MAX / base
            || un == uvarnumber_T::MAX / base && (base != 10 || digit <= uvarnumber_T::MAX % 10)
        {
            un = base * un + digit;
        } else {
            un = uvarnumber_T::MAX;
        }
        ptr = ptr.offset(1);
    }

    if let Some(prep) = prep {
        *prep = pre as i32;
    }
    if let Some(len) = len {
        *len = ptr.offset_from(start) as i32;
    }
    if let Some(nptr) = nptr {
        // account for leading '-' for decimal numbers
        if negative {
            // avoid ubsan error for overflow
            if un > varnumber_T::MAX as uvarnumber_T {
                *nptr = varnumber_T::MIN;
            } else {
                *nptr = -(un as varnumber_T)
            }
        } else {
            if un > varnumber_T::MAX as uvarnumber_T {
                un = varnumber_T::MAX as uvarnumber_T;
            }
            *nptr = un as varnumber_T
        }
    }

    if let Some(unptr) = unptr {
        *unptr = un;
    }
}

/// Return the value of a single hex character.
/// Only valid when the argument is '0' - '9', 'A' - 'F' or 'a' - 'f'.
///
/// @return The value of the hex character.
#[no_mangle]
pub unsafe extern "C" fn hex2nr(c: libc::c_int) -> libc::c_int {
    if c >= 'a' as i32 && c <= 'f' as i32 {
        return c - 'a' as i32 + 10;
    }
    if c >= 'A' as i32 && c <= 'F' as i32 {
        return c - 'A' as i32 + 10;
    }
    return c - '0' as i32;
}

/// Check that "str" starts with a backslash that should be removed.
/// For Windows this is only done when the character after the
/// backslash is not a normal file name character.
/// '$' is a valid file name character, we don't remove the backslash before
/// it.  This means it is not possible to use an environment variable after a
/// backslash.  "C:\$VIM\doc" is taken literally, only "$VIM\doc" works.
/// Although "\ name" is valid, the backslash in "Program\ files" must be
/// removed.  Assume a file name doesn't start with a space.
/// For multi-byte names, never remove a backslash before a non-ascii
/// character, assume that all multi-byte characters are valid file name
/// characters.
///
/// @param  str  file path string to check
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn rem_backslash(str: *const libc::c_uchar) -> bool {
    if cfg!(windows) {
        unimplemented!();
    }
    return *str.offset(0) == '\\' as u8 && *str.offset(1) != 0;
}

/// Halve the number of backslashes in a file name argument.
#[no_mangle]
pub unsafe extern "C" fn backslash_halve(mut p: *mut libc::c_uchar) {
    while *p != 0 {
        if rem_backslash(p) {
            STRMOVE(p, p.offset(1));
        }
        p = p.offset(1);
    }
}

/// backslash_halve() plus save the result in allocated memory.
///
/// @return String with the number of backslashes halved.
#[no_mangle]
pub unsafe extern "C" fn backslash_halve_save(p: *const libc::c_uchar) -> *mut libc::c_uchar {
    // TODO(philix): simplify and improve backslash_halve_save algorithm
    let res = vim_strsave(p);
    backslash_halve(res);
    return res;
}
