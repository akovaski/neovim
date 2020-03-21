pub type khint_t = u32;
pub type khint32_t = u32;
pub type khiter_t = khint_t;

macro_rules! KHASH_IMPL {
    ($name: ident, $mod_name: ident, $khkey_t: ty, $khval_t: ty, $hash_func: expr, $hash_equal: expr) => {
        //, $khkey_t: ident, $khval_t: ident, $kh_is_map: expr, $hash_func: expr, $hash_equal: expr) => {
        mod $mod_name {
            use crate::*;
            use std::mem;
            use std::ptr;
            pub unsafe extern "C" fn init() -> *mut concat_idents!(kh_, $name, _t) {
                return xcalloc(1, mem::size_of::<concat_idents!(kh_, $name, _t)>()) as *mut _;
            }
            pub unsafe extern "C" fn clear(mut h: *mut concat_idents!(kh_, $name, _t)) {
                if !h.is_null() && !(*h).flags.is_null() {
                    memset(
                        (*h).flags,
                        0xaa,
                        (if (*h).n_buckets < 16 {
                            1
                        } else {
                            ((*h).n_buckets) >> 4
                        } as usize)
                            * std::mem::size_of::<khint32_t>(),
                    );
                    (*h).n_occupied = 0;
                    (*h).size = (*h).n_occupied;
                }
            }
            pub unsafe extern "C" fn resize(
                mut h: *mut concat_idents!(kh_, $name, _t),
                mut new_n_buckets: khint_t,
            ) {
                // This function uses 0.25*n_buckets bytes of working space instead of
                // [sizeof(key_t+val_t)+.25]*n_buckets.

                let mut new_flags: *mut khint32_t = ptr::null_mut();
                let mut j: khint_t = 1;
                new_n_buckets = new_n_buckets.wrapping_sub(1);
                new_n_buckets |= new_n_buckets >> 1;
                new_n_buckets |= new_n_buckets >> 2;
                new_n_buckets |= new_n_buckets >> 4;
                new_n_buckets |= new_n_buckets >> 8;
                new_n_buckets |= new_n_buckets >> 16;
                new_n_buckets = new_n_buckets.wrapping_add(1);
                if new_n_buckets < 4 {
                    new_n_buckets = 4
                }
                /* requested size is too small */
                if (*h).size >= (new_n_buckets as f64 * 0.77f64 + 0.5f64) as khint_t {
                    j = 0
                } else {
                    /* hash table size to be changed (shrink or expand); rehash */
                    new_flags = xmalloc(
                        ((if new_n_buckets < 16 {
                            1
                        } else {
                            (new_n_buckets) >> 4
                        }) as usize)
                            * mem::size_of::<khint32_t>(),
                    ) as *mut khint32_t;
                    memset(
                        new_flags as *mut libc::c_void,
                        0xaa as libc::c_int,
                        ((if new_n_buckets < 16 {
                            1
                        } else {
                            (new_n_buckets) >> 4
                        }) as usize)
                            * std::mem::size_of::<khint32_t>(),
                    );
                    if (*h).n_buckets < new_n_buckets {
                        let new_keys: *mut $khkey_t = xrealloc(
                            (*h).keys as *mut libc::c_void,
                            (new_n_buckets as usize) * mem::size_of::<$khkey_t>(),
                        ) as *mut $khkey_t;
                        (*h).keys = new_keys;
                        let new_vals: *mut $khval_t = xrealloc(
                            (*h).vals as *mut libc::c_void,
                            (new_n_buckets as usize) * mem::size_of::<$khval_t>(),
                        ) as *mut $khval_t;
                        (*h).vals = new_vals
                    }
                }
                if j != 0 {
                    j = 0;
                    while j != (*h).n_buckets {
                        if *(*h).flags.offset((j >> 4) as isize) >> ((j & 0xf as libc::c_uint) << 1)
                            & 3
                            == 0
                        {
                            let mut key: $khkey_t = *(*h).keys.offset(j as isize);
                            let new_mask: khint_t;
                            new_mask = new_n_buckets.wrapping_sub(1 as libc::c_uint);
                            let mut val: $khval_t = *(*h).vals.offset(j as isize);
                            let ref mut fresh90 = *(*h).flags.offset((j >> 4) as isize);
                            *fresh90 |= (1) << ((j & 0xf as libc::c_uint) << 1);
                            loop {
                                let k: khint_t;
                                let mut i: khint_t;
                                let mut step: khint_t = 0;
                                k = $hash_func(key);
                                i = k & new_mask;
                                while *new_flags.offset((i >> 4) as isize)
                                    >> ((i & 0xf as libc::c_uint) << 1)
                                    & 2
                                    == 0
                                {
                                    step = step.wrapping_add(1);
                                    i = i.wrapping_add(step) & new_mask
                                }
                                let ref mut fresh91 = *new_flags.offset((i >> 4) as isize);
                                *fresh91 &= !(((2) << ((i & 0xf as libc::c_uint) << 1)) as khint_t);
                                if i < (*h).n_buckets
                                    && *(*h).flags.offset((i >> 4) as isize)
                                        >> ((i & 0xf as libc::c_uint) << 1)
                                        & 3
                                        == 0
                                {
                                    let tmp: $khkey_t = *(*h).keys.offset(i as isize);
                                    *(*h).keys.offset(i as isize) = key;
                                    key = tmp;
                                    let tmp_0: $khval_t = *(*h).vals.offset(i as isize);
                                    *(*h).vals.offset(i as isize) = val;
                                    val = tmp_0;
                                    let ref mut fresh92 = *(*h).flags.offset((i >> 4) as isize);
                                    *fresh92 |= (1) << ((i & 0xf as libc::c_uint) << 1)
                                } else {
                                    *(*h).keys.offset(i as isize) = key;
                                    *(*h).vals.offset(i as isize) = val;
                                    break;
                                }
                            }
                        }
                        j = j.wrapping_add(1)
                    }
                    if (*h).n_buckets > new_n_buckets {
                        (*h).keys = xrealloc(
                            (*h).keys,
                            (new_n_buckets as usize) * mem::size_of::<$khkey_t>(),
                        ) as *mut $khkey_t;
                        (*h).vals = xrealloc(
                            (*h).vals,
                            (new_n_buckets as usize) * mem::size_of::<$khval_t>(),
                        ) as *mut $khval_t;
                    }
                    let ptr_: *mut *mut libc::c_void =
                        &mut (*h).flags as *mut *mut khint32_t as *mut *mut libc::c_void;
                    xfree(*ptr_);
                    *ptr_ = ptr::null_mut();
                    (*h).flags = new_flags;
                    (*h).n_buckets = new_n_buckets;
                    (*h).n_occupied = (*h).size;
                    (*h).upper_bound =
                        ((*h).n_buckets as libc::c_double * 0.77f64 + 0.5f64) as khint_t;
                }
            }

            #[allow(dead_code)]
            pub unsafe extern "C" fn dealloc(h: *mut concat_idents!(kh_, $name, _t)) {
                XFREE_CLEAR(&mut (*h).keys);
                XFREE_CLEAR(&mut (*h).flags);
                XFREE_CLEAR(&mut (*h).vals);
            }
            #[allow(dead_code)]
            pub unsafe extern "C" fn destroy(h: *mut concat_idents!(kh_, $name, _t)) {
                if !h.is_null() {
                    dealloc(h);
                    xfree(h);
                }
            }
            #[allow(dead_code)]
            pub unsafe extern "C" fn get(
                h: *const concat_idents!(kh_, $name, _t),
                key: $khkey_t,
            ) -> khint_t {
                if (*h).n_buckets != 0 {
                    let mut k: khint_t = 0;
                    let mut i: khint_t = 0;
                    let mut last: khint_t = 0;
                    let mut mask: khint_t = 0;
                    let mut step: khint_t = 0;
                    mask = (*h).n_buckets - 1;
                    k = $hash_func(key);
                    i = k & mask;
                    last = i;
                    while *(*h).flags.offset((i >> 4) as isize) >> ((i & 0xf as libc::c_uint) << 1)
                        & 2
                        == 0
                        && (*(*h).flags.offset((i >> 4) as isize)
                            >> ((i & 0xf as libc::c_uint) << 1)
                            & 1
                            != 0
                            || !$hash_equal(*(*h).keys.offset(i as isize), key))
                    {
                        step = step.wrapping_add(1);
                        i = i.wrapping_add(step) & mask;
                        if i == last {
                            return (*h).n_buckets;
                        }
                    }
                    return if *(*h).flags.offset((i >> 4) as isize)
                        >> ((i & 0xf as libc::c_uint) << 1)
                        & 3
                        != 0
                    {
                        (*h).n_buckets
                    } else {
                        i
                    };
                } else {
                    return 0;
                };
            }
            #[allow(dead_code)]
            pub unsafe extern "C" fn put(
                mut h: *mut concat_idents!(kh_, $name, _t),
                key: $khkey_t,
                ret: *mut libc::c_int,
            ) -> khint_t {
                let mut x: khint_t = 0;
                if (*h).n_occupied >= (*h).upper_bound {
                    if (*h).n_buckets > (*h).size << 1 {
                        resize(h, (*h).n_buckets.wrapping_sub(1));
                    } else {
                        resize(h, (*h).n_buckets.wrapping_add(1));
                    }
                }
                let mut k: khint_t = 0;
                let mut i: khint_t = 0;
                let mut site: khint_t = 0;
                let mut last: khint_t = 0;
                let mask: khint_t = (*h).n_buckets.wrapping_sub(1);
                let mut step: khint_t = 0;
                site = (*h).n_buckets;
                x = site;
                k = $hash_func(key);
                i = k & mask;
                if *(*h).flags.offset((i >> 4) as isize) >> ((i & 0xf as libc::c_uint) << 1) & 2
                    != 0
                {
                    x = i
                } else {
                    last = i;
                    while *(*h).flags.offset((i >> 4) as isize) >> ((i & 0xf as libc::c_uint) << 1)
                        & 2
                        == 0
                        && (*(*h).flags.offset((i >> 4) as isize)
                            >> ((i & 0xf as libc::c_uint) << 1)
                            & 1
                            != 0
                            || !$hash_equal(*(*h).keys.offset(i as isize), key))
                    {
                        if *(*h).flags.offset((i >> 4) as isize) >> ((i & 0xf as libc::c_uint) << 1)
                            & 1
                            != 0
                        {
                            site = i
                        }
                        step = step.wrapping_add(1);
                        i = i.wrapping_add(step) & mask;
                        if !(i == last) {
                            continue;
                        }
                        x = site;
                        break;
                    }
                    if x == (*h).n_buckets {
                        if *(*h).flags.offset((i >> 4) as isize) >> ((i & 0xf as libc::c_uint) << 1)
                            & 2
                            != 0
                            && site != (*h).n_buckets
                        {
                            x = site
                        } else {
                            x = i
                        }
                    }
                }
                if *(*h).flags.offset((x >> 4) as isize) >> ((x & 0xf as libc::c_uint) << 1) & 2
                    != 0
                {
                    *(*h).keys.offset(x as isize) = key;
                    let ref mut fresh3 = *(*h).flags.offset((x >> 4) as isize);
                    *fresh3 &= !(((3) << ((x & 0xf as libc::c_uint) << 1)) as khint_t);
                    (*h).size = (*h).size.wrapping_add(1);
                    (*h).n_occupied = (*h).n_occupied.wrapping_add(1);
                    *ret = 1
                } else if *(*h).flags.offset((x >> 4) as isize) >> ((x & 0xf as libc::c_uint) << 1)
                    & 1
                    != 0
                {
                    *(*h).keys.offset(x as isize) = key;
                    let ref mut fresh4 = *(*h).flags.offset((x >> 4) as isize);
                    *fresh4 &= !(((3) << ((x & 0xf as libc::c_uint) << 1)) as khint_t);
                    (*h).size = (*h).size.wrapping_add(1);
                    *ret = 2
                } else {
                    *ret = 0
                }
                return x;
            }
            #[allow(dead_code)]
            pub unsafe extern "C" fn del(mut h: *mut concat_idents!(kh_, $name, _t), x: khint_t) {
                if x != (*h).n_buckets
                    && *(*h).flags.offset((x >> 4) as isize) >> ((x & 0xf as libc::c_uint) << 1) & 3
                        == 0
                {
                    let ref mut fresh5 = *(*h).flags.offset((x >> 4) as isize);
                    *fresh5 |= (1) << ((x & 0xf as libc::c_uint) << 1);
                    (*h).size = (*h).size.wrapping_sub(1)
                };
            }
        }
    };
}

/* ! @function
 @abstract     const char* hash function
 @param  s     Pointer to a null terminated string
 @return       The hash value
*/
#[inline]
pub unsafe extern "C" fn __ac_X31_hash_string(mut s: *const libc::c_char) -> khint_t {
    let mut h: khint_t = *s as khint_t;
    if h != 0 {
        s = s.offset(1);
        while *s != 0 {
            h = (h << 5 as libc::c_int)
                .wrapping_sub(h)
                .wrapping_add(*s as u8 as libc::c_uint);
            s = s.offset(1)
        }
    }
    return h;
}
pub const kh_str_hash_func: unsafe extern "C" fn(s: *const i8) -> khint_t = __ac_X31_hash_string;
pub fn kh_int64_hash_func(key: i64) -> khint_t {
    (key >> 33 ^ key ^ key << 11) as u32
}
pub unsafe fn kh_str_hash_equal(x: *const i8, y: *const i8) -> bool {
    libc::strcmp(x, y) == 0
}
