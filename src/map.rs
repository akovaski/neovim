use crate::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Map_uint64_t_ptr_t {
    pub table: *mut kh_uint64_t_ptr_t_map_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kh_uint64_t_ptr_t_map_t {
    pub n_buckets: khint_t,
    pub size: khint_t,
    pub n_occupied: khint_t,
    pub upper_bound: khint_t,
    pub flags: *mut khint32_t,
    pub keys: *mut u64,
    pub vals: *mut ptr_t,
}
