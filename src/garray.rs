// Functions for handling growwing arrays.
use crate::log::*;
use crate::memory::*;
use crate::path::*;
use crate::strings::*;
use std::ptr;

/// Structure used for growing arrays.
/// This is used to store information that only grows, is deleted all at
/// once, and needs to be accessed by index.  See ga_clear() and ga_grow().
#[derive(Copy, Clone)]
#[repr(C)]
pub struct growarray {
    pub ga_len: libc::c_int,        // current number of items used
    pub ga_maxlen: libc::c_int,     // maximum number of items possible
    pub ga_itemsize: libc::c_int,   // sizeof(item)
    pub ga_growsize: libc::c_int,   // number of items to grow each time
    pub ga_data: *mut libc::c_void, // pointer to the first item
}
pub type garray_T = growarray;

#[allow(non_snake_case)]
pub unsafe fn GA_APPEND<T>(gap: &mut garray_T, item: T) {
    ga_grow(gap, 1);
    let fresh = gap.ga_len;
    gap.ga_len = gap.ga_len + 1;
    *(gap.ga_data as *mut T).offset(fresh as isize) = item;
}

/// Deep free a garray of specific type using a custom free function.
/// Items in the array as well as the array itself are freed.
///
/// @param gap the garray to be freed
/// @param T type of the item in the garray
/// @param free_item_fn free function that takes (*item_type) as parameter
#[allow(non_snake_case)]
pub unsafe fn GA_DEEP_CLEAR<T>(gap: *mut garray_T, free_item_fn: fn(*mut T)) {
    if !(*gap).ga_data.is_null() {
        for i in 0..(*gap).ga_len {
            let item: *mut T = &mut *((*gap).ga_data as *mut T).offset(i as isize) as *mut T;
            free_item_fn(item);
        }
    }
    ga_clear(gap);
}

/// Call `free` for every pointer stored in the garray and then frees the
/// garray.
///
/// @param gap the garray to be freed
#[allow(non_snake_case)]
pub unsafe fn GA_DEEP_CLEAR_PTR(gap: *mut garray_T) {
    GA_DEEP_CLEAR(gap, |ptr: *mut *mut garray_T| xfree(*ptr));
}

/// Clear an allocated growing array.
#[no_mangle]
pub unsafe extern "C" fn ga_clear(gap: *mut garray_T) {
    xfree((*gap).ga_data);

    // Initialize growing array without resetting itemsize or growsize
    (*gap).ga_data = ptr::null_mut();
    (*gap).ga_maxlen = 0;
    (*gap).ga_len = 0;
}

/// Clear a growing array that contains a list of strings.
#[no_mangle]
pub unsafe extern "C" fn ga_clear_strings(gap: *mut garray_T) {
    GA_DEEP_CLEAR_PTR(gap);
}

/// Initialize a growing array.
#[no_mangle]
pub unsafe extern "C" fn ga_init(gap: &mut garray_T, itemsize: libc::c_int, growsize: libc::c_int) {
    gap.ga_data = ptr::null_mut();
    gap.ga_maxlen = 0;
    gap.ga_len = 0;
    gap.ga_itemsize = itemsize;
    ga_set_growsize(gap, growsize);
}

/// A setter for the growsize that guarantees it will be at least 1.
#[no_mangle]
pub unsafe extern "C" fn ga_set_growsize(gap: &mut garray_T, growsize: libc::c_int) {
    if growsize < 1 {
        WLOG!("trying to set an invalid ga_growsize: %d", growsize);
        gap.ga_growsize = 1;
    } else {
        gap.ga_growsize = growsize;
    };
}

/// Make room in growing array "gap" for at least "n" items.
#[no_mangle]
pub unsafe extern "C" fn ga_grow(gap: &mut garray_T, mut n: libc::c_int) {
    if gap.ga_maxlen - gap.ga_len >= n {
        // the garray still has enough space, do nothing
        return;
    }

    if gap.ga_growsize < 1 {
        WLOG!("ga_growsize(%d) is less than 1", gap.ga_growsize);
    }

    // the garray grows by at least growsize
    if n < gap.ga_growsize {
        n = gap.ga_growsize
    }

    // A linear growth is very inefficient when the array grows big.  This
    // is a compromise between allocating memory that won't be used and too
    // many copy operations. A factor of 1.5 seems reasonable.
    if n < gap.ga_len / 2 {
        n = gap.ga_len / 2
    }

    let new_maxlen: libc::c_int = gap.ga_len + n;

    let new_size: libc::size_t = (gap.ga_itemsize * new_maxlen) as libc::size_t;
    let old_size: libc::size_t = (gap.ga_itemsize * gap.ga_maxlen) as libc::size_t;

    // reallocate and clear the new memory
    let pp: *mut libc::c_char = xrealloc(gap.ga_data, new_size);
    memset(pp.offset(old_size as isize), 0, new_size - old_size);

    gap.ga_maxlen = new_maxlen;
    gap.ga_data = pp as *mut libc::c_void;
}

/// Sort "gap" and remove duplicate entries. "gap" is expected to contain a
/// list of file names in allocated memory.
#[no_mangle]
pub unsafe extern "C" fn ga_remove_duplicate_strings(mut gap: *mut garray_T) {
    let fnames: *mut *mut libc::c_uchar = (*gap).ga_data as *mut *mut libc::c_uchar;

    // sort the growing array, which puts duplicates next to each other
    sort_strings(fnames, (*gap).ga_len);

    // loop over the growing array in reverse
    for i in (1..(*gap).ga_len).rev() {
        if fnamecmp(*fnames.offset((i - 1) as isize), *fnames.offset(i as isize)) == 0 {
            xfree(*fnames.offset(i as isize));

            // close the gap (move all strings one slot lower)
            for j in (i + 1)..(*gap).ga_len {
                let ref mut fresh = *fnames.offset((j - 1) as isize);
                *fresh = *fnames.offset(j as isize);
            }
            (*gap).ga_len -= 1
        }
    }
}

/// For a growing array that contains a list of strings: concatenate all the
/// strings with sep as separator.
///
/// @returns the concatenated strings
#[no_mangle]
pub unsafe extern "C" fn ga_concat_strings_sep(
    gap: *const garray_T,
    sep: *const libc::c_char,
) -> *mut libc::c_uchar {
    let nelem: libc::size_t = (*gap).ga_len as libc::size_t;
    let strings: *mut *const libc::c_char = (*gap).ga_data as *mut *const libc::c_char;

    if nelem == 0 {
        return xstrdup("") as *mut libc::c_uchar;
    }

    let mut len: libc::size_t = 0;
    for i in 0..nelem {
        len += strlen(*(strings.offset(i as isize))) as usize;
    }

    // add some space for the (num - 1) separators
    len += (nelem - 1) * strlen(sep) as libc::size_t;
    let ret: *mut libc::c_char = xmallocz(len);

    let mut s: *mut libc::c_char = ret;
    for i in 0..(nelem - 1) {
        s = xstpcpy(s, *strings.offset(i as isize));
        s = xstpcpy(s, sep);
    }
    strcpy(s, *strings.offset((nelem - 1) as isize));

    return ret as *mut libc::c_uchar;
}

/// For a growing array that contains a list of strings: concatenate all the
/// strings with a separating comma.
///
/// @returns the concatenated strings
#[no_mangle]
pub unsafe extern "C" fn ga_concat_strings(gap: *const garray_T) -> *mut libc::c_uchar {
    return ga_concat_strings_sep(gap, b",\x00" as *const u8 as *const libc::c_char);
}

/// Concatenate a string to a growarray which contains characters.
/// When "s" is NULL does not do anything.
///
/// WARNING:
/// - Does NOT copy the NUL at the end!
/// - The parameter may not overlap with the growing array
#[no_mangle]
pub unsafe extern "C" fn ga_concat(gap: &mut garray_T, s: *const libc::c_uchar) {
    if s.is_null() {
        return;
    }
    ga_concat_len(
        gap,
        s as *const libc::c_char,
        strlen(s as *mut libc::c_char) as usize,
    );
}

/// Concatenate a string to a growarray which contains characters
///
/// @param[out]  gap  Growarray to modify.
/// @param[in]  s  String to concatenate.
/// @param[in]  len  String length.
#[no_mangle]
pub unsafe extern "C" fn ga_concat_len(
    gap: &mut garray_T,
    s: *const libc::c_char,
    len: libc::size_t,
) {
    if len != 0 {
        ga_grow(gap, len as libc::c_int);
        let data: *mut libc::c_char = gap.ga_data as *mut libc::c_char;
        memcpy(data.offset(gap.ga_len as isize), s, len);
        gap.ga_len += len as libc::c_int
    };
}

/// Append one byte to a growarray which contains bytes.
#[no_mangle]
pub unsafe extern "C" fn ga_append(gap: &mut garray_T, c: libc::c_char) {
    GA_APPEND(gap, c);
}
