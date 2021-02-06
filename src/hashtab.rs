extern "C" {
    pub fn hash_init(ht: *mut hashtab_T);
    pub fn hash_find(ht: *const hashtab_T, key: *const u8) -> *mut hashitem_T;
    pub fn hash_remove(ht: *mut hashtab_T, hi: *mut hashitem_T);
}
pub type hash_T = libc::size_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashitem_T {
    pub hi_hash: hash_T,
    pub hi_key: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashtab_T {
    pub ht_mask: hash_T,
    pub ht_used: libc::size_t,
    pub ht_filled: libc::size_t,
    pub ht_locked: libc::c_int,
    pub ht_array: *mut hashitem_T,
    pub ht_smallarray: [hashitem_T; 16],
}
