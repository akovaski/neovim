use crate::*;
use std::mem;
use std::ptr;

/// One parsed line
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParserLine {
    pub data: *const u8,    // Parsed line pointer
    pub size: libc::size_t, // Parsed line size
    pub allocated: bool,    // True if line may be freed
}
impl Default for ParserLine {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

/// Line getter type for parser
///
/// Line getter must return {NULL, 0} for EOF.
pub type ParserLineGetter =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut ParserLine) -> ()>;

/// Parser position in the input
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParserPosition {
    pub line: libc::size_t, // Line index in ParserInputReader.lines
    pub col: libc::size_t,  // Byte index in the line
}

/// Parser state item.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParserStateItem {
    pub type_0: kPTopStateParsing,
    pub data: PSI_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub enum kPTopStateParsing {
    Command = 0,
    Expression = 1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union PSI_data {
    pub expr: PSI_data_expr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PSI_data_expr {
    pub type_0: PSI_data_expr_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub enum PSI_data_expr_type {
    kExprUnknown = 0,
}

/// Structure defining input reader
#[repr(C)]
pub struct ParserInputReader {
    // Function used to get next line
    pub get_line: ParserLineGetter,
    // Data for get_line function
    pub cookie: *mut libc::c_void,
    // All lines obtained by get_line
    pub lines: ParserLine_Vec,
    // Conversion, for :scriptencoding
    pub conv: vimconv_T,
}
pub type ParserLine_Vec = kvec_withinit_t!(ParserLine, 4);

/// Highlighted region definition
///
/// Note: one chunk may highlight only one line.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParserHighlightChunk {
    pub start: ParserPosition,      // Start of the highlight: line and column
    pub end_col: libc::size_t,      // End column, points to the start of the next character
    pub group: *const libc::c_char, // Highlight group
}

// Highlighting defined by a parser
pub type ParserHighlight = kvec_withinit_t!(ParserHighlightChunk, 16);

/// Structure defining parser state
#[repr(C)]
pub struct ParserState {
    // Line reader.
    pub reader: ParserInputReader,
    // Position up to which input was parsed.
    pub pos: ParserPosition,
    // Parser state stack.
    pub stack: ParserStateItem_Vec,
    // Highlighting support.
    pub colors: *mut ParserHighlight,
    // True if line continuation can be used.
    pub can_continuate: bool,
}
pub type ParserStateItem_Vec = kvec_withinit_t!(ParserStateItem, 16);

/// Get one line from ParserInputReader
#[no_mangle]
pub unsafe fn viml_preader_get_line(preader: &mut ParserInputReader, ret_pline: &mut ParserLine) {
    let mut pline = ParserLine::default();
    preader.get_line.expect("non-null function pointer")(preader.cookie, &mut pline);
    if preader.conv.vc_type != CONV_NONE as libc::c_int && pline.size != 0 {
        let mut cpline = ParserLine {
            data: ptr::null(),
            size: pline.size,
            allocated: true,
        };
        cpline.data = string_convert(&mut preader.conv, pline.data as *mut u8, &mut cpline.size);
        if pline.allocated {
            xfree(pline.data as *mut libc::c_void);
        }
        pline = cpline
    }
    preader.lines.push(pline);
    *ret_pline = pline;
}

/// Get currently parsed line, shifted to pstate->pos.col
///
/// @param  pstate  Parser state to operate on.
///
/// @return True if there is a line, false in case of EOF.
#[no_mangle]
pub unsafe fn viml_parser_get_remaining_line(
    pstate: &mut ParserState,
    ret_pline: &mut ParserLine,
) -> bool {
    let num_lines = pstate.reader.lines.size();
    if pstate.pos.line == num_lines {
        viml_preader_get_line(&mut pstate.reader, ret_pline);
    } else {
        *ret_pline = *pstate.reader.lines.last();
    }
    c_assert!(pstate.pos.line == pstate.reader.lines.size() - 1);
    if !ret_pline.data.is_null() {
        ret_pline.data = ret_pline.data.offset(pstate.pos.col as isize);
        ret_pline.size -= pstate.pos.col;
    }
    return !ret_pline.data.is_null();
}

/// Advance position by a given number of bytes
///
/// At maximum advances to the next line.
///
/// @param  pstate  Parser state to advance.
/// @param[in]  len  Number of bytes to advance.
#[no_mangle]
pub unsafe fn viml_parser_advance(pstate: &mut ParserState, len: libc::size_t) {
    c_assert!(pstate.pos.line == pstate.reader.lines.size() - 1);
    let pline = *pstate.reader.lines.last();
    if pstate.pos.col + len >= pline.size {
        pstate.pos.line += 1;
        pstate.pos.col = 0;
    } else {
        pstate.pos.col += len;
    }
}

/// Record highlighting of some region of text
///
/// @param  pstate  Parser state to work with.
/// @param[in]  start  Start position of the highlight.
/// @param[in]  len  Highlighting chunk length.
/// @param[in]  group  Highlight group.
#[no_mangle]
pub unsafe fn viml_parser_highlight(
    pstate: &mut ParserState,
    start: ParserPosition,
    len: libc::size_t,
    group: *const libc::c_char,
) {
    if pstate.colors.is_null() || len == 0 {
        return;
    }
    c_assert!(
        (*pstate.colors).size() == 0
            || (*pstate.colors).Z(0).start.line < start.line
            || (*pstate.colors).Z(0).end_col <= start.col
    );
    (*pstate.colors).push(ParserHighlightChunk {
        start: start,
        end_col: start.col + len,
        group: group,
    });
}

#[no_mangle]
pub unsafe fn parser_simple_get_line(cookie: &mut *const ParserLine, ret_pline: &mut ParserLine) {
    *ret_pline = **cookie;
    *cookie = cookie.add(1);
}
