#[repr(C)]
#[derive(Clone, Copy)]
struct ParserLine {
    data: *mut u8,
    size: usize,
    allocated: bool,
}

#[no_mangle]
unsafe extern "C" fn parser_simple_get_line(
    cookie: *mut *const ParserLine,
    ret_pline: *mut ParserLine,
) {
    *ret_pline = **cookie;
    *cookie = (*cookie).add(1);
}
