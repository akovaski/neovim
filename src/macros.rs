use crate::*;

macro_rules! MB_PTR_ADV {
    ($p: expr) => {{
        use crate::mbyte::utfc_ptr2len;
        $p = $p.offset(utfc_ptr2len($p) as isize)
    }};
}

pub unsafe fn BUFEMPTY() -> bool {
    (*curbuf).b_ml.ml_line_count == 1 && *ml_get(1) == 0
}

pub unsafe fn RESET_BINDING(wp: *mut win_T) {
    let wp = wp.as_mut().unwrap();
    wp.w_onebuf_opt.wo_scb = false_0;
    wp.w_onebuf_opt.wo_crb = false_0;
}
