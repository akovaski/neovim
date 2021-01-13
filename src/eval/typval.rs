use crate::*;

extern "C" {
    pub type funccall_T;
    pub fn tv_dict_add_nr(
        d: *mut dict_T,
        key: *const libc::c_char,
        key_len: libc::size_t,
        nr: varnumber_T,
    ) -> libc::c_int;
    pub fn tv_dict_clear(d: *mut dict_T);
    pub fn tv_dict_set_keys_readonly(dict: *mut dict_T);
    pub fn callback_free(callback: *mut Callback);
    pub fn tv_dict_watcher_notify(
        dict: *mut dict_T,
        key: *const libc::c_char,
        newtv: *mut typval_T,
        oldtv: *mut typval_T,
    );
    pub fn tv_dict_item_copy(di: *mut dictitem_T) -> *mut dictitem_T;
    pub fn tv_dict_alloc() -> *mut dict_T;
    pub fn tv_dict_unref(d: *mut dict_T);
    pub fn tv_dict_find(
        d: *const dict_T,
        key: *const libc::c_char,
        len: ptrdiff_t,
    ) -> *mut dictitem_T;
    pub fn tv_dict_add(d: *mut dict_T, item: *mut dictitem_T) -> libc::c_int;
}

pub const DO_NOT_FREE_CNT: i32 = 1073741823;
pub const VARNUMBER_MAX: i64 = i64::max_value();
#[allow(dead_code)]
pub const UVARNUMBER_MAX: u64 = u64::max_value();

pub type float_T = libc::c_double;
pub type varnumber_T = i64;
#[allow(dead_code)]
pub type uvarnumber_T = u64;
pub type scid_T = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sctx_T {
    pub sc_sid: scid_T,
    pub sc_seq: libc::c_int,
    pub sc_lnum: linenr_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ufunc_T {
    pub uf_varargs: libc::c_int,
    pub uf_flags: libc::c_int,
    pub uf_calls: libc::c_int,
    pub uf_cleared: bool,
    pub uf_args: garray_T,
    pub uf_lines: garray_T,
    pub uf_profiling: libc::c_int,
    pub uf_prof_initialized: libc::c_int,
    pub uf_tm_count: libc::c_int,
    pub uf_tm_total: proftime_T,
    pub uf_tm_self: proftime_T,
    pub uf_tm_children: proftime_T,
    pub uf_tml_count: *mut libc::c_int,
    pub uf_tml_total: *mut proftime_T,
    pub uf_tml_self: *mut proftime_T,
    pub uf_tml_start: proftime_T,
    pub uf_tml_children: proftime_T,
    pub uf_tml_wait: proftime_T,
    pub uf_tml_idx: libc::c_int,
    pub uf_tml_execed: libc::c_int,
    pub uf_script_ctx: sctx_T,
    pub uf_refcount: libc::c_int,
    pub uf_scoped: *mut funccall_T,
    pub uf_name: [libc::c_uchar; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct listwatch_T {
    pub lw_item: *mut listitem_T,
    pub lw_next: *mut listwatch_T,
}
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum VarType {
    VAR_UNKNOWN, //< Unknown (unspecified) value.
    VAR_NUMBER,  //< Number, .v_number is used.
    VAR_STRING,  //< String, .v_string is used.
    VAR_FUNC,    //< Function reference, .v_string is used as function name.
    VAR_LIST,    //< List, .v_list is used.
    VAR_DICT,    //< Dictionary, .v_dict is used.
    VAR_FLOAT,   //< Floating-point value, .v_float is used.
    VAR_SPECIAL, //< Special value (true, false, null), .v_special
    //< is used.
    VAR_PARTIAL, //< Partial, .v_partial is used.
}
pub use VarType::*;
#[allow(dead_code)]
#[derive(Copy, Clone)]
#[repr(C)]
pub enum SpecialVarValue {
    kSpecialVarFalse, //< v:false
    kSpecialVarTrue,  //< v:true
    kSpecialVarNull,  //< v:null
}
pub use SpecialVarValue::*;
#[allow(dead_code)]
#[derive(Copy, Clone)]
#[repr(C)]
pub enum ScopeType {
    VAR_NO_SCOPE = 0, //< Not a scope dictionary.
    VAR_SCOPE = 1,    //< Scope dictionary which requires prefix (a:, v:, …).
    VAR_DEF_SCOPE = 2, //< Scope dictionary which may be accessed without prefix
                      //< (l:, g:).
}
pub use ScopeType::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScopeDictDictItem {
    pub di_tv: typval_T,
    pub di_flags: u8,
    pub di_key: [libc::c_uchar; 1],
}
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_T {
    pub lv_first: *mut listitem_T,
    pub lv_last: *mut listitem_T,
    pub lv_watch: *mut listwatch_T,
    pub lv_idx_item: *mut listitem_T,
    pub lv_copylist: *mut list_T,
    pub lv_used_next: *mut list_T,
    pub lv_used_prev: *mut list_T,
    pub lv_refcount: libc::c_int,
    pub lv_len: libc::c_int,
    pub lv_idx: libc::c_int,
    pub lv_copyID: libc::c_int,
    pub lv_lock: VarLockStatus,
}
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum VarLockStatus {
    VAR_UNLOCKED = 0, //< Not locked.
    VAR_LOCKED = 1,   //< User lock, can be unlocked.
    VAR_FIXED = 2,    //< Locked forever.
}
pub use VarLockStatus::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listitem_T {
    pub li_next: *mut listitem_T,
    pub li_prev: *mut listitem_T,
    pub li_tv: typval_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct typval_T {
    pub v_type: VarType,
    pub v_lock: VarLockStatus,
    pub vval: typval_vval_union,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union typval_vval_union {
    pub v_number: varnumber_T,
    pub v_special: SpecialVarValue,
    pub v_float: float_T,
    pub v_string: *mut libc::c_uchar,
    pub v_list: *mut list_T,
    pub v_dict: *mut dict_T,
    pub v_partial: *mut partial_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct partial_T {
    pub pt_refcount: libc::c_int,
    pub pt_name: *mut libc::c_uchar,
    pub pt_func: *mut ufunc_T,
    pub pt_auto: bool,
    pub pt_argc: libc::c_int,
    pub pt_argv: *mut typval_T,
    pub pt_dict: *mut dict_T,
}
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dict_T {
    pub dv_lock: VarLockStatus,
    pub dv_scope: ScopeType,
    pub dv_refcount: libc::c_int,
    pub dv_copyID: libc::c_int,
    pub dv_hashtab: hashtab_T,
    pub dv_copydict: *mut dict_T,
    pub dv_used_next: *mut dict_T,
    pub dv_used_prev: *mut dict_T,
    pub watchers: QUEUE,
}
pub type CallbackType = libc::c_uint;
pub const kCallbackPartial: CallbackType = 2;
pub const kCallbackFuncref: CallbackType = 1;
pub const kCallbackNone: CallbackType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Callback {
    pub data: Callback_data,
    pub type_0: CallbackType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Callback_data {
    pub funcref: *mut u8,
    pub partial: *mut partial_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictitem_T {
    pub di_tv: typval_T,
    pub di_flags: u8,
    pub di_key: [u8; 0],
}
pub type DIFlagsType = u8;
pub const DI_FLAGS_RO: DIFlagsType = 1;
pub const DI_FLAGS_RO_SBX: DIFlagsType = 2;
pub const DI_FLAGS_FIX: DIFlagsType = 4;
pub const DI_FLAGS_LOCK: DIFlagsType = 8;
pub const DI_FLAGS_ALLOC: DIFlagsType = 16;
#[inline]
pub unsafe extern "C" fn tv_dict_is_watched(d: *const dict_T) -> bool {
    return !d.is_null() && QUEUE_EMPTY(&(*d).watchers) == false;
}
