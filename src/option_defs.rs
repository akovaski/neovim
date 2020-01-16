use crate::eval::typval::sctx_T;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LastSet {
    pub script_ctx: sctx_T,
    pub channel_id: u64,
}
