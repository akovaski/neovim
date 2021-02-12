use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:4"]
pub mod types_h {
    #[c2rust::src_loc = "36:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "37:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "38:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "39:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "40:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "41:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "43:1"]
    pub type __int64_t = libc::c_long;
    #[c2rust::src_loc = "44:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "71:1"]
    pub type __intmax_t = libc::c_long;
    #[c2rust::src_loc = "72:1"]
    pub type __uintmax_t = libc::c_ulong;
    #[c2rust::src_loc = "144:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "145:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "158:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "191:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-intn.h:4"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int8_t, __int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h:4"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/usr/include/stdint.h:4"]
pub mod stdint_h {
    #[c2rust::src_loc = "101:1"]
    pub type intmax_t = __intmax_t;
    #[c2rust::src_loc = "102:1"]
    pub type uintmax_t = __uintmax_t;
    #[c2rust::src_loc = "227:11"]
    pub const SIZE_MAX: libc::c_ulong = 18446744073709551615 as libc::c_ulong;
    use super::types_h::{__intmax_t, __uintmax_t};
}
#[c2rust::header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.1/include/stdarg.h:5"]
pub mod stdarg_h {
    #[c2rust::src_loc = "30:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.1/include/stddef.h:7"]
pub mod stddef_h {
    #[c2rust::src_loc = "62:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "51:1"]
    pub type ptrdiff_t = libc::c_long;
    #[c2rust::src_loc = "105:11"]
    pub const NULL_0: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "105:11"]
    pub const NULL_1: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "105:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/types.h:12"]
pub mod nvim_types_h {
    // dummy to pass an ACL to a function
    // Shorthand for unsigned variables. Many systems, but not all, have u_char
// already defined, so we use char_u to avoid trouble.
    #[c2rust::src_loc = "11:1"]
    pub type char_u = libc::c_uchar;
    #[c2rust::src_loc = "17:1"]
    pub type handle_T = libc::c_int;
    #[c2rust::src_loc = "22:1"]
    pub type LuaRef = libc::c_int;
    // NVIM_TYPES_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/eval/typval.h:12"]
pub mod typval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "255:9"]
    pub struct sctx_T {
        pub sc_sid: scid_T,
        pub sc_seq: libc::c_int,
        pub sc_lnum: linenr_T,
    }
    #[c2rust::src_loc = "245:1"]
    pub type scid_T = libc::c_int;
    // for linenr_T
    // / Type used for VimL VAR_NUMBER values
    #[c2rust::src_loc = "26:1"]
    pub type varnumber_T = int64_t;
    // / Type used for VimL VAR_FLOAT values
    #[c2rust::src_loc = "30:1"]
    pub type float_T = libc::c_double;
    // / Maximal possible value of varnumber_T variable
    // / Mimimal possible value of varnumber_T variable
    // / %d printf format specifier for varnumber_T
    // / Structure holding dictionary watcher
    // prevent recursion if the dict is changed in the callback
    // / Special variable values
    // /< v:false
    // /< v:true
    // /< v:null
    // / Variable lock status for typval_T.v_lock
    // /< Not locked.
    // /< User lock, can be unlocked.
    // /< Locked forever.
    // / VimL variable types, for use in typval_T.v_type
    // /< Unknown (unspecified) value.
    // /< Number, .v_number is used.
    // /< String, .v_string is used.
    // /< Function reference, .v_string is used as function name.
    // /< List, .v_list is used.
    // /< Dictionary, .v_dict is used.
    // /< Floating-point value, .v_float is used.
    // /< Special value (true, false, null), .v_special
                    // /< is used.
    // /< Partial, .v_partial is used.
    // / Structure that holds an internal variable value
    // /< Variable type.
    // /< Variable lock status.
    // /< Number, for VAR_NUMBER.
    // /< Special value, for VAR_SPECIAL.
    // /< Floating-point number, for VAR_FLOAT.
    // /< String, for VAR_STRING and VAR_FUNC, can be NULL.
    // /< List for VAR_LIST, can be NULL.
    // /< Dictionary for VAR_DICT, can be NULL.
    // /< Closure: function with args.
    // /< Actual value.
    // / Values for (struct dictvar_S).dv_scope
    // /< Not a scope dictionary.
    // /< Scope dictionary which requires prefix (a:, v:, …).
    // /< Scope dictionary which may be accessed without prefix
                      // /< (l:, g:).
    // / Structure to hold an item of a list
    // /< Next item in list.
    // /< Previous item in list.
    // /< Item value.
    // / Structure used by those that are using an item in a list
    // /< Item being watched.
    // /< Next watcher.
    // / Structure to hold info about a list
// / Order of members is optimized to reduce padding.
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "164:8"]
    pub struct listvar_S {
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
    #[c2rust::src_loc = "102:9"]
    pub type VarLockStatus = libc::c_uint;
    #[c2rust::src_loc = "105:3"]
    pub const VAR_FIXED: VarLockStatus = 2;
    #[c2rust::src_loc = "104:3"]
    pub const VAR_LOCKED: VarLockStatus = 1;
    #[c2rust::src_loc = "103:3"]
    pub const VAR_UNLOCKED: VarLockStatus = 0;
    #[c2rust::src_loc = "64:1"]
    pub type list_T = listvar_S;
    #[c2rust::src_loc = "146:1"]
    pub type listitem_T = listitem_S;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "148:8"]
    pub struct listitem_S {
        pub li_next: *mut listitem_T,
        pub li_prev: *mut listitem_T,
        pub li_tv: typval_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "123:9"]
    pub struct typval_T {
        pub v_type: VarType,
        pub v_lock: VarLockStatus,
        pub vval: typval_vval_union,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:9"]
    pub union typval_vval_union {
        pub v_number: varnumber_T,
        pub v_special: SpecialVarValue,
        pub v_float: float_T,
        pub v_string: *mut char_u,
        pub v_list: *mut list_T,
        pub v_dict: *mut dict_T,
        pub v_partial: *mut partial_T,
    }
    #[c2rust::src_loc = "66:1"]
    pub type partial_T = partial_S;
    // /< Zero, VAR_LOCKED, VAR_FIXED.
    // Static list with 10 items. Use tv_list_init_static10() to initialize.
    // must be first
    /* Structure that holds scope dictionary itself. */
    /* Flags. */
    /* Key value. */
    // / Structure to hold a scope dictionary
// /
// / @warning Must be compatible with dictitem_T.
// /
// / For use in find_var_in_ht to pretend that it found dictionary item when it
// / finds scope dictionary.
    // / Structure to hold an item of a Dictionary
// /
// / @warning Must be compatible with ScopeDictDictItem.
// /
// / Also used for a variable.
    // / Flags for dictitem_T.di_flags
    // /< Read-only value
    // /< Value, read-only in the sandbox
    // /< Fixed value: cannot be :unlet or remove()d.
    // /< Locked value.
    // /< Separately allocated.
    // / Structure representing a Dictionary
    // /< Whole dictionary lock status.
    // /< Non-zero (#VAR_SCOPE, #VAR_DEF_SCOPE) if
                          // /< dictionary represents a scope (i.e. g:, l: …).
    // /< Reference count.
    // /< ID used when recursivery traversing a value.
    // /< Hashtab containing all items.
    // /< Copied dict used by deepcopy().
    // /< Next dictionary in used dictionaries list.
    // /< Previous dictionary in used dictionaries list.
    // /< Dictionary key watchers set by user code.
    // / Type used for script ID
    // / Format argument for scid_T
    // SCript ConteXt (SCTX): identifies a script line.
// When sourcing a script "sc_lnum" is zero, "sourcing_lnum" is the current
// line number. When executing a user function "sc_lnum" is the line where the
// function was defined, "sourcing_lnum" is the line number inside the
// function.  When stored with a function, mapping, option, etc. "sc_lnum" is
// the line number in the script "sc_sid".
    // script ID
    // sourcing sequence number
    // line number
    // Structure to hold info for a function that is currently being executed.
    // / Structure to hold info for a user function.
    // /< variable nr of arguments
    // /< nr of active calls
    // /< func_clear() was already called
    // /< arguments
    // /< function lines
    // /< true when func is being profiled
    // Profiling the function as a whole.
    // /< nr of calls
    // /< time spent in function + children
    // /< time spent in function itself
    // /< time spent in children this call
    // Profiling the function per line.
    // /< nr of times line was executed
    // /< time spent in a line + children
    // /< time spent in a line itself
    // /< start time for current line
    // /< time spent in children for this line
    // /< start wait time for current line
    // /< index of line being timed; -1 if none
    // /< line being timed was executed
    // /< SCTX where function was defined,
                                 // /< used for s: variables
    // /< reference count, see func_name_refcount()
    // /< l: local variables for closure
    // /< Name of function; can start with <SNR>123_
                                 // /< (<SNR> is K_SPECIAL KS_EXTRA KE_SNR)
    // / Maximum number of function arguments
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "299:8"]
    pub struct partial_S {
        pub pt_refcount: libc::c_int,
        pub pt_name: *mut char_u,
        pub pt_func: *mut ufunc_T,
        pub pt_auto: bool,
        pub pt_argc: libc::c_int,
        pub pt_argv: *mut typval_T,
        pub pt_dict: *mut dict_T,
    }
    #[c2rust::src_loc = "65:1"]
    pub type dict_T = dictvar_S;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "231:8"]
    pub struct dictvar_S {
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
    #[c2rust::src_loc = "138:9"]
    pub type ScopeType = libc::c_uint;
    #[c2rust::src_loc = "141:3"]
    pub const VAR_DEF_SCOPE: ScopeType = 2;
    #[c2rust::src_loc = "140:3"]
    pub const VAR_SCOPE: ScopeType = 1;
    #[c2rust::src_loc = "139:3"]
    pub const VAR_NO_SCOPE: ScopeType = 0;
    #[c2rust::src_loc = "68:1"]
    pub type ufunc_T = ufunc;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "265:8"]
    pub struct ufunc {
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
        pub uf_name: [char_u; 0],
    }
    #[c2rust::src_loc = "262:1"]
    pub type funccall_T = funccall_S;
    #[c2rust::src_loc = "95:9"]
    pub type SpecialVarValue = libc::c_uint;
    #[c2rust::src_loc = "98:3"]
    pub const kSpecialVarNull: SpecialVarValue = 2;
    #[c2rust::src_loc = "97:3"]
    pub const kSpecialVarTrue: SpecialVarValue = 1;
    #[c2rust::src_loc = "96:3"]
    pub const kSpecialVarFalse: SpecialVarValue = 0;
    #[c2rust::src_loc = "109:9"]
    pub type VarType = libc::c_uint;
    #[c2rust::src_loc = "119:3"]
    pub const VAR_PARTIAL: VarType = 8;
    #[c2rust::src_loc = "117:3"]
    pub const VAR_SPECIAL: VarType = 7;
    #[c2rust::src_loc = "116:3"]
    pub const VAR_FLOAT: VarType = 6;
    #[c2rust::src_loc = "115:3"]
    pub const VAR_DICT: VarType = 5;
    #[c2rust::src_loc = "114:3"]
    pub const VAR_LIST: VarType = 4;
    #[c2rust::src_loc = "113:3"]
    pub const VAR_FUNC: VarType = 3;
    #[c2rust::src_loc = "112:3"]
    pub const VAR_STRING: VarType = 2;
    #[c2rust::src_loc = "111:3"]
    pub const VAR_NUMBER: VarType = 1;
    #[c2rust::src_loc = "110:3"]
    pub const VAR_UNKNOWN: VarType = 0;
    #[c2rust::src_loc = "155:1"]
    pub type listwatch_T = listwatch_S;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "157:8"]
    pub struct listwatch_S {
        pub lw_item: *mut listitem_T,
        pub lw_next: *mut listwatch_T,
    }
    #[c2rust::src_loc = "70:9"]
    pub type CallbackType = libc::c_uint;
    #[c2rust::src_loc = "73:3"]
    pub const kCallbackPartial: CallbackType = 2;
    #[c2rust::src_loc = "72:3"]
    pub const kCallbackFuncref: CallbackType = 1;
    #[c2rust::src_loc = "71:3"]
    pub const kCallbackNone: CallbackType = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "76:9"]
    pub struct Callback {
        pub data: C2RustUnnamed,
        pub type_0: CallbackType,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:3"]
    pub union C2RustUnnamed {
        pub funcref: *mut char_u,
        pub partial: *mut partial_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:16"]
    pub struct dict_watcher {
        pub callback: Callback,
        pub key_pattern: *mut libc::c_char,
        pub key_pattern_len: size_t,
        pub node: QUEUE,
        pub busy: bool,
    }
    #[c2rust::src_loc = "86:1"]
    pub type DictWatcher = dict_watcher;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "212:9"]
    pub struct ScopeDictDictItem {
        pub di_tv: typval_T,
        pub di_flags: uint8_t,
        pub di_key: [char_u; 1],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "219:9"]
    pub struct dictitem_T {
        pub di_tv: typval_T,
        pub di_flags: uint8_t,
        pub di_key: [char_u; 0],
    }
    #[c2rust::src_loc = "222:9"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "227:3"]
    pub const DI_FLAGS_ALLOC: C2RustUnnamed_0 = 16;
    #[c2rust::src_loc = "226:3"]
    pub const DI_FLAGS_LOCK: C2RustUnnamed_0 = 8;
    #[c2rust::src_loc = "225:3"]
    pub const DI_FLAGS_FIX: C2RustUnnamed_0 = 4;
    #[c2rust::src_loc = "224:3"]
    pub const DI_FLAGS_RO_SBX: C2RustUnnamed_0 = 2;
    #[c2rust::src_loc = "223:3"]
    pub const DI_FLAGS_RO: C2RustUnnamed_0 = 1;
    // In a hashtab item "hi_key" points to "di_key" in a dictitem.
// This avoids adding a pointer to the hashtab item.
    // / Convert a hashitem pointer to a dictitem pointer
    // / Increase reference count for a given list
// /
// / Does nothing for NULL lists.
// /
// / @param[in,out]  l  List to modify.
    #[inline(always)]
    #[c2rust::src_loc = "436:1"]
    pub unsafe extern "C" fn tv_list_ref(l: *mut list_T) {
        if l.is_null() { return }
        (*l).lv_refcount += 1;
    }
    // / Set a list as the return value
// /
// / @param[out]  tv  Object to receive the list
// / @param[in,out]  l  List to pass to the object
    #[inline(always)]
    #[c2rust::src_loc = "451:1"]
    pub unsafe extern "C" fn tv_list_set_ret(tv: *mut typval_T,
                                             l: *mut list_T) {
        (*tv).v_type = VAR_LIST;
        (*tv).vval.v_list = l;
        tv_list_ref(l);
    }
    // / Get list lock status
// /
// / Returns VAR_FIXED for NULL lists.
// /
// / @param[in]  l  List to check.
    #[inline]
    #[c2rust::src_loc = "466:1"]
    pub unsafe extern "C" fn tv_list_locked(l: *const list_T)
     -> VarLockStatus {
        if l.is_null() { return VAR_FIXED }
        return (*l).lv_lock;
    }
    // / Set list lock status
// /
// / May only “set” VAR_FIXED for NULL lists.
// /
// / @param[out]  l  List to modify.
// / @param[in]  lock  New lock status.
    #[inline]
    #[c2rust::src_loc = "480:1"]
    pub unsafe extern "C" fn tv_list_set_lock(l: *mut list_T,
                                              lock: VarLockStatus) {
        if l.is_null() {
            if lock as libc::c_uint ==
                   VAR_FIXED as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"lock == VAR_FIXED\x00" as *const u8 as
                                  *const libc::c_char,
                              b"/home/vole/neovim/src/nvim/eval/typval.h\x00"
                                  as *const u8 as *const libc::c_char,
                              484 as libc::c_int as libc::c_uint,
                              __ASSERT_FUNCTION.as_ptr());
            }
            return
        }
        (*l).lv_lock = lock;
    }
    // / Set list copyID
// /
// / Does not expect NULL list, be careful.
// /
// / @param[out]  l  List to modify.
// / @param[in]  copyid  New copyID.
    #[inline]
    #[c2rust::src_loc = "496:1"]
    pub unsafe extern "C" fn tv_list_set_copyid(l: *mut list_T,
                                                copyid: libc::c_int) {
        (*l).lv_copyID = copyid;
    }
    // / Get the number of items in a list
// /
// / @param[in]  l  List to check.
    #[inline]
    #[c2rust::src_loc = "509:1"]
    pub unsafe extern "C" fn tv_list_len(l: *const list_T) -> libc::c_int {
        if l.is_null() { return 0 as libc::c_int }
        return (*l).lv_len;
    }
    // / Get list copyID
// /
// / Does not expect NULL list, be careful.
// /
// / @param[in]  l  List to check.
    #[inline]
    #[c2rust::src_loc = "526:1"]
    pub unsafe extern "C" fn tv_list_copyid(l: *const list_T) -> libc::c_int {
        return (*l).lv_copyID;
    }
    // / Get latest list copy
// /
// / Gets lv_copylist field assigned by tv_list_copy() earlier.
// /
// / Does not expect NULL list, be careful.
// /
// / @param[in]  l  List to check.
    #[inline]
    #[c2rust::src_loc = "541:1"]
    pub unsafe extern "C" fn tv_list_latest_copy(l: *const list_T)
     -> *mut list_T {
        return (*l).lv_copylist;
    }
    // / Normalize index: that is, return either -1 or non-negative index
// /
// / @param[in]  l  List to index. Used to get length.
// / @param[in]  n  List index, possibly negative.
// /
// / @return -1 or list index in range [0, tv_list_len(l)).
    #[inline]
    #[c2rust::src_loc = "555:1"]
    pub unsafe extern "C" fn tv_list_uidx(l: *const list_T,
                                          mut n: libc::c_int) -> libc::c_int {
        // Negative index is relative to the end.
        if n < 0 as libc::c_int { n += tv_list_len(l) }
        // Check for index out of range.
        if n < 0 as libc::c_int || n >= tv_list_len(l) {
            return -(1 as libc::c_int)
        }
        return n;
    }
    // / Check whether list has watchers
// /
// / E.g. is referenced by a :for loop.
// /
// / @param[in]  l  List to check.
// /
// / @return true if there are watchers, false otherwise.
    #[inline]
    #[c2rust::src_loc = "579:1"]
    pub unsafe extern "C" fn tv_list_has_watchers(l: *const list_T) -> bool {
        return !l.is_null() && !(*l).lv_watch.is_null();
    }
    // / Get first list item
// /
// / @param[in]  l  List to get item from.
// /
// / @return List item or NULL in case of an empty list.
    #[inline]
    #[c2rust::src_loc = "592:1"]
    pub unsafe extern "C" fn tv_list_first(l: *const list_T)
     -> *mut listitem_T {
        if l.is_null() { return NULL_0 as *mut listitem_T }
        return (*l).lv_first;
    }
    // / Get last list item
// /
// / @param[in]  l  List to get item from.
// /
// / @return List item or NULL in case of an empty list.
    #[inline]
    #[c2rust::src_loc = "610:1"]
    pub unsafe extern "C" fn tv_list_last(l: *const list_T)
     -> *mut listitem_T {
        if l.is_null() { return NULL_0 as *mut listitem_T }
        return (*l).lv_last;
    }
    // / Set a dictionary as the return value
// /
// / @param[out]  tv  Object to receive the dictionary
// / @param[in,out]  d  Dictionary to pass to the object
    #[inline(always)]
    #[c2rust::src_loc = "627:1"]
    pub unsafe extern "C" fn tv_dict_set_ret(tv: *mut typval_T,
                                             d: *mut dict_T) {
        (*tv).v_type = VAR_DICT;
        (*tv).vval.v_dict = d;
        if !d.is_null() { (*d).dv_refcount += 1 };
    }
    // / Get the number of items in a Dictionary
// /
// / @param[in]  d  Dictionary to check.
    #[inline]
    #[c2rust::src_loc = "642:1"]
    pub unsafe extern "C" fn tv_dict_len(d: *const dict_T) -> libc::c_long {
        if d.is_null() { return 0 as libc::c_long }
        return (*d).dv_hashtab.ht_used as libc::c_long;
    }
    // / Check if dictionary is watched
// /
// / @param[in]  d  Dictionary to check.
// /
// / @return true if there is at least one watcher.
    #[inline]
    #[c2rust::src_loc = "658:1"]
    pub unsafe extern "C" fn tv_dict_is_watched(d: *const dict_T) -> bool {
        return !d.is_null() && QUEUE_EMPTY(&(*d).watchers) == 0;
    }
    // / Initialize VimL object
// /
// / Initializes to unlocked VAR_UNKNOWN object.
// /
// / @param[out]  tv  Object to initialize.
    #[inline]
    #[c2rust::src_loc = "668:1"]
    pub unsafe extern "C" fn tv_init(tv: *mut typval_T) {
        if !tv.is_null() {
            memset(tv as *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<typval_T>() as libc::c_ulong);
        };
    }
    // / Iterate over a list
// /
// / @param  modifier  Modifier: expected to be const or nothing, volatile should
// /                   also work if you have any uses for the volatile list.
// / @param[in]  l  List to iterate over.
// / @param  li  Name of the variable with current listitem_T entry.
// / @param  code  Cycle body.
    // / Iterate over a list
// /
// / To be used when you need to modify list or values you iterate over, use
// / #TV_LIST_ITER_CONST if you don’t.
// /
// / @param[in]  l  List to iterate over.
// / @param  li  Name of the variable with current listitem_T entry.
// / @param  code  Cycle body.
    // / Iterate over a list
// /
// / To be used when you don’t need to modify list or values you iterate over,
// / use #TV_LIST_ITER if you do.
// /
// / @param[in]  l  List to iterate over.
// / @param  li  Name of the variable with current listitem_T entry.
// / @param  code  Cycle body.
    // Below macros are macros to avoid duplicating code for functionally identical
// const and non-const function variants.
    // / Get typval_T out of list item
// /
// / @param[in]  li  List item to get typval_T from, must not be NULL.
// /
// / @return Pointer to typval_T.
    // / Get next list item given the current one
// /
// / @param[in]  l  List to get item from.
// / @param[in]  li  List item to get typval_T from.
// /
// / @return Pointer to the next item or NULL.
    // / Get previous list item given the current one
// /
// / @param[in]  l  List to get item from.
// / @param[in]  li  List item to get typval_T from.
// /
// / @return Pointer to the previous item or NULL.
    // List argument is not used currently, but it is a must for lists implemented
// as a pair (size(in list), array) without terminator - basically for lists on
// top of kvec.
    // / Iterate over a dictionary
// /
// / @param[in]  d  Dictionary to iterate over.
// / @param  di  Name of the variable with current dictitem_T entry.
// / @param  code  Cycle body.
    // FIXME circular dependency, cannot import message.h.
    // / Get the float value
// /
// / Raises an error if object is not number or floating-point.
// /
// / @param[in]  tv  VimL object to get value from.
// / @param[out]  ret_f  Location where resulting float is stored.
// /
// / @return true in case of success, false if tv is not a number or float.
    #[inline]
    #[c2rust::src_loc = "790:1"]
    pub unsafe extern "C" fn tv_get_float_chk(tv: *const typval_T,
                                              ret_f: *mut float_T) -> bool {
        if (*tv).v_type as libc::c_uint ==
               VAR_FLOAT as libc::c_int as libc::c_uint {
            *ret_f = (*tv).vval.v_float;
            return true_0 != 0
        }
        if (*tv).v_type as libc::c_uint ==
               VAR_NUMBER as libc::c_int as libc::c_uint {
            *ret_f = (*tv).vval.v_number as float_T;
            return true_0 != 0
        }
        emsgf(b"%s\x00" as *const u8 as *const libc::c_char,
              gettext(b"E808: Number or Float required\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char));
        return false_0 != 0;
    }
    // / Compute the `DictWatcher` address from a QUEUE node.
// /
// / This only exists for .asan-blacklist (ASAN doesn't handle QUEUE_DATA pointer
// / arithmetic).
    #[inline(always)]
    #[c2rust::src_loc = "813:1"]
    pub unsafe extern "C" fn tv_dict_watcher_node_data(mut q: *mut QUEUE)
     -> *mut DictWatcher {
        return (q as
                    *mut libc::c_char).offset(-(32 as libc::c_ulong as isize))
                   as *mut DictWatcher;
    }
    // / Check whether given typval_T contains a function
// /
// / That is, whether it contains VAR_FUNC or VAR_PARTIAL.
// /
// / @param[in]  tv  Typval to check.
// /
// / @return True if it is a function or a partial, false otherwise.
    #[inline]
    #[c2rust::src_loc = "828:1"]
    pub unsafe extern "C" fn tv_is_func(tv: typval_T) -> bool {
        return tv.v_type as libc::c_uint ==
                   VAR_FUNC as libc::c_int as libc::c_uint ||
                   tv.v_type as libc::c_uint ==
                       VAR_PARTIAL as libc::c_int as libc::c_uint;
    }
    use super::pos_h::linenr_T;
    use super::stdint_intn_h::int64_t;
    use super::nvim_types_h::char_u;
    use super::hashtab_h::hashtab_T;
    use super::queue_h::{QUEUE, QUEUE_EMPTY};
    use super::garray_h::garray_T;
    use super::profile_h::proftime_T;
    use super::stddef_h::{size_t, NULL_0};
    use super::stdint_uintn_h::uint8_t;
    use super::assert_h::{__assert_fail, __ASSERT_FUNCTION};
    use super::string_h::memset;
    use super::stdbool_h::{true_0, false_0};
    use super::message_h_generated_h::emsgf;
    use super::libintl_h::gettext;
    extern "C" {
        #[c2rust::src_loc = "262:16"]
        pub type funccall_S;
    }
    // NVIM_EVAL_TYPVAL_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/pos.h:12"]
pub mod pos_h {
    #[c2rust::src_loc = "4:1"]
    pub type linenr_T = libc::c_long;
    #[c2rust::src_loc = "9:1"]
    pub type colnr_T = libc::c_int;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "25:9"]
    pub struct pos_T {
        pub lnum: linenr_T,
        pub col: colnr_T,
        pub coladd: colnr_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "35:9"]
    pub struct lpos_T {
        pub lnum: linenr_T,
        pub col: colnr_T,
    }
    // NVIM_POS_H
}
#[c2rust::header_src = "/usr/include/stdio.h:12"]
pub mod stdio_h {
    #[c2rust::src_loc = "77:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/types.h:12"]
pub mod sys_types_h {
    #[c2rust::src_loc = "64:1"]
    pub type gid_t = __gid_t;
    #[c2rust::src_loc = "79:1"]
    pub type uid_t = __uid_t;
    use super::types_h::{__gid_t, __uid_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/time_t.h:12"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/pthreadtypes-arch.h:12"]
pub mod pthreadtypes_arch_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "65:8"]
    pub struct __pthread_rwlock_arch_t {
        pub __readers: libc::c_uint,
        pub __writers: libc::c_uint,
        pub __wrphase_futex: libc::c_uint,
        pub __writers_futex: libc::c_uint,
        pub __pad3: libc::c_uint,
        pub __pad4: libc::c_uint,
        pub __cur_writer: libc::c_int,
        pub __shared: libc::c_int,
        pub __rwelision: libc::c_schar,
        pub __pad1: [libc::c_uchar; 7],
        pub __pad2: libc::c_ulong,
        pub __flags: libc::c_uint,
    }
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/thread-shared-types.h:12"]
pub mod thread_shared_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:16"]
    pub struct __pthread_internal_list {
        pub __prev: *mut __pthread_internal_list,
        pub __next: *mut __pthread_internal_list,
    }
    #[c2rust::src_loc = "82:1"]
    pub type __pthread_list_t = __pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:8"]
    pub struct __pthread_mutex_s {
        pub __lock: libc::c_int,
        pub __count: libc::c_uint,
        pub __owner: libc::c_int,
        pub __nusers: libc::c_uint,
        pub __kind: libc::c_int,
        pub __spins: libc::c_short,
        pub __elision: libc::c_short,
        pub __list: __pthread_list_t,
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/pthreadtypes.h:12"]
pub mod pthreadtypes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub union pthread_mutex_t {
        pub __data: __pthread_mutex_s,
        pub __size: [libc::c_char; 40],
        pub __align: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:9"]
    pub union pthread_rwlock_t {
        pub __data: __pthread_rwlock_arch_t,
        pub __size: [libc::c_char; 56],
        pub __align: libc::c_long,
    }
    use super::thread_shared_types_h::__pthread_mutex_s;
    use super::pthreadtypes_arch_h::__pthread_rwlock_arch_t;
}
#[c2rust::header_src = "/usr/include/stdlib.h:12"]
pub mod stdlib_h {
    #[c2rust::src_loc = "805:1"]
    pub type __compar_fn_t
        =
        Option<unsafe extern "C" fn(_: *const libc::c_void,
                                    _: *const libc::c_void) -> libc::c_int>;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "541:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "549:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "563:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "588:13"]
        pub fn abort() -> !;
        #[no_mangle]
        #[c2rust::src_loc = "827:1"]
        pub fn qsort(__base: *mut libc::c_void, __nmemb: size_t,
                     __size: size_t, __compar: __compar_fn_t);
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/hashtab.h:12"]
pub mod hashtab_h {
    // / Type for hash number (hash calculation result).
    #[c2rust::src_loc = "14:1"]
    pub type hash_T = size_t;
    // / The address of "hash_removed" is used as a magic number
// / for hi_key to indicate a removed item.
    // / Hashtable item.
// /
// / Each item has a NUL terminated string key.
// / A key can appear only once in the table.
// /
// / A hash number is computed from the key for quick lookup.  When the hashes
// / of two different keys point to the same entry an algorithm is used to
// / iterate over other entries in the table until the right one is found.
// / To make the iteration work removed keys are different from entries where a
// / key was never present.
// /
// / Note that this does not contain a pointer to the key and another pointer to
// / the value. Instead, it is assumed that the key is contained within the
// / value, so that you can get a pointer to the value subtracting an offset from
// / the pointer to the key.
// / This reduces the size of this item by 1/3.
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct hashitem_S {
        pub hi_hash: hash_T,
        pub hi_key: *mut char_u,
    }
    #[c2rust::src_loc = "38:1"]
    pub type hashitem_T = hashitem_S;
    // / Initial size for a hashtable.
// / Our items are relatively small and growing is expensive, thus start with 16.
// / Must be a power of 2.
    // / An array-based hashtable.
// /
// / Keys are NUL terminated strings. They cannot be repeated within a table.
// / Values are of any type.
// /
// / The hashtable grows to accommodate more entries when needed.
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:16"]
    pub struct hashtable_S {
        pub ht_mask: hash_T,
        pub ht_used: size_t,
        pub ht_filled: size_t,
        pub ht_locked: libc::c_int,
        pub ht_array: *mut hashitem_T,
        pub ht_smallarray: [hashitem_T; 16],
    }
    #[c2rust::src_loc = "62:1"]
    pub type hashtab_T = hashtable_S;
    use super::stddef_h::size_t;
    use super::nvim_types_h::char_u;
    // / initial array
    // NVIM_HASHTAB_H
    // / Iterate over a hashtab
// /
// / @param[in]  ht  Hashtab to iterate over.
// / @param  hi  Name of the variable with current hashtab entry.
// / @param  code  Cycle body.
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/garray.h:12"]
pub mod garray_h {
    // for size_t
    // for char_u
    // / Structure used for growing arrays.
// / This is used to store information that only grows, is deleted all at
// / once, and needs to be accessed by index.  See ga_clear() and ga_grow().
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "12:16"]
    pub struct growarray {
        pub ga_len: libc::c_int,
        pub ga_maxlen: libc::c_int,
        pub ga_itemsize: libc::c_int,
        pub ga_growsize: libc::c_int,
        pub ga_data: *mut libc::c_void,
    }
    #[c2rust::src_loc = "12:1"]
    pub type garray_T = growarray;
    #[inline]
    #[c2rust::src_loc = "47:1"]
    pub unsafe extern "C" fn ga_append_via_ptr(mut gap: *mut garray_T,
                                               mut item_size: size_t)
     -> *mut libc::c_void {
        if item_size as libc::c_int != (*gap).ga_itemsize {
            logmsg(WARN_LOG_LEVEL, NULL as *const libc::c_char,
                   (*::std::mem::transmute::<&[u8; 18],
                                             &[libc::c_char; 18]>(b"ga_append_via_ptr\x00")).as_ptr(),
                   50 as libc::c_int, true_0 != 0,
                   b"wrong item size (%zu), should be %d\x00" as *const u8 as
                       *const libc::c_char, item_size, (*gap).ga_itemsize);
        }
        ga_grow(gap, 1 as libc::c_int);
        let fresh0 = (*gap).ga_len;
        (*gap).ga_len = (*gap).ga_len + 1;
        return ((*gap).ga_data as
                    *mut libc::c_char).offset(item_size.wrapping_mul(fresh0 as
                                                                         size_t)
                                                  as isize) as
                   *mut libc::c_void;
    }
    use super::stddef_h::{size_t, NULL};
    use super::log_h_generated_h::logmsg;
    use super::log_h::WARN_LOG_LEVEL;
    use super::stdbool_h::true_0;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "38:1"]
        pub fn ga_grow(gap: *mut garray_T, n: libc::c_int);
    }
    // pointer to the first item
    // NVIM_GARRAY_H
    // / Call `free` for every pointer stored in the garray and then frees the
// / garray.
// /
// / @param gap the garray to be freed
    // / Deep free a garray of specific type using a custom free function.
// / Items in the array as well as the array itself are freed.
// /
// / @param gap the garray to be freed
// / @param item_type type of the item in the garray
// / @param free_item_fn free function that takes (*item_type) as parameter
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/lib/queue.h:12"]
pub mod queue_h {
    // Queue implemented by circularly-linked list.
//
// Adapted from libuv. Simpler and more efficient than klist.h for implementing
// queues that support arbitrary insertion/removal.
//
// Copyright (c) 2013, Ben Noordhuis <info@bnoordhuis.nl>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:16"]
    pub struct _queue {
        pub next: *mut _queue,
        pub prev: *mut _queue,
    }
    #[c2rust::src_loc = "27:1"]
    pub type QUEUE = _queue;
    // Public macros.
    // Important note: mutating the list while QUEUE_FOREACH is
// iterating over its elements results in undefined behavior.
    /* NOLINT(readability/braces) */
    // ffi.cdef is unable to swallow `bool` in place of `int` here.
    #[inline]
    #[c2rust::src_loc = "43:1"]
    pub unsafe extern "C" fn QUEUE_EMPTY(q: *const QUEUE) -> libc::c_int {
        return (q == (*q).next) as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "51:1"]
    pub unsafe extern "C" fn QUEUE_INIT(q: *mut QUEUE) {
        (*q).next = q;
        (*q).prev = q;
    }
    #[inline]
    #[c2rust::src_loc = "57:1"]
    pub unsafe extern "C" fn QUEUE_ADD(h: *mut QUEUE, n: *mut QUEUE) {
        (*(*h).prev).next = (*n).next;
        (*(*n).next).prev = (*h).prev;
        (*h).prev = (*n).prev;
        (*(*h).prev).next = h;
    }
    #[inline]
    #[c2rust::src_loc = "66:1"]
    pub unsafe extern "C" fn QUEUE_INSERT_HEAD(h: *mut QUEUE, q: *mut QUEUE) {
        (*q).next = (*h).next;
        (*q).prev = h;
        (*(*q).next).prev = q;
        (*h).next = q;
    }
    #[inline]
    #[c2rust::src_loc = "75:1"]
    pub unsafe extern "C" fn QUEUE_INSERT_TAIL(h: *mut QUEUE, q: *mut QUEUE) {
        (*q).next = h;
        (*q).prev = (*h).prev;
        (*(*q).prev).next = q;
        (*h).prev = q;
    }
    #[inline]
    #[c2rust::src_loc = "84:1"]
    pub unsafe extern "C" fn QUEUE_REMOVE(q: *mut QUEUE) {
        (*(*q).prev).next = (*q).next;
        (*(*q).next).prev = (*q).prev;
    }
    // NVIM_LIB_QUEUE_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/profile.h:12"]
pub mod profile_h {
    #[c2rust::src_loc = "7:1"]
    pub type proftime_T = uint64_t;
    use super::stdint_uintn_h::uint64_t;
    // NVIM_PROFILE_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/grid_defs.h:12"]
pub mod grid_defs_h {
    // maximum value for 'maxcombine'
    // The characters and attributes drawn on grids.
    #[c2rust::src_loc = "13:1"]
    pub type schar_T = [char_u; 29];
    #[c2rust::src_loc = "14:1"]
    pub type sattr_T = int16_t;
    // / ScreenGrid represents a resizable rectuangular grid displayed by UI clients.
// /
// / chars[] contains the UTF-8 text that is currently displayed on the grid.
// / It is stored as a single block of cells. When redrawing a part of the grid,
// / the new state can be compared with the existing state of the grid. This way
// / we can avoid sending bigger updates than necessary to the Ul layer.
// /
// / Screen cells are stored as NUL-terminated UTF-8 strings, and a cell can
// / contain up to MAX_MCO composing characters after the base character.
// / The composing characters are to be drawn on top of the original character.
// / The content after the NUL is not defined (so comparison must be done a
// / single cell at a time). Double-width characters are stored in the left cell,
// / and the right cell should only contain the empty string. When a part of the
// / screen is cleared, the cells should be filled with a single whitespace char.
// /
// / attrs[] contains the highlighting attribute for each cell.
// / line_offset[n] is the offset from chars[] and attrs[] for the
// / start of line 'n'. These offsets are in general not linear, as full screen
// / scrolling is implemented by rotating the offsets in the line_offset array.
// / line_wraps[] is an array of boolean flags indicating if the screen line
// / wraps to the next line. It can only be true if a window occupies the entire
// / screen width.
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:9"]
    pub struct ScreenGrid {
        pub handle: handle_T,
        pub chars: *mut schar_T,
        pub attrs: *mut sattr_T,
        pub line_offset: *mut libc::c_uint,
        pub line_wraps: *mut char_u,
        pub dirty_col: *mut libc::c_int,
        pub Rows: libc::c_int,
        pub Columns: libc::c_int,
        pub valid: bool,
        pub throttled: bool,
        pub row_offset: libc::c_int,
        pub col_offset: libc::c_int,
        pub blending: bool,
        pub focusable: bool,
        pub comp_row: libc::c_int,
        pub comp_col: libc::c_int,
        pub comp_index: size_t,
        pub comp_disabled: bool,
    }
    use super::nvim_types_h::{char_u, handle_T};
    use super::stdint_intn_h::int16_t;
    use super::stddef_h::size_t;
    // NVIM_GRID_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/buffer_defs.h:12"]
pub mod buffer_defs_h {
    // for FILE
    // Forward declaration
    // Reference to a buffer that stores the value of buf_free_count.
// bufref_valid() only needs to check "buf" when the count differs.
    // for garray_T
    // for ScreenGrid
    // for HLF_COUNT
    // for pos_T, lpos_T and linenr_T
    // for the number window-local and buffer-local options
    // for jump list and tag stack sizes in a buffer and mark types
    // for u_header_T; needs buf_T.
    // for hashtab_T
    // for dict_T
    // for proftime_T
    // for String
    // for Map(K, V)
    // for kvec
    // for marktree
    /*
 * Flags for w_valid.
 * These are set when something in a window structure becomes invalid, except
 * when the cursor is moved.  Call check_cursor_moved() before testing one of
 * the flags.
 * These are reset when that thing has been updated and is valid again.
 *
 * Every function that invalidates one of these must call one of the
 * invalidate_* functions.
 *
 * w_valid is supposed to be used only in screen.c.  From other files, use the
 * functions that set or reset the flags.
 *
 * VALID_BOTLINE    VALID_BOTLINE_AP
 *     on       on      w_botline valid
 *     off      on      w_botline approximated
 *     off      off     w_botline not valid
 *     on       off     not possible
 */
    /* w_wrow (window row) is valid */
    /* w_wcol (window col) is valid */
    /* w_virtcol (file col) is valid */
    /* w_cline_height and w_cline_folded valid */
    /* w_cline_row is valid */
    /* w_botine and w_empty_rows are valid */
    /* w_botine is approximated */
    /* w_topline is valid (for cursor position) */
    // flags for b_flags
    // buffer has been recovered
    // need to check readonly when loading file
                                // into buffer (set by ":e", may be reset by
                                // ":buf")
    // file has never been loaded into buffer,
                                // many variables still need to be set
    // Set when file name is changed after
                                // starting to edit, reset when file is
                                // written out.
    // file didn't exist when editing started
    // Warned for BF_NEW and file created
    // got errors while reading the file
    // dummy buffer, only used internally
    // ":preserve" was used
    /* Mask to check for flags that prevent normal writing */
    // display tick type
    // for struct memline (it needs memfile_T)
    // for struct memfile, bhdr_T, blocknr_T... (it needs buf_T)
    // for regprog_T. Needs win_T and buf_T.
    // for synstate_T (needs reg_extmatch_T, win_T, buf_T)
    // for signlist_T
    /*
 * The taggy struct is used to store the information about a :tag command.
 */
    // tag name
    // cursor position BEFORE ":tag"
    // match number
    // buffer number used for cur_match
    // used with tagfunc
    /*
 * structure used to store one block of the stuff/redo/recording buffers
 */
    // pointer to next buffblock
    // contents (actually longer)
    /*
 * header used for the stuff buffer and the redo buffer
 */
    // first (dummy) block of list
    // buffblock for appending
    // index for reading
    // space in bh_curr for appending
    /*
 * Structure that contains all options that are local to a window.
 * Used twice in a window: for the current buffer and for all buffers.
 * Also used in wininfo_T.
 */
    /* 'arabic' */
    // 'breakindent'
    /* 'breakindentopt' */
    // 'diff'
    // 'foldcolumn'
    // 'fdc' saved for diff mode
    /* 'foldenable' */
    /* 'foldenable' saved for diff mode */
    /* 'foldignore' */
    /* 'foldlevel' */
    /* 'foldlevel' state saved for diff mode */
    /* 'foldmethod' */
    /* 'fdm' saved for diff mode */
    /* 'foldminlines' */
    /* 'foldnestmax' */
    /* 'foldexpr' */
    /* 'foldtext' */
    /* 'foldmarker' */
    /* 'linebreak' */
    /* 'list' */
    /* 'number' */
    /* 'relativenumber' */
    /* 'numberwidth' */
    /* 'winfixheight' */
    /* 'winfixwidth' */
    /* 'previewwindow' */
    /* 'rightleft' */
    /* 'rightleftcmd' */
    /* 'scroll' */
    /* 'spell' */
    /* 'cursorcolumn' */
    /* 'cursorline' */
    /* 'colorcolumn' */
    /* 'statusline' */
    /* 'scrollbind' */
    /* options were saved for starting diff mode */
    /* 'scrollbind' saved for diff mode*/
    /* 'wrap' */
    /* 'wrap' state saved for diff mode*/
    /* 'concealcursor' */
    /* 'conceallevel' */
    /* 'cursorbind' */
    /* 'cursorbind' state saved for diff mode*/
    // 'signcolumn'
    // 'winhighlight'
    // 'fillchars'
    // 'listchars'
    // 'winblend'
    // SCTXs for window-local options
    /*
 * Window info stored with a buffer.
 *
 * Two types of info are kept for a buffer which are associated with a
 * specific window:
 * 1. Each window can have a different line number associated with a buffer.
 * 2. The window-local options for a buffer work in a similar way.
 * The window-info is kept in a list at b_wininfo.  It is kept in
 * most-recently-used order.
 */
    /* next entry or NULL for last entry */
    /* previous entry or NULL for first entry */
    /* pointer to window that did set wi_fpos */
    /* last cursor position in the file */
    /* true when wi_opt has useful values */
    /* local window options */
    /* copy of w_fold_manual */
    /* clone of w_folds */
    /*
 * Argument list: Array of file names.
 * Used for the global argument list and the argument lists local to a window.
 *
 * TODO: move struct arglist to another header
 */
    /* growarray with the array of file names */
    /* number of windows using this arglist */
    // /< id of this arglist
    /*
 * For each argument remember the file name as it was given, and the buffer
 * number that contains the expanded file name (required for when ":cd" is
 * used.
 *
 * TODO: move aentry_T to another header
 */
    /* file name as specified */
    /* buffer number with expanded file name */
    /*
 * Used for the typeahead buffer: typebuf.
 */
    /* buffer for typed characters */
    /* mapping flags for characters in tb_buf[] */
    /* size of tb_buf[] */
    /* current position in tb_buf[] */
    /* number of valid bytes in tb_buf[] */
    /* nr of mapped bytes in tb_buf[] */
    /* nr of silently mapped bytes in tb_buf[] */
    /* nr of bytes without abbrev. in tb_buf[] */
    /* nr of time tb_buf was changed; never zero */
    /* Struct to hold the saved typeahead for save_typeahead(). */
    /* TRUE when save_typebuf valid */
    /*
 * Structure used for mappings and abbreviations.
 */
    // next mapblock in list
    // mapped from, lhs
    // mapped to, rhs
    // rhs as entered by the user
    // strlen(m_keys)
    // valid mode
    // if non-zero no re-mapping for m_str
    // <silent> used, don't echo commands
    // <nowait> used
    // <expr> used, m_str is an expression
    // SCTX where map was defined
    /*
 * Used for highlighting in the status line.
 */
    /* 0: no HL, 1-9: User HL, < 0 for syn ID */
    /* values for b_syn_spell: what to do with toplevel text */
    /* spell check if @Spell not defined */
    /* spell check toplevel text */
    /* don't spell check toplevel text */
    /* avoid #ifdefs for when b_spell is not available */
    /*
 * Used for :syntime: timing of executing a syntax pattern.
 */
    /* total time used */
    /* time of slowest call */
    /* nr of times used */
    /* nr of times matched */
    /*
 * These are items normally related to a buffer.  But when using ":ownsyntax"
 * a window may have its own instance.
 */
    // syntax keywords hash table
    // idem, ignore case
    // TRUE when error occurred in HL
    // true when 'redrawtime' reached
    // ignore case for :syn cmds
    // SYNSPL_ values
    // table for syntax patterns
    // table for syntax clusters
    // @Spell cluster ID or 0
    // @NoSpell cluster ID or 0
    // TRUE when there is an item with a
                                        // "containedin" argument
    // flags about how to sync
    // group to sync on
    // minimal sync lines offset
    // maximal sync lines offset
    // offset for multi-line pattern
    // line continuation pattern
    // line continuation program
    /* ignore-case flag for above */
    /* for ":syntax include" */
    /* auto-conceal for :syn cmds */
    /* number of patterns with the HL_FOLD
                                           flag set */
    /*
   * b_sst_array[] contains the state stack for a number of lines, for the
   * start of that line (col == 0).  This avoids having to recompute the
   * syntax state too often.
   * b_sst_array[] is allocated to hold the state for all displayed lines,
   * and states for 1 out of about 20 other lines.
   * b_sst_array        pointer to an array of synstate_T
   * b_sst_len          number of entries in b_sst_array[]
   * b_sst_first        pointer to first used entry in b_sst_array[] or NULL
   * b_sst_firstfree    pointer to first free entry in b_sst_array[] or NULL
   * b_sst_freecount    number of free entries in b_sst_array[]
   * b_sst_check_lnum   entries after this lnum need to be checked for
   *                    validity (MAXLNUM means no check needed)
   */
    // last display tick
    // for spell checking
    // list of pointers to slang_T, see spell.c
    // flags: is midword char
    // multi-byte midword chars
    // 'spellcapcheck'
    // program for 'spellcapcheck'
    // 'spellfile'
    // 'spelllang'
    // all CJK letters as OK
    // syntax iskeyword option
    // iskeyword option
    // / Type used for changedtick_di member in buf_T
// /
// / Primary exists so that literals of relevant type can be made.
    // Maximum number of maphash blocks we will have
    /*
 * buffer: structure that holds information about one file
 *
 * Several windows can share a single Buffer
 * A buffer is unallocated if there is no memfile for it.
 * A buffer is new if the associated file has never been loaded yet.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "485:8"]
    pub struct file_buffer {
        pub handle: handle_T,
        pub b_ml: memline_T,
        pub b_next: *mut buf_T,
        pub b_prev: *mut buf_T,
        pub b_nwindows: libc::c_int,
        pub b_flags: libc::c_int,
        pub b_locked: libc::c_int,
        pub b_ffname: *mut char_u,
        pub b_sfname: *mut char_u,
        pub b_fname: *mut char_u,
        pub file_id_valid: bool,
        pub file_id: FileID,
        pub b_changed: libc::c_int,
        pub changedtick_di: ChangedtickDictItem,
        pub b_last_changedtick: varnumber_T,
        pub b_last_changedtick_pum: varnumber_T,
        pub b_saving: bool,
        pub b_mod_set: bool,
        pub b_mod_top: linenr_T,
        pub b_mod_bot: linenr_T,
        pub b_mod_xlines: libc::c_long,
        pub b_wininfo: *mut wininfo_T,
        pub b_mtime: libc::c_long,
        pub b_mtime_read: libc::c_long,
        pub b_orig_size: uint64_t,
        pub b_orig_mode: libc::c_int,
        pub b_namedm: [fmark_T; 26],
        pub b_visual: visualinfo_T,
        pub b_visual_mode_eval: libc::c_int,
        pub b_last_cursor: fmark_T,
        pub b_last_insert: fmark_T,
        pub b_last_change: fmark_T,
        pub b_changelist: [fmark_T; 100],
        pub b_changelistlen: libc::c_int,
        pub b_new_change: bool,
        pub b_chartab: [uint64_t; 4],
        pub b_maphash: [*mut mapblock_T; 256],
        pub b_first_abbr: *mut mapblock_T,
        pub b_ucmds: garray_T,
        pub b_op_start: pos_T,
        pub b_op_start_orig: pos_T,
        pub b_op_end: pos_T,
        pub b_marks_read: bool,
        pub b_u_oldhead: *mut u_header_T,
        pub b_u_newhead: *mut u_header_T,
        pub b_u_curhead: *mut u_header_T,
        pub b_u_numhead: libc::c_int,
        pub b_u_synced: bool,
        pub b_u_seq_last: libc::c_long,
        pub b_u_save_nr_last: libc::c_long,
        pub b_u_seq_cur: libc::c_long,
        pub b_u_time_cur: time_t,
        pub b_u_save_nr_cur: libc::c_long,
        pub b_u_line_ptr: *mut char_u,
        pub b_u_line_lnum: linenr_T,
        pub b_u_line_colnr: colnr_T,
        pub b_scanned: bool,
        pub b_p_iminsert: libc::c_long,
        pub b_p_imsearch: libc::c_long,
        pub b_kmap_state: libc::c_short,
        pub b_kmap_ga: garray_T,
        pub b_p_initialized: bool,
        pub b_p_script_ctx: [LastSet; 80],
        pub b_p_ai: libc::c_int,
        pub b_p_ai_nopaste: libc::c_int,
        pub b_p_bkc: *mut char_u,
        pub b_bkc_flags: libc::c_uint,
        pub b_p_ci: libc::c_int,
        pub b_p_bin: libc::c_int,
        pub b_p_bomb: libc::c_int,
        pub b_p_bh: *mut char_u,
        pub b_p_bt: *mut char_u,
        pub b_has_qf_entry: libc::c_int,
        pub b_p_bl: libc::c_int,
        pub b_p_channel: libc::c_long,
        pub b_p_cin: libc::c_int,
        pub b_p_cino: *mut char_u,
        pub b_p_cink: *mut char_u,
        pub b_p_cinw: *mut char_u,
        pub b_p_com: *mut char_u,
        pub b_p_cms: *mut char_u,
        pub b_p_cpt: *mut char_u,
        pub b_p_cfu: *mut char_u,
        pub b_p_ofu: *mut char_u,
        pub b_p_tfu: *mut char_u,
        pub b_p_eol: libc::c_int,
        pub b_p_fixeol: libc::c_int,
        pub b_p_et: libc::c_int,
        pub b_p_et_nobin: libc::c_int,
        pub b_p_et_nopaste: libc::c_int,
        pub b_p_fenc: *mut char_u,
        pub b_p_ff: *mut char_u,
        pub b_p_ft: *mut char_u,
        pub b_p_fo: *mut char_u,
        pub b_p_flp: *mut char_u,
        pub b_p_inf: libc::c_int,
        pub b_p_isk: *mut char_u,
        pub b_p_def: *mut char_u,
        pub b_p_inc: *mut char_u,
        pub b_p_inex: *mut char_u,
        pub b_p_inex_flags: uint32_t,
        pub b_p_inde: *mut char_u,
        pub b_p_inde_flags: uint32_t,
        pub b_p_indk: *mut char_u,
        pub b_p_fp: *mut char_u,
        pub b_p_fex: *mut char_u,
        pub b_p_fex_flags: uint32_t,
        pub b_p_kp: *mut char_u,
        pub b_p_lisp: libc::c_int,
        pub b_p_menc: *mut char_u,
        pub b_p_mps: *mut char_u,
        pub b_p_ml: libc::c_int,
        pub b_p_ml_nobin: libc::c_int,
        pub b_p_ma: libc::c_int,
        pub b_p_nf: *mut char_u,
        pub b_p_pi: libc::c_int,
        pub b_p_qe: *mut char_u,
        pub b_p_ro: libc::c_int,
        pub b_p_sw: libc::c_long,
        pub b_p_scbk: libc::c_long,
        pub b_p_si: libc::c_int,
        pub b_p_sts: libc::c_long,
        pub b_p_sts_nopaste: libc::c_long,
        pub b_p_sua: *mut char_u,
        pub b_p_swf: libc::c_int,
        pub b_p_smc: libc::c_long,
        pub b_p_syn: *mut char_u,
        pub b_p_ts: libc::c_long,
        pub b_p_tw: libc::c_long,
        pub b_p_tw_nobin: libc::c_long,
        pub b_p_tw_nopaste: libc::c_long,
        pub b_p_wm: libc::c_long,
        pub b_p_wm_nobin: libc::c_long,
        pub b_p_wm_nopaste: libc::c_long,
        pub b_p_keymap: *mut char_u,
        pub b_p_gp: *mut char_u,
        pub b_p_mp: *mut char_u,
        pub b_p_efm: *mut char_u,
        pub b_p_ep: *mut char_u,
        pub b_p_path: *mut char_u,
        pub b_p_ar: libc::c_int,
        pub b_p_tags: *mut char_u,
        pub b_p_tc: *mut char_u,
        pub b_tc_flags: libc::c_uint,
        pub b_p_dict: *mut char_u,
        pub b_p_tsr: *mut char_u,
        pub b_p_ul: libc::c_long,
        pub b_p_udf: libc::c_int,
        pub b_p_lw: *mut char_u,
        pub b_ind_level: libc::c_int,
        pub b_ind_open_imag: libc::c_int,
        pub b_ind_no_brace: libc::c_int,
        pub b_ind_first_open: libc::c_int,
        pub b_ind_open_extra: libc::c_int,
        pub b_ind_close_extra: libc::c_int,
        pub b_ind_open_left_imag: libc::c_int,
        pub b_ind_jump_label: libc::c_int,
        pub b_ind_case: libc::c_int,
        pub b_ind_case_code: libc::c_int,
        pub b_ind_case_break: libc::c_int,
        pub b_ind_param: libc::c_int,
        pub b_ind_func_type: libc::c_int,
        pub b_ind_comment: libc::c_int,
        pub b_ind_in_comment: libc::c_int,
        pub b_ind_in_comment2: libc::c_int,
        pub b_ind_cpp_baseclass: libc::c_int,
        pub b_ind_continuation: libc::c_int,
        pub b_ind_unclosed: libc::c_int,
        pub b_ind_unclosed2: libc::c_int,
        pub b_ind_unclosed_noignore: libc::c_int,
        pub b_ind_unclosed_wrapped: libc::c_int,
        pub b_ind_unclosed_whiteok: libc::c_int,
        pub b_ind_matching_paren: libc::c_int,
        pub b_ind_paren_prev: libc::c_int,
        pub b_ind_maxparen: libc::c_int,
        pub b_ind_maxcomment: libc::c_int,
        pub b_ind_scopedecl: libc::c_int,
        pub b_ind_scopedecl_code: libc::c_int,
        pub b_ind_java: libc::c_int,
        pub b_ind_js: libc::c_int,
        pub b_ind_keep_case_label: libc::c_int,
        pub b_ind_hash_comment: libc::c_int,
        pub b_ind_cpp_namespace: libc::c_int,
        pub b_ind_if_for_while: libc::c_int,
        pub b_ind_cpp_extern_c: libc::c_int,
        pub b_no_eol_lnum: linenr_T,
        pub b_start_eol: libc::c_int,
        pub b_start_ffc: libc::c_int,
        pub b_start_fenc: *mut char_u,
        pub b_bad_char: libc::c_int,
        pub b_start_bomb: libc::c_int,
        pub b_bufvar: ScopeDictDictItem,
        pub b_vars: *mut dict_T,
        pub b_may_swap: bool,
        pub b_did_warn: bool,
        pub b_help: bool,
        pub b_spell: bool,
        pub b_prompt_text: *mut char_u,
        pub b_prompt_callback: Callback,
        pub b_prompt_interrupt: Callback,
        pub b_prompt_insert: libc::c_int,
        pub b_s: synblock_T,
        pub b_signlist: *mut signlist_T,
        pub b_signcols_max: libc::c_int,
        pub b_signcols: libc::c_int,
        pub terminal: *mut Terminal,
        pub additional_data: *mut dict_T,
        pub b_mapped_ctrl_c: libc::c_int,
        pub b_marktree: [MarkTree; 1],
        pub b_extmark_index: *mut Map_uint64_t_ExtmarkItem,
        pub b_extmark_ns: *mut Map_uint64_t_ExtmarkNs,
        pub update_channels: C2RustUnnamed_2,
        pub update_callbacks: C2RustUnnamed_1,
        pub update_need_codepoints: bool,
        pub deleted_bytes: size_t,
        pub deleted_codepoints: size_t,
        pub deleted_codeunits: size_t,
        pub flush_count: libc::c_int,
        pub b_luahl: bool,
        pub b_luahl_start: LuaRef,
        pub b_luahl_window: LuaRef,
        pub b_luahl_line: LuaRef,
        pub b_luahl_end: LuaRef,
        pub b_diff_failed: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "822:3"]
    pub struct C2RustUnnamed_1 {
        pub size: size_t,
        pub capacity: size_t,
        pub items: *mut BufUpdateCallbacks,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "459:9"]
    pub struct BufUpdateCallbacks {
        pub on_lines: LuaRef,
        pub on_bytes: LuaRef,
        pub on_changedtick: LuaRef,
        pub on_detach: LuaRef,
        pub utf_sizes: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "820:3"]
    pub struct C2RustUnnamed_2 {
        pub size: size_t,
        pub capacity: size_t,
        pub items: *mut uint64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "393:9"]
    pub struct synblock_T {
        pub b_keywtab: hashtab_T,
        pub b_keywtab_ic: hashtab_T,
        pub b_syn_error: libc::c_int,
        pub b_syn_slow: bool,
        pub b_syn_ic: libc::c_int,
        pub b_syn_spell: libc::c_int,
        pub b_syn_patterns: garray_T,
        pub b_syn_clusters: garray_T,
        pub b_spell_cluster_id: libc::c_int,
        pub b_nospell_cluster_id: libc::c_int,
        pub b_syn_containedin: libc::c_int,
        pub b_syn_sync_flags: libc::c_int,
        pub b_syn_sync_id: int16_t,
        pub b_syn_sync_minlines: libc::c_long,
        pub b_syn_sync_maxlines: libc::c_long,
        pub b_syn_sync_linebreaks: libc::c_long,
        pub b_syn_linecont_pat: *mut char_u,
        pub b_syn_linecont_prog: *mut regprog_T,
        pub b_syn_linecont_time: syn_time_T,
        pub b_syn_linecont_ic: libc::c_int,
        pub b_syn_topgrp: libc::c_int,
        pub b_syn_conceal: libc::c_int,
        pub b_syn_folditems: libc::c_int,
        pub b_sst_array: *mut synstate_T,
        pub b_sst_len: libc::c_int,
        pub b_sst_first: *mut synstate_T,
        pub b_sst_firstfree: *mut synstate_T,
        pub b_sst_freecount: libc::c_int,
        pub b_sst_check_lnum: linenr_T,
        pub b_sst_lasttick: disptick_T,
        pub b_langp: garray_T,
        pub b_spell_ismw: [bool; 256],
        pub b_spell_ismw_mb: *mut char_u,
        pub b_p_spc: *mut char_u,
        pub b_cap_prog: *mut regprog_T,
        pub b_p_spf: *mut char_u,
        pub b_p_spl: *mut char_u,
        pub b_cjk: libc::c_int,
        pub b_syn_chartab: [char_u; 32],
        pub b_syn_isk: *mut char_u,
    }
    #[c2rust::src_loc = "9:1"]
    pub type buf_T = file_buffer;
    #[c2rust::src_loc = "98:1"]
    pub type win_T = window_S;
    // internal diff failed for this buffer
    /*
 * Stuff for diff mode.
 */
    // up to four buffers can be diff'ed
    /*
 * Each diffblock defines where a block of lines starts in each of the buffers
 * and how many lines it occupies in that buffer.  When the lines are missing
 * in the buffer the df_count[] is zero.  This is all counted in
 * buffer lines.
 * There is always at least one unchanged line in between the diffs.
 * Otherwise it would have been included in the diff above or below it.
 * df_lnum[] + df_count[] is the lnum below the change.  When in one buffer
 * lines have been inserted, in the other buffer df_lnum[] is the line below
 * the insertion and df_count[] is zero.  When appending lines at the end of
 * the buffer, df_lnum[] is one beyond the end!
 * This is using a linked list, because the number of differences is expected
 * to be reasonable small.  The list is sorted on lnum.
 */
    /* line number in buffer */
    /* nr of inserted/changed lines */
    // / Tab pages point to the top frame of each tab page.
// / Note: Most values are NOT valid for the current tab page!  Use "curwin",
// / "firstwin", etc. for that.  "tp_topframe" is always valid and can be
// / compared against "topframe" to find the current tab page.
    // /< next tabpage or NULL
    // /< topframe for the windows
    // /< current window in this Tab page
    // /< previous window in this Tab page
    // /< first window in this Tab page
    // /< last window in this Tab page
    // /< Rows when Tab page was left
    // /< Columns when Tab page was left
    // /< value of 'cmdheight' when frame size
                                    // /< was set
    // /< list of diffs is outdated
    // /< update diffs before redrawing
    // /< window layout snapshots
    // /< Variable for "t:" Dictionary.
    // /< Internal variables, local to tab page.
    // /< Absolute path of local cwd or NULL.
    /*
 * Structure to cache info for displayed lines in w_lines[].
 * Each logical line has one entry.
 * The entry tells how the logical line is currently displayed in the window.
 * This is updated when displaying the window.
 * When the display is changed (e.g., when clearing the screen) w_lines_valid
 * is changed to exclude invalid entries.
 * When making changes to the buffer, wl_valid is reset to indicate wl_size
 * may not reflect what is actually in the buffer.  When wl_valid is FALSE,
 * the entries can only be used to count the number of displayed lines used.
 * wl_lnum and wl_lastlnum are invalid too.
 */
    /* buffer line number for logical line */
    /* height in screen lines */
    /* TRUE values are valid for text in buffer */
    /* TRUE when this is a range of folded lines */
    /* last buffer line number for logical line */
    /*
 * Windows are kept in a tree of frames.  Each frame has a column (FR_COL)
 * or row (FR_ROW) layout or is a leaf, which has a window.
 */
    // FR_LEAF, FR_COL or FR_ROW
    // new width used in win_equal_rec()
    // new height used in win_equal_rec()
    // containing frame or NULL
    // frame right or below in same parent, NULL
                                // for last
    // frame left or above in same parent, NULL
                                // for first
    // fr_child and fr_win are mutually exclusive
    // first contained frame
    // window that fills this frame
    /* frame is a leaf */
    /* frame with a row of windows */
    /* frame with a column of windows */
    /*
 * Struct used for highlighting 'hlsearch' matches, matches defined by
 * ":match" and matches defined by match functions.
 * For 'hlsearch' there is one pattern for all windows.  For ":match" and the
 * match functions there is a different pattern for each window.
 */
    // points to the regexp program; contains last found
                        // match (may continue in next line)
    // the buffer to search for a match
    // the line to search for a match
    // attributes to be used for a match
    // attributes currently active in win_line()
    // first lnum to search for multi-line pat
    // in win_line() points to char where HL starts
    // in win_line() points to char where HL ends
    // position specified directly by matchaddpos()
    // for a time limit
    // / number of positions supported by matchaddpos()
    // / Same as lpos_T, but with additional field len.
    // /< line number
    // /< column number
    // /< length: 0 - to the end of line
    // / posmatch_T provides an array for storing match items for matchaddpos()
// / function.
    // /< array of positions
    // /< internal position counter
    // /< top buffer line
    // /< bottom buffer line
    /*
 * matchitem_T provides a linked list for storing match items for ":match" and
 * the match functions.
 */
    // /< match ID
    // /< match priority
    // /< pattern to highlight
    // /< highlight group ID
    // /< regexp program for pattern
    // /< position matches
    // /< struct for doing the actual highlighting
    // /< cchar for Conceal highlighting
    // NW -> 0
// NE -> kFloatAnchorEast
// SW -> kFloatAnchorSouth
// SE -> kFloatAnchorSouth | kFloatAnchorEast
    // / Minimal UI: no number column, eob markers, etc
    // Structure to store last cursor position and topline.  Used by check_lnums()
// and reset_lnums().
    // original topline value
    // corrected topline value
    // original cursor position
    // corrected cursor position
    // / Structure which contains all information that belongs to a window.
// /
// / All row numbers are relative to the start of the window, except w_winrow.
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1067:8"]
    pub struct window_S {
        pub handle: handle_T,
        pub w_buffer: *mut buf_T,
        pub w_s: *mut synblock_T,
        pub w_hl_id_normal: libc::c_int,
        pub w_hl_attr_normal: libc::c_int,
        pub w_hl_ids: [libc::c_int; 50],
        pub w_hl_attrs: [libc::c_int; 50],
        pub w_hl_needs_update: libc::c_int,
        pub w_prev: *mut win_T,
        pub w_next: *mut win_T,
        pub w_closing: bool,
        pub w_frame: *mut frame_T,
        pub w_cursor: pos_T,
        pub w_curswant: colnr_T,
        pub w_set_curswant: libc::c_int,
        pub w_last_cursorline: linenr_T,
        pub w_last_cursormoved: pos_T,
        pub w_old_visual_mode: libc::c_char,
        pub w_old_cursor_lnum: linenr_T,
        pub w_old_cursor_fcol: colnr_T,
        pub w_old_cursor_lcol: colnr_T,
        pub w_old_visual_lnum: linenr_T,
        pub w_old_visual_col: colnr_T,
        pub w_old_curswant: colnr_T,
        pub w_p_lcs_chars: C2RustUnnamed_4,
        pub w_p_fcs_chars: C2RustUnnamed_3,
        pub w_topline: linenr_T,
        pub w_topline_was_set: libc::c_char,
        pub w_topfill: libc::c_int,
        pub w_old_topfill: libc::c_int,
        pub w_botfill: bool,
        pub w_old_botfill: bool,
        pub w_leftcol: colnr_T,
        pub w_skipcol: colnr_T,
        pub w_winrow: libc::c_int,
        pub w_height: libc::c_int,
        pub w_status_height: libc::c_int,
        pub w_wincol: libc::c_int,
        pub w_width: libc::c_int,
        pub w_vsep_width: libc::c_int,
        pub w_save_cursor: pos_save_T,
        pub w_height_inner: libc::c_int,
        pub w_width_inner: libc::c_int,
        pub w_height_request: libc::c_int,
        pub w_width_request: libc::c_int,
        pub w_valid: libc::c_int,
        pub w_valid_cursor: pos_T,
        pub w_valid_leftcol: colnr_T,
        pub w_cline_height: libc::c_int,
        pub w_cline_folded: bool,
        pub w_cline_row: libc::c_int,
        pub w_virtcol: colnr_T,
        pub w_wrow: libc::c_int,
        pub w_wcol: libc::c_int,
        pub w_botline: linenr_T,
        pub w_empty_rows: libc::c_int,
        pub w_filler_rows: libc::c_int,
        pub w_lines_valid: libc::c_int,
        pub w_lines: *mut wline_T,
        pub w_folds: garray_T,
        pub w_fold_manual: bool,
        pub w_foldinvalid: bool,
        pub w_nrwidth: libc::c_int,
        pub w_redr_type: libc::c_int,
        pub w_upd_rows: libc::c_int,
        pub w_redraw_top: linenr_T,
        pub w_redraw_bot: linenr_T,
        pub w_redr_status: libc::c_int,
        pub w_ru_cursor: pos_T,
        pub w_ru_virtcol: colnr_T,
        pub w_ru_topline: linenr_T,
        pub w_ru_line_count: linenr_T,
        pub w_ru_topfill: libc::c_int,
        pub w_ru_empty: libc::c_char,
        pub w_alt_fnum: libc::c_int,
        pub w_alist: *mut alist_T,
        pub w_arg_idx: libc::c_int,
        pub w_arg_idx_invalid: libc::c_int,
        pub w_localdir: *mut char_u,
        pub w_onebuf_opt: winopt_T,
        pub w_allbuf_opt: winopt_T,
        pub w_p_stl_flags: uint32_t,
        pub w_p_fde_flags: uint32_t,
        pub w_p_fdt_flags: uint32_t,
        pub w_p_cc_cols: *mut libc::c_int,
        pub w_p_brimin: libc::c_int,
        pub w_p_brishift: libc::c_int,
        pub w_p_brisbr: bool,
        pub w_p_siso: libc::c_long,
        pub w_p_so: libc::c_long,
        pub w_scbind_pos: libc::c_long,
        pub w_winvar: ScopeDictDictItem,
        pub w_vars: *mut dict_T,
        pub w_pcmark: pos_T,
        pub w_prev_pcmark: pos_T,
        pub w_jumplist: [xfmark_T; 100],
        pub w_jumplistlen: libc::c_int,
        pub w_jumplistidx: libc::c_int,
        pub w_changelistidx: libc::c_int,
        pub w_match_head: *mut matchitem_T,
        pub w_next_match_id: libc::c_int,
        pub w_tagstack: [taggy_T; 20],
        pub w_tagstackidx: libc::c_int,
        pub w_tagstacklen: libc::c_int,
        pub w_grid: ScreenGrid,
        pub w_pos_changed: bool,
        pub w_floating: bool,
        pub w_float_config: FloatConfig,
        pub w_fraction: libc::c_int,
        pub w_prev_fraction_row: libc::c_int,
        pub w_nrwidth_line_count: linenr_T,
        pub w_nrwidth_width: libc::c_int,
        pub w_llist: *mut qf_info_T,
        pub w_llist_ref: *mut qf_info_T,
    }
    #[c2rust::src_loc = "377:1"]
    pub type qf_info_T = qf_info_S;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1035:9"]
    pub struct FloatConfig {
        pub window: Window,
        pub bufpos: lpos_T,
        pub height: libc::c_int,
        pub width: libc::c_int,
        pub row: libc::c_double,
        pub col: libc::c_double,
        pub anchor: FloatAnchor,
        pub relative: FloatRelative,
        pub external: bool,
        pub focusable: bool,
        pub style: WinStyle,
    }
    #[c2rust::src_loc = "1030:9"]
    pub type WinStyle = libc::c_uint;
    #[c2rust::src_loc = "1032:3"]
    pub const kWinStyleMinimal: WinStyle = 1;
    #[c2rust::src_loc = "1031:3"]
    pub const kWinStyleUnused: WinStyle = 0;
    #[c2rust::src_loc = "1021:9"]
    pub type FloatRelative = libc::c_uint;
    #[c2rust::src_loc = "1024:3"]
    pub const kFloatRelativeCursor: FloatRelative = 2;
    #[c2rust::src_loc = "1023:3"]
    pub const kFloatRelativeWindow: FloatRelative = 1;
    #[c2rust::src_loc = "1022:3"]
    pub const kFloatRelativeEditor: FloatRelative = 0;
    #[c2rust::src_loc = "1008:1"]
    pub type FloatAnchor = libc::c_int;
    #[c2rust::src_loc = "121:1"]
    pub type taggy_T = taggy;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "121:16"]
    pub struct taggy {
        pub tagname: *mut char_u,
        pub fmark: fmark_T,
        pub cur_match: libc::c_int,
        pub cur_fnum: libc::c_int,
        pub user_data: *mut char_u,
    }
    #[c2rust::src_loc = "995:1"]
    pub type matchitem_T = matchitem;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "996:8"]
    pub struct matchitem {
        pub next: *mut matchitem_T,
        pub id: libc::c_int,
        pub priority: libc::c_int,
        pub pattern: *mut char_u,
        pub hlg_id: libc::c_int,
        pub match_0: regmmatch_T,
        pub pos: posmatch_T,
        pub hl: match_T,
        pub conceal_char: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "955:9"]
    pub struct match_T {
        pub rm: regmmatch_T,
        pub buf: *mut buf_T,
        pub lnum: linenr_T,
        pub attr: libc::c_int,
        pub attr_cur: libc::c_int,
        pub first_lnum: linenr_T,
        pub startcol: colnr_T,
        pub endcol: colnr_T,
        pub is_addpos: bool,
        pub tm: proftime_T,
    }
    #[c2rust::src_loc = "982:1"]
    pub type posmatch_T = posmatch;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "983:8"]
    pub struct posmatch {
        pub pos: [llpos_T; 8],
        pub cur: libc::c_int,
        pub toplnum: linenr_T,
        pub botlnum: linenr_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "973:9"]
    pub struct llpos_T {
        pub lnum: linenr_T,
        pub col: colnr_T,
        pub len: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "161:9"]
    pub struct winopt_T {
        pub wo_arab: libc::c_int,
        pub wo_bri: libc::c_int,
        pub wo_briopt: *mut char_u,
        pub wo_diff: libc::c_int,
        pub wo_fdc: *mut char_u,
        pub wo_fdc_save: *mut char_u,
        pub wo_fen: libc::c_int,
        pub wo_fen_save: libc::c_int,
        pub wo_fdi: *mut char_u,
        pub wo_fdl: libc::c_long,
        pub wo_fdl_save: libc::c_int,
        pub wo_fdm: *mut char_u,
        pub wo_fdm_save: *mut char_u,
        pub wo_fml: libc::c_long,
        pub wo_fdn: libc::c_long,
        pub wo_fde: *mut char_u,
        pub wo_fdt: *mut char_u,
        pub wo_fmr: *mut char_u,
        pub wo_lbr: libc::c_int,
        pub wo_list: libc::c_int,
        pub wo_nu: libc::c_int,
        pub wo_rnu: libc::c_int,
        pub wo_nuw: libc::c_long,
        pub wo_wfh: libc::c_int,
        pub wo_wfw: libc::c_int,
        pub wo_pvw: libc::c_int,
        pub wo_rl: libc::c_int,
        pub wo_rlc: *mut char_u,
        pub wo_scr: libc::c_long,
        pub wo_spell: libc::c_int,
        pub wo_cuc: libc::c_int,
        pub wo_cul: libc::c_int,
        pub wo_cc: *mut char_u,
        pub wo_stl: *mut char_u,
        pub wo_scb: libc::c_int,
        pub wo_diff_saved: libc::c_int,
        pub wo_scb_save: libc::c_int,
        pub wo_wrap: libc::c_int,
        pub wo_wrap_save: libc::c_int,
        pub wo_cocu: *mut char_u,
        pub wo_cole: libc::c_long,
        pub wo_crb: libc::c_int,
        pub wo_crb_save: libc::c_int,
        pub wo_scl: *mut char_u,
        pub wo_winhl: *mut char_u,
        pub wo_fcs: *mut char_u,
        pub wo_lcs: *mut char_u,
        pub wo_winbl: libc::c_long,
        pub wo_script_ctx: [LastSet; 42],
    }
    #[c2rust::src_loc = "290:1"]
    pub type alist_T = arglist;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "290:16"]
    pub struct arglist {
        pub al_ga: garray_T,
        pub al_refcount: libc::c_int,
        pub id: libc::c_int,
    }
    #[c2rust::src_loc = "917:1"]
    pub type wline_T = w_line;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "917:16"]
    pub struct w_line {
        pub wl_lnum: linenr_T,
        pub wl_size: uint16_t,
        pub wl_valid: libc::c_char,
        pub wl_folded: libc::c_char,
        pub wl_lastlnum: linenr_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1056:9"]
    pub struct pos_save_T {
        pub w_topline_save: libc::c_int,
        pub w_topline_corr: libc::c_int,
        pub w_cursor_save: pos_T,
        pub w_cursor_corr: pos_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1127:3"]
    pub struct C2RustUnnamed_3 {
        pub stl: libc::c_int,
        pub stlnc: libc::c_int,
        pub vert: libc::c_int,
        pub fold: libc::c_int,
        pub foldopen: libc::c_int,
        pub foldclosed: libc::c_int,
        pub foldsep: libc::c_int,
        pub diff: libc::c_int,
        pub msgsep: libc::c_int,
        pub eob: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1113:3"]
    pub struct C2RustUnnamed_4 {
        pub eol: libc::c_int,
        pub ext: libc::c_int,
        pub prec: libc::c_int,
        pub nbsp: libc::c_int,
        pub space: libc::c_int,
        pub tab1: libc::c_int,
        pub tab2: libc::c_int,
        pub tab3: libc::c_int,
        pub trail: libc::c_int,
        pub conceal: libc::c_int,
    }
    #[c2rust::src_loc = "100:1"]
    pub type frame_T = frame_S;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "929:8"]
    pub struct frame_S {
        pub fr_layout: libc::c_char,
        pub fr_width: libc::c_int,
        pub fr_newwidth: libc::c_int,
        pub fr_height: libc::c_int,
        pub fr_newheight: libc::c_int,
        pub fr_parent: *mut frame_T,
        pub fr_next: *mut frame_T,
        pub fr_prev: *mut frame_T,
        pub fr_child: *mut frame_T,
        pub fr_win: *mut win_T,
    }
    #[c2rust::src_loc = "101:1"]
    pub type disptick_T = uint16_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "382:9"]
    pub struct syn_time_T {
        pub total: proftime_T,
        pub slowest: proftime_T,
        pub count: libc::c_long,
        pub match_0: libc::c_long,
    }
    #[c2rust::src_loc = "346:1"]
    pub type mapblock_T = mapblock;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "347:8"]
    pub struct mapblock {
        pub m_next: *mut mapblock_T,
        pub m_keys: *mut char_u,
        pub m_str: *mut char_u,
        pub m_orig_str: *mut char_u,
        pub m_keylen: libc::c_int,
        pub m_mode: libc::c_int,
        pub m_noremap: libc::c_int,
        pub m_silent: libc::c_char,
        pub m_nowait: libc::c_char,
        pub m_expr: libc::c_char,
        pub m_script_ctx: sctx_T,
    }
    #[c2rust::src_loc = "99:1"]
    pub type wininfo_T = wininfo_S;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "273:8"]
    pub struct wininfo_S {
        pub wi_next: *mut wininfo_T,
        pub wi_prev: *mut wininfo_T,
        pub wi_win: *mut win_T,
        pub wi_fpos: pos_T,
        pub wi_optset: bool,
        pub wi_opt: winopt_T,
        pub wi_fold_manual: bool,
        pub wi_folds: garray_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "457:9"]
    pub struct ChangedtickDictItem {
        pub di_tv: typval_T,
        pub di_flags: uint8_t,
        pub di_key: [char_u; 12],
    }
    #[inline]
    #[c2rust::src_loc = "1356:1"]
    pub unsafe extern "C" fn win_hl_attr(mut wp: *mut win_T,
                                         mut hlf: libc::c_int)
     -> libc::c_int {
        return (*wp).w_hl_attrs[hlf as usize];
    }
    use super::nvim_types_h::{handle_T, char_u, LuaRef};
    use super::memline_defs_h::memline_T;
    use super::fs_defs_h::FileID;
    use super::typval_h::{varnumber_T, ScopeDictDictItem, dict_T, Callback,
                          sctx_T, typval_T};
    use super::pos_h::{linenr_T, pos_T, colnr_T, lpos_T};
    use super::stdint_uintn_h::{uint64_t, uint32_t, uint16_t, uint8_t};
    use super::mark_defs_h::{fmark_T, xfmark_T};
    use super::undo_defs_h::{visualinfo_T, u_header_T};
    use super::garray_h::garray_T;
    use super::time_t_h::time_t;
    use super::option_defs_h::LastSet;
    use super::sign_defs_h::signlist_T;
    use super::terminal_h::Terminal;
    use super::marktree_h::MarkTree;
    use super::map_h::{Map_uint64_t_ExtmarkItem, Map_uint64_t_ExtmarkNs};
    use super::stddef_h::size_t;
    use super::hashtab_h::hashtab_T;
    use super::stdint_intn_h::int16_t;
    use super::regexp_defs_h::{regprog_T, regmmatch_T};
    use super::syntax_defs_h::synstate_T;
    use super::grid_defs_h::ScreenGrid;
    use super::defs_h::Window;
    use super::profile_h::proftime_T;
    extern "C" {
        #[c2rust::src_loc = "377:16"]
        pub type qf_info_S;
    }
    // NVIM_BUFFER_DEFS_H
    // / Macros defined in Vim, but not in Neovim
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/map.h:12"]
pub mod map_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:1"]
    pub struct Map_uint64_t_ExtmarkNs {
        pub table: *mut kh_uint64_t_ExtmarkNs_map_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:1"]
    pub struct kh_uint64_t_ExtmarkNs_map_t {
        pub n_buckets: khint_t,
        pub size: khint_t,
        pub n_occupied: khint_t,
        pub upper_bound: khint_t,
        pub flags: *mut khint32_t,
        pub keys: *mut uint64_t,
        pub vals: *mut ExtmarkNs,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:16"]
    pub struct ExtmarkNs {
        pub map: *mut Map_uint64_t_uint64_t,
        pub free_id: uint64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:1"]
    pub struct Map_uint64_t_uint64_t {
        pub table: *mut kh_uint64_t_uint64_t_map_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:1"]
    pub struct kh_uint64_t_uint64_t_map_t {
        pub n_buckets: khint_t,
        pub size: khint_t,
        pub n_occupied: khint_t,
        pub upper_bound: khint_t,
        pub flags: *mut khint32_t,
        pub keys: *mut uint64_t,
        pub vals: *mut uint64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:1"]
    pub struct Map_uint64_t_ExtmarkItem {
        pub table: *mut kh_uint64_t_ExtmarkItem_map_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:1"]
    pub struct kh_uint64_t_ExtmarkItem_map_t {
        pub n_buckets: khint_t,
        pub size: khint_t,
        pub n_occupied: khint_t,
        pub upper_bound: khint_t,
        pub flags: *mut khint32_t,
        pub keys: *mut uint64_t,
        pub vals: *mut ExtmarkItem,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:1"]
    pub struct Map_uint64_t_ptr_t {
        pub table: *mut kh_uint64_t_ptr_t_map_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:1"]
    pub struct kh_uint64_t_ptr_t_map_t {
        pub n_buckets: khint_t,
        pub size: khint_t,
        pub n_occupied: khint_t,
        pub upper_bound: khint_t,
        pub flags: *mut khint32_t,
        pub keys: *mut uint64_t,
        pub vals: *mut ptr_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:1"]
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
    #[c2rust::src_loc = "38:1"]
    pub struct Map_cstr_t_ptr_t {
        pub table: *mut kh_cstr_t_ptr_t_map_t,
    }
    use super::khash_h::{khint_t, khint32_t};
    use super::stdint_uintn_h::uint64_t;
    use super::extmark_defs_h::ExtmarkItem;
    use super::map_defs_h::{ptr_t, cstr_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "40:21"]
        pub fn map_uint64_t_ptr_t_get(map: *mut Map_uint64_t_ptr_t,
                                      key: uint64_t) -> ptr_t;
    }
    // NVIM_MAP_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/lib/khash.h:12"]
pub mod khash_h {
    /* The MIT License

   Copyright (c) 2008, 2009, 2011 by Attractive Chaos <attractor@live.co.uk>

   Permission is hereby granted, free of charge, to any person obtaining
   a copy of this software and associated documentation files (the
   "Software"), to deal in the Software without restriction, including
   without limitation the rights to use, copy, modify, merge, publish,
   distribute, sublicense, and/or sell copies of the Software, and to
   permit persons to whom the Software is furnished to do so, subject to
   the following conditions:

   The above copyright notice and this permission notice shall be
   included in all copies or substantial portions of the Software.

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
   EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
   MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
   NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
   BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
   ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
   CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
   SOFTWARE.
*/
    /*
  Example:

#include "nvim/khash.h"
KHASH_MAP_INIT_INT(32, char)
int main() {
    int ret, is_missing;
    khiter_t k;
    khash_t(32) *h = kh_init(32);
    k = kh_put(32, h, 5, &ret);
    kh_value(h, k) = 10;
    k = kh_get(32, h, 10);
    is_missing = (k == kh_end(h));
    k = kh_get(32, h, 5);
    kh_del(32, h, k);
    for (k = kh_begin(h); k != kh_end(h); ++k)
        if (kh_exist(h, k)) kh_value(h, k) = 1;
    kh_destroy(32, h);
    return 0;
}
*/
    /*
  2013-05-02 (0.2.8):

	* Use quadratic probing. When the capacity is power of 2, stepping function
	  i*(i+1)/2 guarantees to traverse each bucket. It is better than double
	  hashing on cache performance and is more robust than linear probing.

	  In theory, double hashing should be more robust than quadratic probing.
	  However, my implementation is probably not for large hash tables, because
	  the second hash function is closely tied to the first hash function,
	  which reduce the effectiveness of double hashing.

	Reference: http://research.cs.vt.edu/AVresearch/hashing/quadratic.php

  2011-12-29 (0.2.7):

    * Minor code clean up; no actual effect.

  2011-09-16 (0.2.6):

	* The capacity is a power of 2. This seems to dramatically improve the
	  speed for simple keys. Thank Zilong Tan for the suggestion. Reference:

	   - http://code.google.com/p/ulib/
	   - http://nothings.org/computer/judy/

	* Allow to optionally use linear probing which usually has better
	  performance for random input. Double hashing is still the default as it
	  is more robust to certain non-random input.

	* Added Wang's integer hash function (not used by default). This hash
	  function is more robust to certain non-random input.

  2011-02-14 (0.2.5):

    * Allow to declare global functions.

  2009-09-26 (0.2.4):

    * Improve portability

  2008-09-19 (0.2.3):

	* Corrected the example
	* Improved interfaces

  2008-09-11 (0.2.2):

	* Improved speed a little in kh_put()

  2008-09-10 (0.2.1):

	* Added kh_clear()
	* Fixed a compiling error

  2008-09-02 (0.2.0):

	* Changed to token concatenation which increases flexibility.

  2008-08-31 (0.1.2):

	* Fixed a bug in kh_get(), which has not been tested previously.

  2008-08-31 (0.1.1):

	* Added destructor
*/
    /* !
  @header

  Generic hash table library.
 */
    /* compiler specific configuration */
    #[c2rust::src_loc = "140:1"]
    pub type khint32_t = libc::c_uint;
    #[c2rust::src_loc = "157:1"]
    pub type khint_t = khint32_t;
    /* This function uses 0.25*n_buckets bytes of working space instead of */
    /* [sizeof(key_t+val_t)+.25]*n_buckets. */
    /* requested size is too small */
    /* hash table size to be changed (shrink or expand); rehash */
    /* expand */
    /* otherwise shrink */
    /* rehashing is needed */
    /* kick-out process; sort of like in Cuckoo hashing */
    /* kick out the existing element */
    /* mark it as deleted in the old hash table */
    /* write the element and jump out of the loop */
    /* shrink the hash table */
    /* free the working space */
    /* update the hash table */
    /* clear "deleted" elements */
    /* expand the hash table */
    /* TODO: implement automatically shrinking; */
    /* resize() already support shrinking */
    /* for speed up */
    /* not present at all */
    /* deleted */
    /* Don't touch h->keys[x] if present and not deleted */
    /* --- BEGIN OF HASH FUNCTIONS --- */
    /* ! @function
  @abstract     Integer hash function
  @param  key   The integer [khint32_t]
  @return       The hash value [khint_t]
 */
    /* ! @function
  @abstract     Integer comparison function
 */
    /* ! @function
  @abstract     64-bit integer hash function
  @param  key   The integer [khint64_t]
  @return       The hash value [khint_t]
 */
    /* ! @function
  @abstract     64-bit integer comparison function
 */
    /* ! @function
  @abstract     const char* hash function
  @param  s     Pointer to a null terminated string
  @return       The hash value
 */
    #[inline]
    #[c2rust::src_loc = "459:1"]
    pub unsafe extern "C" fn __ac_X31_hash_string(mut s: *const libc::c_char)
     -> khint_t {
        let mut h = *s as khint_t;
        if h != 0 {
            s = s.offset(1);
            while *s != 0 {
                h =
                    (h <<
                         5 as
                             libc::c_int).wrapping_sub(h).wrapping_add(*s as
                                                                           uint8_t
                                                                           as
                                                                           libc::c_uint);
                s = s.offset(1)
            }
        }
        return h;
    }
    /* ! @function
  @abstract     Another interface to const char* hash function
  @param  key   Pointer to a null terminated string [const char*]
  @return       The hash value [khint_t]
 */
    /* ! @function
  @abstract     Const char* comparison function
 */
    #[inline]
    #[c2rust::src_loc = "476:1"]
    pub unsafe extern "C" fn __ac_Wang_hash(mut key: khint_t) -> khint_t {
        key =
            (key as libc::c_uint).wrapping_add(!(key << 15 as libc::c_int)) as
                khint_t as khint_t;
        key ^= key >> 10 as libc::c_int;
        key =
            (key as libc::c_uint).wrapping_add(key << 3 as libc::c_int) as
                khint_t as khint_t;
        key ^= key >> 6 as libc::c_int;
        key =
            (key as libc::c_uint).wrapping_add(!(key << 11 as libc::c_int)) as
                khint_t as khint_t;
        key ^= key >> 16 as libc::c_int;
        return key;
    }
    use super::stdint_uintn_h::uint8_t;
    // NVIM_LIB_KHASH_H
    /* ! @function
  @abstract     Return a literal for an empty hash table.
  @param  name  Name of the hash table [symbol]
 */
    /* ! @function
  @abstract     Instantiate a hash map containing const char* keys
  @param  name  Name of the hash table [symbol]
  @param  khval_t  Type of values [type]
 */
    /* ! @function
  @abstract     Instantiate a hash map containing const char* keys
  @param  name  Name of the hash table [symbol]
 */
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/extmark_defs.h:12"]
pub mod extmark_defs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:9"]
    pub struct ExtmarkItem {
        pub ns_id: uint64_t,
        pub mark_id: uint64_t,
        pub hl_id: libc::c_int,
        pub virt_text: VirtText,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "12:9"]
    pub struct VirtText {
        pub size: size_t,
        pub capacity: size_t,
        pub items: *mut VirtTextChunk,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "7:9"]
    pub struct VirtTextChunk {
        pub text: *mut libc::c_char,
        pub hl_id: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:9"]
    pub struct extmark_undo_vec_t {
        pub size: size_t,
        pub capacity: size_t,
        pub items: *mut ExtmarkUndoObject,
    }
    #[c2rust::src_loc = "25:1"]
    pub type ExtmarkUndoObject = undo_object;
    use super::stdint_uintn_h::uint64_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "25:16"]
        pub type undo_object;
    }
    // NVIM_EXTMARK_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/marktree.h:12"]
pub mod marktree_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "59:9"]
    pub struct MarkTree {
        pub root: *mut mtnode_t,
        pub n_keys: size_t,
        pub n_nodes: size_t,
        pub next_id: uint64_t,
        pub id2node: *mut Map_uint64_t_ptr_t,
    }
    #[c2rust::src_loc = "23:1"]
    pub type mtnode_t = mtnode_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:8"]
    pub struct mtnode_s {
        pub n: int32_t,
        pub level: int32_t,
        pub parent: *mut mtnode_t,
        pub key: [mtkey_t; 19],
        pub ptr: [*mut mtnode_t; 0],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:9"]
    pub struct mtkey_t {
        pub pos: mtpos_t,
        pub id: uint64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:9"]
    pub struct mtpos_t {
        pub row: int32_t,
        pub col: int32_t,
    }
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint64_t;
    use super::map_h::Map_uint64_t_ptr_t;
    use super::stdint_intn_h::int32_t;
    // NVIM_MARKTREE_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/map_defs.h:12"]
pub mod map_defs_h {
    #[c2rust::src_loc = "7:1"]
    pub type ptr_t = *mut libc::c_void;
    #[c2rust::src_loc = "6:1"]
    pub type cstr_t = *const libc::c_char;
    // NVIM_MAP_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/terminal.h:12"]
pub mod terminal_h {
    #[c2rust::src_loc = "8:1"]
    pub type Terminal = terminal;
    extern "C" {
        #[c2rust::src_loc = "8:16"]
        pub type terminal;
    }
    // NVIM_TERMINAL_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/sign_defs.h:12"]
pub mod sign_defs_h {
    // signs: line annotations
    // Sign group
    // number of signs in this group
    // next sign id for this group
    // sign group name
    // Macros to get the sign group structure from the group name
    #[c2rust::src_loc = "21:1"]
    pub type signlist_T = signlist;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "23:8"]
    pub struct signlist {
        pub id: libc::c_int,
        pub lnum: linenr_T,
        pub typenr: libc::c_int,
        pub group: *mut signgroup_T,
        pub priority: libc::c_int,
        pub next: *mut signlist_T,
        pub prev: *mut signlist_T,
    }
    #[c2rust::src_loc = "10:1"]
    pub type signgroup_T = signgroup_S;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:16"]
    pub struct signgroup_S {
        pub refcount: uint16_t,
        pub next_sign_id: libc::c_int,
        pub sg_name: [char_u; 1],
    }
    use super::pos_h::linenr_T;
    use super::stdint_uintn_h::uint16_t;
    use super::nvim_types_h::char_u;
    // NVIM_SIGN_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/regexp_defs.h:12"]
pub mod regexp_defs_h {
    /*
 * NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE
 *
 * This is NOT the original regular expression code as written by Henry
 * Spencer.  This code has been modified specifically for use with Vim, and
 * should not be used apart from compiling Vim.  If you want a good regular
 * expression library, get the original code.
 *
 * NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE
 */
    /*
 * The number of sub-matches is limited to 10.
 * The first one (index 0) is the whole match, referenced with "\0".
 * The second one (index 1) is the first sub-match, referenced with "\1".
 * This goes up to the tenth (index 9), referenced with "\9".
 */
    /*
 * In the NFA engine: how many braces are allowed.
 * TODO(RE): Use dynamic memory allocation instead of static, like here
 */
    // In the NFA engine: how many states are allowed.
    // Which regexp engine to use? Needed for vim_regcomp().
// Must match with 'regexpengine'.
    #[c2rust::src_loc = "46:1"]
    pub type regprog_T = regprog;
    // / Structure to be used for multi-line matching.
// / Sub-match "no" starts in line "startpos[no].lnum" column "startpos[no].col"
// / and ends in line "endpos[no].lnum" just before column "endpos[no].col".
// / The line numbers are relative to the first line, thus startpos[0].lnum is
// / always 0.
// / When there is no match, the line number is -1.
    // / when not zero: maximum column
    /*
 * Structure returned by vim_regcomp() to pass on to vim_regexec().
 * This is the general structure. For the actual matcher, two specific
 * structures are used. See code below.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:8"]
    pub struct regprog {
        pub engine: *mut regengine_T,
        pub regflags: libc::c_uint,
        pub re_engine: libc::c_uint,
        pub re_flags: libc::c_uint,
    }
    #[c2rust::src_loc = "45:1"]
    pub type regengine_T = regengine;
    // /< Second argument for vim_regcomp().
    /*
 * Structure used by the back track matcher.
 * These fields are only to be used in regexp.c!
 * See regexp.c for an explanation.
 */
    // These four members implement regprog_T.
    // /< Second argument for vim_regcomp().
    /* actually longer.. */
    // Structure representing a NFA state.
// An NFA state may have no outgoing edge, when it is a NFA_MATCH state.
    /* 0: normal, 1: recursive */
    /*
 * Structure used by the NFA matcher.
 */
    // These four members implement regprog_T.
    // /< Second argument for vim_regcomp().
    /* points into state[] */
    /* pattern starts with ^ */
    /* char at start of pattern */
    /* plain text to match with */
    /* pattern contains \ze */
    /* pattern contains \1 .. \9 */
    /* number of () */
    /* actually longer.. */
    /*
 * Structure to be used for single-line matching.
 * Sub-match "no" starts at "startp[no]" and ends just before "endp[no]".
 * When there is no match, the pointer is NULL.
 */
    /*
 * Structure used to store external references: "\z\(\)" to "\z\1".
 * Use a reference count to avoid the need to copy this around.  When it goes
 * from 1 to zero the matches need to be freed.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "156:8"]
    pub struct regengine {
        pub regcomp: Option<unsafe extern "C" fn(_: *mut char_u,
                                                 _: libc::c_int)
                                -> *mut regprog_T>,
        pub regfree: Option<unsafe extern "C" fn(_: *mut regprog_T) -> ()>,
        pub regexec_nl: Option<unsafe extern "C" fn(_: *mut regmatch_T,
                                                    _: *mut char_u,
                                                    _: colnr_T, _: bool)
                                   -> libc::c_int>,
        pub regexec_multi: Option<unsafe extern "C" fn(_: *mut regmmatch_T,
                                                       _: *mut win_T,
                                                       _: *mut buf_T,
                                                       _: linenr_T,
                                                       _: colnr_T,
                                                       _: *mut proftime_T,
                                                       _: *mut libc::c_int)
                                      -> libc::c_long>,
        pub expr: *mut char_u,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:9"]
    pub struct regmmatch_T {
        pub regprog: *mut regprog_T,
        pub startpos: [lpos_T; 10],
        pub endpos: [lpos_T; 10],
        pub rmm_ic: libc::c_int,
        pub rmm_maxcol: colnr_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "139:9"]
    pub struct regmatch_T {
        pub regprog: *mut regprog_T,
        pub startp: [*mut char_u; 10],
        pub endp: [*mut char_u; 10],
        pub rm_ic: bool,
    }
    #[c2rust::src_loc = "47:1"]
    pub type reg_extmatch_T = reg_extmatch;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "151:8"]
    pub struct reg_extmatch {
        pub refcnt: int16_t,
        pub matches: [*mut char_u; 10],
    }
    use super::nvim_types_h::char_u;
    use super::pos_h::{colnr_T, linenr_T, lpos_T};
    use super::buffer_defs_h::{win_T, buf_T};
    use super::profile_h::proftime_T;
    use super::stdint_intn_h::int16_t;
    // NVIM_REGEXP_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/api/private/defs.h:12"]
pub mod defs_h {
    // Basic types
    // Per msgpack-rpc spec.
    // / Mask for all internal calls
    // / Internal call from VimL code
    // / Internal call from lua code
    // / Check whether call is internal
// /
// / @param[in]  channel_id  Channel id.
// /
// / @return true if channel_id refers to internal channel.
    // / Maximum value of an Integer
    // / Minimum value of an Integer
    #[c2rust::src_loc = "82:1"]
    pub type Window = handle_T;
    #[c2rust::src_loc = "66:1"]
    pub type Boolean = bool;
    #[c2rust::src_loc = "67:1"]
    pub type Integer = int64_t;
    #[c2rust::src_loc = "68:1"]
    pub type Float = libc::c_double;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "76:9"]
    pub struct String_0 {
        pub data: *mut libc::c_char,
        pub size: size_t,
    }
    // EXT types, cannot be split or reordered, see #EXT_OBJECT_TYPE_SHIFT
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "114:8"]
    pub struct object {
        pub type_0: ObjectType,
        pub data: C2RustUnnamed_10,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:3"]
    pub union C2RustUnnamed_10 {
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
    #[c2rust::src_loc = "94:9"]
    pub struct Dictionary {
        pub items: *mut KeyValuePair,
        pub size: size_t,
        pub capacity: size_t,
    }
    #[c2rust::src_loc = "92:1"]
    pub type KeyValuePair = key_value_pair;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:8"]
    pub struct key_value_pair {
        pub key: String_0,
        pub value: Object,
    }
    #[c2rust::src_loc = "85:1"]
    pub type Object = object;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "87:9"]
    pub struct Array {
        pub items: *mut Object,
        pub size: size_t,
        pub capacity: size_t,
    }
    #[c2rust::src_loc = "99:9"]
    pub type ObjectType = libc::c_uint;
    #[c2rust::src_loc = "111:3"]
    pub const kObjectTypeTabpage: ObjectType = 10;
    #[c2rust::src_loc = "110:3"]
    pub const kObjectTypeWindow: ObjectType = 9;
    #[c2rust::src_loc = "109:3"]
    pub const kObjectTypeBuffer: ObjectType = 8;
    #[c2rust::src_loc = "107:3"]
    pub const kObjectTypeLuaRef: ObjectType = 7;
    #[c2rust::src_loc = "106:3"]
    pub const kObjectTypeDictionary: ObjectType = 6;
    #[c2rust::src_loc = "105:3"]
    pub const kObjectTypeArray: ObjectType = 5;
    #[c2rust::src_loc = "104:3"]
    pub const kObjectTypeString: ObjectType = 4;
    #[c2rust::src_loc = "103:3"]
    pub const kObjectTypeFloat: ObjectType = 3;
    #[c2rust::src_loc = "102:3"]
    pub const kObjectTypeInteger: ObjectType = 2;
    #[c2rust::src_loc = "101:3"]
    pub const kObjectTypeBoolean: ObjectType = 1;
    #[c2rust::src_loc = "100:3"]
    pub const kObjectTypeNil: ObjectType = 0;
    #[inline(always)]
    #[c2rust::src_loc = "56:1"]
    pub unsafe extern "C" fn is_internal_call(channel_id: uint64_t) -> bool {
        return channel_id &
                   (1 as libc::c_int as uint64_t) <<
                       (::std::mem::size_of::<uint64_t>() as
                            libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                            libc::c_ulong).wrapping_sub(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_ulong)
                   != 0;
    }
    use super::nvim_types_h::{handle_T, LuaRef};
    use super::stdint_intn_h::int64_t;
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint64_t;
    // NVIM_API_PRIVATE_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/mark_defs.h:12"]
pub mod mark_defs_h {
    // / Total possible number of global marks
    // / Total possible number of local marks
// /
// / That are uppercase marks plus '"', '^' and '.'. There are other local marks,
// / but they are not saved in ShaDa files.
    // / Maximum number of marks in jump list
    // / Maximum number of tags in tag stack
    // / Structure defining single local mark
    #[c2rust::src_loc = "35:1"]
    pub type fmark_T = filemark;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "35:16"]
    pub struct filemark {
        pub mark: pos_T,
        pub fnum: libc::c_int,
        pub timestamp: Timestamp,
        pub additional_data: *mut dict_T,
    }
    // /< Cursor position.
    // /< File number.
    // /< Time when this mark was last set.
    // /< Additional data from ShaDa file.
    // / Structure defining extended mark (mark with file name attached)
    #[c2rust::src_loc = "43:1"]
    pub type xfmark_T = xfilemark;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:16"]
    pub struct xfilemark {
        pub fmark: fmark_T,
        pub fname: *mut char_u,
    }
    // /< Actual mark.
    // /< File name, used when fnum == 0.
    /*
 * marks: positions in a file
 * (a normal mark is a lnum/col pair, the same as a file position)
 */
    // / Number of possible numbered global marks
    // / Maximum possible number of letter marks
    #[c2rust::src_loc = "17:9"]
    pub const NMARKS: libc::c_int =
        'z' as i32 - 'a' as i32 + 1 as libc::c_int;
    use super::pos_h::pos_T;
    use super::time_h::Timestamp;
    use super::typval_h::dict_T;
    use super::nvim_types_h::char_u;
    // NVIM_MARK_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/os/time.h:12"]
pub mod time_h {
    #[c2rust::src_loc = "8:1"]
    pub type Timestamp = uint64_t;
    use super::stdint_uintn_h::uint64_t;
    // NVIM_OS_TIME_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/option_defs.h:12"]
pub mod option_defs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "862:9"]
    pub struct LastSet {
        pub script_ctx: sctx_T,
        pub channel_id: uint64_t,
    }
    use super::typval_h::sctx_T;
    use super::stdint_uintn_h::uint64_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "316:13"]
        pub static mut breakat_flags: [libc::c_char; 256];
    }
    // NVIM_OPTION_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/syntax_defs.h:12"]
pub mod syntax_defs_h {
    /* minimal size for state stack array */
    /* maximal size for state stack array */
    /* size of sst_stack[]. */
    /* normal distance between entries */
    /* invalid syn_state pointer */
    #[c2rust::src_loc = "12:1"]
    pub type synstate_T = syn_state;
    /* struct passed to in_id_list() */
    // ":syn include" unique tag
    // highlight group ID of item
    // cont.in group IDs, if non-zero
    /*
 * Each keyword has one keyentry, which is linked in a hash list.
 */
    // next entry with identical "keyword[]"
    // struct passed to in_id_list()
    // ID list for next match (if non-zero)
    // conceal substitute character
    // actually longer
    /*
 * Struct used to store one state of the state stack.
 */
    /* index of pattern */
    /* flags for pattern */
    /* stores si_seqnr */
    /* stores si_cchar */
    /* external matches from start pattern */
    /*
 * syn_state contains the syntax state stack for the start of one line.
 * Used by b_sst_array[].
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:8"]
    pub struct syn_state {
        pub sst_next: *mut synstate_T,
        pub sst_lnum: linenr_T,
        pub sst_union: C2RustUnnamed_5,
        pub sst_next_flags: libc::c_int,
        pub sst_stacksize: libc::c_int,
        pub sst_next_list: *mut int16_t,
        pub sst_tick: disptick_T,
        pub sst_change_lnum: linenr_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:3"]
    pub union C2RustUnnamed_5 {
        pub sst_stack: [bufstate_T; 7],
        pub sst_ga: garray_T,
    }
    #[c2rust::src_loc = "41:1"]
    pub type bufstate_T = buf_state;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:16"]
    pub struct buf_state {
        pub bs_idx: libc::c_int,
        pub bs_flags: libc::c_int,
        pub bs_seqnr: libc::c_int,
        pub bs_cchar: libc::c_int,
        pub bs_extmatch: *mut reg_extmatch_T,
    }
    use super::pos_h::linenr_T;
    use super::stdint_intn_h::int16_t;
    use super::buffer_defs_h::disptick_T;
    use super::garray_h::garray_T;
    use super::regexp_defs_h::reg_extmatch_T;
    // NVIM_SYNTAX_DEFS_H
    // when non-zero, change in this line
                                // may have made the state invalid
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/undo_defs.h:12"]
pub mod undo_defs_h {
    // for time_t
    #[c2rust::src_loc = "10:1"]
    pub type u_header_T = u_header;
    /* Structure to store info about the Visual area. */
    /* start pos of last VIsual */
    /* end position of last VIsual */
    /* VIsual_mode of last VIsual */
    /* MAXCOL from w_curswant */
    /* pointer to next entry in list */
    /* number of line above undo block */
    /* number of line below undo block */
    /* linecount when u_save called */
    /* array of lines in undo block */
    /* number of lines in ue_array */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "35:8"]
    pub struct u_header {
        pub uh_next: C2RustUnnamed_9,
        pub uh_prev: C2RustUnnamed_8,
        pub uh_alt_next: C2RustUnnamed_7,
        pub uh_alt_prev: C2RustUnnamed_6,
        pub uh_seq: libc::c_long,
        pub uh_walk: libc::c_int,
        pub uh_entry: *mut u_entry_T,
        pub uh_getbot_entry: *mut u_entry_T,
        pub uh_cursor: pos_T,
        pub uh_cursor_vcol: libc::c_long,
        pub uh_flags: libc::c_int,
        pub uh_namedm: [fmark_T; 26],
        pub uh_extmark: extmark_undo_vec_t,
        pub uh_visual: visualinfo_T,
        pub uh_time: time_t,
        pub uh_save_nr: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "13:9"]
    pub struct visualinfo_T {
        pub vi_start: pos_T,
        pub vi_end: pos_T,
        pub vi_mode: libc::c_int,
        pub vi_curswant: colnr_T,
    }
    #[c2rust::src_loc = "22:1"]
    pub type u_entry_T = u_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "23:8"]
    pub struct u_entry {
        pub ue_next: *mut u_entry_T,
        pub ue_top: linenr_T,
        pub ue_bot: linenr_T,
        pub ue_lcount: linenr_T,
        pub ue_array: *mut *mut char_u,
        pub ue_size: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:3"]
    pub union C2RustUnnamed_6 {
        pub ptr: *mut u_header_T,
        pub seq: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:3"]
    pub union C2RustUnnamed_7 {
        pub ptr: *mut u_header_T,
        pub seq: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:3"]
    pub union C2RustUnnamed_8 {
        pub ptr: *mut u_header_T,
        pub seq: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:3"]
    pub union C2RustUnnamed_9 {
        pub ptr: *mut u_header_T,
        pub seq: libc::c_long,
    }
    use super::pos_h::{pos_T, colnr_T, linenr_T};
    use super::mark_defs_h::fmark_T;
    use super::extmark_defs_h::extmark_undo_vec_t;
    use super::time_t_h::time_t;
    use super::nvim_types_h::char_u;
    // NVIM_UNDO_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/os/fs_defs.h:12"]
pub mod fs_defs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "12:9"]
    pub struct FileID {
        pub inode: uint64_t,
        pub device_id: uint64_t,
    }
    use super::stdint_uintn_h::uint64_t;
    // NVIM_OS_FS_DEFS_H
    // non-writable thing (e.g., block device)
    // something we can write to (character
                           // device, fifo, socket, ..)
    // file or directory, check with os_isdir()
    // Values returned by os_nodetype()
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/memline_defs.h:12"]
pub mod memline_defs_h {
    // /
// / When searching for a specific line, we remember what blocks in the tree
// / are the branches leading to that block. This is stored in ml_stack.  Each
// / entry is a pointer to info in a block (may be data block or pointer block)
// /
    // block number
    // lowest lnum in this block
    // highest lnum in this block
    // index for block with current lnum
    // block/index pair
    // Flags when calling ml_updatechunk()
    // / memline structure: the contents of a buffer.
// / Essentially a tree with a branch factor of 128.
// / Lines are stored at leaf nodes.
// / Nodes are stored on ml_mfp (memfile_T):
// /   pointer_block: internal nodes
// /   data_block: leaf nodes
// /
// / Memline also has "chunks" of 800 lines that are separate from the 128-tree
// / structure, primarily used to speed up line2byte() and byte2line().
// /
// / Motivation: If you have a file that is 10000 lines long, and you insert
// /             a line at linenr 1000, you don't want to move 9000 lines in
// /             memory.  With this structure it is roughly (N * 128) pointer
// /             moves, where N is the height (typically 1-3).
// /
    #[c2rust::src_loc = "43:1"]
    pub type memline_T = memline;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:16"]
    pub struct memline {
        pub ml_line_count: linenr_T,
        pub ml_mfp: *mut memfile_T,
        pub ml_flags: libc::c_int,
        pub ml_stack: *mut infoptr_T,
        pub ml_stack_top: libc::c_int,
        pub ml_stack_size: libc::c_int,
        pub ml_line_lnum: linenr_T,
        pub ml_line_ptr: *mut char_u,
        pub ml_locked: *mut bhdr_T,
        pub ml_locked_low: linenr_T,
        pub ml_locked_high: linenr_T,
        pub ml_locked_lineadd: libc::c_int,
        pub ml_chunksize: *mut chunksize_T,
        pub ml_numchunks: libc::c_int,
        pub ml_usedchunks: libc::c_int,
    }
    #[c2rust::src_loc = "18:1"]
    pub type chunksize_T = ml_chunksize;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:16"]
    pub struct ml_chunksize {
        pub mlcs_numlines: libc::c_int,
        pub mlcs_totalsize: libc::c_long,
    }
    #[c2rust::src_loc = "11:1"]
    pub type infoptr_T = info_pointer;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:16"]
    pub struct info_pointer {
        pub ip_bnum: blocknr_T,
        pub ip_low: linenr_T,
        pub ip_high: linenr_T,
        pub ip_index: libc::c_int,
    }
    use super::pos_h::linenr_T;
    use super::memfile_defs_h::{memfile_T, bhdr_T, blocknr_T};
    use super::nvim_types_h::char_u;
    // number of lines in the buffer
    // pointer to associated memfile
    // empty buffer
    // cached line was changed and allocated
    // ml_locked was changed
    // ml_locked needs positive block number
    // stack of pointer blocks (array of IPTRs)
    // current top of ml_stack
    // total number of entries in ml_stack
    // line number of cached line, 0 if not valid
    // pointer to cached line
    // block used by last ml_get
    // first line in ml_locked
    // last line in ml_locked
    // number of lines inserted in ml_locked
    // NVIM_MEMLINE_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/memfile_defs.h:12"]
pub mod memfile_defs_h {
    // / A block number.
// /
// / Blocks numbered from 0 upwards have been assigned a place in the actual
// / file. The block number is equal to the page number in the file. The blocks
// / with negative numbers are currently in memory only.
    // / A hash item.
// /
// / Items' keys are block numbers.
// / Items in the same bucket are organized into a doubly-linked list.
// /
// / Therefore, items can be arbitrary data structures beginning with pointers
// / for the list and and a block number key.
    // / Initial size for a hashtable.
    // / A chained hashtable with block numbers as keys and arbitrary data structures
// / as items.
// /
// / This is an intrusive data structure: we require that items begin with
// / mf_hashitem_T which contains the key and linked list pointers. List of items
// / in each bucket is doubly-linked.
    // / mask used to mod hash value to array index
                                // / (nr of items in array is 'mht_mask + 1')
    // / number of items inserted
    // / points to the array of buckets (can be
                                // / mht_small_buckets or a newly allocated array
                                // / when mht_small_buckets becomes too small)
    // / initial buckets
    // / A block header.
// /
// / There is a block header for each previously used block in the memfile.
// /
// / The block may be linked in the used list OR in the free list.
// / The used blocks are also kept in hash lists.
// /
// / The used list is a doubly linked list, most recently used block first.
// / The blocks in the used list have a block of memory allocated.
// / The hash lists are used to quickly find a block in the used list.
// / The free list is a single linked list, not sorted.
// / The blocks in the free list have no block of memory allocated and
// / the contents of the block in the file (if any) is irrelevant.
    #[c2rust::src_loc = "63:1"]
    pub type bhdr_T = bhdr;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:16"]
    pub struct bhdr {
        pub bh_hashitem: mf_hashitem_T,
        pub bh_next: *mut bhdr,
        pub bh_prev: *mut bhdr,
        pub bh_data: *mut libc::c_void,
        pub bh_page_count: libc::c_uint,
        pub bh_flags: libc::c_uint,
    }
    #[c2rust::src_loc = "25:1"]
    pub type mf_hashitem_T = mf_hashitem;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "25:16"]
    pub struct mf_hashitem {
        pub mhi_next: *mut mf_hashitem,
        pub mhi_prev: *mut mf_hashitem,
        pub mhi_key: blocknr_T,
    }
    #[c2rust::src_loc = "16:1"]
    pub type blocknr_T = int64_t;
    // / header for hash table and key
    // / block number, part of bh_hashitem
    // / next block header in free or used list
    // / previous block header in used list
    // / pointer to memory (for used block)
    // / number of pages in this block
    // BH_DIRTY or BH_LOCKED
    // / A block number translation list item.
// /
// / When a block with a negative number is flushed to the file, it gets
// / a positive number. Because the reference to the block is still the negative
// / number, we remember the translation to the new positive number in the
// / double linked trans lists. The structure is the same as the hash lists.
    // / header for hash table and key
    // / old, negative, number
    // / new, positive, number
    // / A memory file.
    #[c2rust::src_loc = "90:1"]
    pub type memfile_T = memfile;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "90:16"]
    pub struct memfile {
        pub mf_fname: *mut char_u,
        pub mf_ffname: *mut char_u,
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
    #[c2rust::src_loc = "40:1"]
    pub type mf_hashtab_T = mf_hashtab;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:16"]
    pub struct mf_hashtab {
        pub mht_mask: size_t,
        pub mht_count: size_t,
        pub mht_buckets: *mut *mut mf_hashitem_T,
        pub mht_small_buckets: [*mut mf_hashitem_T; 64],
    }
    use super::stdint_intn_h::int64_t;
    use super::nvim_types_h::char_u;
    use super::stddef_h::size_t;
    // / name of the file
    // / idem, full path
    // / file descriptor
    // / first block header in free list
    // / mru block header in used list
    // / lru block header in used list
    // / hash lists
    // / trans lists
    // / highest positive block number + 1
    // / lowest negative block number - 1
    // / number of negative blocks numbers
    // / number of pages in the file
    // / number of bytes in a page
    // / TRUE if there are dirty blocks
    // NVIM_MEMFILE_DEFS_H
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/types/struct_iovec.h:12"]
pub mod struct_iovec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:8"]
    pub struct iovec {
        pub iov_base: *mut libc::c_void,
        pub iov_len: size_t,
    }
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/home/vole/neovim/.deps/usr/include/uv.h:12"]
pub mod uv_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1693:8"]
    pub struct uv_loop_s {
        pub data: *mut libc::c_void,
        pub active_handles: libc::c_uint,
        pub handle_queue: [*mut libc::c_void; 2],
        pub active_reqs: C2RustUnnamed_16,
        pub stop_flag: libc::c_uint,
        pub flags: libc::c_ulong,
        pub backend_fd: libc::c_int,
        pub pending_queue: [*mut libc::c_void; 2],
        pub watcher_queue: [*mut libc::c_void; 2],
        pub watchers: *mut *mut uv__io_t,
        pub nwatchers: libc::c_uint,
        pub nfds: libc::c_uint,
        pub wq: [*mut libc::c_void; 2],
        pub wq_mutex: uv_mutex_t,
        pub wq_async: uv_async_t,
        pub cloexec_lock: uv_rwlock_t,
        pub closing_handles: *mut uv_handle_t,
        pub process_handles: [*mut libc::c_void; 2],
        pub prepare_handles: [*mut libc::c_void; 2],
        pub check_handles: [*mut libc::c_void; 2],
        pub idle_handles: [*mut libc::c_void; 2],
        pub async_handles: [*mut libc::c_void; 2],
        pub async_unused: Option<unsafe extern "C" fn() -> ()>,
        pub async_io_watcher: uv__io_t,
        pub async_wfd: libc::c_int,
        pub timer_heap: C2RustUnnamed_14,
        pub timer_counter: uint64_t,
        pub time: uint64_t,
        pub signal_pipefd: [libc::c_int; 2],
        pub signal_io_watcher: uv__io_t,
        pub child_watcher: uv_signal_t,
        pub emfile_fd: libc::c_int,
        pub inotify_read_watcher: uv__io_t,
        pub inotify_watchers: *mut libc::c_void,
        pub inotify_fd: libc::c_int,
    }
    #[c2rust::src_loc = "225:1"]
    pub type uv_signal_t = uv_signal_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1508:8"]
    pub struct uv_signal_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_13,
        pub next_closing: *mut uv_handle_t,
        pub flags: libc::c_uint,
        pub signal_cb: uv_signal_cb,
        pub signum: libc::c_int,
        pub tree_entry: C2RustUnnamed_11,
        pub caught_signals: libc::c_uint,
        pub dispatched_signals: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1512:3"]
    pub struct C2RustUnnamed_11 {
        pub rbe_left: *mut uv_signal_s,
        pub rbe_right: *mut uv_signal_s,
        pub rbe_parent: *mut uv_signal_s,
        pub rbe_color: libc::c_int,
    }
    #[c2rust::src_loc = "370:1"]
    pub type uv_signal_cb
        =
        Option<unsafe extern "C" fn(_: *mut uv_signal_t, _: libc::c_int)
                   -> ()>;
    #[c2rust::src_loc = "209:1"]
    pub type uv_handle_t = uv_handle_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "435:8"]
    pub struct uv_handle_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_12,
        pub next_closing: *mut uv_handle_t,
        pub flags: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "436:3"]
    pub union C2RustUnnamed_12 {
        pub fd: libc::c_int,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[c2rust::src_loc = "314:1"]
    pub type uv_close_cb
        =
        Option<unsafe extern "C" fn(_: *mut uv_handle_t) -> ()>;
    #[c2rust::src_loc = "188:9"]
    pub type uv_handle_type = libc::c_uint;
    #[c2rust::src_loc = "194:3"]
    pub const UV_HANDLE_TYPE_MAX: uv_handle_type = 18;
    #[c2rust::src_loc = "193:3"]
    pub const UV_FILE: uv_handle_type = 17;
    #[c2rust::src_loc = "191:22"]
    pub const UV_SIGNAL: uv_handle_type = 16;
    #[c2rust::src_loc = "191:22"]
    pub const UV_UDP: uv_handle_type = 15;
    #[c2rust::src_loc = "191:22"]
    pub const UV_TTY: uv_handle_type = 14;
    #[c2rust::src_loc = "191:22"]
    pub const UV_TIMER: uv_handle_type = 13;
    #[c2rust::src_loc = "191:22"]
    pub const UV_TCP: uv_handle_type = 12;
    #[c2rust::src_loc = "191:22"]
    pub const UV_STREAM: uv_handle_type = 11;
    #[c2rust::src_loc = "191:22"]
    pub const UV_PROCESS: uv_handle_type = 10;
    #[c2rust::src_loc = "191:22"]
    pub const UV_PREPARE: uv_handle_type = 9;
    #[c2rust::src_loc = "191:22"]
    pub const UV_POLL: uv_handle_type = 8;
    #[c2rust::src_loc = "191:22"]
    pub const UV_NAMED_PIPE: uv_handle_type = 7;
    #[c2rust::src_loc = "191:22"]
    pub const UV_IDLE: uv_handle_type = 6;
    #[c2rust::src_loc = "191:22"]
    pub const UV_HANDLE: uv_handle_type = 5;
    #[c2rust::src_loc = "191:22"]
    pub const UV_FS_POLL: uv_handle_type = 4;
    #[c2rust::src_loc = "191:22"]
    pub const UV_FS_EVENT: uv_handle_type = 3;
    #[c2rust::src_loc = "191:22"]
    pub const UV_CHECK: uv_handle_type = 2;
    #[c2rust::src_loc = "191:22"]
    pub const UV_ASYNC: uv_handle_type = 1;
    #[c2rust::src_loc = "189:3"]
    pub const UV_UNKNOWN_HANDLE: uv_handle_type = 0;
    #[c2rust::src_loc = "208:1"]
    pub type uv_loop_t = uv_loop_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1509:3"]
    pub union C2RustUnnamed_13 {
        pub fd: libc::c_int,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1705:3"]
    pub struct C2RustUnnamed_14 {
        pub min: *mut libc::c_void,
        pub nelts: libc::c_uint,
    }
    #[c2rust::src_loc = "221:1"]
    pub type uv_async_t = uv_async_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "803:8"]
    pub struct uv_async_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_15,
        pub next_closing: *mut uv_handle_t,
        pub flags: libc::c_uint,
        pub async_cb: uv_async_cb,
        pub queue: [*mut libc::c_void; 2],
        pub pending: libc::c_int,
    }
    #[c2rust::src_loc = "317:1"]
    pub type uv_async_cb
        =
        Option<unsafe extern "C" fn(_: *mut uv_async_t) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "804:3"]
    pub union C2RustUnnamed_15 {
        pub fd: libc::c_int,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1699:3"]
    pub union C2RustUnnamed_16 {
        pub unused: [*mut libc::c_void; 2],
        pub count: libc::c_uint,
    }
    #[c2rust::src_loc = "197:9"]
    pub type uv_req_type = libc::c_uint;
    #[c2rust::src_loc = "203:3"]
    pub const UV_REQ_TYPE_MAX: uv_req_type = 10;
    #[c2rust::src_loc = "200:19"]
    pub const UV_GETNAMEINFO: uv_req_type = 9;
    #[c2rust::src_loc = "200:19"]
    pub const UV_GETADDRINFO: uv_req_type = 8;
    #[c2rust::src_loc = "200:19"]
    pub const UV_WORK: uv_req_type = 7;
    #[c2rust::src_loc = "200:19"]
    pub const UV_FS: uv_req_type = 6;
    #[c2rust::src_loc = "200:19"]
    pub const UV_UDP_SEND: uv_req_type = 5;
    #[c2rust::src_loc = "200:19"]
    pub const UV_SHUTDOWN: uv_req_type = 4;
    #[c2rust::src_loc = "200:19"]
    pub const UV_WRITE: uv_req_type = 3;
    #[c2rust::src_loc = "200:19"]
    pub const UV_CONNECT: uv_req_type = 2;
    #[c2rust::src_loc = "200:19"]
    pub const UV_REQ: uv_req_type = 1;
    #[c2rust::src_loc = "198:3"]
    pub const UV_UNKNOWN_REQ: uv_req_type = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "485:8"]
    pub struct uv_stream_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_17,
        pub next_closing: *mut uv_handle_t,
        pub flags: libc::c_uint,
        pub write_queue_size: size_t,
        pub alloc_cb: uv_alloc_cb,
        pub read_cb: uv_read_cb,
        pub connect_req: *mut uv_connect_t,
        pub shutdown_req: *mut uv_shutdown_t,
        pub io_watcher: uv__io_t,
        pub write_queue: [*mut libc::c_void; 2],
        pub write_completed_queue: [*mut libc::c_void; 2],
        pub connection_cb: uv_connection_cb,
        pub delayed_error: libc::c_int,
        pub accepted_fd: libc::c_int,
        pub queued_fds: *mut libc::c_void,
    }
    #[c2rust::src_loc = "313:1"]
    pub type uv_connection_cb
        =
        Option<unsafe extern "C" fn(_: *mut uv_stream_t, _: libc::c_int)
                   -> ()>;
    #[c2rust::src_loc = "211:1"]
    pub type uv_stream_t = uv_stream_s;
    #[c2rust::src_loc = "231:1"]
    pub type uv_shutdown_t = uv_shutdown_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "411:8"]
    pub struct uv_shutdown_s {
        pub data: *mut libc::c_void,
        pub type_0: uv_req_type,
        pub reserved: [*mut libc::c_void; 6],
        pub handle: *mut uv_stream_t,
        pub cb: uv_shutdown_cb,
    }
    #[c2rust::src_loc = "312:1"]
    pub type uv_shutdown_cb
        =
        Option<unsafe extern "C" fn(_: *mut uv_shutdown_t, _: libc::c_int)
                   -> ()>;
    #[c2rust::src_loc = "233:1"]
    pub type uv_connect_t = uv_connect_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "574:8"]
    pub struct uv_connect_s {
        pub data: *mut libc::c_void,
        pub type_0: uv_req_type,
        pub reserved: [*mut libc::c_void; 6],
        pub cb: uv_connect_cb,
        pub handle: *mut uv_stream_t,
        pub queue: [*mut libc::c_void; 2],
    }
    #[c2rust::src_loc = "311:1"]
    pub type uv_connect_cb
        =
        Option<unsafe extern "C" fn(_: *mut uv_connect_t, _: libc::c_int)
                   -> ()>;
    #[c2rust::src_loc = "307:1"]
    pub type uv_read_cb
        =
        Option<unsafe extern "C" fn(_: *mut uv_stream_t, _: ssize_t,
                                    _: *const uv_buf_t) -> ()>;
    #[c2rust::src_loc = "304:1"]
    pub type uv_alloc_cb
        =
        Option<unsafe extern "C" fn(_: *mut uv_handle_t, _: size_t,
                                    _: *mut uv_buf_t) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "486:3"]
    pub union C2RustUnnamed_17 {
        pub fd: libc::c_int,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "538:8"]
    pub struct uv_tcp_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_18,
        pub next_closing: *mut uv_handle_t,
        pub flags: libc::c_uint,
        pub write_queue_size: size_t,
        pub alloc_cb: uv_alloc_cb,
        pub read_cb: uv_read_cb,
        pub connect_req: *mut uv_connect_t,
        pub shutdown_req: *mut uv_shutdown_t,
        pub io_watcher: uv__io_t,
        pub write_queue: [*mut libc::c_void; 2],
        pub write_completed_queue: [*mut libc::c_void; 2],
        pub connection_cb: uv_connection_cb,
        pub delayed_error: libc::c_int,
        pub accepted_fd: libc::c_int,
        pub queued_fds: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "539:3"]
    pub union C2RustUnnamed_18 {
        pub fd: libc::c_int,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[c2rust::src_loc = "212:1"]
    pub type uv_tcp_t = uv_tcp_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "726:8"]
    pub struct uv_pipe_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_19,
        pub next_closing: *mut uv_handle_t,
        pub flags: libc::c_uint,
        pub write_queue_size: size_t,
        pub alloc_cb: uv_alloc_cb,
        pub read_cb: uv_read_cb,
        pub connect_req: *mut uv_connect_t,
        pub shutdown_req: *mut uv_shutdown_t,
        pub io_watcher: uv__io_t,
        pub write_queue: [*mut libc::c_void; 2],
        pub write_completed_queue: [*mut libc::c_void; 2],
        pub connection_cb: uv_connection_cb,
        pub delayed_error: libc::c_int,
        pub accepted_fd: libc::c_int,
        pub queued_fds: *mut libc::c_void,
        pub ipc: libc::c_int,
        pub pipe_fname: *const libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "727:3"]
    pub union C2RustUnnamed_19 {
        pub fd: libc::c_int,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[c2rust::src_loc = "214:1"]
    pub type uv_pipe_t = uv_pipe_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "819:8"]
    pub struct uv_timer_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_20,
        pub next_closing: *mut uv_handle_t,
        pub flags: libc::c_uint,
        pub timer_cb: uv_timer_cb,
        pub heap_node: [*mut libc::c_void; 3],
        pub timeout: uint64_t,
        pub repeat: uint64_t,
        pub start_id: uint64_t,
    }
    #[c2rust::src_loc = "316:1"]
    pub type uv_timer_cb
        =
        Option<unsafe extern "C" fn(_: *mut uv_timer_t) -> ()>;
    #[c2rust::src_loc = "217:1"]
    pub type uv_timer_t = uv_timer_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "820:3"]
    pub union C2RustUnnamed_20 {
        pub fd: libc::c_int,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "793:8"]
    pub struct uv_idle_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_21,
        pub next_closing: *mut uv_handle_t,
        pub flags: libc::c_uint,
        pub idle_cb: uv_idle_cb,
        pub queue: [*mut libc::c_void; 2],
    }
    #[c2rust::src_loc = "320:1"]
    pub type uv_idle_cb
        =
        Option<unsafe extern "C" fn(_: *mut uv_idle_t) -> ()>;
    #[c2rust::src_loc = "220:1"]
    pub type uv_idle_t = uv_idle_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "794:3"]
    pub union C2RustUnnamed_21 {
        pub fd: libc::c_int,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1006:8"]
    pub struct uv_process_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_22,
        pub next_closing: *mut uv_handle_t,
        pub flags: libc::c_uint,
        pub exit_cb: uv_exit_cb,
        pub pid: libc::c_int,
        pub queue: [*mut libc::c_void; 2],
        pub status: libc::c_int,
    }
    #[c2rust::src_loc = "321:1"]
    pub type uv_exit_cb
        =
        Option<unsafe extern "C" fn(_: *mut uv_process_t, _: int64_t,
                                    _: libc::c_int) -> ()>;
    #[c2rust::src_loc = "222:1"]
    pub type uv_process_t = uv_process_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1007:3"]
    pub union C2RustUnnamed_22 {
        pub fd: libc::c_int,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[c2rust::src_loc = "879:9"]
    pub type uv_stdio_flags = libc::c_uint;
    #[c2rust::src_loc = "897:3"]
    pub const UV_OVERLAPPED_PIPE: uv_stdio_flags = 64;
    #[c2rust::src_loc = "891:3"]
    pub const UV_WRITABLE_PIPE: uv_stdio_flags = 32;
    #[c2rust::src_loc = "890:3"]
    pub const UV_READABLE_PIPE: uv_stdio_flags = 16;
    #[c2rust::src_loc = "883:3"]
    pub const UV_INHERIT_STREAM: uv_stdio_flags = 4;
    #[c2rust::src_loc = "882:3"]
    pub const UV_INHERIT_FD: uv_stdio_flags = 2;
    #[c2rust::src_loc = "881:3"]
    pub const UV_CREATE_PIPE: uv_stdio_flags = 1;
    #[c2rust::src_loc = "880:3"]
    pub const UV_IGNORE: uv_stdio_flags = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "900:16"]
    pub struct uv_stdio_container_s {
        pub flags: uv_stdio_flags,
        pub data: C2RustUnnamed_23,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "903:3"]
    pub union C2RustUnnamed_23 {
        pub stream: *mut uv_stream_t,
        pub fd: libc::c_int,
    }
    #[c2rust::src_loc = "900:1"]
    pub type uv_stdio_container_t = uv_stdio_container_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "909:16"]
    pub struct uv_process_options_s {
        pub exit_cb: uv_exit_cb,
        pub file: *const libc::c_char,
        pub args: *mut *mut libc::c_char,
        pub env: *mut *mut libc::c_char,
        pub cwd: *const libc::c_char,
        pub flags: libc::c_uint,
        pub stdio_count: libc::c_int,
        pub stdio: *mut uv_stdio_container_t,
        pub uid: uv_uid_t,
        pub gid: uv_gid_t,
    }
    #[c2rust::src_loc = "909:1"]
    pub type uv_process_options_t = uv_process_options_s;
    use super::unix_h::{uv__io_t, uv_mutex_t, uv_rwlock_t, uv_buf_t, uv_uid_t,
                        uv_gid_t};
    use super::stdint_uintn_h::uint64_t;
    use super::stddef_h::size_t;
    use super::stdio_h::ssize_t;
    use super::stdint_intn_h::int64_t;
}
#[c2rust::header_src = "/home/vole/neovim/.deps/usr/include/uv/unix.h:12"]
pub mod unix_h {
    #[c2rust::src_loc = "92:1"]
    pub type uv__io_t = uv__io_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:8"]
    pub struct uv__io_s {
        pub cb: uv__io_cb,
        pub pending_queue: [*mut libc::c_void; 2],
        pub watcher_queue: [*mut libc::c_void; 2],
        pub pevents: libc::c_uint,
        pub events: libc::c_uint,
        pub fd: libc::c_int,
    }
    #[c2rust::src_loc = "89:1"]
    pub type uv__io_cb
        =
        Option<unsafe extern "C" fn(_: *mut uv_loop_s, _: *mut uv__io_s,
                                    _: libc::c_uint) -> ()>;
    #[c2rust::src_loc = "136:1"]
    pub type uv_rwlock_t = pthread_rwlock_t;
    #[c2rust::src_loc = "135:1"]
    pub type uv_mutex_t = pthread_mutex_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "121:16"]
    pub struct uv_buf_t {
        pub base: *mut libc::c_char,
        pub len: size_t,
    }
    #[c2rust::src_loc = "126:1"]
    pub type uv_file = libc::c_int;
    #[c2rust::src_loc = "166:1"]
    pub type uv_gid_t = gid_t;
    #[c2rust::src_loc = "167:1"]
    pub type uv_uid_t = uid_t;
    use super::uv_h::uv_loop_s;
    use super::pthreadtypes_h::{pthread_rwlock_t, pthread_mutex_t};
    use super::stddef_h::size_t;
    use super::sys_types_h::{gid_t, uid_t};
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/defs.h:12"]
pub mod event_defs_h {
    #[c2rust::src_loc = "9:1"]
    pub type argv_callback
        =
        Option<unsafe extern "C" fn(_: *mut *mut libc::c_void) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:16"]
    pub struct message {
        pub handler: argv_callback,
        pub argv: [*mut libc::c_void; 10],
    }
    #[c2rust::src_loc = "10:1"]
    pub type Event = message;
    #[inline]
    #[c2rust::src_loc = "30:1"]
    pub unsafe extern "C" fn event_create(mut cb: argv_callback,
                                          mut argc: libc::c_int,
                                          mut args: ...) -> Event {
        if argc <= 10 as libc::c_int {
        } else {
            __assert_fail(b"argc <= EVENT_HANDLER_MAX_ARGC\x00" as *const u8
                              as *const libc::c_char,
                          b"../src/nvim/event/defs.h\x00" as *const u8 as
                              *const libc::c_char,
                          32 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 44],
                                                    &[libc::c_char; 44]>(b"Event event_create(argv_callback, int, ...)\x00")).as_ptr());
        }
        let mut event =
            Event{handler: None, argv: [0 as *mut libc::c_void; 10],};
        if argc <= 10 as libc::c_int {
        } else {
            __assert_fail(b"argc <= EVENT_HANDLER_MAX_ARGC\x00" as *const u8
                              as *const libc::c_char,
                          b"../src/nvim/event/defs.h\x00" as *const u8 as
                              *const libc::c_char,
                          34 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 44],
                                                    &[libc::c_char; 44]>(b"Event event_create(argv_callback, int, ...)\x00")).as_ptr());
        }
        event.handler = cb;
        if argc != 0 {
            let mut args_0: ::std::ffi::VaListImpl;
            args_0 = args.clone();
            let mut i = 0 as libc::c_int;
            while i < argc {
                event.argv[i as usize] = args_0.arg::<*mut libc::c_void>();
                i += 1
            }
        }
        return event;
    }
    use super::assert_h::__assert_fail;
    // NVIM_EVENT_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/multiqueue.h:12"]
pub mod multiqueue_h {
    #[c2rust::src_loc = "10:1"]
    pub type MultiQueue = multiqueue;
    extern "C" {
        #[c2rust::src_loc = "10:16"]
        pub type multiqueue;
    }
    // NVIM_EVENT_MULTIQUEUE_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/loop.h:12"]
pub mod loop_h {
    #[c2rust::src_loc = "12:1"]
    pub type WatcherPtr = *mut libc::c_void;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:1"]
    pub struct __kl1_WatcherPtr {
        pub data: WatcherPtr,
        pub next: *mut __kl1_WatcherPtr,
    }
    #[c2rust::src_loc = "15:1"]
    pub type kl1_WatcherPtr = __kl1_WatcherPtr;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:1"]
    pub struct kmp_WatcherPtr_t {
        pub cnt: size_t,
        pub n: size_t,
        pub max: size_t,
        pub buf: *mut *mut kl1_WatcherPtr,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:1"]
    pub struct kl_WatcherPtr_t {
        pub head: *mut kl1_WatcherPtr,
        pub tail: *mut kl1_WatcherPtr,
        pub mp: *mut kmp_WatcherPtr_t,
        pub size: size_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "17:16"]
    pub struct loop_0 {
        pub uv: uv_loop_t,
        pub events: *mut MultiQueue,
        pub thread_events: *mut MultiQueue,
        pub fast_events: *mut MultiQueue,
        pub children: *mut kl_WatcherPtr_t,
        pub children_watcher: uv_signal_t,
        pub children_kill_timer: uv_timer_t,
        pub poll_timer: uv_timer_t,
        pub async_0: uv_async_t,
        pub mutex: uv_mutex_t,
        pub recursive: libc::c_int,
    }
    #[c2rust::src_loc = "17:1"]
    pub type Loop = loop_0;
    #[inline]
    #[c2rust::src_loc = "15:1"]
    pub unsafe extern "C" fn kmp_init_WatcherPtr() -> *mut kmp_WatcherPtr_t {
        return xcalloc(1 as libc::c_int as size_t,
                       ::std::mem::size_of::<kmp_WatcherPtr_t>() as
                           libc::c_ulong) as *mut kmp_WatcherPtr_t;
    }
    #[inline]
    #[c2rust::src_loc = "15:1"]
    pub unsafe extern "C" fn kmp_destroy_WatcherPtr(mut mp:
                                                        *mut kmp_WatcherPtr_t) {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < (*mp).n {
            let mut ptr_ =
                &mut *(*mp).buf.offset(k as isize) as *mut *mut kl1_WatcherPtr
                    as *mut *mut libc::c_void;
            xfree(*ptr_);
            *ptr_ = NULL_1 as *mut libc::c_void;
            k = k.wrapping_add(1)
        }
        let mut ptr__0 =
            &mut (*mp).buf as *mut *mut *mut kl1_WatcherPtr as
                *mut *mut libc::c_void;
        xfree(*ptr__0);
        *ptr__0 = NULL_1 as *mut libc::c_void;
        let mut ptr__1 =
            &mut mp as *mut *mut kmp_WatcherPtr_t as *mut *mut libc::c_void;
        xfree(*ptr__1);
        *ptr__1 = NULL_1 as *mut libc::c_void;
    }
    #[inline]
    #[c2rust::src_loc = "15:1"]
    pub unsafe extern "C" fn kmp_alloc_WatcherPtr(mut mp:
                                                      *mut kmp_WatcherPtr_t)
     -> *mut kl1_WatcherPtr {
        (*mp).cnt = (*mp).cnt.wrapping_add(1);
        if (*mp).n == 0 as libc::c_int as libc::c_ulong {
            return xcalloc(1 as libc::c_int as size_t,
                           ::std::mem::size_of::<kl1_WatcherPtr>() as
                               libc::c_ulong) as *mut kl1_WatcherPtr
        }
        (*mp).n = (*mp).n.wrapping_sub(1);
        return *(*mp).buf.offset((*mp).n as isize);
    }
    #[inline]
    #[c2rust::src_loc = "15:1"]
    pub unsafe extern "C" fn kmp_free_WatcherPtr(mut mp:
                                                     *mut kmp_WatcherPtr_t,
                                                 mut p: *mut kl1_WatcherPtr) {
        (*mp).cnt = (*mp).cnt.wrapping_sub(1);
        if (*mp).n == (*mp).max {
            (*mp).max =
                if (*mp).max != 0 {
                    ((*mp).max) << 1 as libc::c_int
                } else { 16 as libc::c_int as libc::c_ulong };
            (*mp).buf =
                xrealloc((*mp).buf as *mut libc::c_void,
                         (::std::mem::size_of::<*mut kl1_WatcherPtr>() as
                              libc::c_ulong).wrapping_mul((*mp).max)) as
                    *mut *mut kl1_WatcherPtr
        }
        let fresh1 = (*mp).n;
        (*mp).n = (*mp).n.wrapping_add(1);
        let ref mut fresh2 = *(*mp).buf.offset(fresh1 as isize);
        *fresh2 = p;
    }
    #[inline]
    #[c2rust::src_loc = "15:1"]
    pub unsafe extern "C" fn kl_init_WatcherPtr() -> *mut kl_WatcherPtr_t {
        let mut kl =
            xcalloc(1 as libc::c_int as size_t,
                    ::std::mem::size_of::<kl_WatcherPtr_t>() as libc::c_ulong)
                as *mut kl_WatcherPtr_t;
        (*kl).mp = kmp_init_WatcherPtr();
        (*kl).tail = kmp_alloc_WatcherPtr((*kl).mp);
        (*kl).head = (*kl).tail;
        (*(*kl).head).next = 0 as *mut __kl1_WatcherPtr;
        return kl;
    }
    #[inline]
    #[c2rust::src_loc = "15:1"]
    pub unsafe extern "C" fn kl_destroy_WatcherPtr(mut kl:
                                                       *mut kl_WatcherPtr_t) {
        let mut p = 0 as *mut kl1_WatcherPtr;
        p = (*kl).head;
        while p != (*kl).tail {
            kmp_free_WatcherPtr((*kl).mp, p);
            p = (*p).next
        }
        kmp_free_WatcherPtr((*kl).mp, p);
        kmp_destroy_WatcherPtr((*kl).mp);
        let mut ptr_ =
            &mut kl as *mut *mut kl_WatcherPtr_t as *mut *mut libc::c_void;
        xfree(*ptr_);
        *ptr_ = NULL_1 as *mut libc::c_void;
    }
    #[inline]
    #[c2rust::src_loc = "15:1"]
    pub unsafe extern "C" fn kl_push_WatcherPtr(mut kl: *mut kl_WatcherPtr_t,
                                                mut d: WatcherPtr) {
        let mut q = 0 as *mut kl1_WatcherPtr;
        let mut p = kmp_alloc_WatcherPtr((*kl).mp);
        q = (*kl).tail;
        (*p).next = 0 as *mut __kl1_WatcherPtr;
        (*(*kl).tail).next = p;
        (*kl).tail = p;
        (*kl).size = (*kl).size.wrapping_add(1);
        (*q).data = d;
    }
    #[inline]
    #[c2rust::src_loc = "15:1"]
    pub unsafe extern "C" fn kl_shift_at_WatcherPtr(mut kl:
                                                        *mut kl_WatcherPtr_t,
                                                    mut n:
                                                        *mut *mut kl1_WatcherPtr)
     -> WatcherPtr {
        if !(**n).next.is_null() {
        } else {
            __assert_fail(b"(*n)->next\x00" as *const u8 as
                              *const libc::c_char,
                          b"../src/nvim/event/loop.h\x00" as *const u8 as
                              *const libc::c_char,
                          15 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 72],
                                                    &[libc::c_char; 72]>(b"WatcherPtr kl_shift_at_WatcherPtr(kl_WatcherPtr_t *, kl1_WatcherPtr **)\x00")).as_ptr());
        }
        let mut p = 0 as *mut kl1_WatcherPtr;
        (*kl).size = (*kl).size.wrapping_sub(1);
        p = *n;
        *n = (**n).next;
        if p == (*kl).head { (*kl).head = *n }
        let mut d = (*p).data;
        kmp_free_WatcherPtr((*kl).mp, p);
        return d;
    }
    use super::stddef_h::{size_t, NULL_1};
    use super::uv_h::{uv_loop_t, uv_signal_t, uv_timer_t, uv_async_t};
    use super::multiqueue_h::MultiQueue;
    use super::unix_h::uv_mutex_t;
    use super::memory_h_generated_h::{xcalloc, xfree, xrealloc};
    use super::assert_h::__assert_fail;
    // NVIM_EVENT_LOOP_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/rbuffer.h:16"]
pub mod rbuffer_h {
    // Specialized ring buffer. This is basically an array that wraps read/write
// pointers around the memory region. It should be more efficient than the old
// RBuffer which required memmove() calls to relocate read/write positions.
//
// The main purpose of RBuffer is simplify memory management when reading from
// uv_stream_t instances:
//
// - The event loop writes data to a RBuffer, advancing the write pointer
// - The main loop reads data, advancing the read pointer
// - If the buffer becomes full(size == capacity) the rstream is temporarily
//   stopped(automatic backpressure handling)
//
// Reference: http://en.wikipedia.org/wiki/Circular_buffer
    // Macros that simplify working with the read/write pointers directly by hiding
// ring buffer wrap logic. Some examples:
//
// - Pass the write pointer to a function(write_data) that incrementally
//   produces data, returning the number of bytes actually written to the
//   ring buffer:
//
//       RBUFFER_UNTIL_FULL(rbuf, ptr, cnt)
//         rbuffer_produced(rbuf, write_data(state, ptr, cnt));
//
// - Pass the read pointer to a function(read_data) that incrementally
//   consumes data, returning the number of bytes actually read from the
//   ring buffer:
//
//       RBUFFER_UNTIL_EMPTY(rbuf, ptr, cnt)
//         rbuffer_consumed(rbuf, read_data(state, ptr, cnt));
//
// Note that the rbuffer_{produced,consumed} calls are necessary or these macros
// create infinite loops
    /* NOLINT(readability/braces) */
    /* NOLINT(readability/braces) */
    /* NOLINT(readability/braces) */
    /* NOLINT(readability/braces) */
    // Iteration
    /* NOLINT(readability/braces) */
    /* NOLINT(readability/braces) */
    /* NOLINT(readability/braces) */
    /* NOLINT(readability/braces) */
    // / Type of function invoked during certain events:
// /   - When the RBuffer switches to the full state
// /   - When the RBuffer switches to the non-full state
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:8"]
    pub struct rbuffer {
        pub full_cb: rbuffer_callback,
        pub nonfull_cb: rbuffer_callback,
        pub data: *mut libc::c_void,
        pub size: size_t,
        pub temp: *mut libc::c_char,
        pub end_ptr: *mut libc::c_char,
        pub read_ptr: *mut libc::c_char,
        pub write_ptr: *mut libc::c_char,
        pub start_ptr: [libc::c_char; 0],
    }
    #[c2rust::src_loc = "75:1"]
    pub type rbuffer_callback
        =
        Option<unsafe extern "C" fn(_: *mut RBuffer, _: *mut libc::c_void)
                   -> ()>;
    #[c2rust::src_loc = "71:1"]
    pub type RBuffer = rbuffer;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn rbuffer_size(buf: *mut RBuffer) -> size_t;
    }
    // NVIM_RBUFFER_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/stream.h:16"]
pub mod stream_h {
    // / Type of function called when the Stream buffer is filled with data
// /
// / @param stream The Stream instance
// / @param buf The associated RBuffer instance
// / @param count Number of bytes that was read.
// / @param data User-defined data
// / @param eof If the stream reached EOF.
    // / Type of function called when the Stream has information about a write
// / request.
// /
// / @param stream The Stream instance
// / @param data User-defined data
// / @param status 0 on success, anything else indicates failure
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:8"]
    pub struct stream {
        pub closed: bool,
        pub did_eof: bool,
        pub uv: C2RustUnnamed_24,
        pub uvstream: *mut uv_stream_t,
        pub uvbuf: uv_buf_t,
        pub buffer: *mut RBuffer,
        pub fd: uv_file,
        pub read_cb: stream_read_cb,
        pub write_cb: stream_write_cb,
        pub cb_data: *mut libc::c_void,
        pub close_cb: stream_close_cb,
        pub internal_close_cb: stream_close_cb,
        pub close_cb_data: *mut libc::c_void,
        pub internal_data: *mut libc::c_void,
        pub fpos: size_t,
        pub curmem: size_t,
        pub maxmem: size_t,
        pub pending_reqs: size_t,
        pub num_bytes: size_t,
        pub events: *mut MultiQueue,
    }
    #[c2rust::src_loc = "30:1"]
    pub type stream_close_cb
        =
        Option<unsafe extern "C" fn(_: *mut Stream, _: *mut libc::c_void)
                   -> ()>;
    #[c2rust::src_loc = "12:1"]
    pub type Stream = stream;
    #[c2rust::src_loc = "29:1"]
    pub type stream_write_cb
        =
        Option<unsafe extern "C" fn(_: *mut Stream, _: *mut libc::c_void,
                                    _: libc::c_int) -> ()>;
    #[c2rust::src_loc = "20:1"]
    pub type stream_read_cb
        =
        Option<unsafe extern "C" fn(_: *mut Stream, _: *mut RBuffer,
                                    _: size_t, _: *mut libc::c_void, _: bool)
                   -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "35:3"]
    pub union C2RustUnnamed_24 {
        pub pipe: uv_pipe_t,
        pub tcp: uv_tcp_t,
        pub idle: uv_idle_t,
    }
    use super::uv_h::{uv_stream_t, uv_pipe_t, uv_tcp_t, uv_idle_t};
    use super::unix_h::{uv_buf_t, uv_file};
    use super::rbuffer_h::RBuffer;
    use super::stddef_h::size_t;
    use super::multiqueue_h::MultiQueue;
    // NVIM_EVENT_STREAM_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/process.h:16"]
pub mod process_h {
    #[c2rust::src_loc = "8:9"]
    pub type ProcessType = libc::c_uint;
    #[c2rust::src_loc = "10:3"]
    pub const kProcessTypePty: ProcessType = 1;
    #[c2rust::src_loc = "9:3"]
    pub const kProcessTypeUv: ProcessType = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "17:8"]
    pub struct process {
        pub type_0: ProcessType,
        pub loop_0: *mut Loop,
        pub data: *mut libc::c_void,
        pub pid: libc::c_int,
        pub status: libc::c_int,
        pub refcount: libc::c_int,
        pub exit_signal: uint8_t,
        pub stopped_time: uint64_t,
        pub cwd: *const libc::c_char,
        pub argv: *mut *mut libc::c_char,
        pub env: *mut *mut libc::c_char,
        pub in_0: Stream,
        pub out: Stream,
        pub err: Stream,
        pub cb: process_exit_cb,
        pub internal_exit_cb: internal_process_cb,
        pub internal_close_cb: internal_process_cb,
        pub closed: bool,
        pub detach: bool,
        pub events: *mut MultiQueue,
    }
    #[c2rust::src_loc = "15:1"]
    pub type internal_process_cb
        =
        Option<unsafe extern "C" fn(_: *mut Process) -> ()>;
    #[c2rust::src_loc = "13:1"]
    pub type Process = process;
    #[c2rust::src_loc = "14:1"]
    pub type process_exit_cb
        =
        Option<unsafe extern "C" fn(_: *mut Process, _: libc::c_int,
                                    _: *mut libc::c_void) -> ()>;
    #[inline]
    #[c2rust::src_loc = "35:1"]
    pub unsafe extern "C" fn process_init(mut loop_0: *mut Loop,
                                          mut type_0: ProcessType,
                                          mut data: *mut libc::c_void)
     -> Process {
        return {
                   let mut init =
                       process{type_0: type_0,
                               loop_0: loop_0,
                               data: data,
                               pid: 0 as libc::c_int,
                               status: -(1 as libc::c_int),
                               refcount: 0 as libc::c_int,
                               exit_signal: 0,
                               stopped_time: 0 as libc::c_int as uint64_t,
                               cwd: NULL_1 as *const libc::c_char,
                               argv: NULL_1 as *mut *mut libc::c_char,
                               env: 0 as *mut *mut libc::c_char,
                               in_0:
                                   {
                                       let mut init =
                                           stream{closed: false_0 != 0,
                                                  did_eof: false,
                                                  uv:
                                                      C2RustUnnamed_24{pipe:
                                                                           uv_pipe_t{data:
                                                                                         0
                                                                                             as
                                                                                             *mut libc::c_void,
                                                                                     loop_0:
                                                                                         0
                                                                                             as
                                                                                             *mut uv_loop_t,
                                                                                     type_0:
                                                                                         UV_UNKNOWN_HANDLE,
                                                                                     close_cb:
                                                                                         None,
                                                                                     handle_queue:
                                                                                         [0
                                                                                              as
                                                                                              *mut libc::c_void;
                                                                                             2],
                                                                                     u:
                                                                                         C2RustUnnamed_19{fd:
                                                                                                              0,},
                                                                                     next_closing:
                                                                                         0
                                                                                             as
                                                                                             *mut uv_handle_t,
                                                                                     flags:
                                                                                         0,
                                                                                     write_queue_size:
                                                                                         0,
                                                                                     alloc_cb:
                                                                                         None,
                                                                                     read_cb:
                                                                                         None,
                                                                                     connect_req:
                                                                                         0
                                                                                             as
                                                                                             *mut uv_connect_t,
                                                                                     shutdown_req:
                                                                                         0
                                                                                             as
                                                                                             *mut uv_shutdown_t,
                                                                                     io_watcher:
                                                                                         uv__io_t{cb:
                                                                                                      None,
                                                                                                  pending_queue:
                                                                                                      [0
                                                                                                           as
                                                                                                           *mut libc::c_void;
                                                                                                          2],
                                                                                                  watcher_queue:
                                                                                                      [0
                                                                                                           as
                                                                                                           *mut libc::c_void;
                                                                                                          2],
                                                                                                  pevents:
                                                                                                      0,
                                                                                                  events:
                                                                                                      0,
                                                                                                  fd:
                                                                                                      0,},
                                                                                     write_queue:
                                                                                         [0
                                                                                              as
                                                                                              *mut libc::c_void;
                                                                                             2],
                                                                                     write_completed_queue:
                                                                                         [0
                                                                                              as
                                                                                              *mut libc::c_void;
                                                                                             2],
                                                                                     connection_cb:
                                                                                         None,
                                                                                     delayed_error:
                                                                                         0,
                                                                                     accepted_fd:
                                                                                         0,
                                                                                     queued_fds:
                                                                                         0
                                                                                             as
                                                                                             *mut libc::c_void,
                                                                                     ipc:
                                                                                         0,
                                                                                     pipe_fname:
                                                                                         0
                                                                                             as
                                                                                             *const libc::c_char,},},
                                                  uvstream:
                                                      0 as *mut uv_stream_t,
                                                  uvbuf:
                                                      uv_buf_t{base:
                                                                   0 as
                                                                       *mut libc::c_char,
                                                               len: 0,},
                                                  buffer: 0 as *mut RBuffer,
                                                  fd: 0,
                                                  read_cb: None,
                                                  write_cb: None,
                                                  cb_data:
                                                      0 as *mut libc::c_void,
                                                  close_cb: None,
                                                  internal_close_cb: None,
                                                  close_cb_data:
                                                      0 as *mut libc::c_void,
                                                  internal_data:
                                                      0 as *mut libc::c_void,
                                                  fpos: 0,
                                                  curmem: 0,
                                                  maxmem: 0,
                                                  pending_reqs: 0,
                                                  num_bytes: 0,
                                                  events:
                                                      0 as *mut MultiQueue,};
                                       init
                                   },
                               out:
                                   {
                                       let mut init =
                                           stream{closed: false_0 != 0,
                                                  did_eof: false,
                                                  uv:
                                                      C2RustUnnamed_24{pipe:
                                                                           uv_pipe_t{data:
                                                                                         0
                                                                                             as
                                                                                             *mut libc::c_void,
                                                                                     loop_0:
                                                                                         0
                                                                                             as
                                                                                             *mut uv_loop_t,
                                                                                     type_0:
                                                                                         UV_UNKNOWN_HANDLE,
                                                                                     close_cb:
                                                                                         None,
                                                                                     handle_queue:
                                                                                         [0
                                                                                              as
                                                                                              *mut libc::c_void;
                                                                                             2],
                                                                                     u:
                                                                                         C2RustUnnamed_19{fd:
                                                                                                              0,},
                                                                                     next_closing:
                                                                                         0
                                                                                             as
                                                                                             *mut uv_handle_t,
                                                                                     flags:
                                                                                         0,
                                                                                     write_queue_size:
                                                                                         0,
                                                                                     alloc_cb:
                                                                                         None,
                                                                                     read_cb:
                                                                                         None,
                                                                                     connect_req:
                                                                                         0
                                                                                             as
                                                                                             *mut uv_connect_t,
                                                                                     shutdown_req:
                                                                                         0
                                                                                             as
                                                                                             *mut uv_shutdown_t,
                                                                                     io_watcher:
                                                                                         uv__io_t{cb:
                                                                                                      None,
                                                                                                  pending_queue:
                                                                                                      [0
                                                                                                           as
                                                                                                           *mut libc::c_void;
                                                                                                          2],
                                                                                                  watcher_queue:
                                                                                                      [0
                                                                                                           as
                                                                                                           *mut libc::c_void;
                                                                                                          2],
                                                                                                  pevents:
                                                                                                      0,
                                                                                                  events:
                                                                                                      0,
                                                                                                  fd:
                                                                                                      0,},
                                                                                     write_queue:
                                                                                         [0
                                                                                              as
                                                                                              *mut libc::c_void;
                                                                                             2],
                                                                                     write_completed_queue:
                                                                                         [0
                                                                                              as
                                                                                              *mut libc::c_void;
                                                                                             2],
                                                                                     connection_cb:
                                                                                         None,
                                                                                     delayed_error:
                                                                                         0,
                                                                                     accepted_fd:
                                                                                         0,
                                                                                     queued_fds:
                                                                                         0
                                                                                             as
                                                                                             *mut libc::c_void,
                                                                                     ipc:
                                                                                         0,
                                                                                     pipe_fname:
                                                                                         0
                                                                                             as
                                                                                             *const libc::c_char,},},
                                                  uvstream:
                                                      0 as *mut uv_stream_t,
                                                  uvbuf:
                                                      uv_buf_t{base:
                                                                   0 as
                                                                       *mut libc::c_char,
                                                               len: 0,},
                                                  buffer: 0 as *mut RBuffer,
                                                  fd: 0,
                                                  read_cb: None,
                                                  write_cb: None,
                                                  cb_data:
                                                      0 as *mut libc::c_void,
                                                  close_cb: None,
                                                  internal_close_cb: None,
                                                  close_cb_data:
                                                      0 as *mut libc::c_void,
                                                  internal_data:
                                                      0 as *mut libc::c_void,
                                                  fpos: 0,
                                                  curmem: 0,
                                                  maxmem: 0,
                                                  pending_reqs: 0,
                                                  num_bytes: 0,
                                                  events:
                                                      0 as *mut MultiQueue,};
                                       init
                                   },
                               err:
                                   {
                                       let mut init =
                                           stream{closed: false_0 != 0,
                                                  did_eof: false,
                                                  uv:
                                                      C2RustUnnamed_24{pipe:
                                                                           uv_pipe_t{data:
                                                                                         0
                                                                                             as
                                                                                             *mut libc::c_void,
                                                                                     loop_0:
                                                                                         0
                                                                                             as
                                                                                             *mut uv_loop_t,
                                                                                     type_0:
                                                                                         UV_UNKNOWN_HANDLE,
                                                                                     close_cb:
                                                                                         None,
                                                                                     handle_queue:
                                                                                         [0
                                                                                              as
                                                                                              *mut libc::c_void;
                                                                                             2],
                                                                                     u:
                                                                                         C2RustUnnamed_19{fd:
                                                                                                              0,},
                                                                                     next_closing:
                                                                                         0
                                                                                             as
                                                                                             *mut uv_handle_t,
                                                                                     flags:
                                                                                         0,
                                                                                     write_queue_size:
                                                                                         0,
                                                                                     alloc_cb:
                                                                                         None,
                                                                                     read_cb:
                                                                                         None,
                                                                                     connect_req:
                                                                                         0
                                                                                             as
                                                                                             *mut uv_connect_t,
                                                                                     shutdown_req:
                                                                                         0
                                                                                             as
                                                                                             *mut uv_shutdown_t,
                                                                                     io_watcher:
                                                                                         uv__io_t{cb:
                                                                                                      None,
                                                                                                  pending_queue:
                                                                                                      [0
                                                                                                           as
                                                                                                           *mut libc::c_void;
                                                                                                          2],
                                                                                                  watcher_queue:
                                                                                                      [0
                                                                                                           as
                                                                                                           *mut libc::c_void;
                                                                                                          2],
                                                                                                  pevents:
                                                                                                      0,
                                                                                                  events:
                                                                                                      0,
                                                                                                  fd:
                                                                                                      0,},
                                                                                     write_queue:
                                                                                         [0
                                                                                              as
                                                                                              *mut libc::c_void;
                                                                                             2],
                                                                                     write_completed_queue:
                                                                                         [0
                                                                                              as
                                                                                              *mut libc::c_void;
                                                                                             2],
                                                                                     connection_cb:
                                                                                         None,
                                                                                     delayed_error:
                                                                                         0,
                                                                                     accepted_fd:
                                                                                         0,
                                                                                     queued_fds:
                                                                                         0
                                                                                             as
                                                                                             *mut libc::c_void,
                                                                                     ipc:
                                                                                         0,
                                                                                     pipe_fname:
                                                                                         0
                                                                                             as
                                                                                             *const libc::c_char,},},
                                                  uvstream:
                                                      0 as *mut uv_stream_t,
                                                  uvbuf:
                                                      uv_buf_t{base:
                                                                   0 as
                                                                       *mut libc::c_char,
                                                               len: 0,},
                                                  buffer: 0 as *mut RBuffer,
                                                  fd: 0,
                                                  read_cb: None,
                                                  write_cb: None,
                                                  cb_data:
                                                      0 as *mut libc::c_void,
                                                  close_cb: None,
                                                  internal_close_cb: None,
                                                  close_cb_data:
                                                      0 as *mut libc::c_void,
                                                  internal_data:
                                                      0 as *mut libc::c_void,
                                                  fpos: 0,
                                                  curmem: 0,
                                                  maxmem: 0,
                                                  pending_reqs: 0,
                                                  num_bytes: 0,
                                                  events:
                                                      0 as *mut MultiQueue,};
                                       init
                                   },
                               cb:
                                   ::std::mem::transmute::<libc::intptr_t,
                                                           process_exit_cb>(NULL_1
                                                                                as
                                                                                libc::intptr_t),
                               internal_exit_cb:
                                   ::std::mem::transmute::<libc::intptr_t,
                                                           internal_process_cb>(NULL_1
                                                                                    as
                                                                                    libc::intptr_t),
                               internal_close_cb:
                                   ::std::mem::transmute::<libc::intptr_t,
                                                           internal_process_cb>(NULL_1
                                                                                    as
                                                                                    libc::intptr_t),
                               closed: false_0 != 0,
                               detach: false_0 != 0,
                               events: NULL_1 as *mut MultiQueue,};
                   init
               };
    }
    #[inline]
    #[c2rust::src_loc = "59:1"]
    pub unsafe extern "C" fn process_is_stopped(mut proc_0: *mut Process)
     -> bool {
        let mut exited = (*proc_0).status >= 0 as libc::c_int;
        return exited as libc::c_int != 0 ||
                   (*proc_0).stopped_time !=
                       0 as libc::c_int as libc::c_ulong;
    }
    use super::loop_h::Loop;
    use super::stdint_uintn_h::{uint8_t, uint64_t};
    use super::stream_h::{Stream, C2RustUnnamed_24, stream_read_cb,
                          stream_write_cb, stream_close_cb};
    use super::multiqueue_h::MultiQueue;
    use super::stddef_h::{NULL_1, size_t};
    use super::uv_h::{uv_stream_t, uv_pipe_t, uv_loop_t, uv_handle_type,
                      uv_close_cb, C2RustUnnamed_19, uv_handle_t, uv_alloc_cb,
                      uv_read_cb, uv_connect_t, uv_shutdown_t,
                      uv_connection_cb, UV_UNKNOWN_HANDLE};
    use super::unix_h::{uv_buf_t, uv_file, uv__io_t, uv__io_cb};
    use super::rbuffer_h::RBuffer;
    use super::stdbool_h::false_0;
    // NVIM_EVENT_PROCESS_H
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/ioctl-types.h:16"]
pub mod ioctl_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:8"]
    pub struct winsize {
        pub ws_row: libc::c_ushort,
        pub ws_col: libc::c_ushort,
        pub ws_xpixel: libc::c_ushort,
        pub ws_ypixel: libc::c_ushort,
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/os/pty_process_unix.h:16"]
pub mod pty_process_unix_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:16"]
    pub struct pty_process {
        pub process: Process,
        pub term_name: *mut libc::c_char,
        pub width: uint16_t,
        pub height: uint16_t,
        pub winsize: winsize,
        pub tty_fd: libc::c_int,
    }
    #[c2rust::src_loc = "8:1"]
    pub type PtyProcess = pty_process;
    #[inline]
    #[c2rust::src_loc = "16:1"]
    pub unsafe extern "C" fn pty_process_init(mut loop_0: *mut Loop,
                                              mut data: *mut libc::c_void)
     -> PtyProcess {
        let mut rv =
            PtyProcess{process:
                           Process{type_0: kProcessTypeUv,
                                   loop_0: 0 as *mut Loop,
                                   data: 0 as *mut libc::c_void,
                                   pid: 0,
                                   status: 0,
                                   refcount: 0,
                                   exit_signal: 0,
                                   stopped_time: 0,
                                   cwd: 0 as *const libc::c_char,
                                   argv: 0 as *mut *mut libc::c_char,
                                   env: 0 as *mut *mut libc::c_char,
                                   in_0:
                                       Stream{closed: false,
                                              did_eof: false,
                                              uv:
                                                  C2RustUnnamed_24{pipe:
                                                                       uv_pipe_t{data:
                                                                                     0
                                                                                         as
                                                                                         *mut libc::c_void,
                                                                                 loop_0:
                                                                                     0
                                                                                         as
                                                                                         *mut uv_loop_t,
                                                                                 type_0:
                                                                                     UV_UNKNOWN_HANDLE,
                                                                                 close_cb:
                                                                                     None,
                                                                                 handle_queue:
                                                                                     [0
                                                                                          as
                                                                                          *mut libc::c_void;
                                                                                         2],
                                                                                 u:
                                                                                     C2RustUnnamed_19{fd:
                                                                                                          0,},
                                                                                 next_closing:
                                                                                     0
                                                                                         as
                                                                                         *mut uv_handle_t,
                                                                                 flags:
                                                                                     0,
                                                                                 write_queue_size:
                                                                                     0,
                                                                                 alloc_cb:
                                                                                     None,
                                                                                 read_cb:
                                                                                     None,
                                                                                 connect_req:
                                                                                     0
                                                                                         as
                                                                                         *mut uv_connect_t,
                                                                                 shutdown_req:
                                                                                     0
                                                                                         as
                                                                                         *mut uv_shutdown_t,
                                                                                 io_watcher:
                                                                                     uv__io_t{cb:
                                                                                                  None,
                                                                                              pending_queue:
                                                                                                  [0
                                                                                                       as
                                                                                                       *mut libc::c_void;
                                                                                                      2],
                                                                                              watcher_queue:
                                                                                                  [0
                                                                                                       as
                                                                                                       *mut libc::c_void;
                                                                                                      2],
                                                                                              pevents:
                                                                                                  0,
                                                                                              events:
                                                                                                  0,
                                                                                              fd:
                                                                                                  0,},
                                                                                 write_queue:
                                                                                     [0
                                                                                          as
                                                                                          *mut libc::c_void;
                                                                                         2],
                                                                                 write_completed_queue:
                                                                                     [0
                                                                                          as
                                                                                          *mut libc::c_void;
                                                                                         2],
                                                                                 connection_cb:
                                                                                     None,
                                                                                 delayed_error:
                                                                                     0,
                                                                                 accepted_fd:
                                                                                     0,
                                                                                 queued_fds:
                                                                                     0
                                                                                         as
                                                                                         *mut libc::c_void,
                                                                                 ipc:
                                                                                     0,
                                                                                 pipe_fname:
                                                                                     0
                                                                                         as
                                                                                         *const libc::c_char,},},
                                              uvstream: 0 as *mut uv_stream_t,
                                              uvbuf:
                                                  uv_buf_t{base:
                                                               0 as
                                                                   *mut libc::c_char,
                                                           len: 0,},
                                              buffer: 0 as *mut RBuffer,
                                              fd: 0,
                                              read_cb: None,
                                              write_cb: None,
                                              cb_data: 0 as *mut libc::c_void,
                                              close_cb: None,
                                              internal_close_cb: None,
                                              close_cb_data:
                                                  0 as *mut libc::c_void,
                                              internal_data:
                                                  0 as *mut libc::c_void,
                                              fpos: 0,
                                              curmem: 0,
                                              maxmem: 0,
                                              pending_reqs: 0,
                                              num_bytes: 0,
                                              events: 0 as *mut MultiQueue,},
                                   out:
                                       Stream{closed: false,
                                              did_eof: false,
                                              uv:
                                                  C2RustUnnamed_24{pipe:
                                                                       uv_pipe_t{data:
                                                                                     0
                                                                                         as
                                                                                         *mut libc::c_void,
                                                                                 loop_0:
                                                                                     0
                                                                                         as
                                                                                         *mut uv_loop_t,
                                                                                 type_0:
                                                                                     UV_UNKNOWN_HANDLE,
                                                                                 close_cb:
                                                                                     None,
                                                                                 handle_queue:
                                                                                     [0
                                                                                          as
                                                                                          *mut libc::c_void;
                                                                                         2],
                                                                                 u:
                                                                                     C2RustUnnamed_19{fd:
                                                                                                          0,},
                                                                                 next_closing:
                                                                                     0
                                                                                         as
                                                                                         *mut uv_handle_t,
                                                                                 flags:
                                                                                     0,
                                                                                 write_queue_size:
                                                                                     0,
                                                                                 alloc_cb:
                                                                                     None,
                                                                                 read_cb:
                                                                                     None,
                                                                                 connect_req:
                                                                                     0
                                                                                         as
                                                                                         *mut uv_connect_t,
                                                                                 shutdown_req:
                                                                                     0
                                                                                         as
                                                                                         *mut uv_shutdown_t,
                                                                                 io_watcher:
                                                                                     uv__io_t{cb:
                                                                                                  None,
                                                                                              pending_queue:
                                                                                                  [0
                                                                                                       as
                                                                                                       *mut libc::c_void;
                                                                                                      2],
                                                                                              watcher_queue:
                                                                                                  [0
                                                                                                       as
                                                                                                       *mut libc::c_void;
                                                                                                      2],
                                                                                              pevents:
                                                                                                  0,
                                                                                              events:
                                                                                                  0,
                                                                                              fd:
                                                                                                  0,},
                                                                                 write_queue:
                                                                                     [0
                                                                                          as
                                                                                          *mut libc::c_void;
                                                                                         2],
                                                                                 write_completed_queue:
                                                                                     [0
                                                                                          as
                                                                                          *mut libc::c_void;
                                                                                         2],
                                                                                 connection_cb:
                                                                                     None,
                                                                                 delayed_error:
                                                                                     0,
                                                                                 accepted_fd:
                                                                                     0,
                                                                                 queued_fds:
                                                                                     0
                                                                                         as
                                                                                         *mut libc::c_void,
                                                                                 ipc:
                                                                                     0,
                                                                                 pipe_fname:
                                                                                     0
                                                                                         as
                                                                                         *const libc::c_char,},},
                                              uvstream: 0 as *mut uv_stream_t,
                                              uvbuf:
                                                  uv_buf_t{base:
                                                               0 as
                                                                   *mut libc::c_char,
                                                           len: 0,},
                                              buffer: 0 as *mut RBuffer,
                                              fd: 0,
                                              read_cb: None,
                                              write_cb: None,
                                              cb_data: 0 as *mut libc::c_void,
                                              close_cb: None,
                                              internal_close_cb: None,
                                              close_cb_data:
                                                  0 as *mut libc::c_void,
                                              internal_data:
                                                  0 as *mut libc::c_void,
                                              fpos: 0,
                                              curmem: 0,
                                              maxmem: 0,
                                              pending_reqs: 0,
                                              num_bytes: 0,
                                              events: 0 as *mut MultiQueue,},
                                   err:
                                       Stream{closed: false,
                                              did_eof: false,
                                              uv:
                                                  C2RustUnnamed_24{pipe:
                                                                       uv_pipe_t{data:
                                                                                     0
                                                                                         as
                                                                                         *mut libc::c_void,
                                                                                 loop_0:
                                                                                     0
                                                                                         as
                                                                                         *mut uv_loop_t,
                                                                                 type_0:
                                                                                     UV_UNKNOWN_HANDLE,
                                                                                 close_cb:
                                                                                     None,
                                                                                 handle_queue:
                                                                                     [0
                                                                                          as
                                                                                          *mut libc::c_void;
                                                                                         2],
                                                                                 u:
                                                                                     C2RustUnnamed_19{fd:
                                                                                                          0,},
                                                                                 next_closing:
                                                                                     0
                                                                                         as
                                                                                         *mut uv_handle_t,
                                                                                 flags:
                                                                                     0,
                                                                                 write_queue_size:
                                                                                     0,
                                                                                 alloc_cb:
                                                                                     None,
                                                                                 read_cb:
                                                                                     None,
                                                                                 connect_req:
                                                                                     0
                                                                                         as
                                                                                         *mut uv_connect_t,
                                                                                 shutdown_req:
                                                                                     0
                                                                                         as
                                                                                         *mut uv_shutdown_t,
                                                                                 io_watcher:
                                                                                     uv__io_t{cb:
                                                                                                  None,
                                                                                              pending_queue:
                                                                                                  [0
                                                                                                       as
                                                                                                       *mut libc::c_void;
                                                                                                      2],
                                                                                              watcher_queue:
                                                                                                  [0
                                                                                                       as
                                                                                                       *mut libc::c_void;
                                                                                                      2],
                                                                                              pevents:
                                                                                                  0,
                                                                                              events:
                                                                                                  0,
                                                                                              fd:
                                                                                                  0,},
                                                                                 write_queue:
                                                                                     [0
                                                                                          as
                                                                                          *mut libc::c_void;
                                                                                         2],
                                                                                 write_completed_queue:
                                                                                     [0
                                                                                          as
                                                                                          *mut libc::c_void;
                                                                                         2],
                                                                                 connection_cb:
                                                                                     None,
                                                                                 delayed_error:
                                                                                     0,
                                                                                 accepted_fd:
                                                                                     0,
                                                                                 queued_fds:
                                                                                     0
                                                                                         as
                                                                                         *mut libc::c_void,
                                                                                 ipc:
                                                                                     0,
                                                                                 pipe_fname:
                                                                                     0
                                                                                         as
                                                                                         *const libc::c_char,},},
                                              uvstream: 0 as *mut uv_stream_t,
                                              uvbuf:
                                                  uv_buf_t{base:
                                                               0 as
                                                                   *mut libc::c_char,
                                                           len: 0,},
                                              buffer: 0 as *mut RBuffer,
                                              fd: 0,
                                              read_cb: None,
                                              write_cb: None,
                                              cb_data: 0 as *mut libc::c_void,
                                              close_cb: None,
                                              internal_close_cb: None,
                                              close_cb_data:
                                                  0 as *mut libc::c_void,
                                              internal_data:
                                                  0 as *mut libc::c_void,
                                              fpos: 0,
                                              curmem: 0,
                                              maxmem: 0,
                                              pending_reqs: 0,
                                              num_bytes: 0,
                                              events: 0 as *mut MultiQueue,},
                                   cb: None,
                                   internal_exit_cb: None,
                                   internal_close_cb: None,
                                   closed: false,
                                   detach: false,
                                   events: 0 as *mut MultiQueue,},
                       term_name: 0 as *mut libc::c_char,
                       width: 0,
                       height: 0,
                       winsize:
                           winsize{ws_row: 0,
                                   ws_col: 0,
                                   ws_xpixel: 0,
                                   ws_ypixel: 0,},
                       tty_fd: 0,};
        rv.process = process_init(loop_0, kProcessTypePty, data);
        rv.term_name = NULL_1 as *mut libc::c_char;
        rv.width = 80 as libc::c_int as uint16_t;
        rv.height = 24 as libc::c_int as uint16_t;
        rv.tty_fd = -(1 as libc::c_int);
        return rv;
    }
    use super::process_h::{Process, ProcessType, process_exit_cb,
                           internal_process_cb, process_init,
                           kProcessTypePty};
    use super::stdint_uintn_h::{uint16_t, uint8_t, uint64_t};
    use super::ioctl_types_h::winsize;
    use super::loop_h::Loop;
    use super::stream_h::{Stream, C2RustUnnamed_24, stream_read_cb,
                          stream_write_cb, stream_close_cb};
    use super::multiqueue_h::MultiQueue;
    use super::uv_h::uv_stream_t;
    use super::unix_h::{uv_buf_t, uv_file};
    use super::rbuffer_h::RBuffer;
    use super::stddef_h::{size_t, NULL_1};
    // NVIM_OS_PTY_PROCESS_UNIX_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/libuv_process.h:16"]
pub mod libuv_process_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:16"]
    pub struct libuv_process {
        pub process: Process,
        pub uv: uv_process_t,
        pub uvopts: uv_process_options_t,
        pub uvstdio: [uv_stdio_container_t; 3],
    }
    #[c2rust::src_loc = "8:1"]
    pub type LibuvProcess = libuv_process;
    #[inline]
    #[c2rust::src_loc = "15:1"]
    pub unsafe extern "C" fn libuv_process_init(mut loop_0: *mut Loop,
                                                mut data: *mut libc::c_void)
     -> LibuvProcess {
        let mut rv =
            {
                let mut init =
                    libuv_process{process:
                                      process_init(loop_0, kProcessTypeUv,
                                                   data),
                                  uv:
                                      uv_process_t{data:
                                                       0 as *mut libc::c_void,
                                                   loop_0:
                                                       0 as *mut uv_loop_t,
                                                   type_0: UV_UNKNOWN_HANDLE,
                                                   close_cb: None,
                                                   handle_queue:
                                                       [0 as
                                                            *mut libc::c_void;
                                                           2],
                                                   u:
                                                       C2RustUnnamed_22{fd:
                                                                            0,},
                                                   next_closing:
                                                       0 as *mut uv_handle_t,
                                                   flags: 0,
                                                   exit_cb: None,
                                                   pid: 0,
                                                   queue:
                                                       [0 as
                                                            *mut libc::c_void;
                                                           2],
                                                   status: 0,},
                                  uvopts:
                                      uv_process_options_t{exit_cb: None,
                                                           file:
                                                               0 as
                                                                   *const libc::c_char,
                                                           args:
                                                               0 as
                                                                   *mut *mut libc::c_char,
                                                           env:
                                                               0 as
                                                                   *mut *mut libc::c_char,
                                                           cwd:
                                                               0 as
                                                                   *const libc::c_char,
                                                           flags: 0,
                                                           stdio_count: 0,
                                                           stdio:
                                                               0 as
                                                                   *mut uv_stdio_container_t,
                                                           uid: 0,
                                                           gid: 0,},
                                  uvstdio:
                                      [uv_stdio_container_t{flags: UV_IGNORE,
                                                            data:
                                                                C2RustUnnamed_23{stream:
                                                                                     0
                                                                                         as
                                                                                         *mut uv_stream_t,},};
                                          3],};
                init
            };
        return rv;
    }
    use super::process_h::{Process, process_init, kProcessTypeUv,
                           ProcessType};
    use super::uv_h::{uv_process_t, uv_process_options_t,
                      uv_stdio_container_t, uv_loop_t, uv_handle_type,
                      uv_close_cb, C2RustUnnamed_22, uv_handle_t, uv_exit_cb,
                      uv_stdio_flags, C2RustUnnamed_23, UV_IGNORE,
                      uv_stream_t};
    use super::loop_h::Loop;
    use super::unix_h::{uv_uid_t, uv_gid_t};
    // NVIM_EVENT_LIBUV_PROCESS_H
}
#[c2rust::header_src =
  "/home/vole/neovim/.deps/usr/include/msgpack/zone.h:16"]
pub mod zone_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:16"]
    pub struct msgpack_zone_finalizer {
        pub func: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub data: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:16"]
    pub struct msgpack_zone_finalizer_array {
        pub tail: *mut msgpack_zone_finalizer,
        pub end: *mut msgpack_zone_finalizer,
        pub array: *mut msgpack_zone_finalizer,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:16"]
    pub struct msgpack_zone_chunk_list {
        pub free: size_t,
        pub ptr: *mut libc::c_char,
        pub head: *mut msgpack_zone_chunk,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:16"]
    pub struct msgpack_zone {
        pub chunk_list: msgpack_zone_chunk_list,
        pub finalizer_array: msgpack_zone_finalizer_array,
        pub chunk_size: size_t,
    }
    #[inline]
    #[c2rust::src_loc = "106:1"]
    pub unsafe extern "C" fn msgpack_zone_malloc(mut zone: *mut msgpack_zone,
                                                 mut size: size_t)
     -> *mut libc::c_void {
        let mut aligned =
            ((*zone).chunk_list.ptr.offset(MSGPACK_ZONE_ALIGN.wrapping_sub(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong)
                                               as isize) as
                 size_t).wrapping_div(MSGPACK_ZONE_ALIGN).wrapping_mul(MSGPACK_ZONE_ALIGN)
                as *mut libc::c_char;
        let mut adjusted_size =
            size.wrapping_add(aligned.wrapping_offset_from((*zone).chunk_list.ptr)
                                  as libc::c_long as libc::c_ulong);
        if (*zone).chunk_list.free >= adjusted_size {
            (*zone).chunk_list.free =
                ((*zone).chunk_list.free as
                     libc::c_ulong).wrapping_sub(adjusted_size) as size_t as
                    size_t;
            (*zone).chunk_list.ptr =
                (*zone).chunk_list.ptr.offset(adjusted_size as isize);
            return aligned as *mut libc::c_void
        }
        let mut ptr =
            msgpack_zone_malloc_expand(zone,
                                       size.wrapping_add(MSGPACK_ZONE_ALIGN.wrapping_sub(1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong)));
        if !ptr.is_null() {
            return (ptr as
                        size_t).wrapping_div(MSGPACK_ZONE_ALIGN).wrapping_mul(MSGPACK_ZONE_ALIGN)
                       as *mut libc::c_char as *mut libc::c_void
        }
        return NULL_1 as *mut libc::c_void;
    }
    #[c2rust::src_loc = "84:9"]
    pub const MSGPACK_ZONE_ALIGN: libc::c_ulong =
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong;
    #[inline]
    #[c2rust::src_loc = "90:1"]
    pub unsafe extern "C" fn msgpack_zone_malloc_no_align(mut zone:
                                                              *mut msgpack_zone,
                                                          mut size: size_t)
     -> *mut libc::c_void {
        let mut ptr = 0 as *mut libc::c_char;
        let mut cl: *mut msgpack_zone_chunk_list = &mut (*zone).chunk_list;
        if (*zone).chunk_list.free < size {
            return msgpack_zone_malloc_expand(zone, size)
        }
        ptr = (*cl).ptr;
        (*cl).free =
            ((*cl).free as libc::c_ulong).wrapping_sub(size) as size_t as
                size_t;
        (*cl).ptr = (*cl).ptr.offset(size as isize);
        return ptr as *mut libc::c_void;
    }
    #[inline]
    #[c2rust::src_loc = "133:1"]
    pub unsafe extern "C" fn msgpack_zone_push_finalizer(mut zone:
                                                             *mut msgpack_zone,
                                                         mut func:
                                                             Option<unsafe extern "C" fn(_:
                                                                                             *mut libc::c_void)
                                                                        ->
                                                                            ()>,
                                                         mut data:
                                                             *mut libc::c_void)
     -> bool {
        let fa: *mut msgpack_zone_finalizer_array =
            &mut (*zone).finalizer_array;
        let mut fin = (*fa).tail;
        if fin == (*fa).end {
            return msgpack_zone_push_finalizer_expand(zone, func, data)
        }
        (*fin).func = func;
        (*fin).data = data;
        (*fa).tail = (*fa).tail.offset(1);
        return true_0 != 0;
    }
    #[inline]
    #[c2rust::src_loc = "151:1"]
    pub unsafe extern "C" fn msgpack_zone_swap(mut a: *mut msgpack_zone,
                                               mut b: *mut msgpack_zone) {
        let mut tmp = *a;
        *a = *b;
        *b = tmp;
    }
    use super::stddef_h::{size_t, NULL_1};
    use super::stdbool_h::true_0;
    extern "C" {
        #[c2rust::src_loc = "37:8"]
        pub type msgpack_zone_chunk;
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn msgpack_zone_free(zone: *mut msgpack_zone);
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn msgpack_zone_malloc_expand(zone: *mut msgpack_zone,
                                          size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "130:1"]
        pub fn msgpack_zone_push_finalizer_expand(zone: *mut msgpack_zone,
                                                  func:
                                                      Option<unsafe extern "C" fn(_:
                                                                                      *mut libc::c_void)
                                                                 -> ()>,
                                                  data: *mut libc::c_void)
         -> bool;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/.deps/usr/include/msgpack/object.h:16"]
pub mod object_h {
    #[c2rust::src_loc = "27:9"]
    pub type msgpack_object_type = libc::c_uint;
    #[c2rust::src_loc = "42:5"]
    pub const MSGPACK_OBJECT_EXT: msgpack_object_type = 9;
    #[c2rust::src_loc = "41:5"]
    pub const MSGPACK_OBJECT_BIN: msgpack_object_type = 8;
    #[c2rust::src_loc = "40:5"]
    pub const MSGPACK_OBJECT_MAP: msgpack_object_type = 7;
    #[c2rust::src_loc = "39:5"]
    pub const MSGPACK_OBJECT_ARRAY: msgpack_object_type = 6;
    #[c2rust::src_loc = "38:5"]
    pub const MSGPACK_OBJECT_STR: msgpack_object_type = 5;
    #[c2rust::src_loc = "34:5"]
    pub const MSGPACK_OBJECT_FLOAT: msgpack_object_type = 4;
    #[c2rust::src_loc = "33:5"]
    pub const MSGPACK_OBJECT_FLOAT64: msgpack_object_type = 4;
    #[c2rust::src_loc = "32:5"]
    pub const MSGPACK_OBJECT_FLOAT32: msgpack_object_type = 10;
    #[c2rust::src_loc = "31:5"]
    pub const MSGPACK_OBJECT_NEGATIVE_INTEGER: msgpack_object_type = 3;
    #[c2rust::src_loc = "30:5"]
    pub const MSGPACK_OBJECT_POSITIVE_INTEGER: msgpack_object_type = 2;
    #[c2rust::src_loc = "29:5"]
    pub const MSGPACK_OBJECT_BOOLEAN: msgpack_object_type = 1;
    #[c2rust::src_loc = "28:5"]
    pub const MSGPACK_OBJECT_NIL: msgpack_object_type = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "90:16"]
    pub struct msgpack_object {
        pub type_0: msgpack_object_type,
        pub via: msgpack_object_union,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "75:9"]
    pub union msgpack_object_union {
        pub boolean: bool,
        pub u64_0: uint64_t,
        pub i64_0: int64_t,
        pub f64_0: libc::c_double,
        pub array: msgpack_object_array,
        pub map: msgpack_object_map,
        pub str_0: msgpack_object_str,
        pub bin: msgpack_object_bin,
        pub ext: msgpack_object_ext,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "69:9"]
    pub struct msgpack_object_ext {
        pub type_0: int8_t,
        pub size: uint32_t,
        pub ptr: *const libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "64:9"]
    pub struct msgpack_object_bin {
        pub size: uint32_t,
        pub ptr: *const libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "59:9"]
    pub struct msgpack_object_str {
        pub size: uint32_t,
        pub ptr: *const libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "54:9"]
    pub struct msgpack_object_map {
        pub size: uint32_t,
        pub ptr: *mut msgpack_object_kv,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "95:16"]
    pub struct msgpack_object_kv {
        pub key: msgpack_object,
        pub val: msgpack_object,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:9"]
    pub struct msgpack_object_array {
        pub size: uint32_t,
        pub ptr: *mut msgpack_object,
    }
    use super::stdint_uintn_h::{uint64_t, uint32_t};
    use super::stdint_intn_h::{int64_t, int8_t};
}
#[c2rust::header_src =
  "/home/vole/neovim/.deps/usr/include/msgpack/pack.h:16"]
pub mod pack_h {
    #[c2rust::src_loc = "35:1"]
    pub type msgpack_packer_write
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const libc::c_char, _: size_t)
                   -> libc::c_int>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:16"]
    pub struct msgpack_packer {
        pub data: *mut libc::c_void,
        pub callback: msgpack_packer_write,
    }
    #[inline]
    #[c2rust::src_loc = "124:1"]
    pub unsafe extern "C" fn msgpack_packer_init(mut pk: *mut msgpack_packer,
                                                 mut data: *mut libc::c_void,
                                                 mut callback:
                                                     msgpack_packer_write) {
        (*pk).data = data;
        (*pk).callback = callback;
    }
    #[inline]
    #[c2rust::src_loc = "130:1"]
    pub unsafe extern "C" fn msgpack_packer_new(mut data: *mut libc::c_void,
                                                mut callback:
                                                    msgpack_packer_write)
     -> *mut msgpack_packer {
        let mut pk =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<msgpack_packer>() as libc::c_ulong)
                as *mut msgpack_packer;
        if pk.is_null() { return NULL_1 as *mut msgpack_packer }
        msgpack_packer_init(pk, data, callback);
        return pk;
    }
    #[inline]
    #[c2rust::src_loc = "138:1"]
    pub unsafe extern "C" fn msgpack_packer_free(mut pk:
                                                     *mut msgpack_packer) {
        free(pk as *mut libc::c_void);
    }
    use super::stddef_h::{size_t, NULL_1};
    use super::stdlib_h::{calloc, free};
}
#[c2rust::header_src =
  "/home/vole/neovim/.deps/usr/include/msgpack/pack_template.h:16"]
pub mod pack_template_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "654:5"]
    pub union C2RustUnnamed_25 {
        pub f: libc::c_float,
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "663:5"]
    pub union C2RustUnnamed_26 {
        pub f: libc::c_double,
        pub i: uint64_t,
    }
    #[inline]
    #[c2rust::src_loc = "369:1"]
    pub unsafe extern "C" fn msgpack_pack_char(mut x: *mut msgpack_packer,
                                               mut d: libc::c_char)
     -> libc::c_int {
        if (d as libc::c_int) < -((1 as libc::c_int) << 5 as libc::c_int) {
            let mut buf: [libc::c_uchar; 2] =
                [0xd0 as libc::c_int as libc::c_uchar,
                 *(&mut d as *mut libc::c_char as
                       *mut uint8_t).offset(0 as libc::c_int as isize)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut libc::c_char
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "382:1"]
    pub unsafe extern "C" fn msgpack_pack_signed_char(mut x:
                                                          *mut msgpack_packer,
                                                      mut d: libc::c_schar)
     -> libc::c_int {
        if (d as libc::c_int) < -((1 as libc::c_int) << 5 as libc::c_int) {
            let mut buf: [libc::c_uchar; 2] =
                [0xd0 as libc::c_int as libc::c_uchar,
                 *(&mut d as *mut libc::c_schar as
                       *mut uint8_t).offset(0 as libc::c_int as isize)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut libc::c_schar
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "394:1"]
    pub unsafe extern "C" fn msgpack_pack_short(mut x: *mut msgpack_packer,
                                                mut d: libc::c_short)
     -> libc::c_int {
        if (d as libc::c_int) < -((1 as libc::c_int) << 5 as libc::c_int) {
            if (d as libc::c_int) < -((1 as libc::c_int) << 7 as libc::c_int)
               {
                let mut buf: [libc::c_uchar; 3] = [0; 3];
                buf[0 as libc::c_int as usize] =
                    0xd1 as libc::c_int as libc::c_uchar;
                let mut val = ntohs(d as uint16_t);
                memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as
                                                         isize) as
                           *mut libc::c_uchar as *mut libc::c_void,
                       &mut val as *mut uint16_t as *const libc::c_void,
                       2 as libc::c_int as libc::c_ulong);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   3
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else {
                let mut buf_0: [libc::c_uchar; 2] =
                    [0xd0 as libc::c_int as libc::c_uchar,
                     *(&mut d as *mut libc::c_short as
                           *mut uint8_t).offset(0 as libc::c_int as isize)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_0.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
        } else if (d as libc::c_int) < (1 as libc::c_int) << 7 as libc::c_int
         {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut libc::c_short
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if (d as libc::c_int) < (1 as libc::c_int) << 8 as libc::c_int
         {
            let mut buf_1: [libc::c_uchar; 2] =
                [0xcc as libc::c_int as libc::c_uchar,
                 *(&mut d as *mut libc::c_short as
                       *mut uint8_t).offset(0 as libc::c_int as isize)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_1.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_2: [libc::c_uchar; 3] = [0; 3];
            buf_2[0 as libc::c_int as usize] =
                0xcd as libc::c_int as libc::c_uchar;
            let mut val_0 = ntohs(d as uint16_t);
            memcpy(&mut *buf_2.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_0 as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_2.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "425:1"]
    pub unsafe extern "C" fn msgpack_pack_int(mut x: *mut msgpack_packer,
                                              mut d: libc::c_int)
     -> libc::c_int {
        if d < -((1 as libc::c_int) << 5 as libc::c_int) {
            if d < -((1 as libc::c_int) << 15 as libc::c_int) {
                let mut buf: [libc::c_uchar; 5] = [0; 5];
                buf[0 as libc::c_int as usize] =
                    0xd2 as libc::c_int as libc::c_uchar;
                let mut val = ntohl(d as uint32_t);
                memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as
                                                         isize) as
                           *mut libc::c_uchar as *mut libc::c_void,
                       &mut val as *mut uint32_t as *const libc::c_void,
                       4 as libc::c_int as libc::c_ulong);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   5
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else if d < -((1 as libc::c_int) << 7 as libc::c_int) {
                let mut buf_0: [libc::c_uchar; 3] = [0; 3];
                buf_0[0 as libc::c_int as usize] =
                    0xd1 as libc::c_int as libc::c_uchar;
                let mut val_0 = ntohs(d as int16_t as uint16_t);
                memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as
                                                           isize) as
                           *mut libc::c_uchar as *mut libc::c_void,
                       &mut val_0 as *mut uint16_t as *const libc::c_void,
                       2 as libc::c_int as libc::c_ulong);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_0.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   3
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else {
                let mut buf_1: [libc::c_uchar; 2] =
                    [0xd0 as libc::c_int as libc::c_uchar,
                     *(&mut d as *mut libc::c_int as
                           *mut uint8_t).offset(0 as libc::c_int as isize)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_1.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
        } else if d < (1 as libc::c_int) << 7 as libc::c_int {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut libc::c_int
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if d < (1 as libc::c_int) << 8 as libc::c_int {
            let mut buf_2: [libc::c_uchar; 2] =
                [0xcc as libc::c_int as libc::c_uchar,
                 *(&mut d as *mut libc::c_int as
                       *mut uint8_t).offset(0 as libc::c_int as isize)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_2.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if d < (1 as libc::c_int) << 16 as libc::c_int {
            let mut buf_3: [libc::c_uchar; 3] = [0; 3];
            buf_3[0 as libc::c_int as usize] =
                0xcd as libc::c_int as libc::c_uchar;
            let mut val_1 = ntohs(d as uint16_t);
            memcpy(&mut *buf_3.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_1 as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_3.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_4: [libc::c_uchar; 5] = [0; 5];
            buf_4[0 as libc::c_int as usize] =
                0xce as libc::c_int as libc::c_uchar;
            let mut val_2 = ntohl(d as uint32_t);
            memcpy(&mut *buf_4.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_2 as *mut uint32_t as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_4.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "456:1"]
    pub unsafe extern "C" fn msgpack_pack_long(mut x: *mut msgpack_packer,
                                               mut d: libc::c_long)
     -> libc::c_int {
        if (d as libc::c_longlong) <
               -((1 as libc::c_longlong) << 5 as libc::c_int) {
            if (d as libc::c_longlong) <
                   -((1 as libc::c_longlong) << 15 as libc::c_int) {
                if (d as libc::c_longlong) <
                       -((1 as libc::c_longlong) << 31 as libc::c_int) {
                    let mut buf: [libc::c_uchar; 9] = [0; 9];
                    buf[0 as libc::c_int as usize] =
                        0xd3 as libc::c_int as libc::c_uchar;
                    let mut val = __bswap_64(d as __uint64_t);
                    memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize) as
                               *mut libc::c_uchar as *mut libc::c_void,
                           &mut val as *mut uint64_t as *const libc::c_void,
                           8 as libc::c_int as libc::c_ulong);
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                       buf.as_mut_ptr()
                                                                                                                           as
                                                                                                                           *const libc::c_char,
                                                                                                                       9
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           size_t)
                } else {
                    let mut buf_0: [libc::c_uchar; 5] = [0; 5];
                    buf_0[0 as libc::c_int as usize] =
                        0xd2 as libc::c_int as libc::c_uchar;
                    let mut val_0 = ntohl(d as int32_t as uint32_t);
                    memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as
                                                               isize) as
                               *mut libc::c_uchar as *mut libc::c_void,
                           &mut val_0 as *mut uint32_t as *const libc::c_void,
                           4 as libc::c_int as libc::c_ulong);
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                       buf_0.as_mut_ptr()
                                                                                                                           as
                                                                                                                           *const libc::c_char,
                                                                                                                       5
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           size_t)
                }
            } else if d <
                          -((1 as libc::c_int) << 7 as libc::c_int) as
                              libc::c_long {
                let mut buf_1: [libc::c_uchar; 3] = [0; 3];
                buf_1[0 as libc::c_int as usize] =
                    0xd1 as libc::c_int as libc::c_uchar;
                let mut val_1 = ntohs(d as int16_t as uint16_t);
                memcpy(&mut *buf_1.as_mut_ptr().offset(1 as libc::c_int as
                                                           isize) as
                           *mut libc::c_uchar as *mut libc::c_void,
                       &mut val_1 as *mut uint16_t as *const libc::c_void,
                       2 as libc::c_int as libc::c_ulong);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_1.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   3
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else {
                let mut buf_2: [libc::c_uchar; 2] =
                    [0xd0 as libc::c_int as libc::c_uchar,
                     *(&mut d as *mut libc::c_long as
                           *mut uint8_t).offset(0 as libc::c_int as isize)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_2.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
        } else if d < ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_long
         {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut libc::c_long
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if (d as libc::c_longlong) <
                      (1 as libc::c_longlong) << 16 as libc::c_int {
            if d < ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_long {
                let mut buf_3: [libc::c_uchar; 2] =
                    [0xcc as libc::c_int as libc::c_uchar,
                     *(&mut d as *mut libc::c_long as
                           *mut uint8_t).offset(0 as libc::c_int as isize)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_3.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else {
                let mut buf_4: [libc::c_uchar; 3] = [0; 3];
                buf_4[0 as libc::c_int as usize] =
                    0xcd as libc::c_int as libc::c_uchar;
                let mut val_2 = ntohs(d as uint16_t);
                memcpy(&mut *buf_4.as_mut_ptr().offset(1 as libc::c_int as
                                                           isize) as
                           *mut libc::c_uchar as *mut libc::c_void,
                       &mut val_2 as *mut uint16_t as *const libc::c_void,
                       2 as libc::c_int as libc::c_ulong);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_4.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   3
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
        } else if (d as libc::c_longlong) <
                      (1 as libc::c_longlong) << 32 as libc::c_int {
            let mut buf_5: [libc::c_uchar; 5] = [0; 5];
            buf_5[0 as libc::c_int as usize] =
                0xce as libc::c_int as libc::c_uchar;
            let mut val_3 = ntohl(d as uint32_t);
            memcpy(&mut *buf_5.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_3 as *mut uint32_t as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_5.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_6: [libc::c_uchar; 9] = [0; 9];
            buf_6[0 as libc::c_int as usize] =
                0xcf as libc::c_int as libc::c_uchar;
            let mut val_4 = __bswap_64(d as __uint64_t);
            memcpy(&mut *buf_6.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_4 as *mut uint64_t as *const libc::c_void,
                   8 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_6.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               9
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "487:1"]
    pub unsafe extern "C" fn msgpack_pack_long_long(mut x:
                                                        *mut msgpack_packer,
                                                    mut d: libc::c_longlong)
     -> libc::c_int {
        if d < -((1 as libc::c_longlong) << 5 as libc::c_int) {
            if d < -((1 as libc::c_longlong) << 15 as libc::c_int) {
                if d < -((1 as libc::c_longlong) << 31 as libc::c_int) {
                    let mut buf: [libc::c_uchar; 9] = [0; 9];
                    buf[0 as libc::c_int as usize] =
                        0xd3 as libc::c_int as libc::c_uchar;
                    let mut val = __bswap_64(d as __uint64_t);
                    memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize) as
                               *mut libc::c_uchar as *mut libc::c_void,
                           &mut val as *mut uint64_t as *const libc::c_void,
                           8 as libc::c_int as libc::c_ulong);
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                       buf.as_mut_ptr()
                                                                                                                           as
                                                                                                                           *const libc::c_char,
                                                                                                                       9
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           size_t)
                } else {
                    let mut buf_0: [libc::c_uchar; 5] = [0; 5];
                    buf_0[0 as libc::c_int as usize] =
                        0xd2 as libc::c_int as libc::c_uchar;
                    let mut val_0 = ntohl(d as int32_t as uint32_t);
                    memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as
                                                               isize) as
                               *mut libc::c_uchar as *mut libc::c_void,
                           &mut val_0 as *mut uint32_t as *const libc::c_void,
                           4 as libc::c_int as libc::c_ulong);
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                       buf_0.as_mut_ptr()
                                                                                                                           as
                                                                                                                           *const libc::c_char,
                                                                                                                       5
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           size_t)
                }
            } else if d <
                          -((1 as libc::c_int) << 7 as libc::c_int) as
                              libc::c_longlong {
                let mut buf_1: [libc::c_uchar; 3] = [0; 3];
                buf_1[0 as libc::c_int as usize] =
                    0xd1 as libc::c_int as libc::c_uchar;
                let mut val_1 = ntohs(d as int16_t as uint16_t);
                memcpy(&mut *buf_1.as_mut_ptr().offset(1 as libc::c_int as
                                                           isize) as
                           *mut libc::c_uchar as *mut libc::c_void,
                       &mut val_1 as *mut uint16_t as *const libc::c_void,
                       2 as libc::c_int as libc::c_ulong);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_1.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   3
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else {
                let mut buf_2: [libc::c_uchar; 2] =
                    [0xd0 as libc::c_int as libc::c_uchar,
                     *(&mut d as *mut libc::c_longlong as
                           *mut uint8_t).offset(0 as libc::c_int as isize)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_2.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
        } else if d <
                      ((1 as libc::c_int) << 7 as libc::c_int) as
                          libc::c_longlong {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut libc::c_longlong
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if d < (1 as libc::c_longlong) << 16 as libc::c_int {
            if d <
                   ((1 as libc::c_int) << 8 as libc::c_int) as
                       libc::c_longlong {
                let mut buf_3: [libc::c_uchar; 2] =
                    [0xcc as libc::c_int as libc::c_uchar,
                     *(&mut d as *mut libc::c_longlong as
                           *mut uint8_t).offset(0 as libc::c_int as isize)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_3.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else {
                let mut buf_4: [libc::c_uchar; 3] = [0; 3];
                buf_4[0 as libc::c_int as usize] =
                    0xcd as libc::c_int as libc::c_uchar;
                let mut val_2 = ntohs(d as uint16_t);
                memcpy(&mut *buf_4.as_mut_ptr().offset(1 as libc::c_int as
                                                           isize) as
                           *mut libc::c_uchar as *mut libc::c_void,
                       &mut val_2 as *mut uint16_t as *const libc::c_void,
                       2 as libc::c_int as libc::c_ulong);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_4.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   3
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
        } else if d < (1 as libc::c_longlong) << 32 as libc::c_int {
            let mut buf_5: [libc::c_uchar; 5] = [0; 5];
            buf_5[0 as libc::c_int as usize] =
                0xce as libc::c_int as libc::c_uchar;
            let mut val_3 = ntohl(d as uint32_t);
            memcpy(&mut *buf_5.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_3 as *mut uint32_t as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_5.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_6: [libc::c_uchar; 9] = [0; 9];
            buf_6[0 as libc::c_int as usize] =
                0xcf as libc::c_int as libc::c_uchar;
            let mut val_4 = __bswap_64(d as __uint64_t);
            memcpy(&mut *buf_6.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_4 as *mut uint64_t as *const libc::c_void,
                   8 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_6.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               9
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "387:1"]
    pub unsafe extern "C" fn msgpack_pack_unsigned_char(mut x:
                                                            *mut msgpack_packer,
                                                        mut d: libc::c_uchar)
     -> libc::c_int {
        if (d as libc::c_int) < (1 as libc::c_int) << 7 as libc::c_int {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut libc::c_uchar
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf: [libc::c_uchar; 2] =
                [0xcc as libc::c_int as libc::c_uchar,
                 *(&mut d as *mut libc::c_uchar as
                       *mut uint8_t).offset(0 as libc::c_int as isize)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "518:1"]
    pub unsafe extern "C" fn msgpack_pack_unsigned_short(mut x:
                                                             *mut msgpack_packer,
                                                         mut d:
                                                             libc::c_ushort)
     -> libc::c_int {
        if (d as libc::c_int) < (1 as libc::c_int) << 7 as libc::c_int {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut libc::c_ushort
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if (d as libc::c_int) < (1 as libc::c_int) << 8 as libc::c_int
         {
            let mut buf: [libc::c_uchar; 2] =
                [0xcc as libc::c_int as libc::c_uchar,
                 *(&mut d as *mut libc::c_ushort as
                       *mut uint8_t).offset(0 as libc::c_int as isize)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as libc::c_int as usize] =
                0xcd as libc::c_int as libc::c_uchar;
            let mut val = ntohs(d);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_0.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "549:1"]
    pub unsafe extern "C" fn msgpack_pack_unsigned_int(mut x:
                                                           *mut msgpack_packer,
                                                       mut d: libc::c_uint)
     -> libc::c_int {
        if d < ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint {
            if d < ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint {
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   &mut *(&mut d
                                                                                                                              as
                                                                                                                              *mut libc::c_uint
                                                                                                                              as
                                                                                                                              *mut uint8_t).offset(0
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       as
                                                                                                                                                       isize)
                                                                                                                       as
                                                                                                                       *mut uint8_t
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   1
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else {
                let mut buf: [libc::c_uchar; 2] =
                    [0xcc as libc::c_int as libc::c_uchar,
                     *(&mut d as *mut libc::c_uint as
                           *mut uint8_t).offset(0 as libc::c_int as isize)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
        } else if d <
                      ((1 as libc::c_int) << 16 as libc::c_int) as
                          libc::c_uint {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as libc::c_int as usize] =
                0xcd as libc::c_int as libc::c_uchar;
            let mut val = ntohs(d as uint16_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_0.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_1: [libc::c_uchar; 5] = [0; 5];
            buf_1[0 as libc::c_int as usize] =
                0xce as libc::c_int as libc::c_uchar;
            let mut val_0 = ntohl(d);
            memcpy(&mut *buf_1.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_0 as *mut uint32_t as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_1.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "580:1"]
    pub unsafe extern "C" fn msgpack_pack_unsigned_long(mut x:
                                                            *mut msgpack_packer,
                                                        mut d: libc::c_ulong)
     -> libc::c_int {
        if (d as libc::c_ulonglong) <
               (1 as libc::c_ulonglong) << 8 as libc::c_int {
            if (d as libc::c_ulonglong) <
                   (1 as libc::c_ulonglong) << 7 as libc::c_int {
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   &mut *(&mut d
                                                                                                                              as
                                                                                                                              *mut libc::c_ulong
                                                                                                                              as
                                                                                                                              *mut uint8_t).offset(0
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       as
                                                                                                                                                       isize)
                                                                                                                       as
                                                                                                                       *mut uint8_t
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   1
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else {
                let mut buf: [libc::c_uchar; 2] =
                    [0xcc as libc::c_int as libc::c_uchar,
                     *(&mut d as *mut libc::c_ulong as
                           *mut uint8_t).offset(0 as libc::c_int as isize)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
        } else if (d as libc::c_ulonglong) <
                      (1 as libc::c_ulonglong) << 16 as libc::c_int {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as libc::c_int as usize] =
                0xcd as libc::c_int as libc::c_uchar;
            let mut val = ntohs(d as uint16_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_0.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if (d as libc::c_ulonglong) <
                      (1 as libc::c_ulonglong) << 32 as libc::c_int {
            let mut buf_1: [libc::c_uchar; 5] = [0; 5];
            buf_1[0 as libc::c_int as usize] =
                0xce as libc::c_int as libc::c_uchar;
            let mut val_0 = ntohl(d as uint32_t);
            memcpy(&mut *buf_1.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_0 as *mut uint32_t as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_1.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_2: [libc::c_uchar; 9] = [0; 9];
            buf_2[0 as libc::c_int as usize] =
                0xcf as libc::c_int as libc::c_uchar;
            let mut val_1 = __bswap_64(d);
            memcpy(&mut *buf_2.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_1 as *mut uint64_t as *const libc::c_void,
                   8 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_2.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               9
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "611:1"]
    pub unsafe extern "C" fn msgpack_pack_unsigned_long_long(mut x:
                                                                 *mut msgpack_packer,
                                                             mut d:
                                                                 libc::c_ulonglong)
     -> libc::c_int {
        if d < (1 as libc::c_ulonglong) << 8 as libc::c_int {
            if d < (1 as libc::c_ulonglong) << 7 as libc::c_int {
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   &mut *(&mut d
                                                                                                                              as
                                                                                                                              *mut libc::c_ulonglong
                                                                                                                              as
                                                                                                                              *mut uint8_t).offset(0
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       as
                                                                                                                                                       isize)
                                                                                                                       as
                                                                                                                       *mut uint8_t
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   1
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else {
                let mut buf: [libc::c_uchar; 2] =
                    [0xcc as libc::c_int as libc::c_uchar,
                     *(&mut d as *mut libc::c_ulonglong as
                           *mut uint8_t).offset(0 as libc::c_int as isize)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
        } else if d < (1 as libc::c_ulonglong) << 16 as libc::c_int {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as libc::c_int as usize] =
                0xcd as libc::c_int as libc::c_uchar;
            let mut val = ntohs(d as uint16_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_0.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if d < (1 as libc::c_ulonglong) << 32 as libc::c_int {
            let mut buf_1: [libc::c_uchar; 5] = [0; 5];
            buf_1[0 as libc::c_int as usize] =
                0xce as libc::c_int as libc::c_uchar;
            let mut val_0 = ntohl(d as uint32_t);
            memcpy(&mut *buf_1.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_0 as *mut uint32_t as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_1.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_2: [libc::c_uchar; 9] = [0; 9];
            buf_2[0 as libc::c_int as usize] =
                0xcf as libc::c_int as libc::c_uchar;
            let mut val_1 = __bswap_64(d as __uint64_t);
            memcpy(&mut *buf_2.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_1 as *mut uint64_t as *const libc::c_void,
                   8 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_2.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               9
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "329:1"]
    pub unsafe extern "C" fn msgpack_pack_uint8(mut x: *mut msgpack_packer,
                                                mut d: uint8_t)
     -> libc::c_int {
        if (d as libc::c_int) < (1 as libc::c_int) << 7 as libc::c_int {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf: [libc::c_uchar; 2] =
                [0xcc as libc::c_int as libc::c_uchar,
                 *(&mut d as *mut uint8_t).offset(0 as libc::c_int as isize)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "334:1"]
    pub unsafe extern "C" fn msgpack_pack_uint16(mut x: *mut msgpack_packer,
                                                 mut d: uint16_t)
     -> libc::c_int {
        if (d as libc::c_int) < (1 as libc::c_int) << 7 as libc::c_int {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut uint16_t
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if (d as libc::c_int) < (1 as libc::c_int) << 8 as libc::c_int
         {
            let mut buf: [libc::c_uchar; 2] =
                [0xcc as libc::c_int as libc::c_uchar,
                 *(&mut d as *mut uint16_t as
                       *mut uint8_t).offset(0 as libc::c_int as isize)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as libc::c_int as usize] =
                0xcd as libc::c_int as libc::c_uchar;
            let mut val = ntohs(d);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_0.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "339:1"]
    pub unsafe extern "C" fn msgpack_pack_uint32(mut x: *mut msgpack_packer,
                                                 mut d: uint32_t)
     -> libc::c_int {
        if d < ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint {
            if d < ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint {
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   &mut *(&mut d
                                                                                                                              as
                                                                                                                              *mut uint32_t
                                                                                                                              as
                                                                                                                              *mut uint8_t).offset(0
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       as
                                                                                                                                                       isize)
                                                                                                                       as
                                                                                                                       *mut uint8_t
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   1
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else {
                let mut buf: [libc::c_uchar; 2] =
                    [0xcc as libc::c_int as libc::c_uchar,
                     *(&mut d as *mut uint32_t as
                           *mut uint8_t).offset(0 as libc::c_int as isize)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
        } else if d <
                      ((1 as libc::c_int) << 16 as libc::c_int) as
                          libc::c_uint {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as libc::c_int as usize] =
                0xcd as libc::c_int as libc::c_uchar;
            let mut val = ntohs(d as uint16_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_0.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_1: [libc::c_uchar; 5] = [0; 5];
            buf_1[0 as libc::c_int as usize] =
                0xce as libc::c_int as libc::c_uchar;
            let mut val_0 = ntohl(d);
            memcpy(&mut *buf_1.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_0 as *mut uint32_t as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_1.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "344:1"]
    pub unsafe extern "C" fn msgpack_pack_uint64(mut x: *mut msgpack_packer,
                                                 mut d: uint64_t)
     -> libc::c_int {
        if (d as libc::c_ulonglong) <
               (1 as libc::c_ulonglong) << 8 as libc::c_int {
            if (d as libc::c_ulonglong) <
                   (1 as libc::c_ulonglong) << 7 as libc::c_int {
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   &mut *(&mut d
                                                                                                                              as
                                                                                                                              *mut uint64_t
                                                                                                                              as
                                                                                                                              *mut uint8_t).offset(0
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       as
                                                                                                                                                       isize)
                                                                                                                       as
                                                                                                                       *mut uint8_t
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   1
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else {
                let mut buf: [libc::c_uchar; 2] =
                    [0xcc as libc::c_int as libc::c_uchar,
                     *(&mut d as *mut uint64_t as
                           *mut uint8_t).offset(0 as libc::c_int as isize)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
        } else if (d as libc::c_ulonglong) <
                      (1 as libc::c_ulonglong) << 16 as libc::c_int {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as libc::c_int as usize] =
                0xcd as libc::c_int as libc::c_uchar;
            let mut val = ntohs(d as uint16_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_0.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if (d as libc::c_ulonglong) <
                      (1 as libc::c_ulonglong) << 32 as libc::c_int {
            let mut buf_1: [libc::c_uchar; 5] = [0; 5];
            buf_1[0 as libc::c_int as usize] =
                0xce as libc::c_int as libc::c_uchar;
            let mut val_0 = ntohl(d as uint32_t);
            memcpy(&mut *buf_1.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_0 as *mut uint32_t as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_1.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_2: [libc::c_uchar; 9] = [0; 9];
            buf_2[0 as libc::c_int as usize] =
                0xcf as libc::c_int as libc::c_uchar;
            let mut val_1 = __bswap_64(d);
            memcpy(&mut *buf_2.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_1 as *mut uint64_t as *const libc::c_void,
                   8 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_2.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               9
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "349:1"]
    pub unsafe extern "C" fn msgpack_pack_int8(mut x: *mut msgpack_packer,
                                               mut d: int8_t) -> libc::c_int {
        if (d as libc::c_int) < -((1 as libc::c_int) << 5 as libc::c_int) {
            let mut buf: [libc::c_uchar; 2] =
                [0xd0 as libc::c_int as libc::c_uchar,
                 *(&mut d as *mut int8_t as
                       *mut uint8_t).offset(0 as libc::c_int as isize)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut int8_t
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "354:1"]
    pub unsafe extern "C" fn msgpack_pack_int16(mut x: *mut msgpack_packer,
                                                mut d: int16_t)
     -> libc::c_int {
        if (d as libc::c_int) < -((1 as libc::c_int) << 5 as libc::c_int) {
            if (d as libc::c_int) < -((1 as libc::c_int) << 7 as libc::c_int)
               {
                let mut buf: [libc::c_uchar; 3] = [0; 3];
                buf[0 as libc::c_int as usize] =
                    0xd1 as libc::c_int as libc::c_uchar;
                let mut val = ntohs(d as uint16_t);
                memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as
                                                         isize) as
                           *mut libc::c_uchar as *mut libc::c_void,
                       &mut val as *mut uint16_t as *const libc::c_void,
                       2 as libc::c_int as libc::c_ulong);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   3
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else {
                let mut buf_0: [libc::c_uchar; 2] =
                    [0xd0 as libc::c_int as libc::c_uchar,
                     *(&mut d as *mut int16_t as
                           *mut uint8_t).offset(0 as libc::c_int as isize)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_0.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
        } else if (d as libc::c_int) < (1 as libc::c_int) << 7 as libc::c_int
         {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut int16_t
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if (d as libc::c_int) < (1 as libc::c_int) << 8 as libc::c_int
         {
            let mut buf_1: [libc::c_uchar; 2] =
                [0xcc as libc::c_int as libc::c_uchar,
                 *(&mut d as *mut int16_t as
                       *mut uint8_t).offset(0 as libc::c_int as isize)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_1.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_2: [libc::c_uchar; 3] = [0; 3];
            buf_2[0 as libc::c_int as usize] =
                0xcd as libc::c_int as libc::c_uchar;
            let mut val_0 = ntohs(d as uint16_t);
            memcpy(&mut *buf_2.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_0 as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_2.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "359:1"]
    pub unsafe extern "C" fn msgpack_pack_int32(mut x: *mut msgpack_packer,
                                                mut d: int32_t)
     -> libc::c_int {
        if d < -((1 as libc::c_int) << 5 as libc::c_int) {
            if d < -((1 as libc::c_int) << 15 as libc::c_int) {
                let mut buf: [libc::c_uchar; 5] = [0; 5];
                buf[0 as libc::c_int as usize] =
                    0xd2 as libc::c_int as libc::c_uchar;
                let mut val = ntohl(d as uint32_t);
                memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as
                                                         isize) as
                           *mut libc::c_uchar as *mut libc::c_void,
                       &mut val as *mut uint32_t as *const libc::c_void,
                       4 as libc::c_int as libc::c_ulong);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   5
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else if d < -((1 as libc::c_int) << 7 as libc::c_int) {
                let mut buf_0: [libc::c_uchar; 3] = [0; 3];
                buf_0[0 as libc::c_int as usize] =
                    0xd1 as libc::c_int as libc::c_uchar;
                let mut val_0 = ntohs(d as int16_t as uint16_t);
                memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as
                                                           isize) as
                           *mut libc::c_uchar as *mut libc::c_void,
                       &mut val_0 as *mut uint16_t as *const libc::c_void,
                       2 as libc::c_int as libc::c_ulong);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_0.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   3
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else {
                let mut buf_1: [libc::c_uchar; 2] =
                    [0xd0 as libc::c_int as libc::c_uchar,
                     *(&mut d as *mut int32_t as
                           *mut uint8_t).offset(0 as libc::c_int as isize)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_1.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
        } else if d < (1 as libc::c_int) << 7 as libc::c_int {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut int32_t
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if d < (1 as libc::c_int) << 8 as libc::c_int {
            let mut buf_2: [libc::c_uchar; 2] =
                [0xcc as libc::c_int as libc::c_uchar,
                 *(&mut d as *mut int32_t as
                       *mut uint8_t).offset(0 as libc::c_int as isize)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_2.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if d < (1 as libc::c_int) << 16 as libc::c_int {
            let mut buf_3: [libc::c_uchar; 3] = [0; 3];
            buf_3[0 as libc::c_int as usize] =
                0xcd as libc::c_int as libc::c_uchar;
            let mut val_1 = ntohs(d as uint16_t);
            memcpy(&mut *buf_3.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_1 as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_3.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_4: [libc::c_uchar; 5] = [0; 5];
            buf_4[0 as libc::c_int as usize] =
                0xce as libc::c_int as libc::c_uchar;
            let mut val_2 = ntohl(d as uint32_t);
            memcpy(&mut *buf_4.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_2 as *mut uint32_t as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_4.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "364:1"]
    pub unsafe extern "C" fn msgpack_pack_int64(mut x: *mut msgpack_packer,
                                                mut d: int64_t)
     -> libc::c_int {
        if (d as libc::c_longlong) <
               -((1 as libc::c_longlong) << 5 as libc::c_int) {
            if (d as libc::c_longlong) <
                   -((1 as libc::c_longlong) << 15 as libc::c_int) {
                if (d as libc::c_longlong) <
                       -((1 as libc::c_longlong) << 31 as libc::c_int) {
                    let mut buf: [libc::c_uchar; 9] = [0; 9];
                    buf[0 as libc::c_int as usize] =
                        0xd3 as libc::c_int as libc::c_uchar;
                    let mut val = __bswap_64(d as __uint64_t);
                    memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize) as
                               *mut libc::c_uchar as *mut libc::c_void,
                           &mut val as *mut uint64_t as *const libc::c_void,
                           8 as libc::c_int as libc::c_ulong);
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                       buf.as_mut_ptr()
                                                                                                                           as
                                                                                                                           *const libc::c_char,
                                                                                                                       9
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           size_t)
                } else {
                    let mut buf_0: [libc::c_uchar; 5] = [0; 5];
                    buf_0[0 as libc::c_int as usize] =
                        0xd2 as libc::c_int as libc::c_uchar;
                    let mut val_0 = ntohl(d as int32_t as uint32_t);
                    memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as
                                                               isize) as
                               *mut libc::c_uchar as *mut libc::c_void,
                           &mut val_0 as *mut uint32_t as *const libc::c_void,
                           4 as libc::c_int as libc::c_ulong);
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                       buf_0.as_mut_ptr()
                                                                                                                           as
                                                                                                                           *const libc::c_char,
                                                                                                                       5
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           size_t)
                }
            } else if d <
                          -((1 as libc::c_int) << 7 as libc::c_int) as
                              libc::c_long {
                let mut buf_1: [libc::c_uchar; 3] = [0; 3];
                buf_1[0 as libc::c_int as usize] =
                    0xd1 as libc::c_int as libc::c_uchar;
                let mut val_1 = ntohs(d as int16_t as uint16_t);
                memcpy(&mut *buf_1.as_mut_ptr().offset(1 as libc::c_int as
                                                           isize) as
                           *mut libc::c_uchar as *mut libc::c_void,
                       &mut val_1 as *mut uint16_t as *const libc::c_void,
                       2 as libc::c_int as libc::c_ulong);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_1.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   3
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else {
                let mut buf_2: [libc::c_uchar; 2] =
                    [0xd0 as libc::c_int as libc::c_uchar,
                     *(&mut d as *mut int64_t as
                           *mut uint8_t).offset(0 as libc::c_int as isize)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_2.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
        } else if d < ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_long
         {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut int64_t
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if (d as libc::c_longlong) <
                      (1 as libc::c_longlong) << 16 as libc::c_int {
            if d < ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_long {
                let mut buf_3: [libc::c_uchar; 2] =
                    [0xcc as libc::c_int as libc::c_uchar,
                     *(&mut d as *mut int64_t as
                           *mut uint8_t).offset(0 as libc::c_int as isize)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_3.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            } else {
                let mut buf_4: [libc::c_uchar; 3] = [0; 3];
                buf_4[0 as libc::c_int as usize] =
                    0xcd as libc::c_int as libc::c_uchar;
                let mut val_2 = ntohs(d as uint16_t);
                memcpy(&mut *buf_4.as_mut_ptr().offset(1 as libc::c_int as
                                                           isize) as
                           *mut libc::c_uchar as *mut libc::c_void,
                       &mut val_2 as *mut uint16_t as *const libc::c_void,
                       2 as libc::c_int as libc::c_ulong);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_4.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   3
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
        } else if (d as libc::c_longlong) <
                      (1 as libc::c_longlong) << 32 as libc::c_int {
            let mut buf_5: [libc::c_uchar; 5] = [0; 5];
            buf_5[0 as libc::c_int as usize] =
                0xce as libc::c_int as libc::c_uchar;
            let mut val_3 = ntohl(d as uint32_t);
            memcpy(&mut *buf_5.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_3 as *mut uint32_t as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_5.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_6: [libc::c_uchar; 9] = [0; 9];
            buf_6[0 as libc::c_int as usize] =
                0xcf as libc::c_int as libc::c_uchar;
            let mut val_4 = __bswap_64(d as __uint64_t);
            memcpy(&mut *buf_6.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_4 as *mut uint64_t as *const libc::c_void,
                   8 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_6.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               9
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "271:1"]
    pub unsafe extern "C" fn msgpack_pack_fix_uint8(mut x:
                                                        *mut msgpack_packer,
                                                    mut d: uint8_t)
     -> libc::c_int {
        let mut buf: [libc::c_uchar; 2] =
            [0xcc as libc::c_int as libc::c_uchar,
             *(&mut d as *mut uint8_t).offset(0 as libc::c_int as isize)];
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           buf.as_mut_ptr()
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           2
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               size_t);
    }
    #[inline]
    #[c2rust::src_loc = "277:1"]
    pub unsafe extern "C" fn msgpack_pack_fix_uint16(mut x:
                                                         *mut msgpack_packer,
                                                     mut d: uint16_t)
     -> libc::c_int {
        let mut buf: [libc::c_uchar; 3] = [0; 3];
        buf[0 as libc::c_int as usize] = 0xcd as libc::c_int as libc::c_uchar;
        let mut val = ntohs(d);
        memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut libc::c_uchar as *mut libc::c_void,
               &mut val as *mut uint16_t as *const libc::c_void,
               2 as libc::c_int as libc::c_ulong);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           buf.as_mut_ptr()
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           3
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               size_t);
    }
    #[inline]
    #[c2rust::src_loc = "284:1"]
    pub unsafe extern "C" fn msgpack_pack_fix_uint32(mut x:
                                                         *mut msgpack_packer,
                                                     mut d: uint32_t)
     -> libc::c_int {
        let mut buf: [libc::c_uchar; 5] = [0; 5];
        buf[0 as libc::c_int as usize] = 0xce as libc::c_int as libc::c_uchar;
        let mut val = ntohl(d);
        memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut libc::c_uchar as *mut libc::c_void,
               &mut val as *mut uint32_t as *const libc::c_void,
               4 as libc::c_int as libc::c_ulong);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           buf.as_mut_ptr()
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           5
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               size_t);
    }
    #[inline]
    #[c2rust::src_loc = "291:1"]
    pub unsafe extern "C" fn msgpack_pack_fix_uint64(mut x:
                                                         *mut msgpack_packer,
                                                     mut d: uint64_t)
     -> libc::c_int {
        let mut buf: [libc::c_uchar; 9] = [0; 9];
        buf[0 as libc::c_int as usize] = 0xcf as libc::c_int as libc::c_uchar;
        let mut val = __bswap_64(d);
        memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut libc::c_uchar as *mut libc::c_void,
               &mut val as *mut uint64_t as *const libc::c_void,
               8 as libc::c_int as libc::c_ulong);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           buf.as_mut_ptr()
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           9
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               size_t);
    }
    #[inline]
    #[c2rust::src_loc = "298:1"]
    pub unsafe extern "C" fn msgpack_pack_fix_int8(mut x: *mut msgpack_packer,
                                                   mut d: int8_t)
     -> libc::c_int {
        let mut buf: [libc::c_uchar; 2] =
            [0xd0 as libc::c_int as libc::c_uchar,
             *(&mut d as *mut int8_t as
                   *mut uint8_t).offset(0 as libc::c_int as isize)];
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           buf.as_mut_ptr()
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           2
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               size_t);
    }
    #[inline]
    #[c2rust::src_loc = "304:1"]
    pub unsafe extern "C" fn msgpack_pack_fix_int16(mut x:
                                                        *mut msgpack_packer,
                                                    mut d: int16_t)
     -> libc::c_int {
        let mut buf: [libc::c_uchar; 3] = [0; 3];
        buf[0 as libc::c_int as usize] = 0xd1 as libc::c_int as libc::c_uchar;
        let mut val = ntohs(d as uint16_t);
        memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut libc::c_uchar as *mut libc::c_void,
               &mut val as *mut uint16_t as *const libc::c_void,
               2 as libc::c_int as libc::c_ulong);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           buf.as_mut_ptr()
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           3
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               size_t);
    }
    #[inline]
    #[c2rust::src_loc = "311:1"]
    pub unsafe extern "C" fn msgpack_pack_fix_int32(mut x:
                                                        *mut msgpack_packer,
                                                    mut d: int32_t)
     -> libc::c_int {
        let mut buf: [libc::c_uchar; 5] = [0; 5];
        buf[0 as libc::c_int as usize] = 0xd2 as libc::c_int as libc::c_uchar;
        let mut val = ntohl(d as uint32_t);
        memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut libc::c_uchar as *mut libc::c_void,
               &mut val as *mut uint32_t as *const libc::c_void,
               4 as libc::c_int as libc::c_ulong);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           buf.as_mut_ptr()
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           5
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               size_t);
    }
    #[inline]
    #[c2rust::src_loc = "318:1"]
    pub unsafe extern "C" fn msgpack_pack_fix_int64(mut x:
                                                        *mut msgpack_packer,
                                                    mut d: int64_t)
     -> libc::c_int {
        let mut buf: [libc::c_uchar; 9] = [0; 9];
        buf[0 as libc::c_int as usize] = 0xd3 as libc::c_int as libc::c_uchar;
        let mut val = __bswap_64(d as __uint64_t);
        memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut libc::c_uchar as *mut libc::c_void,
               &mut val as *mut uint64_t as *const libc::c_void,
               8 as libc::c_int as libc::c_ulong);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           buf.as_mut_ptr()
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           9
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               size_t);
    }
    #[inline]
    #[c2rust::src_loc = "651:1"]
    pub unsafe extern "C" fn msgpack_pack_float(mut x: *mut msgpack_packer,
                                                mut d: libc::c_float)
     -> libc::c_int {
        let mut buf: [libc::c_uchar; 5] = [0; 5];
        let mut mem = C2RustUnnamed_25{f: 0.,};
        mem.f = d;
        buf[0 as libc::c_int as usize] = 0xca as libc::c_int as libc::c_uchar;
        let mut val = ntohl(mem.i);
        memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut libc::c_uchar as *mut libc::c_void,
               &mut val as *mut uint32_t as *const libc::c_void,
               4 as libc::c_int as libc::c_ulong);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           buf.as_mut_ptr()
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           5
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               size_t);
    }
    #[inline]
    #[c2rust::src_loc = "660:1"]
    pub unsafe extern "C" fn msgpack_pack_double(mut x: *mut msgpack_packer,
                                                 mut d: libc::c_double)
     -> libc::c_int {
        let mut buf: [libc::c_uchar; 9] = [0; 9];
        let mut mem = C2RustUnnamed_26{f: 0.,};
        mem.f = d;
        buf[0 as libc::c_int as usize] = 0xcb as libc::c_int as libc::c_uchar;
        let mut val = __bswap_64(mem.i);
        memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut libc::c_uchar as *mut libc::c_void,
               &mut val as *mut uint64_t as *const libc::c_void,
               8 as libc::c_int as libc::c_ulong);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           buf.as_mut_ptr()
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           9
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               size_t);
    }
    #[inline]
    #[c2rust::src_loc = "681:1"]
    pub unsafe extern "C" fn msgpack_pack_nil(mut x: *mut msgpack_packer)
     -> libc::c_int {
        pub static mut d: libc::c_uchar =
            0xc0 as libc::c_int as libc::c_uchar;
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           &d
                                                                                                               as
                                                                                                               *const libc::c_uchar
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           1
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               size_t);
    }
    #[inline]
    #[c2rust::src_loc = "692:1"]
    pub unsafe extern "C" fn msgpack_pack_true(mut x: *mut msgpack_packer)
     -> libc::c_int {
        pub static mut d: libc::c_uchar =
            0xc3 as libc::c_int as libc::c_uchar;
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           &d
                                                                                                               as
                                                                                                               *const libc::c_uchar
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           1
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               size_t);
    }
    #[inline]
    #[c2rust::src_loc = "698:1"]
    pub unsafe extern "C" fn msgpack_pack_false(mut x: *mut msgpack_packer)
     -> libc::c_int {
        pub static mut d: libc::c_uchar =
            0xc2 as libc::c_int as libc::c_uchar;
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           &d
                                                                                                               as
                                                                                                               *const libc::c_uchar
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           1
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               size_t);
    }
    #[inline]
    #[c2rust::src_loc = "709:1"]
    pub unsafe extern "C" fn msgpack_pack_array(mut x: *mut msgpack_packer,
                                                mut n: size_t)
     -> libc::c_int {
        if n < 16 as libc::c_int as libc::c_ulong {
            let mut d =
                (0x90 as libc::c_int | n as uint8_t as libc::c_int) as
                    libc::c_uchar;
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut d
                                                                                                                   as
                                                                                                                   *mut libc::c_uchar
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if n < 65536 as libc::c_int as libc::c_ulong {
            let mut buf: [libc::c_uchar; 3] = [0; 3];
            buf[0 as libc::c_int as usize] =
                0xdc as libc::c_int as libc::c_uchar;
            let mut val = ntohs(n as uint16_t);
            memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as isize) as
                       *mut libc::c_uchar as *mut libc::c_void,
                   &mut val as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_0: [libc::c_uchar; 5] = [0; 5];
            buf_0[0 as libc::c_int as usize] =
                0xdd as libc::c_int as libc::c_uchar;
            let mut val_0 = ntohl(n as uint32_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_0 as *mut uint32_t as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_0.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "730:1"]
    pub unsafe extern "C" fn msgpack_pack_map(mut x: *mut msgpack_packer,
                                              mut n: size_t) -> libc::c_int {
        if n < 16 as libc::c_int as libc::c_ulong {
            let mut d =
                (0x80 as libc::c_int | n as uint8_t as libc::c_int) as
                    libc::c_uchar;
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut libc::c_uchar
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if n < 65536 as libc::c_int as libc::c_ulong {
            let mut buf: [libc::c_uchar; 3] = [0; 3];
            buf[0 as libc::c_int as usize] =
                0xde as libc::c_int as libc::c_uchar;
            let mut val = ntohs(n as uint16_t);
            memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as isize) as
                       *mut libc::c_uchar as *mut libc::c_void,
                   &mut val as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_0: [libc::c_uchar; 5] = [0; 5];
            buf_0[0 as libc::c_int as usize] =
                0xdf as libc::c_int as libc::c_uchar;
            let mut val_0 = ntohl(n as uint32_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_0 as *mut uint32_t as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_0.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "751:1"]
    pub unsafe extern "C" fn msgpack_pack_str(mut x: *mut msgpack_packer,
                                              mut l: size_t) -> libc::c_int {
        if l < 32 as libc::c_int as libc::c_ulong {
            let mut d =
                (0xa0 as libc::c_int | l as uint8_t as libc::c_int) as
                    libc::c_uchar;
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut libc::c_uchar
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if l < 256 as libc::c_int as libc::c_ulong {
            let mut buf: [libc::c_uchar; 2] = [0; 2];
            buf[0 as libc::c_int as usize] =
                0xd9 as libc::c_int as libc::c_uchar;
            buf[1 as libc::c_int as usize] = l as uint8_t;
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if l < 65536 as libc::c_int as libc::c_ulong {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as libc::c_int as usize] =
                0xda as libc::c_int as libc::c_uchar;
            let mut val = ntohs(l as uint16_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_0.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_1: [libc::c_uchar; 5] = [0; 5];
            buf_1[0 as libc::c_int as usize] =
                0xdb as libc::c_int as libc::c_uchar;
            let mut val_0 = ntohl(l as uint32_t);
            memcpy(&mut *buf_1.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_0 as *mut uint32_t as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_1.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "771:1"]
    pub unsafe extern "C" fn msgpack_pack_str_body(mut x: *mut msgpack_packer,
                                                   mut b: *const libc::c_void,
                                                   mut l: size_t)
     -> libc::c_int {
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           b
                                                                                                               as
                                                                                                               *const libc::c_uchar
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           l);
    }
    #[inline]
    #[c2rust::src_loc = "780:1"]
    pub unsafe extern "C" fn msgpack_pack_v4raw(mut x: *mut msgpack_packer,
                                                mut l: size_t)
     -> libc::c_int {
        if l < 32 as libc::c_int as libc::c_ulong {
            let mut d =
                (0xa0 as libc::c_int | l as uint8_t as libc::c_int) as
                    libc::c_uchar;
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               &mut *(&mut d
                                                                                                                          as
                                                                                                                          *mut libc::c_uchar
                                                                                                                          as
                                                                                                                          *mut uint8_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                   as
                                                                                                                   *mut uint8_t
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if l < 65536 as libc::c_int as libc::c_ulong {
            let mut buf: [libc::c_uchar; 3] = [0; 3];
            buf[0 as libc::c_int as usize] =
                0xda as libc::c_int as libc::c_uchar;
            let mut val = ntohs(l as uint16_t);
            memcpy(&mut *buf.as_mut_ptr().offset(1 as libc::c_int as isize) as
                       *mut libc::c_uchar as *mut libc::c_void,
                   &mut val as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_0: [libc::c_uchar; 5] = [0; 5];
            buf_0[0 as libc::c_int as usize] =
                0xdb as libc::c_int as libc::c_uchar;
            let mut val_0 = ntohl(l as uint32_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_0 as *mut uint32_t as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_0.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "796:1"]
    pub unsafe extern "C" fn msgpack_pack_v4raw_body(mut x:
                                                         *mut msgpack_packer,
                                                     mut b:
                                                         *const libc::c_void,
                                                     mut l: size_t)
     -> libc::c_int {
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           b
                                                                                                               as
                                                                                                               *const libc::c_uchar
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           l);
    }
    #[inline]
    #[c2rust::src_loc = "805:1"]
    pub unsafe extern "C" fn msgpack_pack_bin(mut x: *mut msgpack_packer,
                                              mut l: size_t) -> libc::c_int {
        if l < 256 as libc::c_int as libc::c_ulong {
            let mut buf: [libc::c_uchar; 2] = [0; 2];
            buf[0 as libc::c_int as usize] =
                0xc4 as libc::c_int as libc::c_uchar;
            buf[1 as libc::c_int as usize] = l as uint8_t;
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else if l < 65536 as libc::c_int as libc::c_ulong {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as libc::c_int as usize] =
                0xc5 as libc::c_int as libc::c_uchar;
            let mut val = ntohs(l as uint16_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val as *mut uint16_t as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_0.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               3
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        } else {
            let mut buf_1: [libc::c_uchar; 5] = [0; 5];
            buf_1[0 as libc::c_int as usize] =
                0xc6 as libc::c_int as libc::c_uchar;
            let mut val_0 = ntohl(l as uint32_t);
            memcpy(&mut *buf_1.as_mut_ptr().offset(1 as libc::c_int as isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   &mut val_0 as *mut uint32_t as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                               buf_1.as_mut_ptr()
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "822:1"]
    pub unsafe extern "C" fn msgpack_pack_bin_body(mut x: *mut msgpack_packer,
                                                   mut b: *const libc::c_void,
                                                   mut l: size_t)
     -> libc::c_int {
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           b
                                                                                                               as
                                                                                                               *const libc::c_uchar
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           l);
    }
    #[inline]
    #[c2rust::src_loc = "831:1"]
    pub unsafe extern "C" fn msgpack_pack_ext(mut x: *mut msgpack_packer,
                                              mut l: size_t,
                                              mut type_0: int8_t)
     -> libc::c_int {
        match l {
            1 => {
                let mut buf: [libc::c_uchar; 2] = [0; 2];
                buf[0 as libc::c_int as usize] =
                    0xd4 as libc::c_int as libc::c_uchar;
                buf[1 as libc::c_int as usize] = type_0 as libc::c_uchar;
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
            2 => {
                let mut buf_0: [libc::c_uchar; 2] = [0; 2];
                buf_0[0 as libc::c_int as usize] =
                    0xd5 as libc::c_int as libc::c_uchar;
                buf_0[1 as libc::c_int as usize] = type_0 as libc::c_uchar;
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_0.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
            4 => {
                let mut buf_1: [libc::c_uchar; 2] = [0; 2];
                buf_1[0 as libc::c_int as usize] =
                    0xd6 as libc::c_int as libc::c_uchar;
                buf_1[1 as libc::c_int as usize] = type_0 as libc::c_uchar;
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_1.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
            8 => {
                let mut buf_2: [libc::c_uchar; 2] = [0; 2];
                buf_2[0 as libc::c_int as usize] =
                    0xd7 as libc::c_int as libc::c_uchar;
                buf_2[1 as libc::c_int as usize] = type_0 as libc::c_uchar;
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_2.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
            16 => {
                let mut buf_3: [libc::c_uchar; 2] = [0; 2];
                buf_3[0 as libc::c_int as usize] =
                    0xd8 as libc::c_int as libc::c_uchar;
                buf_3[1 as libc::c_int as usize] = type_0 as libc::c_uchar;
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                   buf_3.as_mut_ptr()
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       size_t)
            }
            _ => {
                if l < 256 as libc::c_int as libc::c_ulong {
                    let mut buf_4: [libc::c_uchar; 3] = [0; 3];
                    buf_4[0 as libc::c_int as usize] =
                        0xc7 as libc::c_int as libc::c_uchar;
                    buf_4[1 as libc::c_int as usize] = l as libc::c_uchar;
                    buf_4[2 as libc::c_int as usize] =
                        type_0 as libc::c_uchar;
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                       buf_4.as_mut_ptr()
                                                                                                                           as
                                                                                                                           *const libc::c_char,
                                                                                                                       3
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           size_t)
                } else if l < 65536 as libc::c_int as libc::c_ulong {
                    let mut buf_5: [libc::c_uchar; 4] = [0; 4];
                    buf_5[0 as libc::c_int as usize] =
                        0xc8 as libc::c_int as libc::c_uchar;
                    let mut val = ntohs(l as uint16_t);
                    memcpy(&mut *buf_5.as_mut_ptr().offset(1 as libc::c_int as
                                                               isize) as
                               *mut libc::c_uchar as *mut libc::c_void,
                           &mut val as *mut uint16_t as *const libc::c_void,
                           2 as libc::c_int as libc::c_ulong);
                    buf_5[3 as libc::c_int as usize] =
                        type_0 as libc::c_uchar;
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                       buf_5.as_mut_ptr()
                                                                                                                           as
                                                                                                                           *const libc::c_char,
                                                                                                                       4
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           size_t)
                } else {
                    let mut buf_6: [libc::c_uchar; 6] = [0; 6];
                    buf_6[0 as libc::c_int as usize] =
                        0xc9 as libc::c_int as libc::c_uchar;
                    let mut val_0 = ntohl(l as uint32_t);
                    memcpy(&mut *buf_6.as_mut_ptr().offset(1 as libc::c_int as
                                                               isize) as
                               *mut libc::c_uchar as *mut libc::c_void,
                           &mut val_0 as *mut uint32_t as *const libc::c_void,
                           4 as libc::c_int as libc::c_ulong);
                    buf_6[5 as libc::c_int as usize] =
                        type_0 as libc::c_uchar;
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                                       buf_6.as_mut_ptr()
                                                                                                                           as
                                                                                                                           *const libc::c_char,
                                                                                                                       6
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           size_t)
                }
            }
        };
    }
    #[inline]
    #[c2rust::src_loc = "888:1"]
    pub unsafe extern "C" fn msgpack_pack_ext_body(mut x: *mut msgpack_packer,
                                                   mut b: *const libc::c_void,
                                                   mut l: size_t)
     -> libc::c_int {
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data,
                                                                                                           b
                                                                                                               as
                                                                                                               *const libc::c_uchar
                                                                                                               as
                                                                                                               *const libc::c_char,
                                                                                                           l);
    }
    use super::stdint_uintn_h::{uint32_t, uint64_t, uint8_t, uint16_t};
    use super::pack_h::msgpack_packer;
    use super::stddef_h::size_t;
    use super::in_h::{ntohs, ntohl};
    use super::stdint_intn_h::{int16_t, int32_t, int8_t, int64_t};
    use super::string_h::memcpy;
    use super::byteswap_h::__bswap_64;
    use super::types_h::__uint64_t;
}
#[c2rust::header_src =
  "/home/vole/neovim/.deps/usr/include/msgpack/unpack.h:16"]
pub mod unpack_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:16"]
    pub struct msgpack_unpacked {
        pub zone: *mut msgpack_zone,
        pub data: msgpack_object,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:16"]
    pub struct msgpack_unpacker {
        pub buffer: *mut libc::c_char,
        pub used: size_t,
        pub free: size_t,
        pub off: size_t,
        pub parsed: size_t,
        pub z: *mut msgpack_zone,
        pub initial_buffer_size: size_t,
        pub ctx: *mut libc::c_void,
    }
    #[inline]
    #[c2rust::src_loc = "219:1"]
    pub unsafe extern "C" fn msgpack_unpacker_reserve_buffer(mut mpac:
                                                                 *mut msgpack_unpacker,
                                                             mut size: size_t)
     -> bool {
        if (*mpac).free >= size { return true_0 != 0 }
        return msgpack_unpacker_expand_buffer(mpac, size);
    }
    #[inline]
    #[c2rust::src_loc = "225:1"]
    pub unsafe extern "C" fn msgpack_unpacker_buffer(mut mpac:
                                                         *mut msgpack_unpacker)
     -> *mut libc::c_char {
        return (*mpac).buffer.offset((*mpac).used as isize);
    }
    #[inline]
    #[c2rust::src_loc = "230:1"]
    pub unsafe extern "C" fn msgpack_unpacker_buffer_capacity(mut mpac:
                                                                  *const msgpack_unpacker)
     -> size_t {
        return (*mpac).free;
    }
    #[inline]
    #[c2rust::src_loc = "235:1"]
    pub unsafe extern "C" fn msgpack_unpacker_buffer_consumed(mut mpac:
                                                                  *mut msgpack_unpacker,
                                                              mut size:
                                                                  size_t) {
        (*mpac).used =
            ((*mpac).used as libc::c_ulong).wrapping_add(size) as size_t as
                size_t;
        (*mpac).free =
            ((*mpac).free as libc::c_ulong).wrapping_sub(size) as size_t as
                size_t;
    }
    #[inline]
    #[c2rust::src_loc = "252:1"]
    pub unsafe extern "C" fn msgpack_unpacked_init(mut result:
                                                       *mut msgpack_unpacked) {
        memset(result as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<msgpack_unpacked>() as libc::c_ulong);
    }
    #[inline]
    #[c2rust::src_loc = "257:1"]
    pub unsafe extern "C" fn msgpack_unpacked_destroy(mut result:
                                                          *mut msgpack_unpacked) {
        if !(*result).zone.is_null() {
            msgpack_zone_free((*result).zone);
            (*result).zone = NULL_1 as *mut msgpack_zone;
            memset(&mut (*result).data as *mut msgpack_object as
                       *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<msgpack_object>() as libc::c_ulong);
        };
    }
    #[inline]
    #[c2rust::src_loc = "266:1"]
    pub unsafe extern "C" fn msgpack_unpacked_release_zone(mut result:
                                                               *mut msgpack_unpacked)
     -> *mut msgpack_zone {
        if !(*result).zone.is_null() {
            let mut z = (*result).zone;
            (*result).zone = NULL_1 as *mut msgpack_zone;
            return z
        }
        return NULL_1 as *mut msgpack_zone;
    }
    #[inline]
    #[c2rust::src_loc = "241:1"]
    pub unsafe extern "C" fn msgpack_unpacker_message_size(mut mpac:
                                                               *const msgpack_unpacker)
     -> size_t {
        return (*mpac).parsed.wrapping_sub((*mpac).off).wrapping_add((*mpac).used);
    }
    #[inline]
    #[c2rust::src_loc = "246:1"]
    pub unsafe extern "C" fn msgpack_unpacker_parsed_size(mut mpac:
                                                              *const msgpack_unpacker)
     -> size_t {
        return (*mpac).parsed;
    }
    use super::zone_h::{msgpack_zone, msgpack_zone_free};
    use super::object_h::msgpack_object;
    use super::stddef_h::{size_t, NULL_1};
    use super::stdbool_h::true_0;
    use super::string_h::memset;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "217:1"]
        pub fn msgpack_unpacker_expand_buffer(mpac: *mut msgpack_unpacker,
                                              size: size_t) -> bool;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/.deps/usr/include/msgpack/sbuffer.h:16"]
pub mod sbuffer_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:16"]
    pub struct msgpack_sbuffer {
        pub size: size_t,
        pub data: *mut libc::c_char,
        pub alloc: size_t,
    }
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn msgpack_sbuffer_init(mut sbuf:
                                                      *mut msgpack_sbuffer) {
        memset(sbuf as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<msgpack_sbuffer>() as libc::c_ulong);
    }
    #[inline]
    #[c2rust::src_loc = "38:1"]
    pub unsafe extern "C" fn msgpack_sbuffer_destroy(mut sbuf:
                                                         *mut msgpack_sbuffer) {
        free((*sbuf).data as *mut libc::c_void);
    }
    #[inline]
    #[c2rust::src_loc = "43:1"]
    pub unsafe extern "C" fn msgpack_sbuffer_new() -> *mut msgpack_sbuffer {
        return calloc(1 as libc::c_int as libc::c_ulong,
                      ::std::mem::size_of::<msgpack_sbuffer>() as
                          libc::c_ulong) as *mut msgpack_sbuffer;
    }
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn msgpack_sbuffer_free(mut sbuf:
                                                      *mut msgpack_sbuffer) {
        if sbuf.is_null() { return }
        msgpack_sbuffer_destroy(sbuf);
        free(sbuf as *mut libc::c_void);
    }
    #[inline]
    #[c2rust::src_loc = "59:1"]
    pub unsafe extern "C" fn msgpack_sbuffer_write(mut data:
                                                       *mut libc::c_void,
                                                   mut buf:
                                                       *const libc::c_char,
                                                   mut len: size_t)
     -> libc::c_int {
        let mut sbuf = data as *mut msgpack_sbuffer;
        if (*sbuf).alloc.wrapping_sub((*sbuf).size) < len {
            let mut tmp = 0 as *mut libc::c_void;
            let mut nsize =
                if (*sbuf).alloc != 0 {
                    (*sbuf).alloc.wrapping_mul(2 as libc::c_int as
                                                   libc::c_ulong)
                } else { MSGPACK_SBUFFER_INIT_SIZE as libc::c_ulong };
            while nsize < (*sbuf).size.wrapping_add(len) {
                let mut tmp_nsize =
                    nsize.wrapping_mul(2 as libc::c_int as libc::c_ulong);
                if tmp_nsize <= nsize {
                    nsize = (*sbuf).size.wrapping_add(len);
                    break ;
                } else { nsize = tmp_nsize }
            }
            tmp = realloc((*sbuf).data as *mut libc::c_void, nsize);
            if tmp.is_null() { return -(1 as libc::c_int) }
            (*sbuf).data = tmp as *mut libc::c_char;
            (*sbuf).alloc = nsize
        }
        memcpy((*sbuf).data.offset((*sbuf).size as isize) as
                   *mut libc::c_void, buf as *const libc::c_void, len);
        (*sbuf).size =
            ((*sbuf).size as libc::c_ulong).wrapping_add(len) as size_t as
                size_t;
        return 0 as libc::c_int;
    }
    #[c2rust::src_loc = "56:9"]
    pub const MSGPACK_SBUFFER_INIT_SIZE: libc::c_int = 8192 as libc::c_int;
    #[inline]
    #[c2rust::src_loc = "89:1"]
    pub unsafe extern "C" fn msgpack_sbuffer_release(mut sbuf:
                                                         *mut msgpack_sbuffer)
     -> *mut libc::c_char {
        let mut tmp = (*sbuf).data;
        (*sbuf).size = 0 as libc::c_int as size_t;
        (*sbuf).data = NULL_1 as *mut libc::c_char;
        (*sbuf).alloc = 0 as libc::c_int as size_t;
        return tmp;
    }
    #[inline]
    #[c2rust::src_loc = "98:1"]
    pub unsafe extern "C" fn msgpack_sbuffer_clear(mut sbuf:
                                                       *mut msgpack_sbuffer) {
        (*sbuf).size = 0 as libc::c_int as size_t;
    }
    use super::stddef_h::{size_t, NULL_1};
    use super::string_h::{memset, memcpy};
    use super::stdlib_h::{free, calloc, realloc};
}
#[c2rust::header_src =
  "/home/vole/neovim/.deps/usr/include/msgpack/vrefbuffer.h:16"]
pub mod vrefbuffer_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:16"]
    pub struct msgpack_vrefbuffer_inner_buffer {
        pub free: size_t,
        pub ptr: *mut libc::c_char,
        pub head: *mut msgpack_vrefbuffer_chunk,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:16"]
    pub struct msgpack_vrefbuffer {
        pub tail: *mut iovec,
        pub end: *mut iovec,
        pub array: *mut iovec,
        pub chunk_size: size_t,
        pub ref_size: size_t,
        pub inner_buffer: msgpack_vrefbuffer_inner_buffer,
    }
    #[inline]
    #[c2rust::src_loc = "96:1"]
    pub unsafe extern "C" fn msgpack_vrefbuffer_new(mut ref_size: size_t,
                                                    mut chunk_size: size_t)
     -> *mut msgpack_vrefbuffer {
        let mut vbuf =
            malloc(::std::mem::size_of::<msgpack_vrefbuffer>() as
                       libc::c_ulong) as *mut msgpack_vrefbuffer;
        if vbuf.is_null() { return NULL_1 as *mut msgpack_vrefbuffer }
        if !msgpack_vrefbuffer_init(vbuf, ref_size, chunk_size) {
            free(vbuf as *mut libc::c_void);
            return NULL_1 as *mut msgpack_vrefbuffer
        }
        return vbuf;
    }
    #[inline]
    #[c2rust::src_loc = "107:1"]
    pub unsafe extern "C" fn msgpack_vrefbuffer_free(mut vbuf:
                                                         *mut msgpack_vrefbuffer) {
        if vbuf.is_null() { return }
        msgpack_vrefbuffer_destroy(vbuf);
        free(vbuf as *mut libc::c_void);
    }
    #[inline]
    #[c2rust::src_loc = "114:1"]
    pub unsafe extern "C" fn msgpack_vrefbuffer_write(mut data:
                                                          *mut libc::c_void,
                                                      mut buf:
                                                          *const libc::c_char,
                                                      mut len: size_t)
     -> libc::c_int {
        let mut vbuf = data as *mut msgpack_vrefbuffer;
        if len < (*vbuf).ref_size {
            return msgpack_vrefbuffer_append_copy(vbuf, buf, len)
        } else { return msgpack_vrefbuffer_append_ref(vbuf, buf, len) };
    }
    #[inline]
    #[c2rust::src_loc = "125:1"]
    pub unsafe extern "C" fn msgpack_vrefbuffer_vec(mut vref:
                                                        *const msgpack_vrefbuffer)
     -> *const iovec {
        return (*vref).array;
    }
    #[inline]
    #[c2rust::src_loc = "130:1"]
    pub unsafe extern "C" fn msgpack_vrefbuffer_veclen(mut vref:
                                                           *const msgpack_vrefbuffer)
     -> size_t {
        return (*vref).tail.wrapping_offset_from((*vref).array) as
                   libc::c_long as size_t;
    }
    use super::stddef_h::{size_t, NULL_1};
    use super::struct_iovec_h::iovec;
    use super::stdlib_h::{malloc, free};
    extern "C" {
        #[c2rust::src_loc = "36:8"]
        pub type msgpack_vrefbuffer_chunk;
        #[no_mangle]
        #[c2rust::src_loc = "84:1"]
        pub fn msgpack_vrefbuffer_append_ref(vbuf: *mut msgpack_vrefbuffer,
                                             buf: *const libc::c_char,
                                             len: size_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "66:1"]
        pub fn msgpack_vrefbuffer_init(vbuf: *mut msgpack_vrefbuffer,
                                       ref_size: size_t, chunk_size: size_t)
         -> bool;
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn msgpack_vrefbuffer_destroy(vbuf: *mut msgpack_vrefbuffer);
        #[no_mangle]
        #[c2rust::src_loc = "80:1"]
        pub fn msgpack_vrefbuffer_append_copy(vbuf: *mut msgpack_vrefbuffer,
                                              buf: *const libc::c_char,
                                              len: size_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/channel.h:16"]
pub mod channel_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:8"]
    pub struct Channel {
        pub id: uint64_t,
        pub refcount: size_t,
        pub events: *mut MultiQueue,
        pub streamtype: ChannelStreamType,
        pub stream: C2RustUnnamed_28,
        pub is_rpc: bool,
        pub rpc: RpcState,
        pub term: *mut Terminal,
        pub on_data: CallbackReader,
        pub on_stderr: CallbackReader,
        pub on_exit: Callback,
        pub exit_status: libc::c_int,
        pub callback_busy: bool,
        pub callback_scheduled: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:9"]
    pub struct CallbackReader {
        pub cb: Callback,
        pub self_0: *mut dict_T,
        pub buffer: garray_T,
        pub eof: bool,
        pub buffered: bool,
        pub type_0: *const libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "66:3"]
    pub union C2RustUnnamed_28 {
        pub proc_0: Process,
        pub uv: LibuvProcess,
        pub pty: PtyProcess,
        pub socket: Stream,
        pub stdio: StdioPair,
        pub err: StderrState,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:9"]
    pub struct StderrState {
        pub closed: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:9"]
    pub struct StdioPair {
        pub in_0: Stream,
        pub out: Stream,
    }
    #[c2rust::src_loc = "15:9"]
    pub type ChannelStreamType = libc::c_uint;
    #[c2rust::src_loc = "20:3"]
    pub const kChannelStreamInternal: ChannelStreamType = 4;
    #[c2rust::src_loc = "19:3"]
    pub const kChannelStreamStderr: ChannelStreamType = 3;
    #[c2rust::src_loc = "18:3"]
    pub const kChannelStreamStdio: ChannelStreamType = 2;
    #[c2rust::src_loc = "17:3"]
    pub const kChannelStreamSocket: ChannelStreamType = 1;
    #[c2rust::src_loc = "16:3"]
    pub const kChannelStreamProc: ChannelStreamType = 0;
    #[inline]
    #[c2rust::src_loc = "55:1"]
    pub unsafe extern "C" fn callback_reader_set(mut reader: CallbackReader)
     -> bool {
        return reader.cb.type_0 as libc::c_uint !=
                   kCallbackNone as libc::c_int as libc::c_uint ||
                   !reader.self_0.is_null();
    }
    // / @returns Channel with the id or NULL if not found
    #[inline]
    #[c2rust::src_loc = "95:1"]
    pub unsafe extern "C" fn find_channel(mut id: uint64_t) -> *mut Channel {
        return map_uint64_t_ptr_t_get(channels, id) as *mut Channel;
    }
    #[inline]
    #[c2rust::src_loc = "100:1"]
    pub unsafe extern "C" fn channel_instream(mut chan: *mut Channel)
     -> *mut Stream {
        match (*chan).streamtype as libc::c_uint {
            0 => { return &mut (*chan).stream.proc_0.in_0 }
            1 => { return &mut (*chan).stream.socket }
            2 => { return &mut (*chan).stream.stdio.out }
            4 | 3 => { abort(); }
            _ => { }
        }
        abort();
    }
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn channel_outstream(mut chan: *mut Channel)
     -> *mut Stream {
        match (*chan).streamtype as libc::c_uint {
            0 => { return &mut (*chan).stream.proc_0.out }
            1 => { return &mut (*chan).stream.socket }
            2 => { return &mut (*chan).stream.stdio.in_0 }
            4 | 3 => { abort(); }
            _ => { }
        }
        abort();
    }
    use super::stdint_uintn_h::uint64_t;
    use super::stddef_h::size_t;
    use super::multiqueue_h::MultiQueue;
    use super::channel_defs_h::RpcState;
    use super::terminal_h::Terminal;
    use super::typval_h::{Callback, dict_T, C2RustUnnamed, CallbackType,
                          kCallbackNone};
    use super::garray_h::garray_T;
    use super::process_h::Process;
    use super::libuv_process_h::LibuvProcess;
    use super::pty_process_unix_h::PtyProcess;
    use super::stream_h::Stream;
    use super::nvim_types_h::char_u;
    use super::map_h::{Map_uint64_t_ptr_t, map_uint64_t_ptr_t_get};
    use super::stdlib_h::abort;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "88:24"]
        pub static mut channels: *mut Map_uint64_t_ptr_t;
    }
    // NVIM_CHANNEL_H
}
#[c2rust::header_src =
  "/home/vole/neovim/src/nvim/msgpack_rpc/channel_defs.h:16"]
pub mod channel_defs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:9"]
    pub struct RpcState {
        pub subscribed_events: *mut Map_cstr_t_ptr_t,
        pub closed: bool,
        pub unpacker: *mut msgpack_unpacker,
        pub next_request_id: uint32_t,
        pub call_stack: C2RustUnnamed_27,
        pub info: Dictionary,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:3"]
    pub struct C2RustUnnamed_27 {
        pub size: size_t,
        pub capacity: size_t,
        pub items: *mut *mut ChannelCallFrame,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:9"]
    pub struct ChannelCallFrame {
        pub request_id: uint32_t,
        pub returned: bool,
        pub errored: bool,
        pub result: Object,
    }
    use super::map_h::Map_cstr_t_ptr_t;
    use super::unpack_h::msgpack_unpacker;
    use super::stdint_uintn_h::uint32_t;
    use super::defs_h::{Dictionary, Object};
    use super::stddef_h::size_t;
    // NVIM_MSGPACK_RPC_CHANNEL_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/os/fileio.h:16"]
pub mod fileio_h {
    // / Structure used to read from/write to file
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:9"]
    pub struct FileDescriptor {
        pub fd: libc::c_int,
        pub _error: libc::c_int,
        pub rv: *mut RBuffer,
        pub wr: bool,
        pub eof: bool,
        pub non_blocking: bool,
    }
    // / Check whether end of file was encountered
// /
// / @param[in]  fp  File to check.
// /
// / @return true if it was, false if it was not or read operation was never
// /         performed.
    #[inline]
    #[c2rust::src_loc = "49:1"]
    pub unsafe extern "C" fn file_eof(fp: *const FileDescriptor) -> bool {
        return (*fp).eof as libc::c_int != 0 &&
                   rbuffer_size((*fp).rv) ==
                       0 as libc::c_int as libc::c_ulong;
    }
    // / Return the file descriptor associated with the FileDescriptor structure
// /
// / @param[in]  fp  File to check.
// /
// / @return File descriptor.
    #[inline]
    #[c2rust::src_loc = "62:1"]
    pub unsafe extern "C" fn file_fd(fp: *const FileDescriptor)
     -> libc::c_int {
        return (*fp).fd;
    }
    use super::rbuffer_h::{RBuffer, rbuffer_size};
    // NVIM_OS_FILEIO_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/eval/encode.h:52"]
pub mod encode_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "35:9"]
    pub struct ListReaderState {
        pub list: *const list_T,
        pub li: *const listitem_T,
        pub offset: size_t,
        pub li_length: size_t,
    }
    // / Initialize ListReaderState structure
    #[inline]
    #[c2rust::src_loc = "43:1"]
    pub unsafe extern "C" fn encode_init_lrstate(list: *const list_T)
     -> ListReaderState {
        return {
                   let mut init =
                       ListReaderState{list: list,
                                       li: tv_list_first(list),
                                       offset: 0 as libc::c_int as size_t,
                                       li_length:
                                           if (*tv_list_first(list)).li_tv.vval.v_string.is_null()
                                              {
                                               0 as libc::c_int as
                                                   libc::c_ulong
                                           } else {
                                               strlen((*tv_list_first(list)).li_tv.vval.v_string
                                                          as
                                                          *mut libc::c_char)
                                           },};
                   init
               };
    }
    use super::typval_h::{list_T, listitem_T, tv_list_first};
    use super::stddef_h::size_t;
    use super::string_h::strlen;
    // NVIM_EVAL_ENCODE_H
    // / First character that needs to be encoded as surrogate pair
    // / Last codepoint in low surrogates block
    // / First codepoint in low surrogates block
    // / Last codepoint in high surrogates block
    // / First codepoint in high surrogates block
}
#[c2rust::header_src =
  "/usr/lib/llvm-6.0/lib/clang/6.0.1/include/stdbool.h:6"]
pub mod stdbool_h {
    #[c2rust::src_loc = "32:9"]
    pub const true_0: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "33:9"]
    pub const false_0: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/string.h:7"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "46:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "121:14"]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "124:14"]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "329:14"]
        pub fn strstr(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/mathcalls.h:8"]
pub mod mathcalls_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "107:13"]
        pub fn log10(_: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:9"]
pub mod assert_h {
    #[c2rust::src_loc = "129:12"]
    pub const __ASSERT_FUNCTION: [libc::c_char; 58] =
        unsafe {
            *::std::mem::transmute::<&[u8; 58],
                                     &[libc::c_char; 58]>(b"void tv_list_set_lock(list_T *const, const VarLockStatus)\x00")
        };
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
#[c2rust::header_src = "/home/vole/neovim/build/include/log.h.generated.h:12"]
pub mod log_h_generated_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "7:1"]
        pub fn logmsg(log_level: libc::c_int, context: *const libc::c_char,
                      func_name: *const libc::c_char, line_num: libc::c_int,
                      eol: bool, fmt: *const libc::c_char, _: ...) -> bool;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/log.h:12"]
pub mod log_h {
    #[c2rust::src_loc = "11:9"]
    pub const WARN_LOG_LEVEL: libc::c_int = 2 as libc::c_int;
    // NVIM_LOG_H
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/mbyte.h.generated.h:12"]
pub mod mbyte_h_generated_h {
    use super::nvim_types_h::char_u;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "10:1"]
        pub fn utf_ptr2cells(p: *const char_u) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "12:1"]
        pub fn mb_string2cells(str: *const char_u) -> size_t;
        #[no_mangle]
        #[c2rust::src_loc = "14:1"]
        pub fn utf_ptr2char(p: *const char_u) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "20:1"]
        pub fn utf_ptr2len(p: *const char_u) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "23:1"]
        pub fn utfc_ptr2len(p: *const char_u) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "25:1"]
        pub fn utf_char2len(c: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "26:1"]
        pub fn utf_char2bytes(c: libc::c_int, buf: *mut char_u)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "33:1"]
        pub fn mb_toupper(a: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "35:1"]
        pub fn mb_tolower(a: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "40:1"]
        pub fn mb_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "43:1"]
        pub fn mb_copy_char(fp: *mut *const char_u, tp: *mut *mut char_u);
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/mbyte.h:12"]
pub mod mbyte_h {
    // / Compare strings
// /
// / @param[in]  ic  True if case is to be ignored.
// /
// / @return 0 if s1 == s2, <0 if s1 < s2, >0 if s1 > s2.
    #[inline]
    #[c2rust::src_loc = "89:1"]
    pub unsafe extern "C" fn mb_strcmp_ic(mut ic: bool,
                                          mut s1: *const libc::c_char,
                                          mut s2: *const libc::c_char)
     -> libc::c_int {
        return if ic as libc::c_int != 0 {
                   mb_stricmp(s1, s2)
               } else { strcmp(s1, s2) };
    }
    #[c2rust::src_loc = "42:9"]
    pub const mb_ptr2len:
              unsafe extern "C" fn(_: *const char_u) -> libc::c_int =
        utfc_ptr2len;
    use super::mbyte_h_generated_h::{mb_stricmp, utfc_ptr2len};
    use super::string_h::strcmp;
    use super::nvim_types_h::char_u;
    // NVIM_MBYTE_H
}
#[c2rust::header_src = "/usr/include/libintl.h:12"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/message.h.generated.h:12"]
pub mod message_h_generated_h {
    use super::nvim_types_h::char_u;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "20:1"]
        pub fn emsg(s: *const char_u) -> bool;
        #[no_mangle]
        #[c2rust::src_loc = "22:1"]
        pub fn emsgf(fmt: *const libc::c_char, _: ...) -> bool;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/eval/typval.h.generated.h:12"]
pub mod typval_h_generated_h {
    use super::typval_h::{dict_T, typval_T, dictitem_T, varnumber_T};
    use super::stddef_h::ptrdiff_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "43:1"]
        pub fn tv_dict_watcher_notify(dict: *mut dict_T,
                                      key: *const libc::c_char,
                                      newtv: *mut typval_T,
                                      oldtv: *mut typval_T);
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn tv_dict_find(d: *const dict_T, key: *const libc::c_char,
                            len: ptrdiff_t) -> *mut dictitem_T;
        #[no_mangle]
        #[c2rust::src_loc = "86:1"]
        pub fn tv_get_number_chk(tv: *const typval_T, ret_error: *mut bool)
         -> varnumber_T;
        #[no_mangle]
        #[c2rust::src_loc = "90:1"]
        pub fn tv_get_string_chk(tv: *const typval_T) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/strings.h:12"]
pub mod strings_h {
    // / Append string to string and return pointer to the next byte
// /
// / Unlike strcat, this one does *not* add NUL byte and returns pointer to the
// / past of the added string.
// /
// / @param[out]  dst  String to append to.
// / @param[in]  src  String to append.
// /
// / @return pointer to the byte just past the appended byte.
    #[inline]
    #[c2rust::src_loc = "20:1"]
    pub unsafe extern "C" fn strappend(dst: *mut libc::c_char,
                                       src: *const libc::c_char)
     -> *mut libc::c_char {
        let src_len = strlen(src);
        return (memmove(dst as *mut libc::c_void, src as *const libc::c_void,
                        src_len) as
                    *mut libc::c_char).offset(src_len as isize);
    }
    use super::string_h::{strlen, memmove};
    use super::stddef_h::size_t;
    // NVIM_STRINGS_H
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/memory.h.generated.h:12"]
pub mod memory_h_generated_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "23:1"]
        pub fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "11:1"]
        pub fn xmallocz(size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "7:1"]
        pub fn xmalloc(size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "8:1"]
        pub fn xfree(ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "21:1"]
        pub fn xstrlcpy(dst: *mut libc::c_char, src: *const libc::c_char,
                        dsize: size_t) -> size_t;
        #[no_mangle]
        #[c2rust::src_loc = "14:1"]
        pub fn xmemscan(addr: *const libc::c_void, c: libc::c_char,
                        size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "13:1"]
        pub fn xstrchrnul(str: *const libc::c_char, c: libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "9:1"]
        pub fn xcalloc(count: size_t, size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "10:1"]
        pub fn xrealloc(ptr: *mut libc::c_void, size: size_t)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/globals.h:12"]
pub mod globals_h {
    // file I/O and sprintf buffer size
    // length of buffer for small messages
    // cell length (worst case: utf-8
                                         // takes 6 bytes for one cell)
    // Values for "starting".
    // no screen updating yet
    // not all buffers loaded yet
    //                      0          not starting anymore
    // Number of Rows and Columns in the screen.
// Note: Use default_grid.Rows and default_grid.Columns to access items in
// default_grid.chars[]. They may have different values when the screen
// wasn't (re)allocated yet after setting Rows or Columns (e.g., when starting
// up).
    // default value for 'columns'
    // default value for 'lines'
    // nr of rows in the screen
    // nr of columns in the screen
    // We use 64-bit file functions here, if available.  E.g. ftello() returns
// off_t instead of long, which helps if long is 32 bit and off_t is 64 bit.
// We assume that when fseeko() is available then ftello() is too.
// Note that Windows has different function names.
    // When vgetc() is called, it sets mod_mask to the set of modifiers that are
// held down based on the MOD_MASK_* symbols that are read first.
    // current key modifiers
    // Cmdline_row is the row where the command line starts, just below the
// last window.
// When the cmdline gets longer than the available space the screen gets
// scrolled up. After a CTRL-D (show matches), after hitting ':' after
// "hit return", and for the :global command, the command line is
// temporarily moved.  The old position is restored with the next call to
// update_screen().
    // cmdline must be redrawn
    // cmdline must be cleared
    // mode is being displayed
    // cmdline is crypted
    // cmdline is being redrawn
    // cmdline was last drawn
    // executing register
    // When '$' is included in 'cpoptions' option set:
// When a change command is given that deletes only part of a line, a dollar
// is put at the end of the changed text. dollar_vcol is set to the virtual
// column of this '$'.  -1 is used to indicate no $ is being displayed.
    // Variables for Insert mode completion.
    // Length in bytes of the text being completed (this is deleted to be replaced
// by the match.)
    // Set when character typed while looking for matches and it means we should
// stop looking for matches.
    // Set when doing something for completion that may call edit() recursively,
// which is not allowed. Also used to disable folding during completion
    // List of flags for method of completion.
    // "normal" or "adding" expansion
    // a ^X interrupted the current expansion
                                // it's set only iff N_ADDS is set
    // next ^X<> will add-new or expand-current
    // next ^X<> will set initial_pos?
                                // if so, word-wise-expansion will set SOL
    // pattern includes start of line, just for
                                // word-wise expansion, not set for ^X^L
    // for ctrl_x_mode 0, ^X^P/^X^N do a local
                                // expansion, (eg use complete=.)
    // state for putting characters in the message area
    // cmdline is drawn right to left
    // Number of screen lines that windows have
                                // scrolled because of printing messages.
    // when true don't set need_wait_return in msg_puts_attr()
// when msg_scrolled is non-zero
    // Whether the screen is damaged due to scrolling. Sometimes msg_scrolled
// is reset before the screen is redrawn, so we need to keep track of this.
    // msg to be shown after redraw
    // highlight attr for keep_msg
    // keep_msg was set by msgmore()
    // do fileinfo() after redraw
    // msg_start() will scroll
    // msg_outstr() was used in line
    // msg_outstr() was used at all
    // don't wait for this msg
    // don't display errors for now,
                                            // unless 'debug' is set.
    // printing informative message
    // don't add messages to history
    // need to clear text before
                                            // displaying a message.
    // don't display errors for
                                            // expression that is skipped
    // use message of next of several
                                            //  emsg() calls for throw
    // just had ":endif"
    // Dictionary with v: variables
    // Dictionary with g: variables
    // set by emsg() when the message
                                            // is displayed or thrown
    // set if vim_beep() is called
    // did_emsg set because of a
                                            // syntax error
    // always set by emsg()
    // exit value for ex mode
    // there is an error message
    // vim_regcomp() called emsg()
    // don't wait for return for now
    // need to wait for return later
    // wait_return() was used and
                                            // nothing written since then
    // call maketitle() soon
    // 'q' hit at "--more--" msg
    // getexmodeline(): keep indent
    // when inside vgetc() then > 0
    // did set $VIM ourselves
    // idem for $VIMRUNTIME
    // / Lines left before a "more" message.  Ex mode needs to be able to reset this
// / after you type something.
    // lines left for listing
    // don't use more prompt, truncate
                                            // messages
    // name of error message source
    // line number of the source file
    // nesting level
    // break below this level
    // did "debug mode" message
    // breakpoint change count
    // breakpoint backtrace level
    // Values for "do_profiling".
    // /< profiling not started
    // /< profiling busy
    // /< profiling paused
    // /< PROF_ values
    // / Exception currently being thrown.  Used to pass an exception to a different
// / cstack.  Also used for discarding an exception before it is caught or made
// / pending.
    // / Set when a throw that cannot be handled in do_cmdline() must be propagated
// / to the cstack of the previously called do_cmdline().
    // / Set when a ":finish" or ":return" that cannot be handled in do_cmdline()
// / must be propagated to the cstack of the previously called do_cmdline().
    // / Number of nested try conditionals (across function calls and ":source"
// / commands).
    // / When "force_abort" is true, always skip commands after an error message,
// / even after the outermost ":endif", ":endwhile" or ":endfor" or for a
// / function without the "abort" flag.  It is set to true when "trylevel" is
// / non-zero (and ":silent!" was not used) or an exception is being thrown at
// / the time an error is detected.  It is set to false when "trylevel" gets
// / zero again and there was no error or interrupt or throw.
    // / "msg_list" points to a variable in the stack of do_cmdline() which keeps
// / the list of arguments of several emsg() calls, one of which is to be
// / converted to an error exception immediately after the failing command
// / returns.  The message to be used for the exception value is pointed to by
// / the "throw_msg" field of the first element in the list.  It is usually the
// / same as the "msg" field of that element, but can be identical to the "msg"
// / field of a later list element, when the "emsg_severe" flag was set when the
// / emsg() call was made.
    // / When set, don't convert an error to an exception.  Used when displaying the
// / interrupt message or reporting an exception that is still uncaught at the
// / top level (which has already been discarded then).  Also used for the error
// / message when no exception can be thrown.
    // / The stack of all caught and not finished exceptions.  The exception on the
// / top of the stack is the one got by evaluation of v:exception.  The complete
// / stack of all caught and pending exceptions is embedded in the various
// / cstacks; the pending exceptions, however, are not on the caught stack.
    // /
// / Garbage collection can only take place when we are sure there are no Lists
// / or Dictionaries being used internally.  This is flagged with
// / "may_garbage_collect" when we are at the toplevel.
// / "want_garbage_collect" is set by the garbagecollect() function, which means
// / we do garbage collection before waiting for a char at the toplevel.
// / "garbage_collect_at_exit" indicates garbagecollect(1) was called.
// /
    // Special values for current_SID.
    // when using a modeline
    // for "--cmd" argument
    // for "-c" argument
    // for sourcing environment variable
    // option was reset because of an error
    // don't set scriptID
    // for Lua scripts/chunks
    // for API clients
    // for sourcing a string
    // Script CTX being sourced or was sourced to define the current function.
    // ID of the current channel making a client API call
    // Scope information for the code that indirectly triggered the current
// provider function call
    // int value of T_CCO
    // When highlight_match is true, highlight a match, starting at the cursor
// position.  Search_match_lines is the number of lines after the match (0 for
// a match within one line), search_match_endcol the column number of the
// character just after the match in the last line.
    // show search match pos
    // lines of of matched string
    // col nr of match end
    // don't use 'smartcase' once
    // need to check file
                                                 // timestamps asap
    // did check timestamps
                                                 // recently
    // Don't check timestamps
    // Is apply_autocmds() busy?
    // *Enter autocmds disabled
    // *Leave autocmds disabled
    // did ":set modified"
    // FileType event found
    // value for did_filetype when starting to execute autocommands
    // When deleting the current buffer, another one must be loaded.
// If we know which one is preferred, au_new_curbuf is set to it.
    // When deleting a buffer/window and autocmd_busy is true, do not free the
// buffer/window. but link it in the list starting with
// au_pending_free_buf/ap_pending_free_win, using b_next/w_next.
// Free the buffer/window when autocmd_busy is being set to false.
    // Mouse coordinates, set by handle_mouse_event()
    // mouse below last line
    // mouse right of line
    // extending Visual area with
                                              // mouse dragging
    // The root of the menu hierarchy.
    // While defining the system menu, sys_menu is true.  This avoids
// overruling of menus that the user already defined.
    // While redrawing the screen this flag is set.  It means the screen size
// ('lines' and 'rows') must not be changed.
    // All windows are linked in a list. firstwin points to the first entry,
// lastwin to the last entry (can be the same as firstwin) and curwin to the
// currently active window.
    // first window
    // last window
    // previous window
    // NOLINT
    // When using this macro "break" only breaks out of the inner loop. Use "goto"
// to break out of the tabpage loop.
    // -V:FOR_ALL_WINDOWS_IN_TAB:501
    // currently active window
    // window used in aucmd_prepbuf()
    // aucmd_win is being used
    // The window layout is kept in a tree of frames.  topframe points to the top
// of the tree.
    // top of the window frame tree
    // Tab pages are alternative topframes.  "first_tabpage" points to the first
// one in the list, "curtab" is the current one.
    // need to redraw tabline
    // Iterates over all tabs in the tab list
    // All buffers are linked in a list. 'firstbuf' points to the first entry,
// 'lastbuf' to the last entry and 'curbuf' to the currently active buffer.
    // first buffer
    // last buffer
    // currently active buffer
    // Iterates over all buffers in the buffer list.
    // Iterate through all the signs placed in a buffer
    // NOLINT
    // List of files being edited (global argument list).  curwin->w_alist points
// to this when the window is using the global argument list.
    // global argument list
    // /< the previous argument list id
    // accessed last file in
                                            // global_alist
    // column for ruler
    // 'rulerfmt' width of ruler when non-zero
    // column for shown command
    // When starting or exiting some things are done differently (e.g. screen
// updating).
    // First NO_SCREEN, then NO_BUFFERS, then 0 when startup finished.
    // true when planning to exit. Might keep running if there is a changed buffer.
    // is stdin a terminal?
    // is stdout a terminal?
    // true when doing full-screen output, otherwise only writing some messages.
// volatile because it is used in a signal handler.
    // When started in restricted mode (-Z).
    // / Non-zero when only "safe" commands are allowed, e.g. when sourcing .exrc or
// / .vimrc in current directory.
    // / Non-zero when changing text and jumping to another window/buffer is not
// / allowed.
    // / Non-zero when the current buffer can't be changed.  Used for FileChangedRO.
    // / Non-zero when no buffer name can be changed, no buffer can be deleted and
// / current directory can't be changed. Used for SwapExists et al.
    // / Non-zero when evaluating an expression in a "sandbox".  Several things are
// / not allowed then.
    // / Batch-mode: "-es" or "-Es" commandline argument was given.
    // / Start position of active Visual selection.
    // / Whether Visual mode is active.
    // / Whether Select mode is active.
    // / Whether to restart the selection after a Select-mode mapping or menu.
    // / Type of Visual mode.
    // / true when redoing Visual.
    // / When pasting text with the middle mouse button in visual mode with
// / restart_edit set, remember where it started so we can set Insstart.
    // This flag is used to make auto-indent work right on lines where only a
// <RETURN> or <ESC> is typed. It is set when an auto-indent is done, and
// reset when any other editing is done on the line. If an <ESC> or <RETURN>
// is received, and did_ai is true, the line is truncated.
    // Column of first char after autoindent.  0 when no autoindent done.  Used
// when 'backspace' is 0, to avoid backspacing over autoindent.
    // This is a character which will end a start-middle-end comment when typed as
// the first character on a new line.  It is taken from the last character of
// the "end" comment leader when the COM_AUTO_END flag is given for that
// comment end in 'comments'.  It is only valid when did_ai is true.
    // This flag is set after a ":syncbind" to let the check_scrollbind() function
// know that it should not attempt to perform scrollbinding due to the scroll
// that was a result of the ":syncbind." (Otherwise, check_scrollbind() will
// undo some of the work done by ":syncbind.")  -ralston
    // This flag is set when a smart indent has been performed. When the next typed
// character is a '{' the inserted tab will be deleted again.
    // This flag is set after an auto indent. If the next typed character is a '}'
// one indent will be removed.
    // This flag is set after an "O" command. If the next typed character is a '{'
// one indent will be removed.
    // w_cursor before formatting text.
    // Stuff for insert mode.
    // This is where the latest
                                        // insert/append mode started.
    // This is where the latest insert/append mode started. In contrast to
// Insstart, this won't be reset by certain keys and is needed for
// op_insert(), to detect correctly where inserting by the user started.
    // Stuff for VREPLACE mode.
    // Line count when "gR" started
    // #Lines changed by "gR" so far
    // increase around internal delete/replace
    // These flags are set based upon 'fileencoding'.
// Note that "enc_utf8" is also set for "unicode", because the characters are
// internally stored as UTF-8 (to avoid trouble with NUL bytes).
    // japan
    // euc-jp
    // korea
    // euc-kr
    // chinese
    // euc-cn
    // taiwan
    // euc-tw
    // 2byte-
    // mbyte flags that used to depend on 'encoding'. These are now deprecated, as
// 'encoding' is always "utf-8". Code that use them can be refactored to
// remove dead code.
    #[c2rust::src_loc = "602:9"]
    pub const has_mbyte: libc::c_int = true_0;
    use super::stdbool_h::true_0;
    // NVIM_GLOBALS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/lib/kvec.h:12"]
pub mod kvec_h {
    // The MIT License
//
// Copyright (c) 2008, by Attractive Chaos <attractor@live.co.uk>
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
    // An example:
//
//     #include "kvec.h"
//     int main() {
//       kvec_t(int) array = KV_INITIAL_VALUE;
//       kv_push(array, 10); // append
//       kv_a(array, 20) = 5; // dynamic
//       kv_A(array, 20) = 4; // static
//       kv_destroy(array);
//       return 0;
//     }
    // / Drop last n items from kvec without resizing
// /
// / Previously spelled as `(void)kv_pop(v)`, repeated n times.
// /
// / @param[out]  v  Kvec to drop items from.
// / @param[in]  n  Number of elements to drop.
    // / Type of a vector with a few first members allocated on stack
// /
// / Is compatible with #kv_A, #kv_pop, #kv_size, #kv_max, #kv_last.
// / Is not compatible with #kv_resize, #kv_resize_full, #kv_copy, #kv_push,
// / #kv_pushp, #kv_a, #kv_destroy.
// /
// / @param[in]  type  Type of vector elements.
// / @param[in]  init_size  Number of the elements in the initial array.
    // / Initialize vector with preallocated array
// /
// / @param[out]  v  Vector to initialize.
    // / Move data to a new destination and free source
    #[inline]
    #[c2rust::src_loc = "139:1"]
    pub unsafe extern "C" fn _memcpy_free(dest: *mut libc::c_void,
                                          src: *mut libc::c_void,
                                          size: size_t) -> *mut libc::c_void {
        memcpy(dest, src, size);
        let mut ptr_ =
            &src as *const *mut libc::c_void as *mut *mut libc::c_void;
        xfree(*ptr_);
        *ptr_ = NULL_0 as *mut libc::c_void;
        return dest;
    }
    use super::stddef_h::{size_t, NULL_0};
    use super::string_h::memcpy;
    use super::memory_h_generated_h::xfree;
    // NVIM_LIB_KVEC_H
    // / Free array of elements of a vector with preallocated array if needed
// /
// / @param[out]  v  Vector to free.
    // / Push value to a vector with preallocated array
// /
// / @param[out]  v  Vector to push to.
// / @param[in]  x  Value to push.
    // / Get location where to store new element to a vector with preallocated array
// /
// / @param[in,out]  v  Vector to push to.
// /
// / @return Pointer to the place where new value should be stored.
    /* 2^x initial array size. */
    /* hard to fix this here and is not very necessary if users will use */
    /* capacity is not guaranteed to have size that is a power of 2, it is */
    /* not to bother with checking whether (v).capacity is 0. But now */
    /* Thus when vector is full capacity may not be zero and it is safe */
    /* ARRAY_SIZE((v).init_array) is the minimal capacity of this vector. */
    // / Resize vector with preallocated array when it is full
// /
// / @param[out]  v  Vector to resize.
    // / Resize vector with preallocated array
// /
// / @note May not resize to an array smaller then init_array: if requested,
// /       init_array will be used.
// /
// / @param[out]  v  Vector to resize.
// / @param[in]  s  New size.
    // -V:kvi_push:512
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/uintn-identity.h:12"]
pub mod uintn_identity_h {
    #[inline]
    #[c2rust::src_loc = "44:1"]
    pub unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t)
     -> __uint64_t {
        return __x;
    }
    #[inline]
    #[c2rust::src_loc = "38:1"]
    pub unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t)
     -> __uint32_t {
        return __x;
    }
    #[inline]
    #[c2rust::src_loc = "32:1"]
    pub unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t)
     -> __uint16_t {
        return __x;
    }
    use super::types_h::{__uint64_t, __uint32_t, __uint16_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/byteswap.h:12"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "69:15"]
    pub unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
        return ((__bsx as libc::c_ulonglong &
                     0xff00000000000000 as libc::c_ulonglong) >>
                    56 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff000000000000 as libc::c_ulonglong) >>
                        40 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff0000000000 as libc::c_ulonglong) >>
                        24 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff00000000 as libc::c_ulonglong) >>
                        8 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff000000 as libc::c_ulonglong) << 8 as libc::c_int
                    |
                    (__bsx as libc::c_ulonglong &
                         0xff0000 as libc::c_ulonglong) << 24 as libc::c_int |
                    (__bsx as libc::c_ulonglong & 0xff00 as libc::c_ulonglong)
                        << 40 as libc::c_int |
                    (__bsx as libc::c_ulonglong & 0xff as libc::c_ulonglong)
                        << 56 as libc::c_int) as __uint64_t;
    }
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
        return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
                   (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
                   (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
                   (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
        return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
                    |
                    (__bsx as libc::c_int & 0xff as libc::c_int) <<
                        8 as libc::c_int) as __uint16_t;
    }
    use super::types_h::{__uint64_t, __uint32_t, __uint16_t};
}
#[c2rust::header_src = "/usr/include/netinet/in.h:12"]
pub mod in_h {
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "374:1"]
        pub fn ntohl(__netlong: uint32_t) -> uint32_t;
        #[no_mangle]
        #[c2rust::src_loc = "375:1"]
        pub fn ntohs(__netshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/ascii.h:13"]
pub mod ascii_h {
    #[c2rust::src_loc = "19:9"]
    pub const NUL: libc::c_int = '\u{0}' as i32;
    /* CR is used by Mac OS X */
    // Control Sequence Introducer
    /* Device Control String */
    /* String Terminator */
    /* '?' -> DEL, '@' -> ^@, etc. */
    /* @ */
    #[c2rust::src_loc = "67:9"]
    pub const Ctrl_V: libc::c_int = 22 as libc::c_int;
    /* CTRL- [ Left Square Bracket == ESC*/
    /* \ BackSLash */
    /* ] Right Square Bracket */
    /* ^ */
    /*
 * Character that separates dir names in a path.
 */
    // / Checks if `c` is a space or tab character.
// /
// / @see {ascii_isdigit}
    #[inline(always)]
    #[c2rust::src_loc = "117:1"]
    pub unsafe extern "C" fn ascii_iswhite(mut c: libc::c_int) -> bool {
        return c == ' ' as i32 || c == '\t' as i32;
    }
    // / Check whether character is a decimal digit.
// /
// / Library isdigit() function is officially locale-dependent and, for
// / example, returns true for superscript 1 (¹) in locales where encoding
// / contains it in lower 8 bits. Also avoids crashes in case c is below
// / 0 or above 255: library functions are officially defined as accepting
// / only EOF and unsigned char values (otherwise it is undefined behaviour)
// / what may be used for some optimizations (e.g. simple `return
// / isdigit_table[c];`).
    #[inline(always)]
    #[c2rust::src_loc = "131:1"]
    pub unsafe extern "C" fn ascii_isdigit(mut c: libc::c_int) -> bool {
        return c >= '0' as i32 && c <= '9' as i32;
    }
    // / Checks if `c` is a hexadecimal digit, that is, one of 0-9, a-f, A-F.
// /
// / @see {ascii_isdigit}
    #[inline(always)]
    #[c2rust::src_loc = "139:1"]
    pub unsafe extern "C" fn ascii_isxdigit(mut c: libc::c_int) -> bool {
        return c >= '0' as i32 && c <= '9' as i32 ||
                   c >= 'a' as i32 && c <= 'f' as i32 ||
                   c >= 'A' as i32 && c <= 'F' as i32;
    }
    // / Checks if `c` is an “identifier” character
// /
// / That is, whether it is alphanumeric character or underscore.
    #[inline(always)]
    #[c2rust::src_loc = "149:1"]
    pub unsafe extern "C" fn ascii_isident(mut c: libc::c_int) -> bool {
        return c as libc::c_uint >= 'A' as i32 as libc::c_uint &&
                   c as libc::c_uint <= 'Z' as i32 as libc::c_uint ||
                   c as libc::c_uint >= 'a' as i32 as libc::c_uint &&
                       c as libc::c_uint <= 'z' as i32 as libc::c_uint ||
                   ascii_isdigit(c) as libc::c_int != 0 || c == '_' as i32;
    }
    // / Checks if `c` is a binary digit, that is, 0-1.
// /
// / @see {ascii_isdigit}
    #[inline(always)]
    #[c2rust::src_loc = "157:1"]
    pub unsafe extern "C" fn ascii_isbdigit(mut c: libc::c_int) -> bool {
        return c == '0' as i32 || c == '1' as i32;
    }
    // / Checks if `c` is a white-space character, that is,
// / one of \f, \n, \r, \t, \v.
// /
// / @see {ascii_isdigit}
    #[inline(always)]
    #[c2rust::src_loc = "166:1"]
    pub unsafe extern "C" fn ascii_isspace(mut c: libc::c_int) -> bool {
        return c >= 9 as libc::c_int && c <= 13 as libc::c_int ||
                   c == ' ' as i32;
    }
    /* NVIM_ASCII_H */
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/buffer.h:16"]
pub mod buffer_h {
    // for linenr_T
    // for exarg_T
    // Values for buflist_getfile()
    // set pcmark before jumping
    // jumping to alternate file (not buf num)
    // respect 'switchbuf' settings when jumping
    // Return values of getfile()
    // normal error
    // "not written" error
    // success, same file
    // success, opened another file
    // Values for buflist_new() flags
    // May re-use curbuf for new buffer
    // Put new buffer in buffer list
    // Allocating dummy buffer
    // create a new buffer
    // Don't copy options to existing buffer
    // Values for action argument for do_buffer()
    // go to specified buffer
    // split window and go to specified buffer
    // unload specified buffer(s)
    // delete specified buffer(s) from buflist
    // delete specified buffer(s) really
    // Values for start argument for do_buffer()
    // "count" buffer from current buffer
    // "count" buffer from first buffer
    // "count" buffer from last buffer
    // "count" mod. buffer from current buffer
    // flags for buf_freeall()
    // buffer is going to be deleted
    // buffer is going to be wiped out
    // do not free undo information
    // / Set b:changedtick, also checking b: for consistency in debug build
// /
// / @param[out]  buf  Buffer to set changedtick in.
// / @param[in]  changedtick  New value.
    // For some reason formatc does not like the below.
    // / Get b:changedtick value
// /
// / Faster then querying b:.
// /
// / @param[in]  buf  Buffer to get b:changedtick from.
    // / Increment b:changedtick value
// /
// / Also checks b: for consistency in case of debug build.
// /
// / @param[in,out]  buf  Buffer to increment value in.
    #[inline(always)]
    #[c2rust::src_loc = "123:1"]
    pub unsafe extern "C" fn buf_inc_changedtick(buf: *mut buf_T) {
        buf_set_changedtick(buf,
                            buf_get_changedtick(buf) +
                                1 as libc::c_int as libc::c_long);
    }
    #[inline(always)]
    #[c2rust::src_loc = "110:1"]
    pub unsafe extern "C" fn buf_get_changedtick(buf: *const buf_T)
     -> varnumber_T {
        return (*buf).changedtick_di.di_tv.vval.v_number;
    }
    #[inline(always)]
    #[c2rust::src_loc = "74:1"]
    pub unsafe extern "C" fn buf_set_changedtick(buf: *mut buf_T,
                                                 changedtick: varnumber_T) {
        let mut old_val = (*buf).changedtick_di.di_tv;
        let changedtick_di =
            tv_dict_find((*buf).b_vars,
                         b"changedtick\x00" as *const u8 as
                             *const libc::c_char,
                         (::std::mem::size_of::<[libc::c_char; 12]>() as
                              libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                              libc::c_ulong)
                             as ptrdiff_t);
        if !changedtick_di.is_null() {
        } else {
            __assert_fail(b"changedtick_di != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"../src/nvim/buffer.h\x00" as *const u8 as
                              *const libc::c_char,
                          82 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 58],
                                                    &[libc::c_char; 58]>(b"void buf_set_changedtick(buf_T *const, const varnumber_T)\x00")).as_ptr());
        }
        if (*changedtick_di).di_tv.v_type as libc::c_uint ==
               VAR_NUMBER as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(b"changedtick_di->di_tv.v_type == VAR_NUMBER\x00" as
                              *const u8 as *const libc::c_char,
                          b"../src/nvim/buffer.h\x00" as *const u8 as
                              *const libc::c_char,
                          83 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 58],
                                                    &[libc::c_char; 58]>(b"void buf_set_changedtick(buf_T *const, const varnumber_T)\x00")).as_ptr());
        }
        if (*changedtick_di).di_tv.v_lock as libc::c_uint ==
               VAR_FIXED as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(b"changedtick_di->di_tv.v_lock == VAR_FIXED\x00" as
                              *const u8 as *const libc::c_char,
                          b"../src/nvim/buffer.h\x00" as *const u8 as
                              *const libc::c_char,
                          84 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 58],
                                                    &[libc::c_char; 58]>(b"void buf_set_changedtick(buf_T *const, const varnumber_T)\x00")).as_ptr());
        }
        if (*changedtick_di).di_flags as libc::c_int ==
               DI_FLAGS_RO as libc::c_int | DI_FLAGS_FIX as libc::c_int {
        } else {
            __assert_fail(b"changedtick_di->di_flags == (DI_FLAGS_RO|DI_FLAGS_FIX)\x00"
                              as *const u8 as *const libc::c_char,
                          b"../src/nvim/buffer.h\x00" as *const u8 as
                              *const libc::c_char,
                          87 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 58],
                                                    &[libc::c_char; 58]>(b"void buf_set_changedtick(buf_T *const, const varnumber_T)\x00")).as_ptr());
        }
        if changedtick_di ==
               &mut (*buf).changedtick_di as *mut ChangedtickDictItem as
                   *mut dictitem_T {
        } else {
            __assert_fail(b"changedtick_di == (dictitem_T *)&buf->changedtick_di\x00"
                              as *const u8 as *const libc::c_char,
                          b"../src/nvim/buffer.h\x00" as *const u8 as
                              *const libc::c_char,
                          89 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 58],
                                                    &[libc::c_char; 58]>(b"void buf_set_changedtick(buf_T *const, const varnumber_T)\x00")).as_ptr());
        }
        (*buf).changedtick_di.di_tv.vval.v_number = changedtick;
        if tv_dict_is_watched((*buf).b_vars) {
            tv_dict_watcher_notify((*buf).b_vars,
                                   (*buf).changedtick_di.di_key.as_mut_ptr()
                                       as *mut libc::c_char,
                                   &mut (*buf).changedtick_di.di_tv,
                                   &mut old_val);
        };
    }
    use super::buffer_defs_h::{buf_T, ChangedtickDictItem};
    use super::typval_h::{varnumber_T, typval_T, dictitem_T, VarType,
                          VAR_NUMBER, VarLockStatus, VAR_FIXED, DI_FLAGS_RO,
                          DI_FLAGS_FIX, tv_dict_is_watched};
    use super::typval_h_generated_h::{tv_dict_find, tv_dict_watcher_notify};
    use super::stddef_h::ptrdiff_t;
    use super::assert_h::__assert_fail;
    use super::stdint_uintn_h::uint8_t;
    // NVIM_BUFFER_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/charset.h:17"]
pub mod charset_h {
    // / Check if `c` is one of the characters in 'breakat'.
// / Used very often if 'linebreak' is set
    #[inline(always)]
    #[c2rust::src_loc = "106:1"]
    pub unsafe extern "C" fn vim_isbreak(mut c: libc::c_int) -> bool {
        return breakat_flags[c as char_u as usize] != 0;
    }
    use super::nvim_types_h::char_u;
    use super::option_defs_h::breakat_flags;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "96:1"]
        pub fn rem_backslash(str: *const char_u) -> bool;
    }
    // NVIM_CHARSET_H
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/ex_docmd.h.generated.h:22"]
pub mod ex_docmd_h_generated_h {
    use super::nvim_types_h::char_u;
    use super::stddef_h::size_t;
    use super::stdio_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "56:1"]
        pub fn find_cmdline_var(src: *const char_u, usedlen: *mut size_t)
         -> ssize_t;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/mark.h:29"]
pub mod mark_h {
    // for exarg_T
    // / Set fmark using given value
    // / Free and set fmark using given value
    // / Clear given fmark
    // / Set given extended mark (regular mark + file name)
    // / Free and set given extended mark (regular mark + file name)
    // / Convert mark name to the offset
    // / Convert local mark name to the offset
    #[inline]
    #[c2rust::src_loc = "66:1"]
    pub unsafe extern "C" fn mark_local_index(name: libc::c_char)
     -> libc::c_int {
        return if name as libc::c_uint >= 'a' as i32 as libc::c_uint &&
                      name as libc::c_uint <= 'z' as i32 as libc::c_uint {
                   (name as libc::c_int) - 'a' as i32
               } else if name as libc::c_int == '\"' as i32 {
                   NMARKS
               } else if name as libc::c_int == '^' as i32 {
                   (NMARKS) + 1 as libc::c_int
               } else if name as libc::c_int == '.' as i32 {
                   (NMARKS) + 2 as libc::c_int
               } else { -(1 as libc::c_int) };
    }
    // / Return true if position a is before (less than) position b.
    #[inline(always)]
    #[c2rust::src_loc = "89:1"]
    pub unsafe extern "C" fn lt(mut a: pos_T, mut b: pos_T) -> bool {
        if a.lnum != b.lnum {
            return a.lnum < b.lnum
        } else if a.col != b.col {
            return a.col < b.col
        } else { return a.coladd < b.coladd };
    }
    // / Return true if position a and b are equal.
    #[inline(always)]
    #[c2rust::src_loc = "101:1"]
    pub unsafe extern "C" fn equalpos(mut a: pos_T, mut b: pos_T) -> bool {
        return a.lnum == b.lnum && a.col == b.col && a.coladd == b.coladd;
    }
    #[inline]
    #[c2rust::src_loc = "55:1"]
    pub unsafe extern "C" fn mark_global_index(name: libc::c_char)
     -> libc::c_int {
        return if name as libc::c_uint >= 'A' as i32 as libc::c_uint &&
                      name as libc::c_uint <= 'Z' as i32 as libc::c_uint {
                   (name as libc::c_int) - 'A' as i32
               } else if ascii_isdigit(name as libc::c_int) as libc::c_int !=
                             0 {
                   (NMARKS) + (name as libc::c_int - '0' as i32)
               } else { -(1 as libc::c_int) };
    }
    // / Return true if position a is less than or equal to b.
    #[inline(always)]
    #[c2rust::src_loc = "107:1"]
    pub unsafe extern "C" fn ltoreq(mut a: pos_T, mut b: pos_T) -> bool {
        return lt(a, b) as libc::c_int != 0 ||
                   equalpos(a, b) as libc::c_int != 0;
    }
    // / Clear the pos_T structure pointed to by a.
    #[inline(always)]
    #[c2rust::src_loc = "113:1"]
    pub unsafe extern "C" fn clearpos(mut a: *mut pos_T) {
        (*a).lnum = 0 as libc::c_int as linenr_T;
        (*a).col = 0 as libc::c_int;
        (*a).coladd = 0 as libc::c_int;
    }
    use super::mark_defs_h::NMARKS;
    use super::pos_h::{pos_T, linenr_T, colnr_T};
    use super::ascii_h::ascii_isdigit;
    // NVIM_MARK_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/math.h:30"]
pub mod math_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "8:1"]
        pub fn xisinf(d: libc::c_double) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "9:1"]
        pub fn xisnan(d: libc::c_double) -> libc::c_int;
    }
    // NVIM_MATH_H
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/option.h.generated.h:38"]
pub mod option_h_generated_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "70:1"]
        pub fn csh_like_shell() -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/ops.h:39"]
pub mod ops_h {
    // for MotionType and oparg_T
    // for exarg_T
    /* flags for do_put() */
    /* make indent look nice */
    /* leave cursor after end of new text */
    /* leave cursor on last line of new text */
    /* put register as lines */
    /* split line for linewise register */
    /* put linewise register below Visual sel. */
    /*
 * Registers:
 *      0 = register for latest (unnamed) yank
 *   1..9 = registers '1' to '9', for deletes
 * 10..35 = registers 'a' to 'z'
 *     36 = delete register '-'
 *     37 = selection register '*'
 *     38 = clipboard register '+'
 */
    // The following registers should not be saved in ShaDa file:
    #[c2rust::src_loc = "36:9"]
    pub const STAR_REGISTER: libc::c_int = 37 as libc::c_int;
    // Operator IDs; The order must correspond to opchars[] in ops.c!
    // no pending operation
    // "d"  delete operator
    // "y"  yank operator
    // "c"  change operator
    // "<"  left shift operator
    // ">"  right shift operator
    // "!"  filter operator
    // "g~" switch case operator
    // "="  indent operator
    // "gq" format operator
    // ":"  colon operator
    // "gU" make upper case operator
    // "gu" make lower case operator
    // "J"  join operator, only for Visual mode
    // "gJ"  join operator, only for Visual mode
    // "g?" rot-13 encoding
    // "r"  replace chars, only for Visual mode
    // "I"  Insert column, only for Visual mode
    // "A"  Append column, only for Visual mode
    // "zf" define a fold
    // "zo" open folds
    // "zO" open folds recursively
    // "zc" close folds
    // "zC" close folds recursively
    // "zd" delete folds
    // "zD" delete folds recursively
    // "gw" format operator, keeps cursor pos
    // "g@" call 'operatorfunc'
    // "<C-A>" Add to the number or alphabetic
                                // character (OP_ADD conflicts with Perl)
    // "<C-X>" Subtract from the number or
                                // alphabetic character
    // / Flags for get_reg_contents().
    // /< Do not allow expression register.
    // /< Return expression itself for "=" register.
    // /< Return list.
    // / Definition of one register
    // /< Pointer to an array of line pointers.
    // /< Number of lines in y_array.
    // /< Register type
    // /< Register width (only valid for y_type == kBlockWise).
    // /< Time when register was last modified.
    // /< Additional data from ShaDa file.
    // / Convert register name into register index
// /
// / @param[in]  regname  Register name.
// /
// / @return Index in y_regs array or -1 if register name was not recognized.
    #[inline]
    #[c2rust::src_loc = "96:1"]
    pub unsafe extern "C" fn op_reg_index(regname: libc::c_int)
     -> libc::c_int {
        if ascii_isdigit(regname) {
            return regname - '0' as i32
        } else if regname as libc::c_uint >= 'a' as i32 as libc::c_uint &&
                      regname as libc::c_uint <= 'z' as i32 as libc::c_uint {
            return regname as uint8_t as libc::c_int - 'a' as i32 +
                       10 as libc::c_int
        } else if regname as libc::c_uint >= 'A' as i32 as libc::c_uint &&
                      regname as libc::c_uint <= 'Z' as i32 as libc::c_uint {
            return regname as uint8_t as libc::c_int - 'A' as i32 +
                       10 as libc::c_int
        } else if regname == '-' as i32 {
            return DELETION_REGISTER
        } else if regname == '*' as i32 {
            return STAR_REGISTER
        } else if regname == '+' as i32 {
            return PLUS_REGISTER
        } else { return -(1 as libc::c_int) };
    }
    #[c2rust::src_loc = "33:9"]
    pub const DELETION_REGISTER: libc::c_int = 36 as libc::c_int;
    #[c2rust::src_loc = "37:9"]
    pub const PLUS_REGISTER: libc::c_int = 38 as libc::c_int;
    use super::ascii_h::ascii_isdigit;
    use super::stdint_uintn_h::uint8_t;
    // NVIM_OPS_H
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/eval/encode.h.generated.h:52"]
pub mod encode_h_generated_h {
    use super::typval_h::typval_T;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "9:1"]
        pub fn encode_tv2echo(tv: *mut typval_T, len: *mut size_t)
         -> *mut libc::c_char;
    }
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::types_h::{__int8_t, __uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __int64_t, __uint64_t, __intmax_t,
                        __uintmax_t, __uid_t, __gid_t, __time_t, __ssize_t};
pub use self::stdint_intn_h::{int8_t, int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
pub use self::stdint_h::{intmax_t, uintmax_t, SIZE_MAX};
pub use self::stdarg_h::va_list;
pub use self::stddef_h::{size_t, ptrdiff_t, NULL_0, NULL_1, NULL};
pub use self::nvim_types_h::{char_u, handle_T, LuaRef};
pub use self::typval_h::{sctx_T, scid_T, varnumber_T, float_T, listvar_S,
                         VarLockStatus, VAR_FIXED, VAR_LOCKED, VAR_UNLOCKED,
                         list_T, listitem_T, listitem_S, typval_T,
                         typval_vval_union, partial_T, partial_S, dict_T,
                         dictvar_S, ScopeType, VAR_DEF_SCOPE, VAR_SCOPE,
                         VAR_NO_SCOPE, ufunc_T, ufunc, funccall_T,
                         SpecialVarValue, kSpecialVarNull, kSpecialVarTrue,
                         kSpecialVarFalse, VarType, VAR_PARTIAL, VAR_SPECIAL,
                         VAR_FLOAT, VAR_DICT, VAR_LIST, VAR_FUNC, VAR_STRING,
                         VAR_NUMBER, VAR_UNKNOWN, listwatch_T, listwatch_S,
                         CallbackType, kCallbackPartial, kCallbackFuncref,
                         kCallbackNone, Callback, C2RustUnnamed, dict_watcher,
                         DictWatcher, ScopeDictDictItem, dictitem_T,
                         C2RustUnnamed_0, DI_FLAGS_ALLOC, DI_FLAGS_LOCK,
                         DI_FLAGS_FIX, DI_FLAGS_RO_SBX, DI_FLAGS_RO,
                         tv_list_ref, tv_list_set_ret, tv_list_locked,
                         tv_list_set_lock, tv_list_set_copyid, tv_list_len,
                         tv_list_copyid, tv_list_latest_copy, tv_list_uidx,
                         tv_list_has_watchers, tv_list_first, tv_list_last,
                         tv_dict_set_ret, tv_dict_len, tv_dict_is_watched,
                         tv_init, tv_get_float_chk, tv_dict_watcher_node_data,
                         tv_is_func, funccall_S};
pub use self::pos_h::{linenr_T, colnr_T, pos_T, lpos_T};
pub use self::stdio_h::{ssize_t, snprintf};
pub use self::sys_types_h::{gid_t, uid_t};
pub use self::time_t_h::time_t;
pub use self::pthreadtypes_arch_h::__pthread_rwlock_arch_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::{pthread_mutex_t, pthread_rwlock_t};
pub use self::stdlib_h::{__compar_fn_t, malloc, calloc, realloc, free, abort,
                         qsort};
pub use self::hashtab_h::{hash_T, hashitem_S, hashitem_T, hashtable_S,
                          hashtab_T};
pub use self::garray_h::{growarray, garray_T, ga_append_via_ptr, ga_grow};
pub use self::queue_h::{_queue, QUEUE, QUEUE_EMPTY, QUEUE_INIT, QUEUE_ADD,
                        QUEUE_INSERT_HEAD, QUEUE_INSERT_TAIL, QUEUE_REMOVE};
pub use self::profile_h::proftime_T;
pub use self::grid_defs_h::{schar_T, sattr_T, ScreenGrid};
pub use self::buffer_defs_h::{file_buffer, C2RustUnnamed_1,
                              BufUpdateCallbacks, C2RustUnnamed_2, synblock_T,
                              buf_T, win_T, window_S, qf_info_T, FloatConfig,
                              WinStyle, kWinStyleMinimal, kWinStyleUnused,
                              FloatRelative, kFloatRelativeCursor,
                              kFloatRelativeWindow, kFloatRelativeEditor,
                              FloatAnchor, taggy_T, taggy, matchitem_T,
                              matchitem, match_T, posmatch_T, posmatch,
                              llpos_T, winopt_T, alist_T, arglist, wline_T,
                              w_line, pos_save_T, C2RustUnnamed_3,
                              C2RustUnnamed_4, frame_T, frame_S, disptick_T,
                              syn_time_T, mapblock_T, mapblock, wininfo_T,
                              wininfo_S, ChangedtickDictItem, win_hl_attr,
                              qf_info_S};
pub use self::map_h::{Map_uint64_t_ExtmarkNs, kh_uint64_t_ExtmarkNs_map_t,
                      ExtmarkNs, Map_uint64_t_uint64_t,
                      kh_uint64_t_uint64_t_map_t, Map_uint64_t_ExtmarkItem,
                      kh_uint64_t_ExtmarkItem_map_t, Map_uint64_t_ptr_t,
                      kh_uint64_t_ptr_t_map_t, kh_cstr_t_ptr_t_map_t,
                      Map_cstr_t_ptr_t, map_uint64_t_ptr_t_get};
pub use self::khash_h::{khint32_t, khint_t, __ac_X31_hash_string,
                        __ac_Wang_hash};
pub use self::extmark_defs_h::{ExtmarkItem, VirtText, VirtTextChunk,
                               extmark_undo_vec_t, ExtmarkUndoObject,
                               undo_object};
pub use self::marktree_h::{MarkTree, mtnode_t, mtnode_s, mtkey_t, mtpos_t};
pub use self::map_defs_h::{ptr_t, cstr_t};
pub use self::terminal_h::{Terminal, terminal};
pub use self::sign_defs_h::{signlist_T, signlist, signgroup_T, signgroup_S};
pub use self::regexp_defs_h::{regprog_T, regprog, regengine_T, regengine,
                              regmmatch_T, regmatch_T, reg_extmatch_T,
                              reg_extmatch};
pub use self::defs_h::{Window, Boolean, Integer, Float, String_0, object,
                       C2RustUnnamed_10, Dictionary, KeyValuePair,
                       key_value_pair, Object, Array, ObjectType,
                       kObjectTypeTabpage, kObjectTypeWindow,
                       kObjectTypeBuffer, kObjectTypeLuaRef,
                       kObjectTypeDictionary, kObjectTypeArray,
                       kObjectTypeString, kObjectTypeFloat,
                       kObjectTypeInteger, kObjectTypeBoolean, kObjectTypeNil,
                       is_internal_call};
pub use self::mark_defs_h::{fmark_T, filemark, xfmark_T, xfilemark, NMARKS};
pub use self::time_h::Timestamp;
pub use self::option_defs_h::{LastSet, breakat_flags};
pub use self::syntax_defs_h::{synstate_T, syn_state, C2RustUnnamed_5,
                              bufstate_T, buf_state};
pub use self::undo_defs_h::{u_header_T, u_header, visualinfo_T, u_entry_T,
                            u_entry, C2RustUnnamed_6, C2RustUnnamed_7,
                            C2RustUnnamed_8, C2RustUnnamed_9};
pub use self::fs_defs_h::FileID;
pub use self::memline_defs_h::{memline_T, memline, chunksize_T, ml_chunksize,
                               infoptr_T, info_pointer};
pub use self::memfile_defs_h::{bhdr_T, bhdr, mf_hashitem_T, mf_hashitem,
                               blocknr_T, memfile_T, memfile, mf_hashtab_T,
                               mf_hashtab};
pub use self::struct_iovec_h::iovec;
pub use self::uv_h::{uv_loop_s, uv_signal_t, uv_signal_s, C2RustUnnamed_11,
                     uv_signal_cb, uv_handle_t, uv_handle_s, C2RustUnnamed_12,
                     uv_close_cb, uv_handle_type, UV_HANDLE_TYPE_MAX, UV_FILE,
                     UV_SIGNAL, UV_UDP, UV_TTY, UV_TIMER, UV_TCP, UV_STREAM,
                     UV_PROCESS, UV_PREPARE, UV_POLL, UV_NAMED_PIPE, UV_IDLE,
                     UV_HANDLE, UV_FS_POLL, UV_FS_EVENT, UV_CHECK, UV_ASYNC,
                     UV_UNKNOWN_HANDLE, uv_loop_t, C2RustUnnamed_13,
                     C2RustUnnamed_14, uv_async_t, uv_async_s, uv_async_cb,
                     C2RustUnnamed_15, C2RustUnnamed_16, uv_req_type,
                     UV_REQ_TYPE_MAX, UV_GETNAMEINFO, UV_GETADDRINFO, UV_WORK,
                     UV_FS, UV_UDP_SEND, UV_SHUTDOWN, UV_WRITE, UV_CONNECT,
                     UV_REQ, UV_UNKNOWN_REQ, uv_stream_s, uv_connection_cb,
                     uv_stream_t, uv_shutdown_t, uv_shutdown_s,
                     uv_shutdown_cb, uv_connect_t, uv_connect_s,
                     uv_connect_cb, uv_read_cb, uv_alloc_cb, C2RustUnnamed_17,
                     uv_tcp_s, C2RustUnnamed_18, uv_tcp_t, uv_pipe_s,
                     C2RustUnnamed_19, uv_pipe_t, uv_timer_s, uv_timer_cb,
                     uv_timer_t, C2RustUnnamed_20, uv_idle_s, uv_idle_cb,
                     uv_idle_t, C2RustUnnamed_21, uv_process_s, uv_exit_cb,
                     uv_process_t, C2RustUnnamed_22, uv_stdio_flags,
                     UV_OVERLAPPED_PIPE, UV_WRITABLE_PIPE, UV_READABLE_PIPE,
                     UV_INHERIT_STREAM, UV_INHERIT_FD, UV_CREATE_PIPE,
                     UV_IGNORE, uv_stdio_container_s, C2RustUnnamed_23,
                     uv_stdio_container_t, uv_process_options_s,
                     uv_process_options_t};
pub use self::unix_h::{uv__io_t, uv__io_s, uv__io_cb, uv_rwlock_t, uv_mutex_t,
                       uv_buf_t, uv_file, uv_gid_t, uv_uid_t};
pub use self::event_defs_h::{argv_callback, message, Event, event_create};
pub use self::multiqueue_h::{MultiQueue, multiqueue};
pub use self::loop_h::{WatcherPtr, __kl1_WatcherPtr, kl1_WatcherPtr,
                       kmp_WatcherPtr_t, kl_WatcherPtr_t, loop_0, Loop,
                       kmp_init_WatcherPtr, kmp_destroy_WatcherPtr,
                       kmp_alloc_WatcherPtr, kmp_free_WatcherPtr,
                       kl_init_WatcherPtr, kl_destroy_WatcherPtr,
                       kl_push_WatcherPtr, kl_shift_at_WatcherPtr};
pub use self::rbuffer_h::{rbuffer, rbuffer_callback, RBuffer, rbuffer_size};
pub use self::stream_h::{stream, stream_close_cb, Stream, stream_write_cb,
                         stream_read_cb, C2RustUnnamed_24};
pub use self::process_h::{ProcessType, kProcessTypePty, kProcessTypeUv,
                          process, internal_process_cb, Process,
                          process_exit_cb, process_init, process_is_stopped};
pub use self::ioctl_types_h::winsize;
pub use self::pty_process_unix_h::{pty_process, PtyProcess, pty_process_init};
pub use self::libuv_process_h::{libuv_process, LibuvProcess,
                                libuv_process_init};
pub use self::zone_h::{msgpack_zone_finalizer, msgpack_zone_finalizer_array,
                       msgpack_zone_chunk_list, msgpack_zone,
                       msgpack_zone_malloc, MSGPACK_ZONE_ALIGN,
                       msgpack_zone_malloc_no_align,
                       msgpack_zone_push_finalizer, msgpack_zone_swap,
                       msgpack_zone_chunk, msgpack_zone_free,
                       msgpack_zone_malloc_expand,
                       msgpack_zone_push_finalizer_expand};
pub use self::object_h::{msgpack_object_type, MSGPACK_OBJECT_EXT,
                         MSGPACK_OBJECT_BIN, MSGPACK_OBJECT_MAP,
                         MSGPACK_OBJECT_ARRAY, MSGPACK_OBJECT_STR,
                         MSGPACK_OBJECT_FLOAT, MSGPACK_OBJECT_FLOAT64,
                         MSGPACK_OBJECT_FLOAT32,
                         MSGPACK_OBJECT_NEGATIVE_INTEGER,
                         MSGPACK_OBJECT_POSITIVE_INTEGER,
                         MSGPACK_OBJECT_BOOLEAN, MSGPACK_OBJECT_NIL,
                         msgpack_object, msgpack_object_union,
                         msgpack_object_ext, msgpack_object_bin,
                         msgpack_object_str, msgpack_object_map,
                         msgpack_object_kv, msgpack_object_array};
pub use self::pack_h::{msgpack_packer_write, msgpack_packer,
                       msgpack_packer_init, msgpack_packer_new,
                       msgpack_packer_free};
pub use self::pack_template_h::{C2RustUnnamed_25, C2RustUnnamed_26,
                                msgpack_pack_char, msgpack_pack_signed_char,
                                msgpack_pack_short, msgpack_pack_int,
                                msgpack_pack_long, msgpack_pack_long_long,
                                msgpack_pack_unsigned_char,
                                msgpack_pack_unsigned_short,
                                msgpack_pack_unsigned_int,
                                msgpack_pack_unsigned_long,
                                msgpack_pack_unsigned_long_long,
                                msgpack_pack_uint8, msgpack_pack_uint16,
                                msgpack_pack_uint32, msgpack_pack_uint64,
                                msgpack_pack_int8, msgpack_pack_int16,
                                msgpack_pack_int32, msgpack_pack_int64,
                                msgpack_pack_fix_uint8,
                                msgpack_pack_fix_uint16,
                                msgpack_pack_fix_uint32,
                                msgpack_pack_fix_uint64,
                                msgpack_pack_fix_int8, msgpack_pack_fix_int16,
                                msgpack_pack_fix_int32,
                                msgpack_pack_fix_int64, msgpack_pack_float,
                                msgpack_pack_double, msgpack_pack_nil,
                                msgpack_pack_true, msgpack_pack_false,
                                msgpack_pack_array, msgpack_pack_map,
                                msgpack_pack_str, msgpack_pack_str_body,
                                msgpack_pack_v4raw, msgpack_pack_v4raw_body,
                                msgpack_pack_bin, msgpack_pack_bin_body,
                                msgpack_pack_ext, msgpack_pack_ext_body};
pub use self::unpack_h::{msgpack_unpacked, msgpack_unpacker,
                         msgpack_unpacker_reserve_buffer,
                         msgpack_unpacker_buffer,
                         msgpack_unpacker_buffer_capacity,
                         msgpack_unpacker_buffer_consumed,
                         msgpack_unpacked_init, msgpack_unpacked_destroy,
                         msgpack_unpacked_release_zone,
                         msgpack_unpacker_message_size,
                         msgpack_unpacker_parsed_size,
                         msgpack_unpacker_expand_buffer};
pub use self::sbuffer_h::{msgpack_sbuffer, msgpack_sbuffer_init,
                          msgpack_sbuffer_destroy, msgpack_sbuffer_new,
                          msgpack_sbuffer_free, msgpack_sbuffer_write,
                          MSGPACK_SBUFFER_INIT_SIZE, msgpack_sbuffer_release,
                          msgpack_sbuffer_clear};
pub use self::vrefbuffer_h::{msgpack_vrefbuffer_inner_buffer,
                             msgpack_vrefbuffer, msgpack_vrefbuffer_new,
                             msgpack_vrefbuffer_free,
                             msgpack_vrefbuffer_write, msgpack_vrefbuffer_vec,
                             msgpack_vrefbuffer_veclen,
                             msgpack_vrefbuffer_chunk,
                             msgpack_vrefbuffer_append_ref,
                             msgpack_vrefbuffer_init,
                             msgpack_vrefbuffer_destroy,
                             msgpack_vrefbuffer_append_copy};
pub use self::channel_h::{Channel, CallbackReader, C2RustUnnamed_28,
                          StderrState, StdioPair, ChannelStreamType,
                          kChannelStreamInternal, kChannelStreamStderr,
                          kChannelStreamStdio, kChannelStreamSocket,
                          kChannelStreamProc, callback_reader_set,
                          find_channel, channel_instream, channel_outstream,
                          channels};
pub use self::channel_defs_h::{RpcState, C2RustUnnamed_27, ChannelCallFrame};
pub use self::fileio_h::{FileDescriptor, file_eof, file_fd};
pub use self::encode_h::{ListReaderState, encode_init_lrstate};
pub use self::stdbool_h::{true_0, false_0};
use self::string_h::{memcpy, memmove, memset, strcpy, strncpy, strcmp, strchr,
                     strstr, strlen};
use self::mathcalls_h::log10;
pub use self::assert_h::{__ASSERT_FUNCTION, __assert_fail};
use self::log_h_generated_h::logmsg;
pub use self::log_h::WARN_LOG_LEVEL;
use self::mbyte_h_generated_h::{utf_ptr2cells, mb_string2cells, utf_ptr2char,
                                utf_ptr2len, utfc_ptr2len, utf_char2len,
                                utf_char2bytes, mb_toupper, mb_tolower,
                                mb_stricmp, mb_copy_char};
pub use self::mbyte_h::{mb_strcmp_ic, mb_ptr2len};
use self::libintl_h::gettext;
use self::message_h_generated_h::{emsg, emsgf};
use self::typval_h_generated_h::{tv_dict_watcher_notify, tv_dict_find,
                                 tv_get_number_chk, tv_get_string_chk};
pub use self::strings_h::strappend;
use self::memory_h_generated_h::{xstrdup, xmallocz, xmalloc, xfree, xstrlcpy,
                                 xmemscan, xstrchrnul, xcalloc, xrealloc};
pub use self::globals_h::has_mbyte;
pub use self::kvec_h::_memcpy_free;
pub use self::uintn_identity_h::{__uint64_identity, __uint32_identity,
                                 __uint16_identity};
pub use self::byteswap_h::{__bswap_64, __bswap_32, __bswap_16};
use self::in_h::{ntohl, ntohs};
pub use self::ascii_h::{NUL, Ctrl_V, ascii_iswhite, ascii_isdigit,
                        ascii_isxdigit, ascii_isident, ascii_isbdigit,
                        ascii_isspace};
pub use self::buffer_h::{buf_inc_changedtick, buf_get_changedtick,
                         buf_set_changedtick};
pub use self::charset_h::{vim_isbreak, rem_backslash};
use self::ex_docmd_h_generated_h::find_cmdline_var;
pub use self::mark_h::{mark_local_index, lt, equalpos, mark_global_index,
                       ltoreq, clearpos};
use self::math_h::{xisinf, xisnan};
use self::option_h_generated_h::csh_like_shell;
pub use self::ops_h::{STAR_REGISTER, op_reg_index, DELETION_REGISTER,
                      PLUS_REGISTER};
use self::encode_h_generated_h::encode_tv2echo;
// This is an open source non-commercial project. Dear PVS-Studio, please check
// it. PVS-Studio Static Code Analyzer for C, C++ and C#: http://www.viva64.com
// / Copy "string" into newly allocated memory.
#[no_mangle]
#[c2rust::src_loc = "55:1"]
pub unsafe extern "C" fn vim_strsave(mut string: *const char_u)
 -> *mut char_u {
    return xstrdup(string as *mut libc::c_char) as *mut char_u;
}
// / Copy up to `len` bytes of `string` into newly allocated memory and
// / terminate with a NUL. The allocated memory always has size `len + 1`, even
// / when `string` is shorter.
#[no_mangle]
#[c2rust::src_loc = "64:1"]
pub unsafe extern "C" fn vim_strnsave(mut string: *const char_u,
                                      mut len: size_t) -> *mut char_u {
    // strncpy is intentional: some parts of Vim use `string` shorter than `len`
  // and expect the remainder to be zeroed out.
    return strncpy(xmallocz(len) as *mut libc::c_char,
                   string as *mut libc::c_char, len) as *mut char_u;
}
/*
 * Same as vim_strsave(), but any characters found in esc_chars are preceded
 * by a backslash.
 */
#[no_mangle]
#[c2rust::src_loc = "76:1"]
pub unsafe extern "C" fn vim_strsave_escaped(mut string: *const char_u,
                                             mut esc_chars: *const char_u)
 -> *mut char_u {
    return vim_strsave_escaped_ext(string, esc_chars, '\\' as i32 as char_u,
                                   false_0 != 0);
}
/*
 * Same as vim_strsave_escaped(), but when "bsl" is true also escape
 * characters where rem_backslash() would remove the backslash.
 * Escape the characters with "cc".
 */
#[no_mangle]
#[c2rust::src_loc = "87:1"]
pub unsafe extern "C" fn vim_strsave_escaped_ext(mut string: *const char_u,
                                                 mut esc_chars: *const char_u,
                                                 mut cc: char_u,
                                                 mut bsl: bool)
 -> *mut char_u {
    /*
   * First count the number of backslashes required.
   * Then allocate the memory and insert them.
   */
    let mut length = 1 as libc::c_int as size_t; // count the trailing NUL
    let mut p = string;
    while *p != 0 {
        let mut l: size_t = 0;
        if has_mbyte != 0 &&
               {
                   l =
                       Some(Some(mb_ptr2len).expect("non-null function pointer")).expect("non-null function pointer")(p)
                           as size_t;
                   (l) > 1 as libc::c_int as libc::c_ulong
               } {
            /* count an ordinary char */
            length =
                (length as libc::c_ulong).wrapping_add(l) as size_t as
                    size_t; // count a multibyte char
            p =
                p.offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                             isize)
        } else {
            if !vim_strchr(esc_chars, *p as libc::c_int).is_null() ||
                   bsl as libc::c_int != 0 &&
                       rem_backslash(p) as libc::c_int != 0 {
                length = length.wrapping_add(1)
            } /* count a backslash */
            length = length.wrapping_add(1)
        } /* skip multibyte char  */
        p = p.offset(1)
    }
    let mut escaped_string = xmalloc(length) as *mut char_u;
    let mut p2 = escaped_string;
    let mut p_0 = string;
    while *p_0 != 0 {
        let mut l_0: size_t = 0;
        if has_mbyte != 0 &&
               {
                   l_0 =
                       Some(Some(mb_ptr2len).expect("non-null function pointer")).expect("non-null function pointer")(p_0)
                           as size_t;
                   (l_0) > 1 as libc::c_int as libc::c_ulong
               } {
            memcpy(p2 as *mut libc::c_void, p_0 as *const libc::c_void, l_0);
            p2 = p2.offset(l_0 as isize);
            p_0 =
                p_0.offset(l_0.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                               as isize)
        } else {
            if !vim_strchr(esc_chars, *p_0 as libc::c_int).is_null() ||
                   bsl as libc::c_int != 0 &&
                       rem_backslash(p_0) as libc::c_int != 0 {
                let fresh3 = p2;
                p2 = p2.offset(1);
                *fresh3 = cc
            }
            let fresh4 = p2;
            p2 = p2.offset(1);
            *fresh4 = *p_0
        }
        p_0 = p_0.offset(1)
    }
    *p2 = NUL as char_u;
    return escaped_string;
}
// / Save a copy of an unquoted string
// /
// / Turns string like `a\bc"def\"ghi\\\n"jkl` into `a\bcdef"ghi\\njkl`, for use
// / in shell_build_argv: the only purpose of backslash is making next character
// / be treated literally inside the double quotes, if this character is
// / backslash or quote.
// /
// / @param[in]  string  String to copy.
// / @param[in]  length  Length of the string to copy.
// /
// / @return [allocated] Copy of the string.
#[no_mangle]
#[c2rust::src_loc = "138:1"]
pub unsafe extern "C" fn vim_strnsave_unquoted(string: *const libc::c_char,
                                               length: size_t)
 -> *mut libc::c_char {
    let mut ret_length = 0 as libc::c_int as size_t;
    let mut inquote = false_0 != 0;
    let string_end = string.offset(length as isize);
    let mut p = string;
    while p < string_end {
        if *p as libc::c_int == '\"' as i32 {
            inquote = !inquote
        } else if *p as libc::c_int == '\\' as i32 &&
                      inquote as libc::c_int != 0 &&
                      p.offset(1 as libc::c_int as isize) < string_end &&
                      (*p.offset(1 as libc::c_int as isize) as libc::c_int ==
                           '\\' as i32 ||
                           *p.offset(1 as libc::c_int as isize) as libc::c_int
                               == '\"' as i32) {
            ret_length = ret_length.wrapping_add(1);
            p = p.offset(1)
        } else { ret_length = ret_length.wrapping_add(1) }
        p = p.offset(1)
    }
    let ret = xmallocz(ret_length) as *mut libc::c_char;
    let mut rp = ret;
    inquote = false_0 != 0;
    let mut p_0 = string;
    while p_0 < string_end {
        if *p_0 as libc::c_int == '\"' as i32 {
            inquote = !inquote
        } else if *p_0 as libc::c_int == '\\' as i32 &&
                      inquote as libc::c_int != 0 &&
                      p_0.offset(1 as libc::c_int as isize) < string_end &&
                      (*p_0.offset(1 as libc::c_int as isize) as libc::c_int
                           == '\\' as i32 ||
                           *p_0.offset(1 as libc::c_int as isize) as
                               libc::c_int == '\"' as i32) {
            p_0 = p_0.offset(1);
            let fresh5 = rp;
            rp = rp.offset(1);
            *fresh5 = *p_0
        } else { let fresh6 = rp; rp = rp.offset(1); *fresh6 = *p_0 }
        p_0 = p_0.offset(1)
    }
    return ret;
}
/*
 * Escape "string" for use as a shell argument with system().
 * This uses single quotes, except when we know we need to use double quotes
 * (MS-Windows without 'shellslash' set).
 * Escape a newline, depending on the 'shell' option.
 * When "do_special" is true also replace "!", "%", "#" and things starting
 * with "<" like "<cfile>".
 * When "do_newline" is false do not escape newline unless it is csh shell.
 * Returns the result in allocated memory.
 */
#[no_mangle]
#[c2rust::src_loc = "185:1"]
pub unsafe extern "C" fn vim_strsave_shellescape(mut string: *const char_u,
                                                 mut do_special: bool,
                                                 mut do_newline: bool)
 -> *mut char_u {
    let mut d = 0 as *mut char_u;
    let mut escaped_string = 0 as *mut char_u;
    let mut l: size_t = 0;
    let mut csh_like: libc::c_int = 0;
    /* Only csh and similar shells expand '!' within single quotes.  For sh and
   * the like we must not put a backslash before it, it will be taken
   * literally.  If do_special is set the '!' will be escaped twice.
   * Csh also needs to have "\n" escaped twice when do_special is set. */
    csh_like = csh_like_shell();
    /* First count the number of extra bytes required. */
    let mut length =
        strlen(string as
                   *mut libc::c_char).wrapping_add(3 as libc::c_int as
                                                       libc::c_ulong); // two quotes and a trailing NUL
    let mut p = string;
    while *p as libc::c_int != NUL {
        if *p as libc::c_int == '\'' as i32 {
            length =
                (length as
                     libc::c_ulong).wrapping_add(3 as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t
            // ' => '\''
        } /* insert backslash */
        if *p as libc::c_int == '\n' as i32 &&
               (csh_like != 0 || do_newline as libc::c_int != 0) ||
               *p as libc::c_int == '!' as i32 &&
                   (csh_like != 0 || do_special as libc::c_int != 0) {
            length = length.wrapping_add(1);
            if csh_like != 0 && do_special as libc::c_int != 0 {
                length = length.wrapping_add(1)
            }
            /* insert backslash */
        } /* insert backslash */
        if do_special as libc::c_int != 0 &&
               find_cmdline_var(p, &mut l) >= 0 as libc::c_int as libc::c_long
           {
            length = length.wrapping_add(1);
            p =
                p.offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                             isize)
        }
        p = p.offset(utfc_ptr2len(p as *mut char_u) as isize)
    }
    /* Allocate memory for the result and fill it. */
    escaped_string = xmalloc(length) as *mut char_u;
    d = escaped_string;
    // add opening quote
    let fresh7 = d; /* insert backslash */
    d = d.offset(1);
    *fresh7 = '\'' as i32 as char_u;
    let mut p_0 = string;
    while *p_0 as libc::c_int != NUL {
        if *p_0 as libc::c_int == '\'' as i32 {
            let fresh8 = d;
            d = d.offset(1);
            *fresh8 = '\'' as i32 as char_u;
            let fresh9 = d;
            d = d.offset(1);
            *fresh9 = '\\' as i32 as char_u;
            let fresh10 = d;
            d = d.offset(1);
            *fresh10 = '\'' as i32 as char_u;
            let fresh11 = d;
            d = d.offset(1);
            *fresh11 = '\'' as i32 as char_u;
            p_0 = p_0.offset(1)
        } else if *p_0 as libc::c_int == '\n' as i32 &&
                      (csh_like != 0 || do_newline as libc::c_int != 0) ||
                      *p_0 as libc::c_int == '!' as i32 &&
                          (csh_like != 0 || do_special as libc::c_int != 0) {
            let fresh12 = d;
            d = d.offset(1);
            *fresh12 = '\\' as i32 as char_u;
            if csh_like != 0 && do_special as libc::c_int != 0 {
                let fresh13 = d;
                d = d.offset(1);
                *fresh13 = '\\' as i32 as char_u
            }
            let fresh14 = p_0;
            p_0 = p_0.offset(1);
            let fresh15 = d;
            d = d.offset(1);
            *fresh15 = *fresh14
        } else if do_special as libc::c_int != 0 &&
                      find_cmdline_var(p_0, &mut l) >=
                          0 as libc::c_int as libc::c_long {
            let fresh16 = d;
            d = d.offset(1);
            *fresh16 = '\\' as i32 as char_u;
            loop  {
                l = l.wrapping_sub(1);
                if !(l != SIZE_MAX) { break ; }
                /* copy the var */
                let fresh17 = p_0;
                p_0 = p_0.offset(1);
                let fresh18 = d;
                d = d.offset(1);
                *fresh18 = *fresh17
            }
        } else { mb_copy_char(&mut p_0 as *mut *const char_u, &mut d); }
    }
    // add terminating quote and finish with a NUL
    let fresh19 = d;
    d = d.offset(1);
    *fresh19 = '\'' as i32 as char_u;
    *d = NUL as char_u;
    return escaped_string;
}
/*
 * Like vim_strsave(), but make all characters uppercase.
 * This uses ASCII lower-to-upper case translation, language independent.
 */
#[no_mangle]
#[c2rust::src_loc = "290:1"]
pub unsafe extern "C" fn vim_strsave_up(mut string: *const char_u)
 -> *mut char_u {
    let mut p1 = 0 as *mut char_u;
    p1 = vim_strsave(string);
    vim_strup(p1);
    return p1;
}
/*
 * Like vim_strnsave(), but make all characters uppercase.
 * This uses ASCII lower-to-upper case translation, language independent.
 */
#[no_mangle]
#[c2rust::src_loc = "304:1"]
pub unsafe extern "C" fn vim_strnsave_up(mut string: *const char_u,
                                         mut len: size_t) -> *mut char_u {
    let mut p1 = vim_strnsave(string, len);
    vim_strup(p1);
    return p1;
}
/*
 * ASCII lower-to-upper case translation, language independent.
 */
#[no_mangle]
#[c2rust::src_loc = "315:1"]
pub unsafe extern "C" fn vim_strup(mut p: *mut char_u) {
    let mut c: char_u = 0;
    loop  {
        c = *p;
        if !(c as libc::c_int != NUL) { break ; }
        let fresh20 = p;
        p = p.offset(1);
        *fresh20 =
            if (c as libc::c_int) < 'a' as i32 ||
                   c as libc::c_int > 'z' as i32 {
                c as libc::c_int
            } else { (c as libc::c_int) - 0x20 as libc::c_int } as char_u
    };
}
// / Make given string all upper-case or all lower-case
// /
// / Handles multi-byte characters as good as possible.
// /
// / @param[in]  orig  Input string.
// / @param[in]  upper If true make uppercase, otherwise lowercase
// /
// / @return [allocated] upper-cased string.
#[no_mangle]
#[c2rust::src_loc = "332:1"]
pub unsafe extern "C" fn strcase_save(orig: *const libc::c_char,
                                      mut upper: bool) -> *mut libc::c_char {
    let mut res = xstrdup(orig);
    let mut p = res;
    while *p as libc::c_int != NUL {
        let mut c = utf_ptr2char(p as *const char_u);
        let mut l = utf_ptr2len(p as *const char_u);
        if c == 0 as libc::c_int {
            // overlong sequence, use only the first byte
            c = *p as libc::c_int;
            l = 1 as libc::c_int
        }
        let mut uc =
            if upper as libc::c_int != 0 {
                mb_toupper(c)
            } else { mb_tolower(c) };
        // Reallocate string when byte count changes.  This is rare,
    // thus it's OK to do another malloc()/free().
        let mut newl = utf_char2len(uc);
        if newl != l {
            // TODO(philix): use xrealloc() in strup_save()
            let mut s =
                xmalloc(strlen(res).wrapping_add((1 as libc::c_int + newl - l)
                                                     as size_t)) as
                    *mut libc::c_char;
            memcpy(s as *mut libc::c_void, res as *const libc::c_void,
                   p.wrapping_offset_from(res) as libc::c_long as size_t);
            strcpy(s.offset(p.wrapping_offset_from(res) as libc::c_long as
                                isize).offset(newl as isize),
                   p.offset(l as isize));
            p =
                s.offset(p.wrapping_offset_from(res) as libc::c_long as
                             isize);
            xfree(res as *mut libc::c_void);
            res = s
        }
        utf_char2bytes(uc, p as *mut char_u);
        p = p.offset(newl as isize)
    }
    return res;
}
/*
 * delete spaces at the end of a string
 */
#[no_mangle]
#[c2rust::src_loc = "371:1"]
pub unsafe extern "C" fn del_trailing_spaces(mut ptr: *mut char_u) {
    let mut q = 0 as *mut char_u;
    q = ptr.offset(strlen(ptr as *mut libc::c_char) as isize);
    loop  {
        q = q.offset(-1);
        if !(q > ptr &&
                 ascii_iswhite(*q.offset(0 as libc::c_int as isize) as
                                   libc::c_int) as libc::c_int != 0 &&
                 *q.offset(-(1 as libc::c_int) as isize) as libc::c_int !=
                     '\\' as i32 &&
                 *q.offset(-(1 as libc::c_int) as isize) as libc::c_int !=
                     Ctrl_V) {
            break ;
        }
        *q = NUL as char_u
    };
}
// / strchr() version which handles multibyte strings
// /
// / @param[in]  string  String to search in.
// / @param[in]  c  Character to search for.
// /
// / @return Pointer to the first byte of the found character in string or NULL
// /         if it was not found or character is invalid. NUL character is never
// /         found, use `strlen()` instead.
#[no_mangle]
#[c2rust::src_loc = "438:1"]
pub unsafe extern "C" fn vim_strchr(string: *const char_u, c: libc::c_int)
 -> *mut char_u {
    if c <= 0 as libc::c_int {
        return NULL_1 as *mut char_u
    } else if c < 0x80 as libc::c_int {
        return strchr(string as *const libc::c_char, c) as *mut char_u
    } else {
        let mut u8char: [libc::c_char; 22] = [0; 22];
        let len = utf_char2bytes(c, u8char.as_mut_ptr() as *mut char_u);
        u8char[len as usize] = NUL as libc::c_char;
        return strstr(string as *const libc::c_char, u8char.as_mut_ptr()) as
                   *mut char_u
    };
}
/*
 * Sort an array of strings.
 */
#[c2rust::src_loc = "460:1"]
unsafe extern "C" fn sort_compare(mut s1: *const libc::c_void,
                                  mut s2: *const libc::c_void)
 -> libc::c_int {
    return strcmp(*(s1 as *mut *mut libc::c_char),
                  *(s2 as *mut *mut libc::c_char));
}
#[no_mangle]
#[c2rust::src_loc = "466:1"]
pub unsafe extern "C" fn sort_strings(mut files: *mut *mut char_u,
                                      mut count: libc::c_int) {
    qsort(files as *mut libc::c_void, count as size_t,
          ::std::mem::size_of::<*mut char_u>() as libc::c_ulong,
          Some(sort_compare as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
}
/*
 * Return true if string "s" contains a non-ASCII character (128 or higher).
 * When "s" is NULL false is returned.
 */
#[no_mangle]
#[c2rust::src_loc = "475:1"]
pub unsafe extern "C" fn has_non_ascii(mut s: *const char_u) -> bool {
    let mut p = 0 as *const char_u;
    if !s.is_null() {
        p = s;
        while *p as libc::c_int != NUL {
            if *p as libc::c_int >= 128 as libc::c_int { return true_0 != 0 }
            p = p.offset(1)
        }
    }
    return false_0 != 0;
}
// / Return true if string "s" contains a non-ASCII character (128 or higher).
// / When "s" is NULL false is returned.
#[no_mangle]
#[c2rust::src_loc = "489:1"]
pub unsafe extern "C" fn has_non_ascii_len(s: *const libc::c_char,
                                           len: size_t) -> bool {
    if !s.is_null() {
        let mut i = 0 as libc::c_int as size_t;
        while i < len {
            if *s.offset(i as isize) as uint8_t as libc::c_int >=
                   128 as libc::c_int {
                return true_0 != 0
            }
            i = i.wrapping_add(1)
        }
    }
    return false_0 != 0;
}
/*
 * Concatenate two strings and return the result in allocated memory.
 */
#[no_mangle]
#[c2rust::src_loc = "505:1"]
pub unsafe extern "C" fn concat_str(mut str1: *const char_u,
                                    mut str2: *const char_u) -> *mut char_u {
    let mut l = strlen(str1 as *mut libc::c_char);
    let mut dest =
        xmalloc(l.wrapping_add(strlen(str2 as
                                          *mut libc::c_char)).wrapping_add(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong))
            as *mut char_u;
    strcpy(dest as *mut libc::c_char, str1 as *mut libc::c_char);
    strcpy(dest.offset(l as isize) as *mut libc::c_char,
           str2 as *mut libc::c_char);
    return dest;
}
#[c2rust::src_loc = "516:26"]
static mut e_printf: *const libc::c_char =
    b"E766: Insufficient arguments for printf()\x00" as *const u8 as
        *const libc::c_char;
// / Get number argument from idxp entry in tvs
// /
// / Will give an error message for VimL entry with invalid type or for
// / insufficient entries.
// /
// / @param[in]  tvs  List of VimL values. List is terminated by VAR_UNKNOWN
// /                  value.
// / @param[in,out]  idxp  Index in a list. Will be incremented. Indexing starts
// /                       at 1.
// /
// / @return Number value or 0 in case of error.
#[c2rust::src_loc = "530:1"]
unsafe extern "C" fn tv_nr(mut tvs: *mut typval_T, mut idxp: *mut libc::c_int)
 -> varnumber_T {
    let mut idx = *idxp - 1 as libc::c_int;
    let mut n = 0 as libc::c_int as varnumber_T;
    if (*tvs.offset(idx as isize)).v_type as libc::c_uint ==
           VAR_UNKNOWN as libc::c_int as libc::c_uint {
        emsg(gettext(e_printf as *mut libc::c_char) as *mut char_u);
    } else {
        *idxp += 1;
        let mut err = false_0 != 0;
        n = tv_get_number_chk(&mut *tvs.offset(idx as isize), &mut err);
        if err { n = 0 as libc::c_int as varnumber_T }
    }
    return n;
}
// / Get string argument from idxp entry in tvs
// /
// / Will give an error message for VimL entry with invalid type or for
// / insufficient entries.
// /
// / @param[in]  tvs  List of VimL values. List is terminated by VAR_UNKNOWN
// /                  value.
// / @param[in,out]  idxp  Index in a list. Will be incremented.
// / @param[out]  tofree  If the idxp entry in tvs is not a String or a Number,
// /                      it will be converted to String in the same format
// /                      as ":echo" and stored in "*tofree". The caller must
// /                      free "*tofree".
// /
// / @return String value or NULL in case of error.
#[c2rust::src_loc = "563:1"]
unsafe extern "C" fn tv_str(mut tvs: *mut typval_T,
                            mut idxp: *mut libc::c_int,
                            tofree: *mut *mut libc::c_char)
 -> *const libc::c_char {
    let mut idx = *idxp - 1 as libc::c_int;
    let mut s = NULL_1 as *const libc::c_char;
    if (*tvs.offset(idx as isize)).v_type as libc::c_uint ==
           VAR_UNKNOWN as libc::c_int as libc::c_uint {
        emsg(gettext(e_printf as *mut libc::c_char) as *mut char_u);
    } else {
        *idxp += 1;
        if (*tvs.offset(idx as isize)).v_type as libc::c_uint ==
               VAR_STRING as libc::c_int as libc::c_uint ||
               (*tvs.offset(idx as isize)).v_type as libc::c_uint ==
                   VAR_NUMBER as libc::c_int as libc::c_uint {
            s = tv_get_string_chk(&mut *tvs.offset(idx as isize));
            *tofree = NULL_1 as *mut libc::c_char
        } else {
            *tofree =
                encode_tv2echo(&mut *tvs.offset(idx as isize),
                               NULL_1 as *mut size_t);
            s = *tofree
        }
    }
    return s;
}
// / Get pointer argument from the next entry in tvs
// /
// / Will give an error message for VimL entry with invalid type or for
// / insufficient entries.
// /
// / @param[in]  tvs  List of typval_T values.
// / @param[in,out]  idxp  Pointer to the index of the current value.
// /
// / @return Pointer stored in typval_T or NULL.
#[c2rust::src_loc = "592:1"]
unsafe extern "C" fn tv_ptr(tvs: *const typval_T, idxp: *mut libc::c_int)
 -> *const libc::c_void {
    extern "C" {
        #[link_name = "__Static_assert_function"]
        pub fn __Static_assert_function_0() -> *mut [libc::c_int; 1];
    }
    let idx = *idxp - 1 as libc::c_int;
    if (*tvs.offset(idx as isize)).v_type as libc::c_uint ==
           VAR_UNKNOWN as libc::c_int as libc::c_uint {
        emsg(gettext(e_printf as *mut libc::c_char) as *mut char_u);
        return NULL_1 as *const libc::c_void
    } else {
        *idxp += 1;
        return (*tvs.offset(idx as isize)).vval.v_string as
                   *const libc::c_void
    };
}
// / Get float argument from idxp entry in tvs
// /
// / Will give an error message for VimL entry with invalid type or for
// / insufficient entries.
// /
// / @param[in]  tvs  List of VimL values. List is terminated by VAR_UNKNOWN
// /                  value.
// / @param[in,out]  idxp  Index in a list. Will be incremented.
// /
// / @return Floating-point value or zero in case of error.
#[c2rust::src_loc = "626:1"]
unsafe extern "C" fn tv_float(tvs: *mut typval_T, idxp: *mut libc::c_int)
 -> float_T {
    let mut idx = *idxp - 1 as libc::c_int;
    let mut f = 0 as libc::c_int as float_T;
    if (*tvs.offset(idx as isize)).v_type as libc::c_uint ==
           VAR_UNKNOWN as libc::c_int as libc::c_uint {
        emsg(gettext(e_printf as *mut libc::c_char) as *mut char_u);
    } else {
        *idxp += 1;
        if (*tvs.offset(idx as isize)).v_type as libc::c_uint ==
               VAR_FLOAT as libc::c_int as libc::c_uint {
            f = (*tvs.offset(idx as isize)).vval.v_float
        } else if (*tvs.offset(idx as isize)).v_type as libc::c_uint ==
                      VAR_NUMBER as libc::c_int as libc::c_uint {
            f = (*tvs.offset(idx as isize)).vval.v_number as float_T
        } else {
            emsg(gettext(b"E807: Expected Float argument for printf()\x00" as
                             *const u8 as *const libc::c_char as
                             *mut libc::c_char) as *mut char_u);
        }
    }
    return f;
}
// This code was included to provide a portable vsnprintf() and snprintf().
// Some systems may provide their own, but we always use this one for
// consistency.
//
// This code is based on snprintf.c - a portable implementation of snprintf
// by Mark Martinec <mark.martinec@ijs.si>, Version 2.2, 2000-10-06.
// Included with permission.  It was heavily modified to fit in Vim.
// The original code, including useful comments, can be found here:
//
//     http://www.ijs.si/software/snprintf/
//
// This snprintf() only supports the following conversion specifiers:
// s, c, b, B, d, u, o, x, X, p  (and synonyms: i, D, U, O - see below)
// with flags: '-', '+', ' ', '0' and '#'.
// An asterisk is supported for field width as well as precision.
//
// Limited support for floating point was added: 'f', 'e', 'E', 'g', 'G'.
//
// Length modifiers 'h' (short int), 'l' (long int) and "ll" (long long int) are
// supported.
//
// The locale is not used, the string is used as a byte string.  This is only
// relevant for double-byte encodings where the second byte may be '%'.
//
// It is permitted for "str_m" to be zero, and it is permitted to specify NULL
// pointer for resulting string argument if "str_m" is zero (as per ISO C99).
//
// The return value is the number of characters which would be generated
// for the given input, excluding the trailing NUL. If this value
// is greater or equal to "str_m", not all characters from the result
// have been stored in str, output bytes beyond the ("str_m"-1) -th character
// are discarded. If "str_m" is greater than zero it is guaranteed
// the resulting string will be NUL-terminated.
// vim_vsnprintf_typval() can be invoked with either "va_list" or a list of
// "typval_T".  When the latter is not used it must be NULL.
// / Append a formatted value to the string
// /
// / @see vim_vsnprintf_typval().
#[no_mangle]
#[c2rust::src_loc = "687:1"]
pub unsafe extern "C" fn vim_snprintf_add(mut str: *mut libc::c_char,
                                          mut str_m: size_t,
                                          mut fmt: *mut libc::c_char,
                                          mut args: ...) -> libc::c_int {
    let len = strlen(str);
    let mut space: size_t = 0;
    if str_m <= len {
        space = 0 as libc::c_int as size_t
    } else { space = str_m.wrapping_sub(len) }
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    let str_l =
        vim_vsnprintf(str.offset(len as isize), space, fmt, ap.as_va_list());
    return str_l;
}
// / Write formatted value to the string
// /
// / @param[out]  str  String to write to.
// / @param[in]  str_m  String length.
// / @param[in]  fmt  String format.
// /
// / @return Number of bytes excluding NUL byte that would be written to the
// /         string if str_m was greater or equal to the return value.
#[no_mangle]
#[c2rust::src_loc = "713:1"]
pub unsafe extern "C" fn vim_snprintf(mut str: *mut libc::c_char,
                                      mut str_m: size_t,
                                      mut fmt: *const libc::c_char,
                                      mut args: ...) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    let str_l = vim_vsnprintf(str, str_m, fmt, ap.as_va_list());
    return str_l;
}
// Return the representation of infinity for printf() function:
// "-inf", "inf", "+inf", " inf", "-INF", "INF", "+INF" or " INF".
#[c2rust::src_loc = "725:1"]
unsafe extern "C" fn infinity_str(mut positive: bool,
                                  mut fmt_spec: libc::c_char,
                                  mut force_sign: libc::c_int,
                                  mut space_for_positive: libc::c_int)
 -> *const libc::c_char {
    static mut table: [*const libc::c_char; 8] =
        [b"-inf\x00" as *const u8 as *const libc::c_char,
         b"inf\x00" as *const u8 as *const libc::c_char,
         b"+inf\x00" as *const u8 as *const libc::c_char,
         b" inf\x00" as *const u8 as *const libc::c_char,
         b"-INF\x00" as *const u8 as *const libc::c_char,
         b"INF\x00" as *const u8 as *const libc::c_char,
         b"+INF\x00" as *const u8 as *const libc::c_char,
         b" INF\x00" as *const u8 as *const libc::c_char];
    let mut idx =
        positive as libc::c_int *
            (1 as libc::c_int + force_sign + force_sign * space_for_positive);
    if fmt_spec as libc::c_uint >= 'A' as i32 as libc::c_uint &&
           fmt_spec as libc::c_uint <= 'Z' as i32 as libc::c_uint {
        idx += 4 as libc::c_int
    }
    return table[idx as usize];
}
#[no_mangle]
#[c2rust::src_loc = "739:1"]
pub unsafe extern "C" fn vim_vsnprintf(mut str: *mut libc::c_char,
                                       mut str_m: size_t,
                                       mut fmt: *const libc::c_char,
                                       mut ap: ::std::ffi::VaList)
 -> libc::c_int {
    return vim_vsnprintf_typval(str, str_m, fmt, ap.as_va_list(),
                                NULL_1 as *mut typval_T);
}
// / Write formatted value to the string
// /
// / @param[out]  str  String to write to.
// / @param[in]  str_m  String length.
// / @param[in]  fmt  String format.
// / @param[in]  ap  Values that should be formatted. Ignored if tvs is not NULL.
// / @param[in]  tvs  Values that should be formatted, for printf() VimL
// /                  function. Must be NULL in other cases.
// /
// / @return Number of bytes excluding NUL byte that would be written to the
// /         string if str_m was greater or equal to the return value.
#[no_mangle]
#[c2rust::src_loc = "755:1"]
pub unsafe extern "C" fn vim_vsnprintf_typval(mut str: *mut libc::c_char,
                                              mut str_m: size_t,
                                              mut fmt: *const libc::c_char,
                                              mut ap: ::std::ffi::VaList,
                                              tvs: *mut typval_T)
 -> libc::c_int {
    let mut str_l = 0 as libc::c_int as size_t;
    let mut str_avail = str_l < str_m;
    let mut p = fmt;
    let mut arg_idx = 1 as libc::c_int;
    if p.is_null() { p = b"\x00" as *const u8 as *const libc::c_char }
    while *p != 0 {
        if *p as libc::c_int != '%' as i32 {
            // copy up to the next '%' or NUL without any changes
            let mut n =
                xstrchrnul(p.offset(1 as libc::c_int as isize),
                           '%' as i32 as libc::c_char).wrapping_offset_from(p)
                    as libc::c_long as size_t;
            if str_avail {
                let mut avail = str_m.wrapping_sub(str_l);
                memmove(str.offset(str_l as isize) as *mut libc::c_void,
                        p as *const libc::c_void,
                        if n < avail { n } else { avail });
                str_avail = n < avail
            }
            p = p.offset(n as isize);
            if n <=
                   (18446744073709551615 as libc::c_ulong).wrapping_sub(str_l)
               {
            } else {
                __assert_fail(b"n <= SIZE_MAX - str_l\x00" as *const u8 as
                                  *const libc::c_char,
                              b"/home/vole/neovim/src/nvim/strings.c\x00" as
                                  *const u8 as *const libc::c_char,
                              776 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 96],
                                                        &[libc::c_char; 96]>(b"int vim_vsnprintf_typval(char *, size_t, const char *, struct __va_list_tag *, typval_T *const)\x00")).as_ptr());
            }
            str_l =
                (str_l as libc::c_ulong).wrapping_add(n) as size_t as size_t
        } else {
            let mut min_field_width = 0 as libc::c_int as size_t;
            let mut precision = 0 as libc::c_int as size_t;
            let mut zero_padding = 0 as libc::c_int;
            let mut precision_specified = 0 as libc::c_int;
            let mut justify_left = 0 as libc::c_int;
            let mut alternate_form = 0 as libc::c_int;
            let mut force_sign = 0 as libc::c_int;
            // if both ' ' and '+' flags appear, ' ' flag should be ignored
            let mut space_for_positive = 1 as libc::c_int;
            // allowed values: \0, h, l, 2 (for ll), z, L
            let mut length_modifier = '\u{0}' as i32 as libc::c_char;
            // temporary buffer for simple numeric->string conversion
            // 1e308 seems reasonable as the maximum printable
            let mut tmp: [libc::c_char; 350] = [0; 350];
            // string address in case of string argument
            let mut str_arg = NULL_1 as *const libc::c_char;
            // natural field width of arg without padding and sign
            let mut str_arg_l: size_t = 0;
            // unsigned char argument value (only defined for c conversion);
      // standard explicitly states the char argument for the c
      // conversion is unsigned
            let mut uchar_arg: libc::c_uchar = 0;
            // number of zeros to be inserted for numeric conversions as
      // required by the precision or minimal field width
            let mut number_of_zeros_to_pad = 0 as libc::c_int as size_t;
            // index into tmp where zero padding is to be inserted
            let mut zero_padding_insertion_ind = 0 as libc::c_int as size_t;
            // current conversion specifier character
            let mut fmt_spec = '\u{0}' as i32 as libc::c_char;
            // buffer for 's' and 'S' specs
            let mut tofree = NULL_1 as *mut libc::c_char; // skip '%'
            p = p.offset(1);
            loop 
                 // parse flags
                 {
                match *p as libc::c_int {
                    48 => { zero_padding = 1 as libc::c_int; p = p.offset(1) }
                    45 => { justify_left = 1 as libc::c_int; p = p.offset(1) }
                    43 => {
                        // if both '0' and '-' flags appear, '0' should be ignored
                        force_sign = 1 as libc::c_int;
                        space_for_positive = 0 as libc::c_int;
                        p = p.offset(1)
                    }
                    32 => { force_sign = 1 as libc::c_int; p = p.offset(1) }
                    35 => {
                        // if both ' ' and '+' flags appear, ' ' should be ignored
                        alternate_form = 1 as libc::c_int;
                        p = p.offset(1)
                    }
                    39 => { p = p.offset(1) }
                    _ => { break ; }
                }
            }
            // parse field width
            if *p as libc::c_int == '*' as i32 {
                p = p.offset(1);
                let j =
                    if !tvs.is_null() {
                        tv_nr(tvs, &mut arg_idx) as libc::c_int
                    } else { ap.arg::<libc::c_int>() };
                if j >= 0 as libc::c_int {
                    min_field_width = j as size_t
                } else {
                    min_field_width = -j as size_t;
                    justify_left = 1 as libc::c_int
                }
            } else if ascii_isdigit(*p as libc::c_int) {
                // size_t could be wider than unsigned int; make sure we treat
        // argument like common implementations do
                let fresh21 = p;
                p = p.offset(1);
                let mut uj =
                    (*fresh21 as libc::c_int - '0' as i32) as libc::c_uint;
                while ascii_isdigit(*p as libc::c_int) {
                    let fresh22 = p;
                    p = p.offset(1);
                    uj =
                        (10 as libc::c_int as
                             libc::c_uint).wrapping_mul(uj).wrapping_add((*fresh22
                                                                              as
                                                                              libc::c_int
                                                                              -
                                                                              '0'
                                                                                  as
                                                                                  i32)
                                                                             as
                                                                             libc::c_uint)
                }
                min_field_width = uj as size_t
            }
            // parse precision
            if *p as libc::c_int == '.' as i32 {
                p = p.offset(1);
                precision_specified = 1 as libc::c_int;
                if *p as libc::c_int == '*' as i32 {
                    let j_0 =
                        if !tvs.is_null() {
                            tv_nr(tvs, &mut arg_idx) as libc::c_int
                        } else { ap.arg::<libc::c_int>() };
                    p = p.offset(1);
                    if j_0 >= 0 as libc::c_int {
                        precision = j_0 as size_t
                    } else {
                        precision_specified = 0 as libc::c_int;
                        precision = 0 as libc::c_int as size_t
                    }
                } else if ascii_isdigit(*p as libc::c_int) {
                    // size_t could be wider than unsigned int; make sure we
          // treat argument like common implementations do
                    let fresh23 = p;
                    p = p.offset(1);
                    let mut uj_0 =
                        (*fresh23 as libc::c_int - '0' as i32) as
                            libc::c_uint;
                    while ascii_isdigit(*p as libc::c_int) {
                        let fresh24 = p;
                        p = p.offset(1);
                        uj_0 =
                            (10 as libc::c_int as
                                 libc::c_uint).wrapping_mul(uj_0).wrapping_add((*fresh24
                                                                                    as
                                                                                    libc::c_int
                                                                                    -
                                                                                    '0'
                                                                                        as
                                                                                        i32)
                                                                                   as
                                                                                   libc::c_uint)
                    }
                    precision = uj_0 as size_t
                }
            }
            // parse 'h', 'l', 'll' and 'z' length modifiers
            if *p as libc::c_int == 'h' as i32 ||
                   *p as libc::c_int == 'l' as i32 ||
                   *p as libc::c_int == 'z' as i32 {
                length_modifier = *p;
                p = p.offset(1);
                if length_modifier as libc::c_int == 'l' as i32 &&
                       *p as libc::c_int == 'l' as i32 {
                    // ll, encoded as 2
                    length_modifier = '2' as i32 as libc::c_char;
                    p = p.offset(1)
                }
            }
            fmt_spec = *p;
            // common synonyms
            match fmt_spec as libc::c_int {
                105 => { fmt_spec = 'd' as i32 as libc::c_char }
                68 => {
                    fmt_spec = 'd' as i32 as libc::c_char;
                    length_modifier = 'l' as i32 as libc::c_char
                }
                85 => {
                    fmt_spec = 'u' as i32 as libc::c_char;
                    length_modifier = 'l' as i32 as libc::c_char
                }
                79 => {
                    fmt_spec = 'o' as i32 as libc::c_char;
                    length_modifier = 'l' as i32 as libc::c_char
                }
                _ => { }
            }
            match fmt_spec as libc::c_int {
                100 | 117 | 111 | 120 | 88 => {
                    if !tvs.is_null() &&
                           length_modifier as libc::c_int == '\u{0}' as i32 {
                        length_modifier = '2' as i32 as libc::c_char
                    }
                }
                _ => { }
            }
            // get parameter value, do initial processing
            match fmt_spec as libc::c_int {
                37 | 99 | 115 | 83 => {
                    // '%' and 'c' behave similar to 's' regarding flags and field widths
                    str_arg_l = 1 as libc::c_int as size_t;
                    match fmt_spec as libc::c_int {
                        37 => { str_arg = p }
                        99 => {
                            let j_1 =
                                if !tvs.is_null() {
                                    tv_nr(tvs, &mut arg_idx) as libc::c_int
                                } else { ap.arg::<libc::c_int>() };
                            // standard demands unsigned char
                            uchar_arg = j_1 as libc::c_uchar;
                            str_arg =
                                &mut uchar_arg as *mut libc::c_uchar as
                                    *mut libc::c_char
                        }
                        115 | 83 => {
                            str_arg =
                                if !tvs.is_null() {
                                    tv_str(tvs, &mut arg_idx, &mut tofree)
                                } else { ap.arg::<*const libc::c_char>() };
                            if str_arg.is_null() {
                                str_arg =
                                    b"[NULL]\x00" as *const u8 as
                                        *const libc::c_char;
                                str_arg_l = 6 as libc::c_int as size_t
                            } else if precision_specified == 0 {
                                // make sure not to address string beyond the specified
                // precision
                                str_arg_l = strlen(str_arg)
                            } else if precision ==
                                          0 as libc::c_int as libc::c_ulong {
                                // truncate string if necessary as requested by precision
                                str_arg_l = 0 as libc::c_int as size_t
                            } else {
                                // memchr on HP does not like n > 2^31
                // TODO(elmart): check if this still holds / is relevant
                                str_arg_l =
                                    (xmemscan(str_arg as *const libc::c_void,
                                              NUL as libc::c_char,
                                              (if precision <
                                                      0x7fffffff as
                                                          libc::c_int as
                                                          libc::c_ulong {
                                                   precision
                                               } else {
                                                   0x7fffffff as libc::c_int
                                                       as libc::c_ulong
                                               })) as
                                         *mut libc::c_char).wrapping_offset_from(str_arg)
                                        as libc::c_long as size_t
                            }
                            if fmt_spec as libc::c_int == 'S' as i32 {
                                if min_field_width !=
                                       0 as libc::c_int as libc::c_ulong {
                                    min_field_width =
                                        (min_field_width as
                                             libc::c_ulong).wrapping_add(strlen(str_arg).wrapping_sub(mb_string2cells(str_arg
                                                                                                                          as
                                                                                                                          *mut char_u)))
                                            as size_t as size_t
                                }
                                if precision != 0 {
                                    let mut p1 = 0 as *mut char_u;
                                    let mut i = 0 as libc::c_int as size_t;
                                    p1 = str_arg as *mut char_u;
                                    while *p1 != 0 {
                                        i =
                                            (i as
                                                 libc::c_ulong).wrapping_add(utf_ptr2cells(p1)
                                                                                 as
                                                                                 size_t)
                                                as size_t as size_t;
                                        if i > precision { break ; }
                                        p1 =
                                            p1.offset(utfc_ptr2len(p1) as
                                                          isize)
                                    }
                                    precision =
                                        p1.wrapping_offset_from(str_arg as
                                                                    *mut char_u)
                                            as libc::c_long as size_t;
                                    str_arg_l = precision
                                }
                            }
                        }
                        _ => { }
                    }
                }
                100 | 117 | 98 | 66 | 111 | 120 | 88 | 112 => {
                    // u, b, B, o, x, X and p conversion specifiers imply
          // the value is unsigned; d implies a signed value
                    // 0 if numeric argument is zero (or if pointer is NULL for 'p'),
          // +1 if greater than zero (or non NULL for 'p'),
          // -1 if negative (unsigned argument is never negative)
                    let mut arg_sign = 0 as libc::c_int;
                    let mut arg = 0 as libc::c_int as intmax_t;
                    let mut uarg = 0 as libc::c_int as uintmax_t;
                    // only defined for p conversion
                    let mut ptr_arg = NULL_1 as *const libc::c_void;
                    if fmt_spec as libc::c_int == 'p' as i32 {
                        ptr_arg =
                            if !tvs.is_null() {
                                tv_ptr(tvs, &mut arg_idx)
                            } else { ap.arg::<*mut libc::c_void>() };
                        if !ptr_arg.is_null() { arg_sign = 1 as libc::c_int }
                    } else if fmt_spec as libc::c_int == 'd' as i32 {
                        // signed
                        match length_modifier as libc::c_int {
                            0 => {
                                arg =
                                    if !tvs.is_null() {
                                        tv_nr(tvs, &mut arg_idx)
                                    } else {
                                        ap.arg::<libc::c_int>() as
                                            libc::c_long
                                    } as libc::c_int as intmax_t
                            }
                            104 => {
                                // char and short arguments are passed as int16_t
                                arg =
                                    if !tvs.is_null() {
                                        tv_nr(tvs, &mut arg_idx)
                                    } else {
                                        ap.arg::<libc::c_int>() as
                                            libc::c_long
                                    } as int16_t as intmax_t
                            }
                            108 => {
                                arg =
                                    if !tvs.is_null() {
                                        tv_nr(tvs, &mut arg_idx)
                                    } else { ap.arg::<libc::c_long>() }
                            }
                            50 => {
                                arg =
                                    if !tvs.is_null() {
                                        tv_nr(tvs, &mut arg_idx) as
                                            libc::c_longlong
                                    } else { ap.arg::<libc::c_longlong>() } as
                                        intmax_t
                            }
                            122 => {
                                arg =
                                    if !tvs.is_null() {
                                        tv_nr(tvs, &mut arg_idx)
                                    } else { ap.arg::<ptrdiff_t>() }
                            }
                            _ => { }
                        } // NOLINT (runtime/int)
                        if arg > 0 as libc::c_int as libc::c_long {
                            arg_sign = 1 as libc::c_int
                        } else if arg < 0 as libc::c_int as libc::c_long {
                            arg_sign = -(1 as libc::c_int)
                        }
                    } else {
                        // unsigned
                        match length_modifier as libc::c_int {
                            0 => {
                                uarg =
                                    if !tvs.is_null() {
                                        tv_nr(tvs, &mut arg_idx)
                                    } else {
                                        ap.arg::<libc::c_uint>() as
                                            libc::c_long
                                    } as libc::c_uint as uintmax_t
                            }
                            104 => {
                                uarg =
                                    if !tvs.is_null() {
                                        tv_nr(tvs, &mut arg_idx)
                                    } else {
                                        ap.arg::<libc::c_uint>() as
                                            libc::c_long
                                    } as uint16_t as uintmax_t
                            }
                            108 => {
                                uarg =
                                    if !tvs.is_null() {
                                        tv_nr(tvs, &mut arg_idx) as
                                            libc::c_ulong
                                    } else { ap.arg::<libc::c_ulong>() }
                            }
                            50 => {
                                uarg =
                                    if !tvs.is_null() {
                                        tv_nr(tvs, &mut arg_idx) as
                                            libc::c_ulonglong
                                    } else { ap.arg::<libc::c_ulonglong>() }
                                        as uintmax_t
                            }
                            122 => {
                                uarg =
                                    if !tvs.is_null() {
                                        tv_nr(tvs, &mut arg_idx) as size_t
                                    } else { ap.arg::<size_t>() }
                            }
                            _ => { }
                        } // NOLINT (runtime/int)
                        arg_sign =
                            (uarg != 0 as libc::c_int as libc::c_ulong) as
                                libc::c_int
                    }
                    str_arg = tmp.as_mut_ptr();
                    str_arg_l = 0 as libc::c_int as size_t;
                    // For d, i, u, o, x, and X conversions, if precision is specified,
          // '0' flag should be ignored. This is so with Solaris 2.6, Digital
          // UNIX 4.0, HPUX 10, Linux, FreeBSD, NetBSD; but not with Perl.
                    if precision_specified != 0 {
                        zero_padding = 0 as libc::c_int
                    }
                    if fmt_spec as libc::c_int == 'd' as i32 {
                        if force_sign != 0 && arg_sign >= 0 as libc::c_int {
                            let fresh25 = str_arg_l;
                            str_arg_l = str_arg_l.wrapping_add(1);
                            tmp[fresh25 as usize] =
                                if space_for_positive != 0 {
                                    ' ' as i32
                                } else { '+' as i32 } as libc::c_char
                        }
                        // leave negative numbers for snprintf to handle, to
            // avoid handling tricky cases like (short int)-32768
                    } else if alternate_form != 0 {
                        if arg_sign != 0 as libc::c_int &&
                               (fmt_spec as libc::c_int == 'x' as i32 ||
                                    fmt_spec as libc::c_int == 'X' as i32 ||
                                    fmt_spec as libc::c_int == 'b' as i32 ||
                                    fmt_spec as libc::c_int == 'B' as i32) {
                            let fresh26 = str_arg_l;
                            str_arg_l = str_arg_l.wrapping_add(1);
                            tmp[fresh26 as usize] =
                                '0' as i32 as libc::c_char;
                            let fresh27 = str_arg_l;
                            str_arg_l = str_arg_l.wrapping_add(1);
                            tmp[fresh27 as usize] = fmt_spec
                        }
                        // alternate form should have no effect for p * conversion, but ...
                    }
                    zero_padding_insertion_ind = str_arg_l;
                    if precision_specified == 0 {
                        precision = 1 as libc::c_int as size_t
                        // default precision is 1
                    }
                    if !(precision == 0 as libc::c_int as libc::c_ulong &&
                             arg_sign == 0 as libc::c_int) {
                        match fmt_spec as libc::c_int {
                            112 => {
                                // pointer
                                str_arg_l =
                                    (str_arg_l as
                                         libc::c_ulong).wrapping_add(snprintf(tmp.as_mut_ptr().offset(str_arg_l
                                                                                                          as
                                                                                                          isize),
                                                                              (::std::mem::size_of::<[libc::c_char; 350]>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_sub(str_arg_l),
                                                                              b"%p\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char,
                                                                              ptr_arg)
                                                                         as
                                                                         size_t)
                                        as size_t as size_t
                            }
                            100 => {
                                // signed
                                str_arg_l =
                                    (str_arg_l as
                                         libc::c_ulong).wrapping_add(snprintf(tmp.as_mut_ptr().offset(str_arg_l
                                                                                                          as
                                                                                                          isize),
                                                                              (::std::mem::size_of::<[libc::c_char; 350]>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_sub(str_arg_l),
                                                                              b"%ld\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char,
                                                                              arg)
                                                                         as
                                                                         size_t)
                                        as size_t as size_t
                            }
                            98 | 66 => {
                                // binary
                                let mut bits = 0 as libc::c_int as size_t;
                                bits =
                                    (::std::mem::size_of::<uintmax_t>() as
                                         libc::c_ulong).wrapping_mul(8 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong);
                                while bits > 0 as libc::c_int as libc::c_ulong
                                      {
                                    if uarg >>
                                           bits.wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)
                                           &
                                           0x1 as libc::c_int as libc::c_ulong
                                           != 0 {
                                        break ;
                                    }
                                    bits = bits.wrapping_sub(1)
                                }
                                while bits > 0 as libc::c_int as libc::c_ulong
                                      {
                                    bits = bits.wrapping_sub(1);
                                    let fresh28 = str_arg_l;
                                    str_arg_l = str_arg_l.wrapping_add(1);
                                    tmp[fresh28 as usize] =
                                        if uarg >> bits &
                                               0x1 as libc::c_int as
                                                   libc::c_ulong != 0 {
                                            '1' as i32
                                        } else { '0' as i32 } as libc::c_char
                                }
                            }
                            _ => {
                                // unsigned
                                // construct a simple format string for snprintf
                                let mut f: [libc::c_char; 4] =
                                    *::std::mem::transmute::<&[u8; 4],
                                                             &mut [libc::c_char; 4]>(b"%lu\x00");
                                f[(::std::mem::size_of::<[libc::c_char; 4]>()
                                       as
                                       libc::c_ulong).wrapping_sub(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong).wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)
                                      as usize] = fmt_spec;
                                if (*::std::mem::transmute::<&[u8; 3],
                                                             &[libc::c_char; 3]>(b"lu\x00"))[(::std::mem::size_of::<[libc::c_char; 3]>()
                                                                                                  as
                                                                                                  libc::c_ulong).wrapping_sub(1
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_ulong).wrapping_sub(1
                                                                                                                                                                  as
                                                                                                                                                                  libc::c_int
                                                                                                                                                                  as
                                                                                                                                                                  libc::c_ulong)
                                                                                                 as
                                                                                                 usize]
                                       as libc::c_int == 'u' as i32 {
                                } else {
                                    __assert_fail(b"PRIuMAX[sizeof(PRIuMAX) - 1 - 1] == \'u\'\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"/home/vole/neovim/src/nvim/strings.c\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  1138 as libc::c_int as
                                                      libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 96],
                                                                            &[libc::c_char; 96]>(b"int vim_vsnprintf_typval(char *, size_t, const char *, struct __va_list_tag *, typval_T *const)\x00")).as_ptr());
                                }
                                str_arg_l =
                                    (str_arg_l as
                                         libc::c_ulong).wrapping_add(snprintf(tmp.as_mut_ptr().offset(str_arg_l
                                                                                                          as
                                                                                                          isize),
                                                                              (::std::mem::size_of::<[libc::c_char; 350]>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_sub(str_arg_l),
                                                                              f.as_mut_ptr(),
                                                                              uarg)
                                                                         as
                                                                         size_t)
                                        as size_t as size_t
                            }
                        }
                        if str_arg_l <
                               ::std::mem::size_of::<[libc::c_char; 350]>() as
                                   libc::c_ulong {
                        } else {
                            __assert_fail(b"str_arg_l < sizeof(tmp)\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          b"/home/vole/neovim/src/nvim/strings.c\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          1145 as libc::c_int as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 96],
                                                                    &[libc::c_char; 96]>(b"int vim_vsnprintf_typval(char *, size_t, const char *, struct __va_list_tag *, typval_T *const)\x00")).as_ptr());
                        }
                        // include the optional minus sign and possible "0x" in the region
            // before the zero padding insertion point
                        if zero_padding_insertion_ind < str_arg_l &&
                               tmp[zero_padding_insertion_ind as usize] as
                                   libc::c_int == '-' as i32 {
                            zero_padding_insertion_ind =
                                zero_padding_insertion_ind.wrapping_add(1)
                        }
                        if zero_padding_insertion_ind.wrapping_add(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong)
                               < str_arg_l &&
                               tmp[zero_padding_insertion_ind as usize] as
                                   libc::c_int == '0' as i32 &&
                               (tmp[zero_padding_insertion_ind.wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                                        as usize] as libc::c_int == 'x' as i32
                                    ||
                                    tmp[zero_padding_insertion_ind.wrapping_add(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                                            as usize] as libc::c_int ==
                                        'X' as i32 ||
                                    tmp[zero_padding_insertion_ind.wrapping_add(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                                            as usize] as libc::c_int ==
                                        'b' as i32 ||
                                    tmp[zero_padding_insertion_ind.wrapping_add(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                                            as usize] as libc::c_int ==
                                        'B' as i32) {
                            zero_padding_insertion_ind =
                                (zero_padding_insertion_ind as
                                     libc::c_ulong).wrapping_add(2 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                    as size_t as size_t
                        }
                    }
                    let mut num_of_digits =
                        str_arg_l.wrapping_sub(zero_padding_insertion_ind);
                    if alternate_form != 0 &&
                           fmt_spec as libc::c_int == 'o' as i32 &&
                           !(zero_padding_insertion_ind < str_arg_l &&
                                 tmp[zero_padding_insertion_ind as usize] as
                                     libc::c_int == '0' as i32) {
                        // assure leading zero for alternate-form octal numbers
                        if precision_specified == 0 ||
                               precision <
                                   num_of_digits.wrapping_add(1 as libc::c_int
                                                                  as
                                                                  libc::c_ulong)
                           {
                            // precision is increased to force the first character to be
                // zero, except if a zero value is formatted with an explicit
                // precision of zero
                            precision =
                                num_of_digits.wrapping_add(1 as libc::c_int as
                                                               libc::c_ulong)
                        }
                    }
                    // zero padding to specified precision?
                    if num_of_digits < precision {
                        number_of_zeros_to_pad =
                            precision.wrapping_sub(num_of_digits)
                    }
                    // zero padding to specified minimal field width?
                    if justify_left == 0 && zero_padding != 0 {
                        let n_0 =
                            min_field_width.wrapping_sub(str_arg_l.wrapping_add(number_of_zeros_to_pad))
                                as libc::c_int;
                        if n_0 > 0 as libc::c_int {
                            number_of_zeros_to_pad =
                                (number_of_zeros_to_pad as
                                     libc::c_ulong).wrapping_add(n_0 as
                                                                     size_t)
                                    as size_t as size_t
                        }
                    }
                }
                102 | 70 | 101 | 69 | 103 | 71 => {
                    // floating point
                    let mut format: [libc::c_char; 40] = [0; 40];
                    let mut remove_trailing_zeroes = false_0;
                    let mut f_0 =
                        if !tvs.is_null() {
                            tv_float(tvs, &mut arg_idx)
                        } else { ap.arg::<libc::c_double>() };
                    let mut abs_f =
                        if f_0 < 0 as libc::c_int as libc::c_double {
                            -f_0
                        } else { f_0 };
                    if fmt_spec as libc::c_int == 'g' as i32 ||
                           fmt_spec as libc::c_int == 'G' as i32 {
                        // can't use %g directly, cause it prints "1.0" as "1"
                        if abs_f >= 0.001f64 && abs_f < 10000000.0f64 ||
                               abs_f == 0.0f64 {
                            fmt_spec =
                                if fmt_spec as libc::c_uint >=
                                       'A' as i32 as libc::c_uint &&
                                       fmt_spec as libc::c_uint <=
                                           'Z' as i32 as libc::c_uint {
                                    'F' as i32
                                } else { 'f' as i32 } as libc::c_char
                        } else {
                            fmt_spec =
                                if fmt_spec as libc::c_int == 'g' as i32 {
                                    'e' as i32
                                } else { 'E' as i32 } as libc::c_char
                        }
                        remove_trailing_zeroes = true_0
                    }
                    if xisinf(f_0) != 0 ||
                           !strchr(b"fF\x00" as *const u8 as
                                       *const libc::c_char,
                                   fmt_spec as libc::c_int).is_null() &&
                               abs_f > 1.0e307f64 {
                        xstrlcpy(tmp.as_mut_ptr(),
                                 infinity_str(f_0 > 0.0f64, fmt_spec,
                                              force_sign, space_for_positive),
                                 ::std::mem::size_of::<[libc::c_char; 350]>()
                                     as libc::c_ulong);
                        str_arg_l = strlen(tmp.as_mut_ptr());
                        zero_padding = 0 as libc::c_int
                    } else if xisnan(f_0) != 0 {
                        // Not a number: nan or NAN
                        memmove(tmp.as_mut_ptr() as *mut libc::c_void,
                                if fmt_spec as libc::c_uint >=
                                       'A' as i32 as libc::c_uint &&
                                       fmt_spec as libc::c_uint <=
                                           'Z' as i32 as libc::c_uint {
                                    b"NAN\x00" as *const u8 as
                                        *const libc::c_char
                                } else {
                                    b"nan\x00" as *const u8 as
                                        *const libc::c_char
                                } as *const libc::c_void,
                                4 as libc::c_int as libc::c_ulong);
                        str_arg_l = 3 as libc::c_int as size_t;
                        zero_padding = 0 as libc::c_int
                    } else {
                        // Regular float number
                        format[0 as libc::c_int as usize] =
                            '%' as i32 as libc::c_char;
                        let mut l = 1 as libc::c_int as size_t;
                        if force_sign != 0 {
                            let fresh29 = l;
                            l = l.wrapping_add(1);
                            format[fresh29 as usize] =
                                if space_for_positive != 0 {
                                    ' ' as i32
                                } else { '+' as i32 } as libc::c_char
                        }
                        if precision_specified != 0 {
                            let mut max_prec =
                                (TMP_LEN - 10 as libc::c_int) as size_t;
                            // make sure we don't get more digits than we have room for
                            if (fmt_spec as libc::c_int == 'f' as i32 ||
                                    fmt_spec as libc::c_int == 'F' as i32) &&
                                   abs_f > 1.0f64 {
                                max_prec =
                                    (max_prec as
                                         libc::c_ulong).wrapping_sub(log10(abs_f)
                                                                         as
                                                                         size_t)
                                        as size_t as size_t
                            }
                            if precision > max_prec { precision = max_prec }
                            l =
                                (l as
                                     libc::c_ulong).wrapping_add(snprintf(format.as_mut_ptr().offset(l
                                                                                                         as
                                                                                                         isize),
                                                                          (::std::mem::size_of::<[libc::c_char; 40]>()
                                                                               as
                                                                               libc::c_ulong).wrapping_sub(l),
                                                                          b".%d\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char,
                                                                          precision
                                                                              as
                                                                              libc::c_int)
                                                                     as
                                                                     size_t)
                                    as size_t as size_t
                        }
                        // Cast to char to avoid a conversion warning on Ubuntu 12.04.
                        if l.wrapping_add(1 as libc::c_int as libc::c_ulong) <
                               ::std::mem::size_of::<[libc::c_char; 40]>() as
                                   libc::c_ulong {
                        } else {
                            __assert_fail(b"l + 1 < sizeof(format)\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          b"/home/vole/neovim/src/nvim/strings.c\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          1253 as libc::c_int as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 96],
                                                                    &[libc::c_char; 96]>(b"int vim_vsnprintf_typval(char *, size_t, const char *, struct __va_list_tag *, typval_T *const)\x00")).as_ptr());
                        }
                        format[l as usize] =
                            if fmt_spec as libc::c_int == 'F' as i32 {
                                'f' as i32
                            } else { fmt_spec as libc::c_int } as
                                libc::c_char;
                        format[l.wrapping_add(1 as libc::c_int as
                                                  libc::c_ulong) as usize] =
                            NUL as libc::c_char;
                        str_arg_l =
                            snprintf(tmp.as_mut_ptr(),
                                     ::std::mem::size_of::<[libc::c_char; 350]>()
                                         as libc::c_ulong,
                                     format.as_mut_ptr(), f_0) as size_t;
                        if str_arg_l <
                               ::std::mem::size_of::<[libc::c_char; 350]>() as
                                   libc::c_ulong {
                        } else {
                            __assert_fail(b"str_arg_l < sizeof(tmp)\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          b"/home/vole/neovim/src/nvim/strings.c\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          1258 as libc::c_int as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 96],
                                                                    &[libc::c_char; 96]>(b"int vim_vsnprintf_typval(char *, size_t, const char *, struct __va_list_tag *, typval_T *const)\x00")).as_ptr());
                        }
                        if remove_trailing_zeroes != 0 {
                            let mut i_0: libc::c_int = 0;
                            let mut tp = 0 as *mut libc::c_char;
                            // using %g or %G: remove superfluous zeroes
                            if fmt_spec as libc::c_int == 'f' as i32 ||
                                   fmt_spec as libc::c_int == 'F' as i32 {
                                tp =
                                    tmp.as_mut_ptr().offset(str_arg_l as
                                                                isize).offset(-(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                            } else {
                                tp =
                                    vim_strchr(tmp.as_mut_ptr() as
                                                   *mut char_u,
                                               if fmt_spec as libc::c_int ==
                                                      'e' as i32 {
                                                   'e' as i32
                                               } else { 'E' as i32 }) as
                                        *mut libc::c_char;
                                if !tp.is_null() {
                                    // remove superfluous '+' and leading zeroes from exponent
                                    if *tp.offset(1 as libc::c_int as isize)
                                           as libc::c_int == '+' as i32 {
                                        // change "1.0e+07" to "1.0e07"
                                        memmove(tp.offset(1 as libc::c_int as
                                                              isize) as
                                                    *mut libc::c_void,
                                                tp.offset(2 as libc::c_int as
                                                              isize) as
                                                    *const libc::c_void,
                                                strlen(tp.offset(2 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).wrapping_add(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong));
                                        str_arg_l = str_arg_l.wrapping_sub(1)
                                    }
                                    i_0 =
                                        if *tp.offset(1 as libc::c_int as
                                                          isize) as
                                               libc::c_int == '-' as i32 {
                                            2 as libc::c_int
                                        } else { 1 as libc::c_int };
                                    while *tp.offset(i_0 as isize) as
                                              libc::c_int == '0' as i32 {
                                        // change "1.0e07" to "1.0e7"
                                        memmove(tp.offset(i_0 as isize) as
                                                    *mut libc::c_void,
                                                tp.offset(i_0 as
                                                              isize).offset(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                                    as *const libc::c_void,
                                                strlen(tp.offset(i_0 as
                                                                     isize).offset(1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize)).wrapping_add(1
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_ulong));
                                        str_arg_l = str_arg_l.wrapping_sub(1)
                                    }
                                    tp = tp.offset(-1)
                                }
                            }
                            if !tp.is_null() && precision_specified == 0 {
                                // remove trailing zeroes, but keep the one just after a dot
                                while tp >
                                          tmp.as_mut_ptr().offset(2 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                          && *tp as libc::c_int == '0' as i32
                                          &&
                                          *tp.offset(-(1 as libc::c_int) as
                                                         isize) as libc::c_int
                                              != '.' as i32 {
                                    memmove(tp as *mut libc::c_void,
                                            tp.offset(1 as libc::c_int as
                                                          isize) as
                                                *const libc::c_void,
                                            strlen(tp.offset(1 as libc::c_int
                                                                 as
                                                                 isize)).wrapping_add(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong));
                                    tp = tp.offset(-1);
                                    str_arg_l = str_arg_l.wrapping_sub(1)
                                }
                            }
                        } else {
                            // Be consistent: some printf("%e") use 1.0e+12 and some
                // 1.0e+012; remove one zero in the last case.
                            let mut tp_0 =
                                vim_strchr(tmp.as_mut_ptr() as *mut char_u,
                                           if fmt_spec as libc::c_int ==
                                                  'e' as i32 {
                                               'e' as i32
                                           } else { 'E' as i32 }) as
                                    *mut libc::c_char;
                            if !tp_0.is_null() &&
                                   (*tp_0.offset(1 as libc::c_int as isize) as
                                        libc::c_int == '+' as i32 ||
                                        *tp_0.offset(1 as libc::c_int as
                                                         isize) as libc::c_int
                                            == '-' as i32) &&
                                   *tp_0.offset(2 as libc::c_int as isize) as
                                       libc::c_int == '0' as i32 &&
                                   ascii_isdigit(*tp_0.offset(3 as libc::c_int
                                                                  as isize) as
                                                     libc::c_int) as
                                       libc::c_int != 0 &&
                                   ascii_isdigit(*tp_0.offset(4 as libc::c_int
                                                                  as isize) as
                                                     libc::c_int) as
                                       libc::c_int != 0 {
                                memmove(tp_0.offset(2 as libc::c_int as isize)
                                            as *mut libc::c_void,
                                        tp_0.offset(3 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        strlen(tp_0.offset(3 as libc::c_int as
                                                               isize)).wrapping_add(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong));
                                str_arg_l = str_arg_l.wrapping_sub(1)
                            }
                        }
                    }
                    if zero_padding != 0 && min_field_width > str_arg_l &&
                           (tmp[0 as libc::c_int as usize] as libc::c_int ==
                                '-' as i32 || force_sign != 0) {
                        // Padding 0's should be inserted after the sign.
                        number_of_zeros_to_pad =
                            min_field_width.wrapping_sub(str_arg_l); // turn zero padding off for non-numeric conversion
                        zero_padding_insertion_ind =
                            1 as libc::c_int as size_t
                    }
                    str_arg = tmp.as_mut_ptr()
                }
                _ => {
                    // unrecognized conversion specifier, keep format string as-is
                    zero_padding = 0 as libc::c_int; // reset flags
                    justify_left = 1 as libc::c_int;
                    min_field_width = 0 as libc::c_int as size_t;
                    // discard the unrecognized conversion, just keep
          // the unrecognized conversion character
                    str_arg = p;
                    str_arg_l = 0 as libc::c_int as size_t;
                    if *p != 0 {
                        str_arg_l = str_arg_l.wrapping_add(1)
                        // include invalid conversion specifier
                    }
                }
            }
            if *p != 0 {
                p = p.offset(1)
                // step over the just processed conversion specifier
            }
            // insert padding to the left as requested by min_field_width;
      // this does not include the zero padding in case of numerical conversions
            if justify_left == 0 {
                if str_arg_l <=
                       (18446744073709551615 as
                            libc::c_ulong).wrapping_sub(number_of_zeros_to_pad)
                   {
                } else {
                    __assert_fail(b"str_arg_l <= SIZE_MAX - number_of_zeros_to_pad\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"/home/vole/neovim/src/nvim/strings.c\x00"
                                      as *const u8 as *const libc::c_char,
                                  1341 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 96],
                                                            &[libc::c_char; 96]>(b"int vim_vsnprintf_typval(char *, size_t, const char *, struct __va_list_tag *, typval_T *const)\x00")).as_ptr());
                }
                if min_field_width >
                       str_arg_l.wrapping_add(number_of_zeros_to_pad) {
                    // left padding with blank or zero
                    let mut pn =
                        min_field_width.wrapping_sub(str_arg_l.wrapping_add(number_of_zeros_to_pad));
                    if str_avail {
                        let mut avail_0 = str_m.wrapping_sub(str_l);
                        memset(str.offset(str_l as isize) as
                                   *mut libc::c_void,
                               if zero_padding != 0 {
                                   '0' as i32
                               } else { ' ' as i32 },
                               if pn < avail_0 { pn } else { avail_0 });
                        str_avail = pn < avail_0
                    }
                    if pn <=
                           (18446744073709551615 as
                                libc::c_ulong).wrapping_sub(str_l) {
                    } else {
                        __assert_fail(b"pn <= SIZE_MAX - str_l\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"/home/vole/neovim/src/nvim/strings.c\x00"
                                          as *const u8 as *const libc::c_char,
                                      1350 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 96],
                                                                &[libc::c_char; 96]>(b"int vim_vsnprintf_typval(char *, size_t, const char *, struct __va_list_tag *, typval_T *const)\x00")).as_ptr());
                    }
                    str_l =
                        (str_l as libc::c_ulong).wrapping_add(pn) as size_t as
                            size_t
                }
            }
            // zero padding as requested by the precision or by the minimal
      // field width for numeric conversions required?
            if number_of_zeros_to_pad == 0 as libc::c_int as libc::c_ulong {
                // will not copy first part of numeric right now,
        // force it to be copied later in its entirety
                zero_padding_insertion_ind = 0 as libc::c_int as size_t
            } else {
                // insert first part of numerics (sign or '0x') before zero padding
                if zero_padding_insertion_ind >
                       0 as libc::c_int as libc::c_ulong {
                    let mut zn = zero_padding_insertion_ind;
                    if str_avail {
                        let mut avail_1 = str_m.wrapping_sub(str_l);
                        memmove(str.offset(str_l as isize) as
                                    *mut libc::c_void,
                                str_arg as *const libc::c_void,
                                if zn < avail_1 { zn } else { avail_1 });
                        str_avail = zn < avail_1
                    }
                    if zn <=
                           (18446744073709551615 as
                                libc::c_ulong).wrapping_sub(str_l) {
                    } else {
                        __assert_fail(b"zn <= SIZE_MAX - str_l\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"/home/vole/neovim/src/nvim/strings.c\x00"
                                          as *const u8 as *const libc::c_char,
                                      1370 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 96],
                                                                &[libc::c_char; 96]>(b"int vim_vsnprintf_typval(char *, size_t, const char *, struct __va_list_tag *, typval_T *const)\x00")).as_ptr());
                    }
                    str_l =
                        (str_l as libc::c_ulong).wrapping_add(zn) as size_t as
                            size_t
                }
                // insert zero padding as requested by precision or min field width
                let mut zn_0 = number_of_zeros_to_pad;
                if str_avail {
                    let mut avail_2 = str_m.wrapping_sub(str_l);
                    memset(str.offset(str_l as isize) as *mut libc::c_void,
                           '0' as i32,
                           if zn_0 < avail_2 { zn_0 } else { avail_2 });
                    str_avail = zn_0 < avail_2
                }
                if zn_0 <=
                       (18446744073709551615 as
                            libc::c_ulong).wrapping_sub(str_l) {
                } else {
                    __assert_fail(b"zn <= SIZE_MAX - str_l\x00" as *const u8
                                      as *const libc::c_char,
                                  b"/home/vole/neovim/src/nvim/strings.c\x00"
                                      as *const u8 as *const libc::c_char,
                                  1381 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 96],
                                                            &[libc::c_char; 96]>(b"int vim_vsnprintf_typval(char *, size_t, const char *, struct __va_list_tag *, typval_T *const)\x00")).as_ptr());
                }
                str_l =
                    (str_l as libc::c_ulong).wrapping_add(zn_0) as size_t as
                        size_t
            }
            // insert formatted string
      // (or as-is conversion specifier for unknown conversions)
            if str_arg_l > zero_padding_insertion_ind {
                let mut sn =
                    str_arg_l.wrapping_sub(zero_padding_insertion_ind);
                if str_avail {
                    let mut avail_3 = str_m.wrapping_sub(str_l);
                    memmove(str.offset(str_l as isize) as *mut libc::c_void,
                            str_arg.offset(zero_padding_insertion_ind as
                                               isize) as *const libc::c_void,
                            if sn < avail_3 { sn } else { avail_3 });
                    str_avail = sn < avail_3
                }
                if sn <=
                       (18446744073709551615 as
                            libc::c_ulong).wrapping_sub(str_l) {
                } else {
                    __assert_fail(b"sn <= SIZE_MAX - str_l\x00" as *const u8
                                      as *const libc::c_char,
                                  b"/home/vole/neovim/src/nvim/strings.c\x00"
                                      as *const u8 as *const libc::c_char,
                                  1396 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 96],
                                                            &[libc::c_char; 96]>(b"int vim_vsnprintf_typval(char *, size_t, const char *, struct __va_list_tag *, typval_T *const)\x00")).as_ptr());
                }
                str_l =
                    (str_l as libc::c_ulong).wrapping_add(sn) as size_t as
                        size_t
            }
            // insert right padding
            if justify_left != 0 {
                if str_arg_l <=
                       (18446744073709551615 as
                            libc::c_ulong).wrapping_sub(number_of_zeros_to_pad)
                   {
                } else {
                    __assert_fail(b"str_arg_l <= SIZE_MAX - number_of_zeros_to_pad\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"/home/vole/neovim/src/nvim/strings.c\x00"
                                      as *const u8 as *const libc::c_char,
                                  1402 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 96],
                                                            &[libc::c_char; 96]>(b"int vim_vsnprintf_typval(char *, size_t, const char *, struct __va_list_tag *, typval_T *const)\x00")).as_ptr());
                }
                if min_field_width >
                       str_arg_l.wrapping_add(number_of_zeros_to_pad) {
                    // right blank padding to the field width
                    let mut pn_0 =
                        min_field_width.wrapping_sub(str_arg_l.wrapping_add(number_of_zeros_to_pad));
                    if str_avail {
                        let mut avail_4 = str_m.wrapping_sub(str_l);
                        memset(str.offset(str_l as isize) as
                                   *mut libc::c_void, ' ' as i32,
                               if pn_0 < avail_4 { pn_0 } else { avail_4 });
                        str_avail = pn_0 < avail_4
                    }
                    if pn_0 <=
                           (18446744073709551615 as
                                libc::c_ulong).wrapping_sub(str_l) {
                    } else {
                        __assert_fail(b"pn <= SIZE_MAX - str_l\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"/home/vole/neovim/src/nvim/strings.c\x00"
                                          as *const u8 as *const libc::c_char,
                                      1411 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 96],
                                                                &[libc::c_char; 96]>(b"int vim_vsnprintf_typval(char *, size_t, const char *, struct __va_list_tag *, typval_T *const)\x00")).as_ptr());
                    }
                    str_l =
                        (str_l as libc::c_ulong).wrapping_add(pn_0) as size_t
                            as size_t
                }
            }
            xfree(tofree as *mut libc::c_void);
        }
    }
    if str_m > 0 as libc::c_int as libc::c_ulong {
        // make sure the string is nul-terminated even at the expense of
    // overwriting the last character (shouldn't happen, but just in case)
        *str.offset(if str_l <=
                           str_m.wrapping_sub(1 as libc::c_int as
                                                  libc::c_ulong) {
                        str_l
                    } else {
                        str_m.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } as isize) = '\u{0}' as i32 as libc::c_char
    }
    if !tvs.is_null() &&
           (*tvs.offset((arg_idx - 1 as libc::c_int) as isize)).v_type as
               libc::c_uint != VAR_UNKNOWN as libc::c_int as libc::c_uint {
        emsg(gettext(b"E767: Too many arguments to printf()\x00" as *const u8
                         as *const libc::c_char as *mut libc::c_char) as
                 *mut char_u);
    }
    // return the number of characters formatted (excluding trailing nul
  // character); that is, the number of characters that would have been
  // written to the buffer if it were large enough.
    return str_l as libc::c_int;
}
#[c2rust::src_loc = "790:10"]
pub const TMP_LEN: libc::c_int = 350 as libc::c_int;
