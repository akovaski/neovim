macro_rules! MB_PTR_ADV {
    ($p: expr) => {{
        use crate::mbyte::utfc_ptr2len;
        $p = $p.offset(utfc_ptr2len($p) as isize)
    }};
}
