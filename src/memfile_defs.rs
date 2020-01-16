use crate::pos::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct memfile_T {
    pub mf_fname: *mut libc::c_uchar,
    pub mf_ffname: *mut libc::c_uchar,
    pub mf_fd: libc::c_int,
    pub mf_free_first: *mut bhdr_T,
    pub mf_used_first: *mut bhdr_T,
    pub mf_used_last: *mut bhdr_T,
    pub mf_hash: mf_hashtab_T,
    pub mf_trans: mf_hashtab_T,
    pub mf_blocknr_max: blocknr_T,
    pub mf_blocknr_min: blocknr_T,
    pub mf_neg_count: blocknr_T,
    pub mf_infile_count: blocknr_T,
    pub mf_page_size: libc::c_uint,
    pub mf_dirty: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bhdr_T {
    pub bh_hashitem: mf_hashitem_T,
    pub bh_next: *mut bhdr_T,
    pub bh_prev: *mut bhdr_T,
    pub bh_data: *mut libc::c_void,
    pub bh_page_count: libc::c_uint,
    pub bh_flags: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mf_hashtab_T {
    pub mht_mask: libc::size_t,
    pub mht_count: libc::size_t,
    pub mht_buckets: *mut *mut mf_hashitem_T,
    pub mht_small_buckets: [*mut mf_hashitem_T; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mf_hashitem_T {
    pub mhi_next: *mut mf_hashitem_T,
    pub mhi_prev: *mut mf_hashitem_T,
    pub mhi_key: blocknr_T,
}
pub type blocknr_T = i64;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct chunksize_T {
    pub mlcs_numlines: libc::c_int,
    pub mlcs_totalsize: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infoptr_T {
    pub ip_bnum: blocknr_T,
    pub ip_low: linenr_T,
    pub ip_high: linenr_T,
    pub ip_index: libc::c_int,
}
