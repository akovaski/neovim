extern "C" {
    pub fn trans_special(
        srcp: *mut *const libc::c_uchar,
        src_len: libc::size_t,
        dst: *mut libc::c_uchar,
        keycode: bool,
        in_string: bool,
    ) -> libc::c_uint;
}

/*
 * K_SPECIAL is the first byte of a special key code and is always followed by
 * two bytes.
 * The second byte can have any value. ASCII is used for normal termcap
 * entries, 0x80 and higher for special keys, see below.
 * The third byte is guaranteed to be between 0x02 and 0x7f.
 */
const K_SPECIAL: i32 = 0x80;

/*
 * NUL cannot be in the input string, therefore it is replaced by
 *	K_SPECIAL   KS_ZERO	KE_FILLER
 */
const KS_ZERO: i32 = 255;

/*
 * K_SPECIAL cannot be in the input string, therefore it is replaced by
 *	K_SPECIAL   KS_SPECIAL	KE_FILLER
 */
const KS_SPECIAL: i32 = 254;

/*
 * Positive characters are "normal" characters.
 * Negative characters are special key codes.  Only characters below -0x200
 * are used to so that the absolute value can't be mistaken for a single-byte
 * character.
 */
pub fn IS_SPECIAL(c: i32) -> bool {
    c < 0
}

/*
 * translation of three byte code "K_SPECIAL a b" into int "K_xxx" and back
 */
fn KEY2TERMCAP0(x: i32) -> i32 {
    (-x) & 0xff
}

/*
 * get second or third byte when translating special key code into three bytes
 */
pub fn K_SECOND(c: i32) -> i32 {
    if c == K_SPECIAL {
        KS_SPECIAL
    } else if c == 0 {
        KS_ZERO
    } else {
        KEY2TERMCAP0(c)
    }
}
