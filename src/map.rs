#![allow(unused_assignments)]
use crate::*;
use std::ptr;

pub unsafe fn cstr_t_hash(x: *const i8) -> u32 {
    kh_str_hash_func(x)
}
pub unsafe fn cstr_t_eq(x: *const i8, y: *const i8) -> bool {
    kh_str_hash_equal(x, y)
}
pub fn uint64_t_hash(x: u64) -> u32 {
    kh_int64_hash_func(x as i64)
}
pub fn uint64_t_eq(x: u64, y: u64) -> bool {
    x == y
}
pub fn uint32_t_hash(x: u32) -> u32 {
    x as u32
}
pub fn uint32_t_eq(x: u32, y: u32) -> bool {
    x == y
}
pub fn int_hash(x: i32) -> u32 {
    x as u32
}
pub fn int_eq(x: i32, y: i32) -> bool {
    x == y
}
pub fn linenr_T_hash(x: linenr_T) -> u32 {
    x as u32
}
pub fn linenr_T_eq(x: linenr_T, y: linenr_T) -> bool {
    x as i32 == y as i32
}
pub fn handle_T_hash(x: handle_T) -> u32 {
    x as u32
}
pub fn handle_T_eq(x: handle_T, y: handle_T) -> bool {
    x as i32 == y as i32
}
pub fn ptr_t_hash(x: ptr_t) -> u32 {
    uint64_t_hash(x as u64)
}
pub fn ptr_t_eq(x: ptr_t, y: ptr_t) -> bool {
    x as u64 == y as u64
}
const int_int_initializer: i32 = 0;
const uint64_t_ssize_t_initializer: isize = -1;
const uint64_t_uint64_t_initializer: u64 = 0;
const HlEntry_int_initializer: i32 = 0;
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct String_0 {
    data: *const i8,
    size: libc::size_t,
}
pub mod defs_h {
    // Basic types
    pub type ErrorType = libc::c_int;
    pub const kErrorTypeValidation: ErrorType = 1;
    pub const kErrorTypeException: ErrorType = 0;
    pub const kErrorTypeNone: ErrorType = -1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Error {
        pub type_0: ErrorType,
        pub msg: *mut libc::c_char,
    }
    pub type Boolean = bool;
    pub type Integer = i64;
    pub type Float = libc::c_double;
    // / Maximum value of an Integer
    // / Minimum value of an Integer
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct String_0 {
        pub data: *mut libc::c_char,
        pub size: libc::size_t,
    }
    // EXT types, cannot be split or reordered, see #EXT_OBJECT_TYPE_SHIFT
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct object {
        pub type_0: ObjectType,
        pub data: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed {
        pub boolean: Boolean,
        pub integer: Integer,
        pub floating: Float,
        pub string: String_0,
        pub array: Array,
        pub dictionary: Dictionary,
        pub luaref: LuaRef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Dictionary {
        pub items: *mut KeyValuePair,
        pub size: libc::size_t,
        pub capacity: libc::size_t,
    }
    pub type KeyValuePair = key_value_pair;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct key_value_pair {
        pub key: String_0,
        pub value: Object,
    }
    pub type Object = object;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Array {
        pub items: *mut Object,
        pub size: libc::size_t,
        pub capacity: libc::size_t,
    }
    pub type ObjectType = libc::c_uint;
    pub const kObjectTypeTabpage: ObjectType = 10;
    pub const kObjectTypeWindow: ObjectType = 9;
    pub const kObjectTypeBuffer: ObjectType = 8;
    pub const kObjectTypeLuaRef: ObjectType = 7;
    pub const kObjectTypeDictionary: ObjectType = 6;
    pub const kObjectTypeArray: ObjectType = 5;
    pub const kObjectTypeString: ObjectType = 4;
    pub const kObjectTypeFloat: ObjectType = 3;
    pub const kObjectTypeInteger: ObjectType = 2;
    pub const kObjectTypeBoolean: ObjectType = 1;
    pub const kObjectTypeNil: ObjectType = 0;
    use crate::*;
    // NVIM_API_PRIVATE_DEFS_H
}
pub use defs_h::*;
pub mod dispatch_h {
    use crate::*;
    pub type ApiDispatchWrapper =
        Option<unsafe extern "C" fn(_: u64, _: Array, _: *mut Error) -> Object>;
    // / The rpc_method_handlers table, used in msgpack_rpc_dispatch(), stores
    // / functions of this type.
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct MsgpackRpcRequestHandler {
        pub fn_0: ApiDispatchWrapper,
        pub fast: bool,
    }
    // NVIM_API_PRIVATE_DISPATCH_H
}
pub use dispatch_h::*;
pub mod highlight_defs_h {
    pub type RgbValue = i32;
    // / Stores a complete highlighting entry, including colors and attributes
    // / for both TUI and GUI.
    #[derive(Copy, Clone, PartialEq)]
    #[repr(C)]
    pub struct attr_entry {
        pub rgb_ae_attr: i16,
        pub cterm_ae_attr: i16,
        pub rgb_fg_color: RgbValue,
        pub rgb_bg_color: RgbValue,
        pub rgb_sp_color: RgbValue,
        pub cterm_fg_color: libc::c_int,
        pub cterm_bg_color: libc::c_int,
        pub hl_blend: libc::c_int,
    }
    pub type HlAttrs = attr_entry;
    pub type HlKind = libc::c_uint;
    pub const kHlBlendThrough: HlKind = 6;
    pub const kHlBlend: HlKind = 5;
    pub const kHlCombine: HlKind = 4;
    pub const kHlTerminal: HlKind = 3;
    pub const kHlSyntax: HlKind = 2;
    pub const kHlUI: HlKind = 1;
    pub const kHlUnknown: HlKind = 0;
    #[derive(Copy, Clone, PartialEq)]
    #[repr(C)]
    pub struct HlEntry {
        pub attr: HlAttrs,
        pub kind: HlKind,
        pub id1: libc::c_int,
        pub id2: libc::c_int,
    }
    // NVIM_HIGHLIGHT_DEFS_H
}
pub use highlight_defs_h::*;
pub mod map_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct kh_int_int_map_t {
        pub n_buckets: khint_t,
        pub size: khint_t,
        pub n_occupied: khint_t,
        pub upper_bound: khint_t,
        pub flags: *mut khint32_t,
        pub keys: *mut libc::c_int,
        pub vals: *mut libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExtmarkNs {
        pub map: *mut Map_uint64_t_uint64_t,
        pub free_id: u64,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Map_uint64_t_uint64_t {
        pub table: *mut kh_uint64_t_uint64_t_map_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct kh_uint64_t_uint64_t_map_t {
        pub n_buckets: khint_t,
        pub size: khint_t,
        pub n_occupied: khint_t,
        pub upper_bound: khint_t,
        pub flags: *mut khint32_t,
        pub keys: *mut u64,
        pub vals: *mut u64,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Map_int_int {
        pub table: *mut kh_int_int_map_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct kh_cstr_t_ptr_t_map_t {
        pub n_buckets: khint_t,
        pub size: khint_t,
        pub n_occupied: khint_t,
        pub upper_bound: khint_t,
        pub flags: *mut khint32_t,
        pub keys: *mut cstr_t,
        pub vals: *mut ptr_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Map_cstr_t_ptr_t {
        pub table: *mut kh_cstr_t_ptr_t_map_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct kh_ptr_t_ptr_t_map_t {
        pub n_buckets: khint_t,
        pub size: khint_t,
        pub n_occupied: khint_t,
        pub upper_bound: khint_t,
        pub flags: *mut khint32_t,
        pub keys: *mut ptr_t,
        pub vals: *mut ptr_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Map_ptr_t_ptr_t {
        pub table: *mut kh_ptr_t_ptr_t_map_t,
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Map_uint64_t_ptr_t {
        pub table: *mut kh_uint64_t_ptr_t_map_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct kh_uint64_t_ssize_t_map_t {
        pub n_buckets: khint_t,
        pub size: khint_t,
        pub n_occupied: khint_t,
        pub upper_bound: khint_t,
        pub flags: *mut khint32_t,
        pub keys: *mut u64,
        pub vals: *mut libc::ssize_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Map_uint64_t_ssize_t {
        pub table: *mut kh_uint64_t_ssize_t_map_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct kh_uint64_t_ExtmarkNs_map_t {
        pub n_buckets: khint_t,
        pub size: khint_t,
        pub n_occupied: khint_t,
        pub upper_bound: khint_t,
        pub flags: *mut khint32_t,
        pub keys: *mut u64,
        pub vals: *mut ExtmarkNs,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Map_uint64_t_ExtmarkNs {
        pub table: *mut kh_uint64_t_ExtmarkNs_map_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct kh_uint64_t_ExtmarkItem_map_t {
        pub n_buckets: khint_t,
        pub size: khint_t,
        pub n_occupied: khint_t,
        pub upper_bound: khint_t,
        pub flags: *mut khint32_t,
        pub keys: *mut u64,
        pub vals: *mut ExtmarkItem,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Map_uint64_t_ExtmarkItem {
        pub table: *mut kh_uint64_t_ExtmarkItem_map_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct kh_handle_T_ptr_t_map_t {
        pub n_buckets: khint_t,
        pub size: khint_t,
        pub n_occupied: khint_t,
        pub upper_bound: khint_t,
        pub flags: *mut khint32_t,
        pub keys: *mut handle_T,
        pub vals: *mut ptr_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Map_handle_T_ptr_t {
        pub table: *mut kh_handle_T_ptr_t_map_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct kh_String_MsgpackRpcRequestHandler_map_t {
        pub n_buckets: khint_t,
        pub size: khint_t,
        pub n_occupied: khint_t,
        pub upper_bound: khint_t,
        pub flags: *mut khint32_t,
        pub keys: *mut String_0,
        pub vals: *mut MsgpackRpcRequestHandler,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Map_String_MsgpackRpcRequestHandler {
        pub table: *mut kh_String_MsgpackRpcRequestHandler_map_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct kh_HlEntry_int_map_t {
        pub n_buckets: khint_t,
        pub size: khint_t,
        pub n_occupied: khint_t,
        pub upper_bound: khint_t,
        pub flags: *mut khint32_t,
        pub keys: *mut HlEntry,
        pub vals: *mut libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Map_HlEntry_int {
        pub table: *mut kh_HlEntry_int_map_t,
    }
    //
    // NOTE: Keys AND values must be allocated! khash.h does not make a copy.
    //
    // NB: this is the only way to define a struct both containing and contained
    // in a map...
    // For namespacing extmarks
    // For fast lookup
    // For automatically assigning id's
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct kh_String_handle_T_map_t {
        pub n_buckets: khint_t,
        pub size: khint_t,
        pub n_occupied: khint_t,
        pub upper_bound: khint_t,
        pub flags: *mut khint32_t,
        pub keys: *mut String_0,
        pub vals: *mut handle_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Map_String_handle_T {
        pub table: *mut kh_String_handle_T_map_t,
    }
    use crate::*;
    // NVIM_MAP_H
}
pub use map_h::*;
// This is an open source non-commercial project. Dear PVS-Studio, please check
// it. PVS-Studio Static Code Analyzer for C, C++ and C#: http://www.viva64.com
//
// map.c: khash.h wrapper
//
// NOTE: Callers must manage memory (allocate) for keys and values.
//       khash.h does not make its own copy of the key or value.
//
/* Caller must check map_has(). */
#[inline]
pub unsafe extern "C" fn String_hash(s: String_0) -> khint_t {
    let mut h: khint_t = 0;
    let mut i: libc::size_t = 0;
    while i < s.size && *s.data.offset(i as isize) as libc::c_int != 0 {
        h = (h << 5)
            .wrapping_sub(h)
            .wrapping_add(*s.data.offset(i as isize) as u8 as libc::c_uint);
        i = i.wrapping_add(1)
    }
    return h;
}
#[inline]
pub unsafe extern "C" fn String_eq(a: String_0, b: String_0) -> bool {
    if a.size != b.size {
        return false;
    }
    return memcmp(
        a.data as *const libc::c_void,
        b.data as *const libc::c_void,
        a.size,
    ) == 0;
}
#[inline]
pub unsafe extern "C" fn HlEntry_hash(mut ae: HlEntry) -> khint_t {
    let data: *const u8 = &mut ae as *mut HlEntry as *const u8;
    let mut h: khint_t = 0;
    let mut i: libc::size_t = 0;
    while i < ::std::mem::size_of::<HlEntry>() {
        h = (h << 5)
            .wrapping_sub(h)
            .wrapping_add(*data.offset(i as isize) as libc::c_uint);
        i = i.wrapping_add(1)
    }
    return h;
}
#[inline]
pub unsafe extern "C" fn HlEntry_eq(mut ae1: HlEntry, mut ae2: HlEntry) -> bool {
    return memcmp(
        &mut ae1 as *mut HlEntry as *const libc::c_void,
        &mut ae2 as *mut HlEntry as *const libc::c_void,
        ::std::mem::size_of::<HlEntry>(),
    ) == 0;
}
macro_rules! Map {
    ($T: ident, $U: ident) => {
        concat_idents!(Map_, $T, _, $U)
    };
}
macro_rules! INITIALIZER {
    ($T: ident, $U: ident) => {
        concat_idents!($T, _, $U, _initializer)
    };
}
macro_rules! MAP_new {
    ($T: ident, $U: ident, $map_kh_t: ident, $map_kh_mod: ident, $map_new_name: ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $map_new_name() -> *mut Map!($T, $U) {
            let mut rv: *mut Map!($T, $U) = xmalloc(::std::mem::size_of::<Map!($T, $U)>());
            (*rv).table = $map_kh_mod::init();
            return rv;
        }
    };
    ($T: ident, $U: ident, $khkey_t: ty, $khvalue_t: ty, $map_kh_t: ident, $map_kh_mod: ident, $map_new_name: ident, $map_free_name: ident) => {
        MAP_new!($T, $U, $map_kh_t, $map_kh_mod, $map_new_name);
        #[no_mangle]
        pub unsafe extern "C" fn $map_free_name(map: *mut Map!($T, $U)) {
            $map_kh_mod::destroy((*map).table);
            xfree(map as *mut libc::c_void);
        }
    };
    ($T: ident, $U: ident, $khkey_t: ty, $khvalue_t: ty, $map_kh_t: ident, $map_kh_mod: ident, $map_new_name: ident, $map_free_name: ident, $map_has_name: ident) => {
        MAP_new!(
            $T,
            $U,
            $khkey_t,
            $khvalue_t,
            $map_kh_t,
            $map_kh_mod,
            $map_new_name,
            $map_free_name
        );
        #[no_mangle]
        pub unsafe extern "C" fn $map_has_name(map: *mut Map!($T, $U), key: $khkey_t) -> bool {
            return $map_kh_mod::get((*map).table, key) != (*(*map).table).n_buckets;
        }
    };
    ($T: ident, $U: ident, $khkey_t: ty, $khvalue_t: ty, $map_kh_t: ident, $map_kh_mod: ident, $map_new_name: ident, $map_free_name: ident, $map_has_name: ident, $map_clear_name: ident, $map_key_name: ident, $map_ref_name: ident, $map_get_name: ident, $map_put_name: ident, $map_del_name: ident) => {
        MAP_new!(
            $T,
            $U,
            $khkey_t,
            $khvalue_t,
            $map_kh_t,
            $map_kh_mod,
            $map_new_name,
            $map_free_name,
            $map_has_name
        );
        #[no_mangle]
        pub unsafe extern "C" fn $map_clear_name(map: *mut Map!($T, $U)) {
            $map_kh_mod::clear((*map).table);
        }
        #[no_mangle]
        pub unsafe extern "C" fn $map_key_name(map: *mut Map!($T, $U), key: $khkey_t) -> $khkey_t {
            let mut k: khiter_t = 0;
            k = $map_kh_mod::get((*map).table, key);
            if k == (*(*map).table).n_buckets {
                abort();
            }
            return *(*(*map).table).keys.offset(k as isize);
        }
        #[no_mangle]
        pub unsafe extern "C" fn $map_ref_name(
            map: *mut Map!($T, $U),
            key: $khkey_t,
            put: bool,
        ) -> *mut $khvalue_t {
            let mut ret: libc::c_int = 0;
            let mut k: khiter_t = 0;
            if put {
                k = $map_kh_mod::put((*map).table, key, &mut ret);
                if ret != 0 {
                    *(*(*map).table).vals.offset(k as isize) = INITIALIZER!($T, $U);
                }
            } else {
                k = $map_kh_mod::get((*map).table, key);
                if k == (*(*map).table).n_buckets {
                    return ptr::null_mut();
                }
            }
            return &mut *(*(*map).table).vals.offset(k as isize) as *mut $khvalue_t;
        }
        #[no_mangle]
        pub unsafe extern "C" fn $map_get_name(
            map: *mut Map!($T, $U),
            key: $khkey_t,
        ) -> $khvalue_t {
            let mut k: khiter_t = 0;
            k = $map_kh_mod::get((*map).table, key);
            if k == (*(*map).table).n_buckets {
                return INITIALIZER!($T, $U);
            }
            return *(*(*map).table).vals.offset(k as isize);
        }
        #[no_mangle]
        pub unsafe extern "C" fn $map_put_name(
            map: *mut Map!($T, $U),
            key: $khkey_t,
            value: $khvalue_t,
        ) -> $khvalue_t {
            let mut ret: libc::c_int = 0;
            let mut rv: $khvalue_t = INITIALIZER!($T, $U);
            let k: khiter_t = $map_kh_mod::put((*map).table, key, &mut ret);
            if ret == 0 {
                rv = *(*(*map).table).vals.offset(k as isize)
            }
            *(*(*map).table).vals.offset(k as isize) = value;
            return rv;
        }
        #[no_mangle]
        pub unsafe extern "C" fn $map_del_name(
            map: *mut Map!($T, $U),
            key: $khkey_t,
        ) -> $khvalue_t {
            let mut rv: $khvalue_t = INITIALIZER!($T, $U);
            let mut k: khiter_t = 0;
            k = $map_kh_mod::get((*map).table, key);
            if k != (*(*map).table).n_buckets {
                rv = *(*(*map).table).vals.offset(k as isize);
                $map_kh_mod::del((*map).table, k);
            }
            return rv;
        }
    };
}
KHASH_IMPL!(int_int_map, kh_int_int_map, i32, i32, int_hash, int_eq);
MAP_new!(
    int,
    int,
    libc::c_int,
    libc::c_int,
    int_int_map,
    kh_int_int_map,
    map_int_int_new,
    map_int_int_free,
    map_int_int_has,
    map_int_int_clear,
    map_int_int_key,
    map_int_int_ref,
    map_int_int_get,
    map_int_int_put,
    map_int_int_del
);
MAP_new!(
    cstr_t,
    ptr_t,
    cstr_t,
    ptr_t,
    cstr_t_ptr_t_map,
    kh_cstr_t_ptr_t_map,
    map_cstr_t_ptr_t_new,
    map_cstr_t_ptr_t_free,
    map_cstr_t_ptr_t_has,
    map_cstr_t_ptr_t_clear,
    map_cstr_t_ptr_t_key,
    map_cstr_t_ptr_t_ref,
    map_cstr_t_ptr_t_get,
    map_cstr_t_ptr_t_put,
    map_cstr_t_ptr_t_del
);
KHASH_IMPL!(
    cstr_t_ptr_t_map,
    kh_cstr_t_ptr_t_map,
    cstr_t,
    ptr_t,
    cstr_t_hash,
    cstr_t_eq
);
#[no_mangle]
pub static mut cstr_t_ptr_t_initializer: ptr_t = ptr::null_mut();
#[no_mangle]
pub static mut ptr_t_ptr_t_initializer: ptr_t = ptr::null_mut();
MAP_new!(
    ptr_t,
    ptr_t,
    ptr_t,
    ptr_t,
    ptr_t_ptr_t_map,
    kh_ptr_t_ptr_t_map,
    map_ptr_t_ptr_t_new,
    map_ptr_t_ptr_t_free,
    map_ptr_t_ptr_t_has,
    map_ptr_t_ptr_t_clear,
    map_ptr_t_ptr_t_key,
    map_ptr_t_ptr_t_ref,
    map_ptr_t_ptr_t_get,
    map_ptr_t_ptr_t_put,
    map_ptr_t_ptr_t_del
);
KHASH_IMPL!(
    ptr_t_ptr_t_map,
    kh_ptr_t_ptr_t_map,
    ptr_t,
    ptr_t,
    ptr_t_hash,
    ptr_t_eq
);
MAP_new!(
    uint64_t,
    ptr_t,
    u64,
    ptr_t,
    uint64_t_ptr_t_map,
    kh_uint64_t_ptr_t_map,
    map_uint64_t_ptr_t_new,
    map_uint64_t_ptr_t_free,
    map_uint64_t_ptr_t_has,
    map_uint64_t_ptr_t_clear,
    map_uint64_t_ptr_t_key,
    map_uint64_t_ptr_t_ref,
    map_uint64_t_ptr_t_get,
    map_uint64_t_ptr_t_put,
    map_uint64_t_ptr_t_del
);
const uint64_t_ptr_t_initializer: ptr_t = ptr::null_mut();
KHASH_IMPL!(
    uint64_t_ptr_t_map,
    kh_uint64_t_ptr_t_map,
    u64,
    ptr_t,
    uint64_t_hash,
    uint64_t_eq
);
MAP_new!(
    uint64_t,
    ssize_t,
    u64,
    isize,
    uint64_t_ssize_t_map,
    kh_uint64_t_ssize_t_map,
    map_uint64_t_ssize_t_new,
    map_uint64_t_ssize_t_free,
    map_uint64_t_ssize_t_has,
    map_uint64_t_ssize_t_clear,
    map_uint64_t_ssize_t_key,
    map_uint64_t_ssize_t_ref,
    map_uint64_t_ssize_t_get,
    map_uint64_t_ssize_t_put,
    map_uint64_t_ssize_t_del
);
KHASH_IMPL!(
    uint64_t_ssize_t_map,
    kh_uint64_t_ssize_t_map,
    u64,
    isize,
    uint64_t_hash,
    uint64_t_eq
);
KHASH_IMPL!(
    uint64_t_uint64_t_map,
    kh_uint64_t_uint64_t_map,
    u64,
    u64,
    uint64_t_hash,
    uint64_t_eq
);
MAP_new!(
    uint64_t,
    uint64_t,
    u64,
    u64,
    uint64_t_uint64_t_map,
    kh_uint64_t_uint64_t_map,
    map_uint64_t_uint64_t_new,
    map_uint64_t_uint64_t_free,
    map_uint64_t_uint64_t_has,
    map_uint64_t_uint64_t_clear,
    map_uint64_t_uint64_t_key,
    map_uint64_t_uint64_t_ref,
    map_uint64_t_uint64_t_get,
    map_uint64_t_uint64_t_put,
    map_uint64_t_uint64_t_del
);
MAP_new!(
    uint64_t,
    ExtmarkNs,
    u64,
    ExtmarkNs,
    uint64_t_ExtmarkNs_map,
    kh_uint64_t_ExtmarkNs_map,
    map_uint64_t_ExtmarkNs_new,
    map_uint64_t_ExtmarkNs_free,
    map_uint64_t_ExtmarkNs_has,
    map_uint64_t_ExtmarkNs_clear,
    map_uint64_t_ExtmarkNs_key,
    map_uint64_t_ExtmarkNs_ref,
    map_uint64_t_ExtmarkNs_get,
    map_uint64_t_ExtmarkNs_put,
    map_uint64_t_ExtmarkNs_del
);
KHASH_IMPL!(
    uint64_t_ExtmarkNs_map,
    kh_uint64_t_ExtmarkNs_map,
    u64,
    ExtmarkNs,
    uint64_t_hash,
    uint64_t_eq
);
#[no_mangle]
pub static mut uint64_t_ExtmarkNs_initializer: ExtmarkNs = {
    let init = ExtmarkNs {
        map: ptr::null_mut(),
        free_id: 0,
    };
    init
};
KHASH_IMPL!(
    uint64_t_ExtmarkItem_map,
    kh_uint64_t_ExtmarkItem_map,
    u64,
    ExtmarkItem,
    uint64_t_hash,
    uint64_t_eq
);
const uint64_t_ExtmarkItem_initializer: ExtmarkItem = {
    let init = ExtmarkItem {
        ns_id: 0,
        mark_id: 0,
        hl_id: 0,
        virt_text: {
            let init = VirtText {
                size: 0,
                capacity: 0,
                items: ptr::null_mut(),
            };
            init
        },
    };
    init
};
MAP_new!(
    uint64_t,
    ExtmarkItem,
    u64,
    ExtmarkItem,
    uint64_t_ExtmarkItem_map,
    kh_uint64_t_ExtmarkItem_map,
    map_uint64_t_ExtmarkItem_new,
    map_uint64_t_ExtmarkItem_free,
    map_uint64_t_ExtmarkItem_has,
    map_uint64_t_ExtmarkItem_clear,
    map_uint64_t_ExtmarkItem_key,
    map_uint64_t_ExtmarkItem_ref,
    map_uint64_t_ExtmarkItem_get,
    map_uint64_t_ExtmarkItem_put,
    map_uint64_t_ExtmarkItem_del
);
MAP_new!(
    handle_T,
    ptr_t,
    handle_T,
    ptr_t,
    handle_T_ptr_t_map,
    kh_handle_T_ptr_t_map,
    map_handle_T_ptr_t_new,
    map_handle_T_ptr_t_free,
    map_handle_T_ptr_t_has,
    map_handle_T_ptr_t_clear,
    map_handle_T_ptr_t_key,
    map_handle_T_ptr_t_ref,
    map_handle_T_ptr_t_get,
    map_handle_T_ptr_t_put,
    map_handle_T_ptr_t_del
);
KHASH_IMPL!(
    handle_T_ptr_t_map,
    kh_handle_T_ptr_t_map,
    handle_T,
    ptr_t,
    handle_T_hash,
    handle_T_eq
);
const handle_T_ptr_t_initializer: ptr_t = ptr::null_mut();
KHASH_IMPL!(
    String_MsgpackRpcRequestHandler_map,
    kh_String_MsgpackRpcRequestHandler_map,
    String_0,
    MsgpackRpcRequestHandler,
    String_hash,
    String_eq
);
const String_MsgpackRpcRequestHandler_initializer: MsgpackRpcRequestHandler = {
    let init = MsgpackRpcRequestHandler {
        fn_0: None,
        fast: false,
    };
    init
};
MAP_new!(
    String,
    MsgpackRpcRequestHandler,
    String_0,
    MsgpackRpcRequestHandler,
    String_MsgpackRpcRequestHandler_map,
    kh_String_MsgpackRpcRequestHandler_map,
    map_String_MsgpackRpcRequestHandler_new,
    map_String_MsgpackRpcRequestHandler_free,
    map_String_MsgpackRpcRequestHandler_has,
    map_String_MsgpackRpcRequestHandler_clear,
    map_String_MsgpackRpcRequestHandler_key,
    map_String_MsgpackRpcRequestHandler_ref,
    map_String_MsgpackRpcRequestHandler_get,
    map_String_MsgpackRpcRequestHandler_put,
    map_String_MsgpackRpcRequestHandler_del
);
KHASH_IMPL!(
    HlEntry_int_map,
    kh_HlEntry_int_map,
    HlEntry,
    i32,
    HlEntry_hash,
    HlEntry_eq
);
MAP_new!(
    HlEntry,
    int,
    HlEntry,
    i32,
    HlEntry_int_map,
    kh_HlEntry_int_map,
    map_HlEntry_int_new,
    map_HlEntry_int_free,
    map_HlEntry_int_has,
    map_HlEntry_int_clear,
    map_HlEntry_int_key,
    map_HlEntry_int_ref,
    map_HlEntry_int_get,
    map_HlEntry_int_put,
    map_HlEntry_int_del
);
KHASH_IMPL!(
    String_handle_T_map,
    kh_String_handle_T_map,
    String_0,
    handle_T,
    String_hash,
    String_eq
);
MAP_new!(
    String,
    handle_T,
    String_0,
    handle_T,
    String_handle_T_map,
    kh_String_handle_T_map,
    map_String_handle_T_new,
    map_String_handle_T_free,
    map_String_handle_T_has,
    map_String_handle_T_clear,
    map_String_handle_T_key,
    map_String_handle_T_ref,
    map_String_handle_T_get,
    map_String_handle_T_put,
    map_String_handle_T_del
);
const String_handle_T_initializer: handle_T = 0;

/// @see pmap_del2
/// Deletes a key:value pair from a string:pointer map, and frees the
/// storage of both key and value.
///
#[no_mangle]
pub unsafe extern "C" fn pmap_del2(map: *mut Map_cstr_t_ptr_t, key: *const libc::c_char) {
    if map_cstr_t_ptr_t_has(map, key) {
        let k: *mut libc::c_void = map_cstr_t_ptr_t_key(map, key) as *mut libc::c_void;
        let v: *mut libc::c_void = map_cstr_t_ptr_t_get(map, key);
        map_cstr_t_ptr_t_del(map, key);
        xfree(k);
        xfree(v);
    };
}
