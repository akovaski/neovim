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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:9"]
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
    #[c2rust::src_loc = "144:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "145:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "158:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "191:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-intn.h:9"]
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h:9"]
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
#[c2rust::header_src =
  "/usr/lib/llvm-6.0/lib/clang/6.0.1/include/stddef.h:10"]
pub mod stddef_h {
    #[c2rust::src_loc = "62:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "51:1"]
    pub type ptrdiff_t = libc::c_long;
    #[c2rust::src_loc = "105:11"]
    pub const NULL_1: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "105:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "105:11"]
    pub const NULL_0: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/types.h:13"]
pub mod nvim_types_h {
    // dummy to pass an ACL to a function
    // Shorthand for unsigned variables. Many systems, but not all, have u_char
// already defined, so we use char_u to avoid trouble.
    #[c2rust::src_loc = "11:1"]
    pub type char_u = libc::c_uchar;
    #[c2rust::src_loc = "17:1"]
    pub type handle_T = libc::c_int;
    // Can hold one decoded UTF-8 character.
    // Opaque handle used by API clients to refer to various objects in vim
    // Opaque handle to a lua value. Must be free with `executor_free_luaref` when
// not needed anymore! LUA_NOREF represents missing reference, i e to indicate
// absent callback etc.
    #[c2rust::src_loc = "22:1"]
    pub type LuaRef = libc::c_int;
    // NVIM_TYPES_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/eval/typval.h:13"]
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
        pub data: C2RustUnnamed_1,
        pub type_0: CallbackType,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:3"]
    pub union C2RustUnnamed_1 {
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
    pub type C2RustUnnamed_2 = libc::c_uint;
    #[c2rust::src_loc = "227:3"]
    pub const DI_FLAGS_ALLOC: C2RustUnnamed_2 = 16;
    #[c2rust::src_loc = "226:3"]
    pub const DI_FLAGS_LOCK: C2RustUnnamed_2 = 8;
    #[c2rust::src_loc = "225:3"]
    pub const DI_FLAGS_FIX: C2RustUnnamed_2 = 4;
    #[c2rust::src_loc = "224:3"]
    pub const DI_FLAGS_RO_SBX: C2RustUnnamed_2 = 2;
    #[c2rust::src_loc = "223:3"]
    pub const DI_FLAGS_RO: C2RustUnnamed_2 = 1;
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/pos.h:13"]
pub mod pos_h {
    #[c2rust::src_loc = "4:1"]
    pub type linenr_T = libc::c_long;
    #[c2rust::src_loc = "9:1"]
    pub type colnr_T = libc::c_int;
    #[c2rust::src_loc = "14:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "14:8"]
    pub const MAXLNUM: C2RustUnnamed = 2147483647;
    #[c2rust::src_loc = "16:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "16:8"]
    pub const MAXCOL: C2RustUnnamed_0 = 2147483647;
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
#[c2rust::header_src =
  "/usr/lib/llvm-6.0/lib/clang/6.0.1/include/stdarg.h:13"]
pub mod stdarg_h {
    #[c2rust::src_loc = "30:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/stdio.h:13"]
pub mod stdio_h {
    #[c2rust::src_loc = "77:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "334:12"]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/types.h:13"]
pub mod sys_types_h {
    #[c2rust::src_loc = "64:1"]
    pub type gid_t = __gid_t;
    #[c2rust::src_loc = "79:1"]
    pub type uid_t = __uid_t;
    use super::types_h::{__gid_t, __uid_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/time_t.h:13"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/pthreadtypes-arch.h:13"]
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
  "/usr/include/x86_64-linux-gnu/bits/thread-shared-types.h:13"]
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/pthreadtypes.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/hashtab.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/garray.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/lib/queue.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/profile.h:13"]
pub mod profile_h {
    #[c2rust::src_loc = "7:1"]
    pub type proftime_T = uint64_t;
    use super::stdint_uintn_h::uint64_t;
    // NVIM_PROFILE_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/grid_defs.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/vim.h:13"]
pub mod vim_h {
    // special attribute addition: Put message in history
    // values for State
//
// The lower bits up to 0x20 are used to distinguish normal/visual/op_pending
// and cmdline/insert+replace mode.  This is used for mapping.  If none of
// these bits are set, no mapping is done.
// The upper bits are used to distinguish between other states.
    // Normal mode, command expected
    // Visual mode - use get_real_state()
    // Normal mode, operator is pending - use
                                // get_real_state()
    // Editing command line
    // Insert mode
    // Language mapping, can be combined with
                                // INSERT and CMDLINE
    // Replace mode flag
    // Virtual-replace mode flag
    // Normal mode, busy with a command
    // waiting for return or command
    // Asking if you want --more--
    // window size has changed
    // abbreviation instead of mapping
    // executing an external command
    // show matching paren
    // ":confirm" prompt
    // Select mode, only for mappings
    // Terminal focus mode
    // Showing 'inccommand' command "live" preview.
    // all mode bits used for mapping
    // / Directions.
    #[c2rust::src_loc = "80:9"]
    pub type Direction = libc::c_int;
    #[c2rust::src_loc = "85:3"]
    pub const BACKWARD_FILE: Direction = -3;
    #[c2rust::src_loc = "84:3"]
    pub const FORWARD_FILE: Direction = 3;
    #[c2rust::src_loc = "83:3"]
    pub const BACKWARD: Direction = -1;
    #[c2rust::src_loc = "82:3"]
    pub const FORWARD: Direction = 1;
    #[c2rust::src_loc = "81:3"]
    pub const kDirectionNotSet: Direction = 0;
    #[c2rust::src_loc = "180:10"]
    pub const TRUE: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "179:10"]
    pub const FALSE: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "91:10"]
    pub const OK: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "93:9"]
    pub const FAIL: libc::c_int = 0 as libc::c_int;
    // NVIM_VIM_H
    // Replacement for nchar used by nv_replace().
    // BSD is supposed to cover FreeBSD and similar systems.
    // Lowest number used for window ID. Cannot have this many windows per tab.
    // only use "after" directories
    // skip "after" directories
    // do not use 'runtimepath'
    // also use "opt" directory in 'packpath'
    // also use "start" directory in 'packpath'
    // give an error message when none found
    // find directories instead of files
    // all matches, not just the first one
    // Used for flags in do_in_path()
    // Ex command defines
    // buffer and windows
    // This has to go after the include of proto.h, as proto/gui.pro declares
// functions of these names. The declarations would break if the defines had
// been seen at that stage.  But it must be before globals.h, where error_ga
// is declared.
    // / Maximum number of bytes in a multi-byte character.  It can be one 32-bit
// / character of up to 6 bytes, or one 16-bit character of up to three bytes
// / plus six following composing characters of three bytes each.
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/buffer_defs.h:13"]
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
        pub update_channels: C2RustUnnamed_4,
        pub update_callbacks: C2RustUnnamed_3,
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
    pub struct C2RustUnnamed_3 {
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
    pub struct C2RustUnnamed_4 {
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
        pub w_p_lcs_chars: C2RustUnnamed_6,
        pub w_p_fcs_chars: C2RustUnnamed_5,
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
    pub struct C2RustUnnamed_5 {
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
    pub struct C2RustUnnamed_6 {
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "867:8"]
    pub struct diffblock_S {
        pub df_next: *mut diff_T,
        pub df_lnum: [linenr_T; 8],
        pub df_count: [linenr_T; 8],
    }
    #[c2rust::src_loc = "866:1"]
    pub type diff_T = diffblock_S;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "882:8"]
    pub struct tabpage_S {
        pub handle: handle_T,
        pub tp_next: *mut tabpage_T,
        pub tp_topframe: *mut frame_T,
        pub tp_curwin: *mut win_T,
        pub tp_prevwin: *mut win_T,
        pub tp_firstwin: *mut win_T,
        pub tp_lastwin: *mut win_T,
        pub tp_old_Rows: libc::c_long,
        pub tp_old_Columns: libc::c_long,
        pub tp_ch_used: libc::c_long,
        pub tp_first_diff: *mut diff_T,
        pub tp_diffbuf: [*mut buf_T; 8],
        pub tp_diff_invalid: libc::c_int,
        pub tp_diff_update: libc::c_int,
        pub tp_snapshot: [*mut frame_T; 2],
        pub tp_winvar: ScopeDictDictItem,
        pub tp_vars: *mut dict_T,
        pub tp_localdir: *mut char_u,
    }
    #[c2rust::src_loc = "881:1"]
    pub type tabpage_T = tabpage_S;
    #[c2rust::src_loc = "471:9"]
    pub const BUF_HAS_QF_ENTRY: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "472:9"]
    pub const BUF_HAS_LL_ENTRY: libc::c_int = 2 as libc::c_int;
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/map.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/lib/khash.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/extmark_defs.h:13"]
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
    // Undo/redo extmarks
    #[c2rust::src_loc = "30:9"]
    pub type ExtmarkOp = libc::c_uint;
    // Operation should be undoable, but not redoable
    // Operation should not be reversable
    #[c2rust::src_loc = "34:3"]
    pub const kExtmarkUndoNoRedo: ExtmarkOp = 3;
    // Operation should be reversable/undoable
    #[c2rust::src_loc = "33:3"]
    pub const kExtmarkNoUndo: ExtmarkOp = 2;
    // Extmarks shouldn't be moved
    #[c2rust::src_loc = "32:3"]
    pub const kExtmarkUndo: ExtmarkOp = 1;
    #[c2rust::src_loc = "31:3"]
    pub const kExtmarkNOOP: ExtmarkOp = 0;
    use super::stdint_uintn_h::uint64_t;
    use super::stddef_h::size_t;
    use super::extmark_h::undo_object;
    // NVIM_EXTMARK_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/marktree.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/map_defs.h:13"]
pub mod map_defs_h {
    #[c2rust::src_loc = "7:1"]
    pub type ptr_t = *mut libc::c_void;
    #[c2rust::src_loc = "6:1"]
    pub type cstr_t = *const libc::c_char;
    // NVIM_MAP_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/terminal.h:13"]
pub mod terminal_h {
    #[c2rust::src_loc = "8:1"]
    pub type Terminal = terminal;
    extern "C" {
        #[c2rust::src_loc = "8:16"]
        pub type terminal;
    }
    // NVIM_TERMINAL_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/sign_defs.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/regexp_defs.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/api/private/defs.h:13"]
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
        pub data: C2RustUnnamed_14,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:3"]
    pub union C2RustUnnamed_14 {
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/mark_defs.h:13"]
pub mod mark_defs_h {
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
    // / Total possible number of global marks
    #[c2rust::src_loc = "20:9"]
    pub const NGLOBALMARKS: libc::c_int = NMARKS + EXTRA_MARKS;
    #[c2rust::src_loc = "14:9"]
    pub const EXTRA_MARKS: libc::c_int =
        '9' as i32 - '0' as i32 + 1 as libc::c_int;
    #[c2rust::src_loc = "29:9"]
    pub const JUMPLISTSIZE: libc::c_int = 100 as libc::c_int;
    #[c2rust::src_loc = "17:9"]
    pub const NMARKS: libc::c_int =
        'z' as i32 - 'a' as i32 + 1 as libc::c_int;
    use super::pos_h::pos_T;
    use super::time_h::Timestamp;
    use super::typval_h::dict_T;
    use super::nvim_types_h::char_u;
    // NVIM_MARK_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/os/time.h:13"]
pub mod time_h {
    #[c2rust::src_loc = "8:1"]
    pub type Timestamp = uint64_t;
    use super::stdint_uintn_h::uint64_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "18:1"]
        pub fn os_time() -> Timestamp;
    }
    // NVIM_OS_TIME_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/option_defs.h:13"]
pub mod option_defs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "862:9"]
    pub struct LastSet {
        pub script_ctx: sctx_T,
        pub channel_id: uint64_t,
    }
    #[c2rust::src_loc = "482:9"]
    pub const JOP_STACK: libc::c_int = 0x1 as libc::c_int;
    use super::typval_h::sctx_T;
    use super::stdint_uintn_h::uint64_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "316:13"]
        pub static mut breakat_flags: [libc::c_char; 256];
        #[no_mangle]
        #[c2rust::src_loc = "478:17"]
        pub static mut jop_flags: libc::c_uint;
    }
    // NVIM_OPTION_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/syntax_defs.h:13"]
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
        pub sst_union: C2RustUnnamed_7,
        pub sst_next_flags: libc::c_int,
        pub sst_stacksize: libc::c_int,
        pub sst_next_list: *mut int16_t,
        pub sst_tick: disptick_T,
        pub sst_change_lnum: linenr_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:3"]
    pub union C2RustUnnamed_7 {
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/undo_defs.h:13"]
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
        pub uh_next: C2RustUnnamed_12,
        pub uh_prev: C2RustUnnamed_11,
        pub uh_alt_next: C2RustUnnamed_10,
        pub uh_alt_prev: C2RustUnnamed_9,
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
    pub union C2RustUnnamed_9 {
        pub ptr: *mut u_header_T,
        pub seq: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:3"]
    pub union C2RustUnnamed_10 {
        pub ptr: *mut u_header_T,
        pub seq: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:3"]
    pub union C2RustUnnamed_11 {
        pub ptr: *mut u_header_T,
        pub seq: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:3"]
    pub union C2RustUnnamed_12 {
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/extmark.h:23"]
pub mod extmark_h {
    // TODO(bfredl): reduce the number of undo action types
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "59:8"]
    pub struct undo_object {
        pub type_0: UndoObjectType,
        pub data: C2RustUnnamed_8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "61:3"]
    pub union C2RustUnnamed_8 {
        pub splice: ExtmarkSplice,
        pub move_0: ExtmarkMove,
        pub savepos: ExtmarkSavePos,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:9"]
    pub struct ExtmarkSavePos {
        pub mark: uint64_t,
        pub old_row: libc::c_int,
        pub old_col: colnr_T,
        pub row: libc::c_int,
        pub col: colnr_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:9"]
    pub struct ExtmarkMove {
        pub start_row: libc::c_int,
        pub start_col: libc::c_int,
        pub extent_row: libc::c_int,
        pub extent_col: libc::c_int,
        pub new_row: libc::c_int,
        pub new_col: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "22:9"]
    pub struct ExtmarkSplice {
        pub start_row: libc::c_int,
        pub start_col: colnr_T,
        pub oldextent_row: libc::c_int,
        pub oldextent_col: colnr_T,
        pub newextent_row: libc::c_int,
        pub newextent_col: colnr_T,
    }
    #[c2rust::src_loc = "50:9"]
    pub type UndoObjectType = libc::c_uint;
    #[c2rust::src_loc = "55:3"]
    pub const kExtmarkClear: UndoObjectType = 4;
    #[c2rust::src_loc = "54:3"]
    pub const kExtmarkSavePos: UndoObjectType = 3;
    #[c2rust::src_loc = "53:3"]
    pub const kExtmarkUpdate: UndoObjectType = 2;
    #[c2rust::src_loc = "52:3"]
    pub const kExtmarkMove: UndoObjectType = 1;
    #[c2rust::src_loc = "51:3"]
    pub const kExtmarkSplice: UndoObjectType = 0;
    use super::stdint_uintn_h::uint64_t;
    use super::pos_h::colnr_T;
    // NVIM_EXTMARK_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/os/fs_defs.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/memline_defs.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/memfile_defs.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/highlight_defs.h:13"]
pub mod highlight_defs_h {
    // / Values for index in highlight_attr[].
// / When making changes, also update hlf_names below!
    #[c2rust::src_loc = "48:9"]
    pub type C2RustUnnamed_13 = libc::c_uint;
    // MUST be the last one
    // Message area
    #[c2rust::src_loc = "101:5"]
    pub const HLF_COUNT: C2RustUnnamed_13 = 50;
    // Floating window
    #[c2rust::src_loc = "100:5"]
    pub const HLF_MSG: C2RustUnnamed_13 = 49;
    // message separator line
    #[c2rust::src_loc = "99:5"]
    pub const HLF_NFLOAT: C2RustUnnamed_13 = 48;
    // NormalNC: Normal text in non-current windows
    #[c2rust::src_loc = "98:5"]
    pub const HLF_MSGSEP: C2RustUnnamed_13 = 47;
    // Whitespace
    #[c2rust::src_loc = "97:5"]
    pub const HLF_INACTIVE: C2RustUnnamed_13 = 46;
    // selected quickfix line
    #[c2rust::src_loc = "96:5"]
    pub const HLF_0: C2RustUnnamed_13 = 45;
    // 'colorcolumn'
    #[c2rust::src_loc = "95:5"]
    pub const HLF_QFL: C2RustUnnamed_13 = 44;
    // 'cursorline'
    #[c2rust::src_loc = "94:5"]
    pub const HLF_MC: C2RustUnnamed_13 = 43;
    // 'cursorcolumn'
    #[c2rust::src_loc = "93:5"]
    pub const HLF_CUL: C2RustUnnamed_13 = 42;
    // tabpage line filler
    #[c2rust::src_loc = "92:5"]
    pub const HLF_CUC: C2RustUnnamed_13 = 41;
    // tabpage line selected
    #[c2rust::src_loc = "91:5"]
    pub const HLF_TPF: C2RustUnnamed_13 = 40;
    // tabpage line
    #[c2rust::src_loc = "90:5"]
    pub const HLF_TPS: C2RustUnnamed_13 = 39;
    // popup menu scrollbar thumb
    #[c2rust::src_loc = "89:5"]
    pub const HLF_TP: C2RustUnnamed_13 = 38;
    // popup menu scrollbar
    #[c2rust::src_loc = "88:5"]
    pub const HLF_PST: C2RustUnnamed_13 = 37;
    // popup menu selected item
    #[c2rust::src_loc = "87:5"]
    pub const HLF_PSB: C2RustUnnamed_13 = 36;
    // popup menu normal item
    #[c2rust::src_loc = "86:5"]
    pub const HLF_PSI: C2RustUnnamed_13 = 35;
    // SpellLocal
    #[c2rust::src_loc = "85:5"]
    pub const HLF_PNI: C2RustUnnamed_13 = 34;
    // SpellRare
    #[c2rust::src_loc = "84:5"]
    pub const HLF_SPL: C2RustUnnamed_13 = 33;
    // SpellCap
    #[c2rust::src_loc = "83:5"]
    pub const HLF_SPR: C2RustUnnamed_13 = 32;
    // SpellBad
    #[c2rust::src_loc = "82:5"]
    pub const HLF_SPC: C2RustUnnamed_13 = 31;
    // Concealed text
    #[c2rust::src_loc = "81:5"]
    pub const HLF_SPB: C2RustUnnamed_13 = 30;
    // Sign column
    #[c2rust::src_loc = "80:5"]
    pub const HLF_CONCEAL: C2RustUnnamed_13 = 29;
    // Text Changed in diff line
    #[c2rust::src_loc = "79:5"]
    pub const HLF_SC: C2RustUnnamed_13 = 28;
    // Deleted diff line
    #[c2rust::src_loc = "78:5"]
    pub const HLF_TXD: C2RustUnnamed_13 = 27;
    // Changed diff line
    #[c2rust::src_loc = "77:5"]
    pub const HLF_DED: C2RustUnnamed_13 = 26;
    // Added diff line
    #[c2rust::src_loc = "76:5"]
    pub const HLF_CHD: C2RustUnnamed_13 = 25;
    // Fold column
    #[c2rust::src_loc = "75:5"]
    pub const HLF_ADD: C2RustUnnamed_13 = 24;
    // Folded line
    #[c2rust::src_loc = "74:5"]
    pub const HLF_FC: C2RustUnnamed_13 = 23;
    // Wildmenu highlight
    #[c2rust::src_loc = "73:5"]
    pub const HLF_FL: C2RustUnnamed_13 = 22;
    // warning messages
    #[c2rust::src_loc = "72:5"]
    pub const HLF_WM: C2RustUnnamed_13 = 21;
    // Visual mode, autoselecting and not clipboard owner
    #[c2rust::src_loc = "71:5"]
    pub const HLF_W: C2RustUnnamed_13 = 20;
    // Visual mode
    #[c2rust::src_loc = "70:5"]
    pub const HLF_VNC: C2RustUnnamed_13 = 19;
    // Titles for output from ":set all", ":autocmd" etc.
    #[c2rust::src_loc = "69:5"]
    pub const HLF_V: C2RustUnnamed_13 = 18;
    // column to separate vertically split windows
    #[c2rust::src_loc = "68:5"]
    pub const HLF_T: C2RustUnnamed_13 = 17;
    // status lines of not-current windows
    #[c2rust::src_loc = "67:5"]
    pub const HLF_C: C2RustUnnamed_13 = 16;
    // status lines
    #[c2rust::src_loc = "66:5"]
    pub const HLF_SNC: C2RustUnnamed_13 = 15;
    // return to continue message and yes/no questions
    #[c2rust::src_loc = "65:5"]
    pub const HLF_S: C2RustUnnamed_13 = 14;
    // current line number
    #[c2rust::src_loc = "64:5"]
    pub const HLF_R: C2RustUnnamed_13 = 13;
    // line number for ":number" and ":#" commands
    #[c2rust::src_loc = "63:5"]
    pub const HLF_CLN: C2RustUnnamed_13 = 12;
    // Mode (e.g., "-- INSERT --")
    #[c2rust::src_loc = "62:5"]
    pub const HLF_N: C2RustUnnamed_13 = 11;
    // "--More--" message
    #[c2rust::src_loc = "61:5"]
    pub const HLF_CM: C2RustUnnamed_13 = 10;
    // last search string
    #[c2rust::src_loc = "60:5"]
    pub const HLF_M: C2RustUnnamed_13 = 9;
    // incremental search
    #[c2rust::src_loc = "59:5"]
    pub const HLF_L: C2RustUnnamed_13 = 8;
    // error messages
    #[c2rust::src_loc = "58:5"]
    pub const HLF_I: C2RustUnnamed_13 = 7;
    // directories in CTRL-D listing
    #[c2rust::src_loc = "57:5"]
    pub const HLF_E: C2RustUnnamed_13 = 6;
    // @ characters at end of screen, characters that
                    // don't really exist in the text
    #[c2rust::src_loc = "56:5"]
    pub const HLF_D: C2RustUnnamed_13 = 5;
    // terminal cursor unfocused
    #[c2rust::src_loc = "54:5"]
    pub const HLF_AT: C2RustUnnamed_13 = 4;
    // terminal cursor focused
    #[c2rust::src_loc = "53:5"]
    pub const HLF_TERMNC: C2RustUnnamed_13 = 3;
    // after the last line in the buffer
    #[c2rust::src_loc = "52:5"]
    pub const HLF_TERM: C2RustUnnamed_13 = 2;
    // Meta & special keys listed with ":map", text that is
                    // displayed different from what it is
    #[c2rust::src_loc = "51:5"]
    pub const HLF_EOB: C2RustUnnamed_13 = 1;
    #[c2rust::src_loc = "49:3"]
    pub const HLF_8: C2RustUnnamed_13 = 0;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "158:12"]
        pub static mut highlight_attr: [libc::c_int; 50];
    }
    // NVIM_HIGHLIGHT_DEFS_H
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/types/struct_iovec.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/.deps/usr/include/uv.h:13"]
pub mod uv_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1693:8"]
    pub struct uv_loop_s {
        pub data: *mut libc::c_void,
        pub active_handles: libc::c_uint,
        pub handle_queue: [*mut libc::c_void; 2],
        pub active_reqs: C2RustUnnamed_20,
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
        pub timer_heap: C2RustUnnamed_18,
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
        pub u: C2RustUnnamed_17,
        pub next_closing: *mut uv_handle_t,
        pub flags: libc::c_uint,
        pub signal_cb: uv_signal_cb,
        pub signum: libc::c_int,
        pub tree_entry: C2RustUnnamed_15,
        pub caught_signals: libc::c_uint,
        pub dispatched_signals: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1512:3"]
    pub struct C2RustUnnamed_15 {
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
        pub u: C2RustUnnamed_16,
        pub next_closing: *mut uv_handle_t,
        pub flags: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "436:3"]
    pub union C2RustUnnamed_16 {
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
    pub union C2RustUnnamed_17 {
        pub fd: libc::c_int,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1705:3"]
    pub struct C2RustUnnamed_18 {
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
        pub u: C2RustUnnamed_19,
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
    pub union C2RustUnnamed_19 {
        pub fd: libc::c_int,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1699:3"]
    pub union C2RustUnnamed_20 {
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
        pub u: C2RustUnnamed_21,
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
    pub union C2RustUnnamed_21 {
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
        pub u: C2RustUnnamed_22,
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
    pub union C2RustUnnamed_22 {
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
        pub u: C2RustUnnamed_23,
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
    pub union C2RustUnnamed_23 {
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
        pub u: C2RustUnnamed_24,
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
    pub union C2RustUnnamed_24 {
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
        pub u: C2RustUnnamed_25,
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
    pub union C2RustUnnamed_25 {
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
        pub u: C2RustUnnamed_26,
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
    pub union C2RustUnnamed_26 {
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
        pub data: C2RustUnnamed_27,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "903:3"]
    pub union C2RustUnnamed_27 {
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
#[c2rust::header_src = "/home/vole/neovim/.deps/usr/include/uv/unix.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/normal.h:13"]
pub mod normal_h {
    /* Values for find_ident_under_cursor() */
    /* find identifier (word) */
    /* find any string (WORD) */
    /* include "->", "[]" and "." */
    // / Motion types, used for operators and for yank/delete registers.
// /
// / The three valid numerical values must not be changed, as they
// / are used in external communication and serialization.
    #[c2rust::src_loc = "17:9"]
    pub type MotionType = libc::c_int;
    // /< Unknown or invalid motion type
    // /< block-wise movement/register
    #[c2rust::src_loc = "21:3"]
    pub const kMTUnknown: MotionType = -1;
    // /< line-wise movement/register
    #[c2rust::src_loc = "20:3"]
    pub const kMTBlockWise: MotionType = 2;
    // /< character-wise movement/register
    #[c2rust::src_loc = "19:3"]
    pub const kMTLineWise: MotionType = 1;
    #[c2rust::src_loc = "18:3"]
    pub const kMTCharWise: MotionType = 0;
    /*
 * Arguments for operators.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:16"]
    pub struct oparg_S {
        pub op_type: libc::c_int,
        pub regname: libc::c_int,
        pub motion_type: MotionType,
        pub motion_force: libc::c_int,
        pub use_reg_one: bool,
        pub inclusive: bool,
        pub end_adjusted: bool,
        pub start: pos_T,
        pub end: pos_T,
        pub cursor_start: pos_T,
        pub line_count: libc::c_long,
        pub empty: bool,
        pub is_VIsual: bool,
        pub start_vcol: colnr_T,
        pub end_vcol: colnr_T,
        pub prev_opcount: libc::c_long,
        pub prev_count0: libc::c_long,
    }
    #[c2rust::src_loc = "27:1"]
    pub type oparg_T = oparg_S;
    use super::pos_h::{pos_T, colnr_T};
    // ca.count0 saved for K_EVENT
    // NVIM_NORMAL_H
    /* don't adjust operator end */
    /* skip restarting edit() once */
    /* values for retval: */
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/ex_cmds_enum.generated.h:13"]
pub mod ex_cmds_enum_generated_h {
    #[c2rust::src_loc = "1:9"]
    pub type CMD_index = libc::c_int;
    #[c2rust::src_loc = "544:3"]
    pub const CMD_USER_BUF: CMD_index = -2;
    #[c2rust::src_loc = "543:3"]
    pub const CMD_USER: CMD_index = -1;
    #[c2rust::src_loc = "542:3"]
    pub const CMD_SIZE: CMD_index = 540;
    #[c2rust::src_loc = "541:3"]
    pub const CMD_tilde: CMD_index = 539;
    #[c2rust::src_loc = "540:3"]
    pub const CMD_Next: CMD_index = 538;
    #[c2rust::src_loc = "539:3"]
    pub const CMD_at: CMD_index = 537;
    #[c2rust::src_loc = "538:3"]
    pub const CMD_rshift: CMD_index = 536;
    #[c2rust::src_loc = "537:3"]
    pub const CMD_equal: CMD_index = 535;
    #[c2rust::src_loc = "536:3"]
    pub const CMD_lshift: CMD_index = 534;
    #[c2rust::src_loc = "535:3"]
    pub const CMD_and: CMD_index = 533;
    #[c2rust::src_loc = "534:3"]
    pub const CMD_pound: CMD_index = 532;
    #[c2rust::src_loc = "533:3"]
    pub const CMD_bang: CMD_index = 531;
    #[c2rust::src_loc = "532:3"]
    pub const CMD_z: CMD_index = 530;
    #[c2rust::src_loc = "531:3"]
    pub const CMD_yank: CMD_index = 529;
    #[c2rust::src_loc = "530:3"]
    pub const CMD_xunmenu: CMD_index = 528;
    #[c2rust::src_loc = "529:3"]
    pub const CMD_xunmap: CMD_index = 527;
    #[c2rust::src_loc = "528:3"]
    pub const CMD_xnoremenu: CMD_index = 526;
    #[c2rust::src_loc = "527:3"]
    pub const CMD_xnoremap: CMD_index = 525;
    #[c2rust::src_loc = "526:3"]
    pub const CMD_xmenu: CMD_index = 524;
    #[c2rust::src_loc = "525:3"]
    pub const CMD_xmapclear: CMD_index = 523;
    #[c2rust::src_loc = "524:3"]
    pub const CMD_xmap: CMD_index = 522;
    #[c2rust::src_loc = "523:3"]
    pub const CMD_xall: CMD_index = 521;
    #[c2rust::src_loc = "522:3"]
    pub const CMD_xit: CMD_index = 520;
    #[c2rust::src_loc = "521:3"]
    pub const CMD_wviminfo: CMD_index = 519;
    #[c2rust::src_loc = "520:3"]
    pub const CMD_wundo: CMD_index = 518;
    #[c2rust::src_loc = "519:3"]
    pub const CMD_wshada: CMD_index = 517;
    #[c2rust::src_loc = "518:3"]
    pub const CMD_wqall: CMD_index = 516;
    #[c2rust::src_loc = "517:3"]
    pub const CMD_wq: CMD_index = 515;
    #[c2rust::src_loc = "516:3"]
    pub const CMD_wprevious: CMD_index = 514;
    #[c2rust::src_loc = "515:3"]
    pub const CMD_wnext: CMD_index = 513;
    #[c2rust::src_loc = "514:3"]
    pub const CMD_winpos: CMD_index = 512;
    #[c2rust::src_loc = "513:3"]
    pub const CMD_windo: CMD_index = 511;
    #[c2rust::src_loc = "512:3"]
    pub const CMD_wincmd: CMD_index = 510;
    #[c2rust::src_loc = "511:3"]
    pub const CMD_winsize: CMD_index = 509;
    #[c2rust::src_loc = "510:3"]
    pub const CMD_while: CMD_index = 508;
    #[c2rust::src_loc = "509:3"]
    pub const CMD_wall: CMD_index = 507;
    #[c2rust::src_loc = "508:3"]
    pub const CMD_wNext: CMD_index = 506;
    #[c2rust::src_loc = "507:3"]
    pub const CMD_write: CMD_index = 505;
    #[c2rust::src_loc = "506:3"]
    pub const CMD_vunmenu: CMD_index = 504;
    #[c2rust::src_loc = "505:3"]
    pub const CMD_vunmap: CMD_index = 503;
    #[c2rust::src_loc = "504:3"]
    pub const CMD_vsplit: CMD_index = 502;
    #[c2rust::src_loc = "503:3"]
    pub const CMD_vnoremenu: CMD_index = 501;
    #[c2rust::src_loc = "502:3"]
    pub const CMD_vnew: CMD_index = 500;
    #[c2rust::src_loc = "501:3"]
    pub const CMD_vnoremap: CMD_index = 499;
    #[c2rust::src_loc = "500:3"]
    pub const CMD_vmenu: CMD_index = 498;
    #[c2rust::src_loc = "499:3"]
    pub const CMD_vmapclear: CMD_index = 497;
    #[c2rust::src_loc = "498:3"]
    pub const CMD_vmap: CMD_index = 496;
    #[c2rust::src_loc = "497:3"]
    pub const CMD_viusage: CMD_index = 495;
    #[c2rust::src_loc = "496:3"]
    pub const CMD_vimgrepadd: CMD_index = 494;
    #[c2rust::src_loc = "495:3"]
    pub const CMD_vimgrep: CMD_index = 493;
    #[c2rust::src_loc = "494:3"]
    pub const CMD_view: CMD_index = 492;
    #[c2rust::src_loc = "493:3"]
    pub const CMD_visual: CMD_index = 491;
    #[c2rust::src_loc = "492:3"]
    pub const CMD_vertical: CMD_index = 490;
    #[c2rust::src_loc = "491:3"]
    pub const CMD_verbose: CMD_index = 489;
    #[c2rust::src_loc = "490:3"]
    pub const CMD_version: CMD_index = 488;
    #[c2rust::src_loc = "489:3"]
    pub const CMD_vglobal: CMD_index = 487;
    #[c2rust::src_loc = "488:3"]
    pub const CMD_update: CMD_index = 486;
    #[c2rust::src_loc = "487:3"]
    pub const CMD_unsilent: CMD_index = 485;
    #[c2rust::src_loc = "486:3"]
    pub const CMD_unmenu: CMD_index = 484;
    #[c2rust::src_loc = "485:3"]
    pub const CMD_unmap: CMD_index = 483;
    #[c2rust::src_loc = "484:3"]
    pub const CMD_unlockvar: CMD_index = 482;
    #[c2rust::src_loc = "483:3"]
    pub const CMD_unlet: CMD_index = 481;
    #[c2rust::src_loc = "482:3"]
    pub const CMD_unhide: CMD_index = 480;
    #[c2rust::src_loc = "481:3"]
    pub const CMD_unabbreviate: CMD_index = 479;
    #[c2rust::src_loc = "480:3"]
    pub const CMD_undolist: CMD_index = 478;
    #[c2rust::src_loc = "479:3"]
    pub const CMD_undojoin: CMD_index = 477;
    #[c2rust::src_loc = "478:3"]
    pub const CMD_undo: CMD_index = 476;
    #[c2rust::src_loc = "477:3"]
    pub const CMD_tunmap: CMD_index = 475;
    #[c2rust::src_loc = "476:3"]
    pub const CMD_tunmenu: CMD_index = 474;
    #[c2rust::src_loc = "475:3"]
    pub const CMD_tselect: CMD_index = 473;
    #[c2rust::src_loc = "474:3"]
    pub const CMD_try: CMD_index = 472;
    #[c2rust::src_loc = "473:3"]
    pub const CMD_trewind: CMD_index = 471;
    #[c2rust::src_loc = "472:3"]
    pub const CMD_tprevious: CMD_index = 470;
    #[c2rust::src_loc = "471:3"]
    pub const CMD_topleft: CMD_index = 469;
    #[c2rust::src_loc = "470:3"]
    pub const CMD_tnoremap: CMD_index = 468;
    #[c2rust::src_loc = "469:3"]
    pub const CMD_tnext: CMD_index = 467;
    #[c2rust::src_loc = "468:3"]
    pub const CMD_tmapclear: CMD_index = 466;
    #[c2rust::src_loc = "467:3"]
    pub const CMD_tmap: CMD_index = 465;
    #[c2rust::src_loc = "466:3"]
    pub const CMD_tmenu: CMD_index = 464;
    #[c2rust::src_loc = "465:3"]
    pub const CMD_tlast: CMD_index = 463;
    #[c2rust::src_loc = "464:3"]
    pub const CMD_tjump: CMD_index = 462;
    #[c2rust::src_loc = "463:3"]
    pub const CMD_throw: CMD_index = 461;
    #[c2rust::src_loc = "462:3"]
    pub const CMD_tfirst: CMD_index = 460;
    #[c2rust::src_loc = "461:3"]
    pub const CMD_terminal: CMD_index = 459;
    #[c2rust::src_loc = "460:3"]
    pub const CMD_tclfile: CMD_index = 458;
    #[c2rust::src_loc = "459:3"]
    pub const CMD_tcldo: CMD_index = 457;
    #[c2rust::src_loc = "458:3"]
    pub const CMD_tcl: CMD_index = 456;
    #[c2rust::src_loc = "457:3"]
    pub const CMD_tabs: CMD_index = 455;
    #[c2rust::src_loc = "456:3"]
    pub const CMD_tabrewind: CMD_index = 454;
    #[c2rust::src_loc = "455:3"]
    pub const CMD_tabNext: CMD_index = 453;
    #[c2rust::src_loc = "454:3"]
    pub const CMD_tabprevious: CMD_index = 452;
    #[c2rust::src_loc = "453:3"]
    pub const CMD_tabonly: CMD_index = 451;
    #[c2rust::src_loc = "452:3"]
    pub const CMD_tabnew: CMD_index = 450;
    #[c2rust::src_loc = "451:3"]
    pub const CMD_tabnext: CMD_index = 449;
    #[c2rust::src_loc = "450:3"]
    pub const CMD_tablast: CMD_index = 448;
    #[c2rust::src_loc = "449:3"]
    pub const CMD_tabmove: CMD_index = 447;
    #[c2rust::src_loc = "448:3"]
    pub const CMD_tabfirst: CMD_index = 446;
    #[c2rust::src_loc = "447:3"]
    pub const CMD_tabfind: CMD_index = 445;
    #[c2rust::src_loc = "446:3"]
    pub const CMD_tabedit: CMD_index = 444;
    #[c2rust::src_loc = "445:3"]
    pub const CMD_tabdo: CMD_index = 443;
    #[c2rust::src_loc = "444:3"]
    pub const CMD_tabclose: CMD_index = 442;
    #[c2rust::src_loc = "443:3"]
    pub const CMD_tab: CMD_index = 441;
    #[c2rust::src_loc = "442:3"]
    pub const CMD_tags: CMD_index = 440;
    #[c2rust::src_loc = "441:3"]
    pub const CMD_tag: CMD_index = 439;
    #[c2rust::src_loc = "440:3"]
    pub const CMD_tNext: CMD_index = 438;
    #[c2rust::src_loc = "439:3"]
    pub const CMD_tchdir: CMD_index = 437;
    #[c2rust::src_loc = "438:3"]
    pub const CMD_tcd: CMD_index = 436;
    #[c2rust::src_loc = "437:3"]
    pub const CMD_t: CMD_index = 435;
    #[c2rust::src_loc = "436:3"]
    pub const CMD_syncbind: CMD_index = 434;
    #[c2rust::src_loc = "435:3"]
    pub const CMD_syntime: CMD_index = 433;
    #[c2rust::src_loc = "434:3"]
    pub const CMD_syntax: CMD_index = 432;
    #[c2rust::src_loc = "433:3"]
    pub const CMD_swapname: CMD_index = 431;
    #[c2rust::src_loc = "432:3"]
    pub const CMD_sview: CMD_index = 430;
    #[c2rust::src_loc = "431:3"]
    pub const CMD_suspend: CMD_index = 429;
    #[c2rust::src_loc = "430:3"]
    pub const CMD_sunmenu: CMD_index = 428;
    #[c2rust::src_loc = "429:3"]
    pub const CMD_sunmap: CMD_index = 427;
    #[c2rust::src_loc = "428:3"]
    pub const CMD_sunhide: CMD_index = 426;
    #[c2rust::src_loc = "427:3"]
    pub const CMD_stselect: CMD_index = 425;
    #[c2rust::src_loc = "426:3"]
    pub const CMD_stjump: CMD_index = 424;
    #[c2rust::src_loc = "425:3"]
    pub const CMD_stopinsert: CMD_index = 423;
    #[c2rust::src_loc = "424:3"]
    pub const CMD_startreplace: CMD_index = 422;
    #[c2rust::src_loc = "423:3"]
    pub const CMD_startgreplace: CMD_index = 421;
    #[c2rust::src_loc = "422:3"]
    pub const CMD_startinsert: CMD_index = 420;
    #[c2rust::src_loc = "421:3"]
    pub const CMD_stag: CMD_index = 419;
    #[c2rust::src_loc = "420:3"]
    pub const CMD_stop: CMD_index = 418;
    #[c2rust::src_loc = "419:3"]
    pub const CMD_srewind: CMD_index = 417;
    #[c2rust::src_loc = "418:3"]
    pub const CMD_sprevious: CMD_index = 416;
    #[c2rust::src_loc = "417:3"]
    pub const CMD_spellwrong: CMD_index = 415;
    #[c2rust::src_loc = "416:3"]
    pub const CMD_spellundo: CMD_index = 414;
    #[c2rust::src_loc = "415:3"]
    pub const CMD_spellrepall: CMD_index = 413;
    #[c2rust::src_loc = "414:3"]
    pub const CMD_spellinfo: CMD_index = 412;
    #[c2rust::src_loc = "413:3"]
    pub const CMD_spelldump: CMD_index = 411;
    #[c2rust::src_loc = "412:3"]
    pub const CMD_spellgood: CMD_index = 410;
    #[c2rust::src_loc = "411:3"]
    pub const CMD_split: CMD_index = 409;
    #[c2rust::src_loc = "410:3"]
    pub const CMD_sort: CMD_index = 408;
    #[c2rust::src_loc = "409:3"]
    pub const CMD_source: CMD_index = 407;
    #[c2rust::src_loc = "408:3"]
    pub const CMD_snoremenu: CMD_index = 406;
    #[c2rust::src_loc = "407:3"]
    pub const CMD_snoremap: CMD_index = 405;
    #[c2rust::src_loc = "406:3"]
    pub const CMD_snomagic: CMD_index = 404;
    #[c2rust::src_loc = "405:3"]
    pub const CMD_snext: CMD_index = 403;
    #[c2rust::src_loc = "404:3"]
    pub const CMD_smenu: CMD_index = 402;
    #[c2rust::src_loc = "403:3"]
    pub const CMD_smapclear: CMD_index = 401;
    #[c2rust::src_loc = "402:3"]
    pub const CMD_smap: CMD_index = 400;
    #[c2rust::src_loc = "401:3"]
    pub const CMD_smagic: CMD_index = 399;
    #[c2rust::src_loc = "400:3"]
    pub const CMD_slast: CMD_index = 398;
    #[c2rust::src_loc = "399:3"]
    pub const CMD_sleep: CMD_index = 397;
    #[c2rust::src_loc = "398:3"]
    pub const CMD_silent: CMD_index = 396;
    #[c2rust::src_loc = "397:3"]
    pub const CMD_sign: CMD_index = 395;
    #[c2rust::src_loc = "396:3"]
    pub const CMD_simalt: CMD_index = 394;
    #[c2rust::src_loc = "395:3"]
    pub const CMD_sfirst: CMD_index = 393;
    #[c2rust::src_loc = "394:3"]
    pub const CMD_sfind: CMD_index = 392;
    #[c2rust::src_loc = "393:3"]
    pub const CMD_setlocal: CMD_index = 391;
    #[c2rust::src_loc = "392:3"]
    pub const CMD_setglobal: CMD_index = 390;
    #[c2rust::src_loc = "391:3"]
    pub const CMD_setfiletype: CMD_index = 389;
    #[c2rust::src_loc = "390:3"]
    pub const CMD_set: CMD_index = 388;
    #[c2rust::src_loc = "389:3"]
    pub const CMD_scscope: CMD_index = 387;
    #[c2rust::src_loc = "388:3"]
    pub const CMD_scriptencoding: CMD_index = 386;
    #[c2rust::src_loc = "387:3"]
    pub const CMD_scriptnames: CMD_index = 385;
    #[c2rust::src_loc = "386:3"]
    pub const CMD_sbrewind: CMD_index = 384;
    #[c2rust::src_loc = "385:3"]
    pub const CMD_sbprevious: CMD_index = 383;
    #[c2rust::src_loc = "384:3"]
    pub const CMD_sbnext: CMD_index = 382;
    #[c2rust::src_loc = "383:3"]
    pub const CMD_sbmodified: CMD_index = 381;
    #[c2rust::src_loc = "382:3"]
    pub const CMD_sblast: CMD_index = 380;
    #[c2rust::src_loc = "381:3"]
    pub const CMD_sbfirst: CMD_index = 379;
    #[c2rust::src_loc = "380:3"]
    pub const CMD_sball: CMD_index = 378;
    #[c2rust::src_loc = "379:3"]
    pub const CMD_sbNext: CMD_index = 377;
    #[c2rust::src_loc = "378:3"]
    pub const CMD_sbuffer: CMD_index = 376;
    #[c2rust::src_loc = "377:3"]
    pub const CMD_saveas: CMD_index = 375;
    #[c2rust::src_loc = "376:3"]
    pub const CMD_sandbox: CMD_index = 374;
    #[c2rust::src_loc = "375:3"]
    pub const CMD_sall: CMD_index = 373;
    #[c2rust::src_loc = "374:3"]
    pub const CMD_sargument: CMD_index = 372;
    #[c2rust::src_loc = "373:3"]
    pub const CMD_sNext: CMD_index = 371;
    #[c2rust::src_loc = "372:3"]
    pub const CMD_substitute: CMD_index = 370;
    #[c2rust::src_loc = "371:3"]
    pub const CMD_rviminfo: CMD_index = 369;
    #[c2rust::src_loc = "370:3"]
    pub const CMD_rubyfile: CMD_index = 368;
    #[c2rust::src_loc = "369:3"]
    pub const CMD_rubydo: CMD_index = 367;
    #[c2rust::src_loc = "368:3"]
    pub const CMD_ruby: CMD_index = 366;
    #[c2rust::src_loc = "367:3"]
    pub const CMD_rundo: CMD_index = 365;
    #[c2rust::src_loc = "366:3"]
    pub const CMD_runtime: CMD_index = 364;
    #[c2rust::src_loc = "365:3"]
    pub const CMD_rshada: CMD_index = 363;
    #[c2rust::src_loc = "364:3"]
    pub const CMD_rightbelow: CMD_index = 362;
    #[c2rust::src_loc = "363:3"]
    pub const CMD_right: CMD_index = 361;
    #[c2rust::src_loc = "362:3"]
    pub const CMD_rewind: CMD_index = 360;
    #[c2rust::src_loc = "361:3"]
    pub const CMD_return: CMD_index = 359;
    #[c2rust::src_loc = "360:3"]
    pub const CMD_retab: CMD_index = 358;
    #[c2rust::src_loc = "359:3"]
    pub const CMD_resize: CMD_index = 357;
    #[c2rust::src_loc = "358:3"]
    pub const CMD_registers: CMD_index = 356;
    #[c2rust::src_loc = "357:3"]
    pub const CMD_redrawtabline: CMD_index = 355;
    #[c2rust::src_loc = "356:3"]
    pub const CMD_redrawstatus: CMD_index = 354;
    #[c2rust::src_loc = "355:3"]
    pub const CMD_redraw: CMD_index = 353;
    #[c2rust::src_loc = "354:3"]
    pub const CMD_redir: CMD_index = 352;
    #[c2rust::src_loc = "353:3"]
    pub const CMD_redo: CMD_index = 351;
    #[c2rust::src_loc = "352:3"]
    pub const CMD_recover: CMD_index = 350;
    #[c2rust::src_loc = "351:3"]
    pub const CMD_read: CMD_index = 349;
    #[c2rust::src_loc = "350:3"]
    pub const CMD_qall: CMD_index = 348;
    #[c2rust::src_loc = "349:3"]
    pub const CMD_quitall: CMD_index = 347;
    #[c2rust::src_loc = "348:3"]
    pub const CMD_quit: CMD_index = 346;
    #[c2rust::src_loc = "347:3"]
    pub const CMD_pyxfile: CMD_index = 345;
    #[c2rust::src_loc = "346:3"]
    pub const CMD_pythonx: CMD_index = 344;
    #[c2rust::src_loc = "345:3"]
    pub const CMD_pyxdo: CMD_index = 343;
    #[c2rust::src_loc = "344:3"]
    pub const CMD_pyx: CMD_index = 342;
    #[c2rust::src_loc = "343:3"]
    pub const CMD_py3file: CMD_index = 341;
    #[c2rust::src_loc = "342:3"]
    pub const CMD_python3: CMD_index = 340;
    #[c2rust::src_loc = "341:3"]
    pub const CMD_py3do: CMD_index = 339;
    #[c2rust::src_loc = "340:3"]
    pub const CMD_py3: CMD_index = 338;
    #[c2rust::src_loc = "339:3"]
    pub const CMD_pyfile: CMD_index = 337;
    #[c2rust::src_loc = "338:3"]
    pub const CMD_pydo: CMD_index = 336;
    #[c2rust::src_loc = "337:3"]
    pub const CMD_python: CMD_index = 335;
    #[c2rust::src_loc = "336:3"]
    pub const CMD_pwd: CMD_index = 334;
    #[c2rust::src_loc = "335:3"]
    pub const CMD_put: CMD_index = 333;
    #[c2rust::src_loc = "334:3"]
    pub const CMD_ptselect: CMD_index = 332;
    #[c2rust::src_loc = "333:3"]
    pub const CMD_ptrewind: CMD_index = 331;
    #[c2rust::src_loc = "332:3"]
    pub const CMD_ptprevious: CMD_index = 330;
    #[c2rust::src_loc = "331:3"]
    pub const CMD_ptnext: CMD_index = 329;
    #[c2rust::src_loc = "330:3"]
    pub const CMD_ptlast: CMD_index = 328;
    #[c2rust::src_loc = "329:3"]
    pub const CMD_ptjump: CMD_index = 327;
    #[c2rust::src_loc = "328:3"]
    pub const CMD_ptfirst: CMD_index = 326;
    #[c2rust::src_loc = "327:3"]
    pub const CMD_ptNext: CMD_index = 325;
    #[c2rust::src_loc = "326:3"]
    pub const CMD_ptag: CMD_index = 324;
    #[c2rust::src_loc = "325:3"]
    pub const CMD_psearch: CMD_index = 323;
    #[c2rust::src_loc = "324:3"]
    pub const CMD_profdel: CMD_index = 322;
    #[c2rust::src_loc = "323:3"]
    pub const CMD_profile: CMD_index = 321;
    #[c2rust::src_loc = "322:3"]
    pub const CMD_previous: CMD_index = 320;
    #[c2rust::src_loc = "321:3"]
    pub const CMD_preserve: CMD_index = 319;
    #[c2rust::src_loc = "320:3"]
    pub const CMD_ppop: CMD_index = 318;
    #[c2rust::src_loc = "319:3"]
    pub const CMD_popup: CMD_index = 317;
    #[c2rust::src_loc = "318:3"]
    pub const CMD_pop: CMD_index = 316;
    #[c2rust::src_loc = "317:3"]
    pub const CMD_pedit: CMD_index = 315;
    #[c2rust::src_loc = "316:3"]
    pub const CMD_perldo: CMD_index = 314;
    #[c2rust::src_loc = "315:3"]
    pub const CMD_perl: CMD_index = 313;
    #[c2rust::src_loc = "314:3"]
    pub const CMD_pclose: CMD_index = 312;
    #[c2rust::src_loc = "313:3"]
    pub const CMD_packloadall: CMD_index = 311;
    #[c2rust::src_loc = "312:3"]
    pub const CMD_packadd: CMD_index = 310;
    #[c2rust::src_loc = "311:3"]
    pub const CMD_print: CMD_index = 309;
    #[c2rust::src_loc = "310:3"]
    pub const CMD_ownsyntax: CMD_index = 308;
    #[c2rust::src_loc = "309:3"]
    pub const CMD_ounmenu: CMD_index = 307;
    #[c2rust::src_loc = "308:3"]
    pub const CMD_ounmap: CMD_index = 306;
    #[c2rust::src_loc = "307:3"]
    pub const CMD_options: CMD_index = 305;
    #[c2rust::src_loc = "306:3"]
    pub const CMD_onoremenu: CMD_index = 304;
    #[c2rust::src_loc = "305:3"]
    pub const CMD_onoremap: CMD_index = 303;
    #[c2rust::src_loc = "304:3"]
    pub const CMD_only: CMD_index = 302;
    #[c2rust::src_loc = "303:3"]
    pub const CMD_omenu: CMD_index = 301;
    #[c2rust::src_loc = "302:3"]
    pub const CMD_omapclear: CMD_index = 300;
    #[c2rust::src_loc = "301:3"]
    pub const CMD_omap: CMD_index = 299;
    #[c2rust::src_loc = "300:3"]
    pub const CMD_oldfiles: CMD_index = 298;
    #[c2rust::src_loc = "299:3"]
    pub const CMD_nunmenu: CMD_index = 297;
    #[c2rust::src_loc = "298:3"]
    pub const CMD_nunmap: CMD_index = 296;
    #[c2rust::src_loc = "297:3"]
    pub const CMD_number: CMD_index = 295;
    #[c2rust::src_loc = "296:3"]
    pub const CMD_normal: CMD_index = 294;
    #[c2rust::src_loc = "295:3"]
    pub const CMD_noswapfile: CMD_index = 293;
    #[c2rust::src_loc = "294:3"]
    pub const CMD_noremenu: CMD_index = 292;
    #[c2rust::src_loc = "293:3"]
    pub const CMD_noreabbrev: CMD_index = 291;
    #[c2rust::src_loc = "292:3"]
    pub const CMD_nohlsearch: CMD_index = 290;
    #[c2rust::src_loc = "291:3"]
    pub const CMD_noautocmd: CMD_index = 289;
    #[c2rust::src_loc = "290:3"]
    pub const CMD_noremap: CMD_index = 288;
    #[c2rust::src_loc = "289:3"]
    pub const CMD_nnoremenu: CMD_index = 287;
    #[c2rust::src_loc = "288:3"]
    pub const CMD_nnoremap: CMD_index = 286;
    #[c2rust::src_loc = "287:3"]
    pub const CMD_nmenu: CMD_index = 285;
    #[c2rust::src_loc = "286:3"]
    pub const CMD_nmapclear: CMD_index = 284;
    #[c2rust::src_loc = "285:3"]
    pub const CMD_nmap: CMD_index = 283;
    #[c2rust::src_loc = "284:3"]
    pub const CMD_new: CMD_index = 282;
    #[c2rust::src_loc = "283:3"]
    pub const CMD_next: CMD_index = 281;
    #[c2rust::src_loc = "282:3"]
    pub const CMD_mzfile: CMD_index = 280;
    #[c2rust::src_loc = "281:3"]
    pub const CMD_mzscheme: CMD_index = 279;
    #[c2rust::src_loc = "280:3"]
    pub const CMD_mode: CMD_index = 278;
    #[c2rust::src_loc = "279:3"]
    pub const CMD_mkview: CMD_index = 277;
    #[c2rust::src_loc = "278:3"]
    pub const CMD_mkvimrc: CMD_index = 276;
    #[c2rust::src_loc = "277:3"]
    pub const CMD_mkspell: CMD_index = 275;
    #[c2rust::src_loc = "276:3"]
    pub const CMD_mksession: CMD_index = 274;
    #[c2rust::src_loc = "275:3"]
    pub const CMD_mkexrc: CMD_index = 273;
    #[c2rust::src_loc = "274:3"]
    pub const CMD_messages: CMD_index = 272;
    #[c2rust::src_loc = "273:3"]
    pub const CMD_menutranslate: CMD_index = 271;
    #[c2rust::src_loc = "272:3"]
    pub const CMD_menu: CMD_index = 270;
    #[c2rust::src_loc = "271:3"]
    pub const CMD_match: CMD_index = 269;
    #[c2rust::src_loc = "270:3"]
    pub const CMD_marks: CMD_index = 268;
    #[c2rust::src_loc = "269:3"]
    pub const CMD_mapclear: CMD_index = 267;
    #[c2rust::src_loc = "268:3"]
    pub const CMD_map: CMD_index = 266;
    #[c2rust::src_loc = "267:3"]
    pub const CMD_make: CMD_index = 265;
    #[c2rust::src_loc = "266:3"]
    pub const CMD_mark: CMD_index = 264;
    #[c2rust::src_loc = "265:3"]
    pub const CMD_move: CMD_index = 263;
    #[c2rust::src_loc = "264:3"]
    pub const CMD_ls: CMD_index = 262;
    #[c2rust::src_loc = "263:3"]
    pub const CMD_lwindow: CMD_index = 261;
    #[c2rust::src_loc = "262:3"]
    pub const CMD_lvimgrepadd: CMD_index = 260;
    #[c2rust::src_loc = "261:3"]
    pub const CMD_lvimgrep: CMD_index = 259;
    #[c2rust::src_loc = "260:3"]
    pub const CMD_luafile: CMD_index = 258;
    #[c2rust::src_loc = "259:3"]
    pub const CMD_luado: CMD_index = 257;
    #[c2rust::src_loc = "258:3"]
    pub const CMD_lua: CMD_index = 256;
    #[c2rust::src_loc = "257:3"]
    pub const CMD_lunmap: CMD_index = 255;
    #[c2rust::src_loc = "256:3"]
    pub const CMD_ltag: CMD_index = 254;
    #[c2rust::src_loc = "255:3"]
    pub const CMD_lrewind: CMD_index = 253;
    #[c2rust::src_loc = "254:3"]
    pub const CMD_lpfile: CMD_index = 252;
    #[c2rust::src_loc = "253:3"]
    pub const CMD_lprevious: CMD_index = 251;
    #[c2rust::src_loc = "252:3"]
    pub const CMD_lopen: CMD_index = 250;
    #[c2rust::src_loc = "251:3"]
    pub const CMD_lolder: CMD_index = 249;
    #[c2rust::src_loc = "250:3"]
    pub const CMD_lockvar: CMD_index = 248;
    #[c2rust::src_loc = "249:3"]
    pub const CMD_lockmarks: CMD_index = 247;
    #[c2rust::src_loc = "248:3"]
    pub const CMD_loadkeymap: CMD_index = 246;
    #[c2rust::src_loc = "247:3"]
    pub const CMD_loadview: CMD_index = 245;
    #[c2rust::src_loc = "246:3"]
    pub const CMD_lnfile: CMD_index = 244;
    #[c2rust::src_loc = "245:3"]
    pub const CMD_lnewer: CMD_index = 243;
    #[c2rust::src_loc = "244:3"]
    pub const CMD_lnext: CMD_index = 242;
    #[c2rust::src_loc = "243:3"]
    pub const CMD_lnoremap: CMD_index = 241;
    #[c2rust::src_loc = "242:3"]
    pub const CMD_lmake: CMD_index = 240;
    #[c2rust::src_loc = "241:3"]
    pub const CMD_lmapclear: CMD_index = 239;
    #[c2rust::src_loc = "240:3"]
    pub const CMD_lmap: CMD_index = 238;
    #[c2rust::src_loc = "239:3"]
    pub const CMD_llist: CMD_index = 237;
    #[c2rust::src_loc = "238:3"]
    pub const CMD_llast: CMD_index = 236;
    #[c2rust::src_loc = "237:3"]
    pub const CMD_ll: CMD_index = 235;
    #[c2rust::src_loc = "236:3"]
    pub const CMD_lhistory: CMD_index = 234;
    #[c2rust::src_loc = "235:3"]
    pub const CMD_lhelpgrep: CMD_index = 233;
    #[c2rust::src_loc = "234:3"]
    pub const CMD_lgrepadd: CMD_index = 232;
    #[c2rust::src_loc = "233:3"]
    pub const CMD_lgrep: CMD_index = 231;
    #[c2rust::src_loc = "232:3"]
    pub const CMD_lgetexpr: CMD_index = 230;
    #[c2rust::src_loc = "231:3"]
    pub const CMD_lgetbuffer: CMD_index = 229;
    #[c2rust::src_loc = "230:3"]
    pub const CMD_lgetfile: CMD_index = 228;
    #[c2rust::src_loc = "229:3"]
    pub const CMD_lfirst: CMD_index = 227;
    #[c2rust::src_loc = "228:3"]
    pub const CMD_lfdo: CMD_index = 226;
    #[c2rust::src_loc = "227:3"]
    pub const CMD_lfile: CMD_index = 225;
    #[c2rust::src_loc = "226:3"]
    pub const CMD_lexpr: CMD_index = 224;
    #[c2rust::src_loc = "225:3"]
    pub const CMD_let: CMD_index = 223;
    #[c2rust::src_loc = "224:3"]
    pub const CMD_leftabove: CMD_index = 222;
    #[c2rust::src_loc = "223:3"]
    pub const CMD_left: CMD_index = 221;
    #[c2rust::src_loc = "222:3"]
    pub const CMD_ldo: CMD_index = 220;
    #[c2rust::src_loc = "221:3"]
    pub const CMD_lcscope: CMD_index = 219;
    #[c2rust::src_loc = "220:3"]
    pub const CMD_lclose: CMD_index = 218;
    #[c2rust::src_loc = "219:3"]
    pub const CMD_lchdir: CMD_index = 217;
    #[c2rust::src_loc = "218:3"]
    pub const CMD_lcd: CMD_index = 216;
    #[c2rust::src_loc = "217:3"]
    pub const CMD_lbottom: CMD_index = 215;
    #[c2rust::src_loc = "216:3"]
    pub const CMD_lbelow: CMD_index = 214;
    #[c2rust::src_loc = "215:3"]
    pub const CMD_lbuffer: CMD_index = 213;
    #[c2rust::src_loc = "214:3"]
    pub const CMD_later: CMD_index = 212;
    #[c2rust::src_loc = "213:3"]
    pub const CMD_laddfile: CMD_index = 211;
    #[c2rust::src_loc = "212:3"]
    pub const CMD_laddbuffer: CMD_index = 210;
    #[c2rust::src_loc = "211:3"]
    pub const CMD_laddexpr: CMD_index = 209;
    #[c2rust::src_loc = "210:3"]
    pub const CMD_language: CMD_index = 208;
    #[c2rust::src_loc = "209:3"]
    pub const CMD_labove: CMD_index = 207;
    #[c2rust::src_loc = "208:3"]
    pub const CMD_last: CMD_index = 206;
    #[c2rust::src_loc = "207:3"]
    pub const CMD_lNfile: CMD_index = 205;
    #[c2rust::src_loc = "206:3"]
    pub const CMD_lNext: CMD_index = 204;
    #[c2rust::src_loc = "205:3"]
    pub const CMD_list: CMD_index = 203;
    #[c2rust::src_loc = "204:3"]
    pub const CMD_keepalt: CMD_index = 202;
    #[c2rust::src_loc = "203:3"]
    pub const CMD_keeppatterns: CMD_index = 201;
    #[c2rust::src_loc = "202:3"]
    pub const CMD_keepjumps: CMD_index = 200;
    #[c2rust::src_loc = "201:3"]
    pub const CMD_keepmarks: CMD_index = 199;
    #[c2rust::src_loc = "200:3"]
    pub const CMD_k: CMD_index = 198;
    #[c2rust::src_loc = "199:3"]
    pub const CMD_jumps: CMD_index = 197;
    #[c2rust::src_loc = "198:3"]
    pub const CMD_join: CMD_index = 196;
    #[c2rust::src_loc = "197:3"]
    pub const CMD_iunmenu: CMD_index = 195;
    #[c2rust::src_loc = "196:3"]
    pub const CMD_iunabbrev: CMD_index = 194;
    #[c2rust::src_loc = "195:3"]
    pub const CMD_iunmap: CMD_index = 193;
    #[c2rust::src_loc = "194:3"]
    pub const CMD_isplit: CMD_index = 192;
    #[c2rust::src_loc = "193:3"]
    pub const CMD_isearch: CMD_index = 191;
    #[c2rust::src_loc = "192:3"]
    pub const CMD_intro: CMD_index = 190;
    #[c2rust::src_loc = "191:3"]
    pub const CMD_inoremenu: CMD_index = 189;
    #[c2rust::src_loc = "190:3"]
    pub const CMD_inoreabbrev: CMD_index = 188;
    #[c2rust::src_loc = "189:3"]
    pub const CMD_inoremap: CMD_index = 187;
    #[c2rust::src_loc = "188:3"]
    pub const CMD_imenu: CMD_index = 186;
    #[c2rust::src_loc = "187:3"]
    pub const CMD_imapclear: CMD_index = 185;
    #[c2rust::src_loc = "186:3"]
    pub const CMD_imap: CMD_index = 184;
    #[c2rust::src_loc = "185:3"]
    pub const CMD_ilist: CMD_index = 183;
    #[c2rust::src_loc = "184:3"]
    pub const CMD_ijump: CMD_index = 182;
    #[c2rust::src_loc = "183:3"]
    pub const CMD_if: CMD_index = 181;
    #[c2rust::src_loc = "182:3"]
    pub const CMD_iabclear: CMD_index = 180;
    #[c2rust::src_loc = "181:3"]
    pub const CMD_iabbrev: CMD_index = 179;
    #[c2rust::src_loc = "180:3"]
    pub const CMD_insert: CMD_index = 178;
    #[c2rust::src_loc = "179:3"]
    pub const CMD_history: CMD_index = 177;
    #[c2rust::src_loc = "178:3"]
    pub const CMD_hide: CMD_index = 176;
    #[c2rust::src_loc = "177:3"]
    pub const CMD_highlight: CMD_index = 175;
    #[c2rust::src_loc = "176:3"]
    pub const CMD_hardcopy: CMD_index = 174;
    #[c2rust::src_loc = "175:3"]
    pub const CMD_helptags: CMD_index = 173;
    #[c2rust::src_loc = "174:3"]
    pub const CMD_helpgrep: CMD_index = 172;
    #[c2rust::src_loc = "173:3"]
    pub const CMD_helpclose: CMD_index = 171;
    #[c2rust::src_loc = "172:3"]
    pub const CMD_help: CMD_index = 170;
    #[c2rust::src_loc = "171:3"]
    pub const CMD_gvim: CMD_index = 169;
    #[c2rust::src_loc = "170:3"]
    pub const CMD_gui: CMD_index = 168;
    #[c2rust::src_loc = "169:3"]
    pub const CMD_grepadd: CMD_index = 167;
    #[c2rust::src_loc = "168:3"]
    pub const CMD_grep: CMD_index = 166;
    #[c2rust::src_loc = "167:3"]
    pub const CMD_goto: CMD_index = 165;
    #[c2rust::src_loc = "166:3"]
    pub const CMD_global: CMD_index = 164;
    #[c2rust::src_loc = "165:3"]
    pub const CMD_function: CMD_index = 163;
    #[c2rust::src_loc = "164:3"]
    pub const CMD_for: CMD_index = 162;
    #[c2rust::src_loc = "163:3"]
    pub const CMD_foldopen: CMD_index = 161;
    #[c2rust::src_loc = "162:3"]
    pub const CMD_folddoclosed: CMD_index = 160;
    #[c2rust::src_loc = "161:3"]
    pub const CMD_folddoopen: CMD_index = 159;
    #[c2rust::src_loc = "160:3"]
    pub const CMD_foldclose: CMD_index = 158;
    #[c2rust::src_loc = "159:3"]
    pub const CMD_fold: CMD_index = 157;
    #[c2rust::src_loc = "158:3"]
    pub const CMD_first: CMD_index = 156;
    #[c2rust::src_loc = "157:3"]
    pub const CMD_finish: CMD_index = 155;
    #[c2rust::src_loc = "156:3"]
    pub const CMD_finally: CMD_index = 154;
    #[c2rust::src_loc = "155:3"]
    pub const CMD_find: CMD_index = 153;
    #[c2rust::src_loc = "154:3"]
    pub const CMD_filter: CMD_index = 152;
    #[c2rust::src_loc = "153:3"]
    pub const CMD_filetype: CMD_index = 151;
    #[c2rust::src_loc = "152:3"]
    pub const CMD_files: CMD_index = 150;
    #[c2rust::src_loc = "151:3"]
    pub const CMD_file: CMD_index = 149;
    #[c2rust::src_loc = "150:3"]
    pub const CMD_exusage: CMD_index = 148;
    #[c2rust::src_loc = "149:3"]
    pub const CMD_exit: CMD_index = 147;
    #[c2rust::src_loc = "148:3"]
    pub const CMD_execute: CMD_index = 146;
    #[c2rust::src_loc = "147:3"]
    pub const CMD_ex: CMD_index = 145;
    #[c2rust::src_loc = "146:3"]
    pub const CMD_enew: CMD_index = 144;
    #[c2rust::src_loc = "145:3"]
    pub const CMD_endwhile: CMD_index = 143;
    #[c2rust::src_loc = "144:3"]
    pub const CMD_endtry: CMD_index = 142;
    #[c2rust::src_loc = "143:3"]
    pub const CMD_endfor: CMD_index = 141;
    #[c2rust::src_loc = "142:3"]
    pub const CMD_endfunction: CMD_index = 140;
    #[c2rust::src_loc = "141:3"]
    pub const CMD_endif: CMD_index = 139;
    #[c2rust::src_loc = "140:3"]
    pub const CMD_emenu: CMD_index = 138;
    #[c2rust::src_loc = "139:3"]
    pub const CMD_elseif: CMD_index = 137;
    #[c2rust::src_loc = "138:3"]
    pub const CMD_else: CMD_index = 136;
    #[c2rust::src_loc = "137:3"]
    pub const CMD_echon: CMD_index = 135;
    #[c2rust::src_loc = "136:3"]
    pub const CMD_echomsg: CMD_index = 134;
    #[c2rust::src_loc = "135:3"]
    pub const CMD_echohl: CMD_index = 133;
    #[c2rust::src_loc = "134:3"]
    pub const CMD_echoerr: CMD_index = 132;
    #[c2rust::src_loc = "133:3"]
    pub const CMD_echo: CMD_index = 131;
    #[c2rust::src_loc = "132:3"]
    pub const CMD_earlier: CMD_index = 130;
    #[c2rust::src_loc = "131:3"]
    pub const CMD_edit: CMD_index = 129;
    #[c2rust::src_loc = "130:3"]
    pub const CMD_dsplit: CMD_index = 128;
    #[c2rust::src_loc = "129:3"]
    pub const CMD_dsearch: CMD_index = 127;
    #[c2rust::src_loc = "128:3"]
    pub const CMD_drop: CMD_index = 126;
    #[c2rust::src_loc = "127:3"]
    pub const CMD_doautoall: CMD_index = 125;
    #[c2rust::src_loc = "126:3"]
    pub const CMD_doautocmd: CMD_index = 124;
    #[c2rust::src_loc = "125:3"]
    pub const CMD_dlist: CMD_index = 123;
    #[c2rust::src_loc = "124:3"]
    pub const CMD_djump: CMD_index = 122;
    #[c2rust::src_loc = "123:3"]
    pub const CMD_digraphs: CMD_index = 121;
    #[c2rust::src_loc = "122:3"]
    pub const CMD_diffthis: CMD_index = 120;
    #[c2rust::src_loc = "121:3"]
    pub const CMD_diffsplit: CMD_index = 119;
    #[c2rust::src_loc = "120:3"]
    pub const CMD_diffput: CMD_index = 118;
    #[c2rust::src_loc = "119:3"]
    pub const CMD_diffpatch: CMD_index = 117;
    #[c2rust::src_loc = "118:3"]
    pub const CMD_diffoff: CMD_index = 116;
    #[c2rust::src_loc = "117:3"]
    pub const CMD_diffget: CMD_index = 115;
    #[c2rust::src_loc = "116:3"]
    pub const CMD_diffupdate: CMD_index = 114;
    #[c2rust::src_loc = "115:3"]
    pub const CMD_display: CMD_index = 113;
    #[c2rust::src_loc = "114:3"]
    pub const CMD_delfunction: CMD_index = 112;
    #[c2rust::src_loc = "113:3"]
    pub const CMD_delcommand: CMD_index = 111;
    #[c2rust::src_loc = "112:3"]
    pub const CMD_debuggreedy: CMD_index = 110;
    #[c2rust::src_loc = "111:3"]
    pub const CMD_debug: CMD_index = 109;
    #[c2rust::src_loc = "110:3"]
    pub const CMD_delmarks: CMD_index = 108;
    #[c2rust::src_loc = "109:3"]
    pub const CMD_delete: CMD_index = 107;
    #[c2rust::src_loc = "108:3"]
    pub const CMD_cwindow: CMD_index = 106;
    #[c2rust::src_loc = "107:3"]
    pub const CMD_cunmenu: CMD_index = 105;
    #[c2rust::src_loc = "106:3"]
    pub const CMD_cunabbrev: CMD_index = 104;
    #[c2rust::src_loc = "105:3"]
    pub const CMD_cunmap: CMD_index = 103;
    #[c2rust::src_loc = "104:3"]
    pub const CMD_cstag: CMD_index = 102;
    #[c2rust::src_loc = "103:3"]
    pub const CMD_cscope: CMD_index = 101;
    #[c2rust::src_loc = "102:3"]
    pub const CMD_crewind: CMD_index = 100;
    #[c2rust::src_loc = "101:3"]
    pub const CMD_cquit: CMD_index = 99;
    #[c2rust::src_loc = "100:3"]
    pub const CMD_cpfile: CMD_index = 98;
    #[c2rust::src_loc = "99:3"]
    pub const CMD_cprevious: CMD_index = 97;
    #[c2rust::src_loc = "98:3"]
    pub const CMD_copen: CMD_index = 96;
    #[c2rust::src_loc = "97:3"]
    pub const CMD_const: CMD_index = 95;
    #[c2rust::src_loc = "96:3"]
    pub const CMD_confirm: CMD_index = 94;
    #[c2rust::src_loc = "95:3"]
    pub const CMD_continue: CMD_index = 93;
    #[c2rust::src_loc = "94:3"]
    pub const CMD_compiler: CMD_index = 92;
    #[c2rust::src_loc = "93:3"]
    pub const CMD_comclear: CMD_index = 91;
    #[c2rust::src_loc = "92:3"]
    pub const CMD_command: CMD_index = 90;
    #[c2rust::src_loc = "91:3"]
    pub const CMD_colorscheme: CMD_index = 89;
    #[c2rust::src_loc = "90:3"]
    pub const CMD_colder: CMD_index = 88;
    #[c2rust::src_loc = "89:3"]
    pub const CMD_copy: CMD_index = 87;
    #[c2rust::src_loc = "88:3"]
    pub const CMD_cnoremenu: CMD_index = 86;
    #[c2rust::src_loc = "87:3"]
    pub const CMD_cnoreabbrev: CMD_index = 85;
    #[c2rust::src_loc = "86:3"]
    pub const CMD_cnoremap: CMD_index = 84;
    #[c2rust::src_loc = "85:3"]
    pub const CMD_cnfile: CMD_index = 83;
    #[c2rust::src_loc = "84:3"]
    pub const CMD_cnewer: CMD_index = 82;
    #[c2rust::src_loc = "83:3"]
    pub const CMD_cnext: CMD_index = 81;
    #[c2rust::src_loc = "82:3"]
    pub const CMD_cmenu: CMD_index = 80;
    #[c2rust::src_loc = "81:3"]
    pub const CMD_cmapclear: CMD_index = 79;
    #[c2rust::src_loc = "80:3"]
    pub const CMD_cmap: CMD_index = 78;
    #[c2rust::src_loc = "79:3"]
    pub const CMD_clearjumps: CMD_index = 77;
    #[c2rust::src_loc = "78:3"]
    pub const CMD_close: CMD_index = 76;
    #[c2rust::src_loc = "77:3"]
    pub const CMD_clast: CMD_index = 75;
    #[c2rust::src_loc = "76:3"]
    pub const CMD_clist: CMD_index = 74;
    #[c2rust::src_loc = "75:3"]
    pub const CMD_chistory: CMD_index = 73;
    #[c2rust::src_loc = "74:3"]
    pub const CMD_checktime: CMD_index = 72;
    #[c2rust::src_loc = "73:3"]
    pub const CMD_checkpath: CMD_index = 71;
    #[c2rust::src_loc = "72:3"]
    pub const CMD_checkhealth: CMD_index = 70;
    #[c2rust::src_loc = "71:3"]
    pub const CMD_changes: CMD_index = 69;
    #[c2rust::src_loc = "70:3"]
    pub const CMD_chdir: CMD_index = 68;
    #[c2rust::src_loc = "69:3"]
    pub const CMD_cgetexpr: CMD_index = 67;
    #[c2rust::src_loc = "68:3"]
    pub const CMD_cgetbuffer: CMD_index = 66;
    #[c2rust::src_loc = "67:3"]
    pub const CMD_cgetfile: CMD_index = 65;
    #[c2rust::src_loc = "66:3"]
    pub const CMD_cfirst: CMD_index = 64;
    #[c2rust::src_loc = "65:3"]
    pub const CMD_cfdo: CMD_index = 63;
    #[c2rust::src_loc = "64:3"]
    pub const CMD_cfile: CMD_index = 62;
    #[c2rust::src_loc = "63:3"]
    pub const CMD_cexpr: CMD_index = 61;
    #[c2rust::src_loc = "62:3"]
    pub const CMD_center: CMD_index = 60;
    #[c2rust::src_loc = "61:3"]
    pub const CMD_cdo: CMD_index = 59;
    #[c2rust::src_loc = "60:3"]
    pub const CMD_cd: CMD_index = 58;
    #[c2rust::src_loc = "59:3"]
    pub const CMD_cclose: CMD_index = 57;
    #[c2rust::src_loc = "58:3"]
    pub const CMD_cc: CMD_index = 56;
    #[c2rust::src_loc = "57:3"]
    pub const CMD_cbottom: CMD_index = 55;
    #[c2rust::src_loc = "56:3"]
    pub const CMD_cbelow: CMD_index = 54;
    #[c2rust::src_loc = "55:3"]
    pub const CMD_cbuffer: CMD_index = 53;
    #[c2rust::src_loc = "54:3"]
    pub const CMD_catch: CMD_index = 52;
    #[c2rust::src_loc = "53:3"]
    pub const CMD_call: CMD_index = 51;
    #[c2rust::src_loc = "52:3"]
    pub const CMD_caddfile: CMD_index = 50;
    #[c2rust::src_loc = "51:3"]
    pub const CMD_caddexpr: CMD_index = 49;
    #[c2rust::src_loc = "50:3"]
    pub const CMD_caddbuffer: CMD_index = 48;
    #[c2rust::src_loc = "49:3"]
    pub const CMD_cabove: CMD_index = 47;
    #[c2rust::src_loc = "48:3"]
    pub const CMD_cabclear: CMD_index = 46;
    #[c2rust::src_loc = "47:3"]
    pub const CMD_cabbrev: CMD_index = 45;
    #[c2rust::src_loc = "46:3"]
    pub const CMD_cNfile: CMD_index = 44;
    #[c2rust::src_loc = "45:3"]
    pub const CMD_cNext: CMD_index = 43;
    #[c2rust::src_loc = "44:3"]
    pub const CMD_change: CMD_index = 42;
    #[c2rust::src_loc = "43:3"]
    pub const CMD_bwipeout: CMD_index = 41;
    #[c2rust::src_loc = "42:3"]
    pub const CMD_bunload: CMD_index = 40;
    #[c2rust::src_loc = "41:3"]
    pub const CMD_bufdo: CMD_index = 39;
    #[c2rust::src_loc = "40:3"]
    pub const CMD_buffers: CMD_index = 38;
    #[c2rust::src_loc = "39:3"]
    pub const CMD_browse: CMD_index = 37;
    #[c2rust::src_loc = "38:3"]
    pub const CMD_breaklist: CMD_index = 36;
    #[c2rust::src_loc = "37:3"]
    pub const CMD_breakdel: CMD_index = 35;
    #[c2rust::src_loc = "36:3"]
    pub const CMD_breakadd: CMD_index = 34;
    #[c2rust::src_loc = "35:3"]
    pub const CMD_break: CMD_index = 33;
    #[c2rust::src_loc = "34:3"]
    pub const CMD_brewind: CMD_index = 32;
    #[c2rust::src_loc = "33:3"]
    pub const CMD_bprevious: CMD_index = 31;
    #[c2rust::src_loc = "32:3"]
    pub const CMD_botright: CMD_index = 30;
    #[c2rust::src_loc = "31:3"]
    pub const CMD_bnext: CMD_index = 29;
    #[c2rust::src_loc = "30:3"]
    pub const CMD_bmodified: CMD_index = 28;
    #[c2rust::src_loc = "29:3"]
    pub const CMD_blast: CMD_index = 27;
    #[c2rust::src_loc = "28:3"]
    pub const CMD_bfirst: CMD_index = 26;
    #[c2rust::src_loc = "27:3"]
    pub const CMD_belowright: CMD_index = 25;
    #[c2rust::src_loc = "26:3"]
    pub const CMD_behave: CMD_index = 24;
    #[c2rust::src_loc = "25:3"]
    pub const CMD_bdelete: CMD_index = 23;
    #[c2rust::src_loc = "24:3"]
    pub const CMD_badd: CMD_index = 22;
    #[c2rust::src_loc = "23:3"]
    pub const CMD_ball: CMD_index = 21;
    #[c2rust::src_loc = "22:3"]
    pub const CMD_bNext: CMD_index = 20;
    #[c2rust::src_loc = "21:3"]
    pub const CMD_buffer: CMD_index = 19;
    #[c2rust::src_loc = "20:3"]
    pub const CMD_aunmenu: CMD_index = 18;
    #[c2rust::src_loc = "19:3"]
    pub const CMD_augroup: CMD_index = 17;
    #[c2rust::src_loc = "18:3"]
    pub const CMD_autocmd: CMD_index = 16;
    #[c2rust::src_loc = "17:3"]
    pub const CMD_ascii: CMD_index = 15;
    #[c2rust::src_loc = "16:3"]
    pub const CMD_argument: CMD_index = 14;
    #[c2rust::src_loc = "15:3"]
    pub const CMD_arglocal: CMD_index = 13;
    #[c2rust::src_loc = "14:3"]
    pub const CMD_argglobal: CMD_index = 12;
    #[c2rust::src_loc = "13:3"]
    pub const CMD_argedit: CMD_index = 11;
    #[c2rust::src_loc = "12:3"]
    pub const CMD_argdo: CMD_index = 10;
    #[c2rust::src_loc = "11:3"]
    pub const CMD_argdelete: CMD_index = 9;
    #[c2rust::src_loc = "10:3"]
    pub const CMD_argadd: CMD_index = 8;
    #[c2rust::src_loc = "9:3"]
    pub const CMD_args: CMD_index = 7;
    #[c2rust::src_loc = "8:3"]
    pub const CMD_anoremenu: CMD_index = 6;
    #[c2rust::src_loc = "7:3"]
    pub const CMD_amenu: CMD_index = 5;
    #[c2rust::src_loc = "6:3"]
    pub const CMD_all: CMD_index = 4;
    #[c2rust::src_loc = "5:3"]
    pub const CMD_aboveleft: CMD_index = 3;
    #[c2rust::src_loc = "4:3"]
    pub const CMD_abclear: CMD_index = 2;
    #[c2rust::src_loc = "3:3"]
    pub const CMD_abbreviate: CMD_index = 1;
    #[c2rust::src_loc = "2:3"]
    pub const CMD_append: CMD_index = 0;
    #[c2rust::src_loc = "1:1"]
    pub type cmdidx_T = CMD_index;
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/ex_cmds_defs.h:13"]
pub mod ex_cmds_defs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "142:8"]
    pub struct exarg {
        pub arg: *mut char_u,
        pub nextcmd: *mut char_u,
        pub cmd: *mut char_u,
        pub cmdlinep: *mut *mut char_u,
        pub cmdidx: cmdidx_T,
        pub argt: uint32_t,
        pub skip: libc::c_int,
        pub forceit: libc::c_int,
        pub addr_count: libc::c_int,
        pub line1: linenr_T,
        pub line2: linenr_T,
        pub addr_type: libc::c_int,
        pub flags: libc::c_int,
        pub do_ecmd_cmd: *mut char_u,
        pub do_ecmd_lnum: linenr_T,
        pub append: libc::c_int,
        pub usefilter: libc::c_int,
        pub amount: libc::c_int,
        pub regname: libc::c_int,
        pub force_bin: libc::c_int,
        pub read_edit: libc::c_int,
        pub force_ff: libc::c_int,
        pub force_enc: libc::c_int,
        pub bad_char: libc::c_int,
        pub useridx: libc::c_int,
        pub errmsg: *mut char_u,
        pub getline: LineGetter,
        pub cookie: *mut libc::c_void,
        pub cstack: *mut cstack_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "115:9"]
    pub struct cstack_T {
        pub cs_flags: [libc::c_int; 50],
        pub cs_pending: [libc::c_char; 50],
        pub cs_pend: C2RustUnnamed_28,
        pub cs_forinfo: [*mut libc::c_void; 50],
        pub cs_line: [libc::c_int; 50],
        pub cs_idx: libc::c_int,
        pub cs_looplevel: libc::c_int,
        pub cs_trylevel: libc::c_int,
        pub cs_emsg_silent_list: *mut eslist_T,
        pub cs_lflags: libc::c_int,
    }
    #[c2rust::src_loc = "103:1"]
    pub type eslist_T = eslist_elem;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "104:8"]
    pub struct eslist_elem {
        pub saved_emsg_silent: libc::c_int,
        pub next: *mut eslist_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:3"]
    pub union C2RustUnnamed_28 {
        pub csp_rv: [*mut libc::c_void; 50],
        pub csp_ex: [*mut libc::c_void; 50],
    }
    #[c2rust::src_loc = "90:1"]
    pub type LineGetter
        =
        Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_void,
                                    _: libc::c_int, _: bool) -> *mut char_u>;
    #[c2rust::src_loc = "81:1"]
    pub type exarg_T = exarg;
    // cursor position in line
    // values for xp_backslash
    // nothing special for backslashes
    // uses one backslash before a space
    // uses three backslashes before a space
    // / Command modifiers ":vertical", ":browse", ":confirm", ":hide", etc. set a
// / flag.  This needs to be saved for recursive commands, put them in a
// / structure for easy manipulation.
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:9"]
    pub struct cmdmod_T {
        pub split: libc::c_int,
        pub tab: libc::c_int,
        pub browse: bool,
        pub confirm: bool,
        pub hide: bool,
        pub keepalt: bool,
        pub keepjumps: bool,
        pub keepmarks: bool,
        pub keeppatterns: bool,
        pub lockmarks: bool,
        pub noswapfile: bool,
        pub save_ei: *mut char_u,
        pub filter_regmatch: regmatch_T,
        pub filter_force: bool,
    }
    use super::nvim_types_h::char_u;
    use super::ex_cmds_enum_generated_h::cmdidx_T;
    use super::stdint_uintn_h::uint32_t;
    use super::pos_h::linenr_T;
    use super::regexp_defs_h::regmatch_T;
    // NVIM_EX_CMDS_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/defs.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/multiqueue.h:13"]
pub mod multiqueue_h {
    #[c2rust::src_loc = "10:1"]
    pub type MultiQueue = multiqueue;
    extern "C" {
        #[c2rust::src_loc = "10:16"]
        pub type multiqueue;
    }
    // NVIM_EVENT_MULTIQUEUE_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/loop.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/buffer.h:16"]
pub mod buffer_h {
    #[c2rust::src_loc = "15:3"]
    pub const GETF_SETMARK: getf_values = 1;
    #[c2rust::src_loc = "14:1"]
    pub type getf_values = libc::c_uint;
    #[c2rust::src_loc = "17:3"]
    pub const GETF_SWITCH: getf_values = 4;
    #[c2rust::src_loc = "16:3"]
    pub const GETF_ALT: getf_values = 2;
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
        pub uv: C2RustUnnamed_29,
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
    pub union C2RustUnnamed_29 {
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
                                                      C2RustUnnamed_29{pipe:
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
                                                                                         C2RustUnnamed_23{fd:
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
                                                      C2RustUnnamed_29{pipe:
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
                                                                                         C2RustUnnamed_23{fd:
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
                                                      C2RustUnnamed_29{pipe:
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
                                                                                         C2RustUnnamed_23{fd:
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
    use super::stream_h::{Stream, C2RustUnnamed_29, stream_read_cb,
                          stream_write_cb, stream_close_cb};
    use super::multiqueue_h::MultiQueue;
    use super::stddef_h::{NULL_1, size_t};
    use super::uv_h::{uv_stream_t, uv_pipe_t, uv_loop_t, uv_handle_type,
                      uv_close_cb, C2RustUnnamed_23, uv_handle_t, uv_alloc_cb,
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
                                                  C2RustUnnamed_29{pipe:
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
                                                                                     C2RustUnnamed_23{fd:
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
                                                  C2RustUnnamed_29{pipe:
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
                                                                                     C2RustUnnamed_23{fd:
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
                                                  C2RustUnnamed_29{pipe:
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
                                                                                     C2RustUnnamed_23{fd:
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
    use super::stream_h::{Stream, C2RustUnnamed_29, stream_read_cb,
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
                                                       C2RustUnnamed_26{fd:
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
                                                                C2RustUnnamed_27{stream:
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
                      uv_close_cb, C2RustUnnamed_26, uv_handle_t, uv_exit_cb,
                      uv_stdio_flags, C2RustUnnamed_27, UV_IGNORE,
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
    pub union C2RustUnnamed_30 {
        pub f: libc::c_float,
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "663:5"]
    pub union C2RustUnnamed_31 {
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
        let mut mem = C2RustUnnamed_30{f: 0.,};
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
        let mut mem = C2RustUnnamed_31{f: 0.,};
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
        #[c2rust::src_loc = "66:1"]
        pub fn msgpack_vrefbuffer_init(vbuf: *mut msgpack_vrefbuffer,
                                       ref_size: size_t, chunk_size: size_t)
         -> bool;
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn msgpack_vrefbuffer_destroy(vbuf: *mut msgpack_vrefbuffer);
        #[no_mangle]
        #[c2rust::src_loc = "84:1"]
        pub fn msgpack_vrefbuffer_append_ref(vbuf: *mut msgpack_vrefbuffer,
                                             buf: *const libc::c_char,
                                             len: size_t) -> libc::c_int;
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
        pub stream: C2RustUnnamed_33,
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
    pub union C2RustUnnamed_33 {
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
    use super::typval_h::{Callback, dict_T, C2RustUnnamed_1, CallbackType,
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
        pub call_stack: C2RustUnnamed_32,
        pub info: Dictionary,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:3"]
    pub struct C2RustUnnamed_32 {
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
#[c2rust::header_src = "/usr/include/assert.h:8"]
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
#[c2rust::header_src = "/usr/include/string.h:10"]
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
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/linux/limits.h:11"]
pub mod limits_h {
    #[c2rust::src_loc = "13:9"]
    pub const PATH_MAX: libc::c_int = 4096 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/stdlib.h:13"]
pub mod stdlib_h {
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
    }
}
#[c2rust::header_src = "/home/vole/neovim/build/include/log.h.generated.h:13"]
pub mod log_h_generated_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "7:1"]
        pub fn logmsg(log_level: libc::c_int, context: *const libc::c_char,
                      func_name: *const libc::c_char, line_num: libc::c_int,
                      eol: bool, fmt: *const libc::c_char, _: ...) -> bool;
    }
}
#[c2rust::header_src =
  "/usr/lib/llvm-6.0/lib/clang/6.0.1/include/stdbool.h:13"]
pub mod stdbool_h {
    #[c2rust::src_loc = "32:9"]
    pub const true_0: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "33:9"]
    pub const false_0: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/log.h:13"]
pub mod log_h {
    #[c2rust::src_loc = "11:9"]
    pub const WARN_LOG_LEVEL: libc::c_int = 2 as libc::c_int;
    // NVIM_LOG_H
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/mbyte.h.generated.h:13"]
pub mod mbyte_h_generated_h {
    use super::nvim_types_h::char_u;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "14:1"]
        pub fn utf_ptr2char(p: *const char_u) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "23:1"]
        pub fn utfc_ptr2len(p: *const char_u) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "40:1"]
        pub fn mb_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "42:1"]
        pub fn utf_head_off(base: *const char_u, p: *const char_u)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/mbyte.h:13"]
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
#[c2rust::header_src = "/usr/include/libintl.h:13"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/message.h.generated.h:13"]
pub mod message_h_generated_h {
    use super::nvim_types_h::char_u;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "7:1"]
        pub fn msg(s: *mut char_u) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "20:1"]
        pub fn emsg(s: *const char_u) -> bool;
        #[no_mangle]
        #[c2rust::src_loc = "22:1"]
        pub fn emsgf(fmt: *const libc::c_char, _: ...) -> bool;
        #[no_mangle]
        #[c2rust::src_loc = "38:1"]
        pub fn msg_putchar(c: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "43:1"]
        pub fn msg_outtrans(str: *mut char_u) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn msg_outtrans_attr(str: *const char_u, attr: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn msg_puts(s: *const libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "55:1"]
        pub fn msg_puts_title(s: *const libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "61:1"]
        pub fn message_filtered(msg_0: *mut char_u) -> bool;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/eval/typval.h.generated.h:13"]
pub mod typval_h_generated_h {
    use super::typval_h::{dict_T, typval_T, dictitem_T};
    use super::stddef_h::ptrdiff_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "43:1"]
        pub fn tv_dict_watcher_notify(dict: *mut dict_T,
                                      key: *const libc::c_char,
                                      newtv: *mut typval_T,
                                      oldtv: *mut typval_T);
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn tv_dict_unref(d: *mut dict_T);
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn tv_dict_find(d: *const dict_T, key: *const libc::c_char,
                            len: ptrdiff_t) -> *mut dictitem_T;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/strings.h:13"]
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
  "/home/vole/neovim/build/include/strings.h.generated.h:13"]
pub mod strings_h_generated_h {
    use super::nvim_types_h::char_u;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "4:1"]
        pub fn vim_strsave(string: *const char_u) -> *mut char_u;
        #[no_mangle]
        #[c2rust::src_loc = "5:1"]
        pub fn vim_strnsave(string: *const char_u, len: size_t)
         -> *mut char_u;
        #[no_mangle]
        #[c2rust::src_loc = "15:1"]
        pub fn vim_strchr(string: *const char_u, c: libc::c_int)
         -> *mut char_u;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/os/os_defs.h:13"]
pub mod os_defs_h {
    // Use the system path length if it makes sense.
    #[c2rust::src_loc = "20:10"]
    pub const MAXPATHL: libc::c_int = PATH_MAX;
    use super::limits_h::PATH_MAX;
    // NVIM_OS_OS_DEFS_H
    // / Converts system error code to libuv error code.
    // / Converts libuv error (negative int) to error description string.
    // Note: Some systems need both string.h and strings.h (Savage).  However,
// some systems can't handle both, only use string.h in that case.
    // Command-processing buffer. Use large buffers for all platforms.
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/globals.h:13"]
pub mod globals_h {
    #[c2rust::src_loc = "17:9"]
    pub const IOSIZE: libc::c_int = 1024 as libc::c_int + 1 as libc::c_int;
    use super::nvim_types_h::char_u;
    use super::buffer_defs_h::{win_T, tabpage_T, buf_T};
    use super::pos_h::{pos_T, linenr_T, colnr_T};
    use super::ex_cmds_defs_h::cmdmod_T;
    use super::regexp_defs_h::{regmatch_T, regprog_T};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "963:15"]
        pub static mut e_umark: [char_u; 0];
        #[no_mangle]
        #[c2rust::src_loc = "899:15"]
        pub static mut e_marknotset: [char_u; 0];
        #[no_mangle]
        #[c2rust::src_loc = "898:15"]
        pub static mut e_markinval: [char_u; 0];
        #[no_mangle]
        #[c2rust::src_loc = "872:15"]
        pub static mut e_invarg2: [char_u; 0];
        #[no_mangle]
        #[c2rust::src_loc = "871:15"]
        pub static mut e_invarg: [char_u; 0];
        #[no_mangle]
        #[c2rust::src_loc = "853:15"]
        pub static mut e_argreq: [char_u; 0];
        #[no_mangle]
        #[c2rust::src_loc = "93:12"]
        pub static mut Columns: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "409:18"]
        pub static mut firstwin: *mut win_T;
        #[no_mangle]
        #[c2rust::src_loc = "427:18"]
        pub static mut curwin: *mut win_T;
        #[no_mangle]
        #[c2rust::src_loc = "438:22"]
        pub static mut first_tabpage: *mut tabpage_T;
        #[no_mangle]
        #[c2rust::src_loc = "440:22"]
        pub static mut curtab: *mut tabpage_T;
        #[no_mangle]
        #[c2rust::src_loc = "450:18"]
        pub static mut curbuf: *mut buf_T;
        #[no_mangle]
        #[c2rust::src_loc = "566:14"]
        pub static mut saved_cursor: pos_T;
        #[no_mangle]
        #[c2rust::src_loc = "652:17"]
        pub static mut cmdmod: cmdmod_T;
        #[no_mangle]
        #[c2rust::src_loc = "671:15"]
        pub static mut IObuff: [char_u; 1025];
        #[no_mangle]
        #[c2rust::src_loc = "672:15"]
        pub static mut NameBuff: [char_u; 4096];
        #[no_mangle]
        #[c2rust::src_loc = "710:21"]
        pub static mut got_int: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "726:12"]
        pub static mut global_busy: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "727:12"]
        pub static mut listcmd_busy: libc::c_int;
    }
    // NVIM_GLOBALS_H
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/memory.h.generated.h:13"]
pub mod memory_h_generated_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "8:1"]
        pub fn xfree(ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "9:1"]
        pub fn xcalloc(count: size_t, size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "10:1"]
        pub fn xrealloc(ptr: *mut libc::c_void, size: size_t)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "21:1"]
        pub fn xstrlcpy(dst: *mut libc::c_char, src: *const libc::c_char,
                        dsize: size_t) -> size_t;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/lib/kvec.h:13"]
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
#[c2rust::header_src = "/usr/include/netinet/in.h:13"]
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
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/uintn-identity.h:13"]
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/byteswap.h:13"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/ascii.h:14"]
pub mod ascii_h {
    // Definitions of various common control characters.
    #[c2rust::src_loc = "19:9"]
    pub const NUL: libc::c_int = '\u{0}' as i32;
    #[c2rust::src_loc = "22:9"]
    pub const TAB: libc::c_int = '\t' as i32;
    /* CR is used by Mac OS X */
    // Control Sequence Introducer
    /* Device Control String */
    /* String Terminator */
    /* '?' -> DEL, '@' -> ^@, etc. */
    /* @ */
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/mark.h:15"]
pub mod mark_h {
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
    // for exarg_T
    // / Set fmark using given value
    // / Free and set fmark using given value
    // / Clear given fmark
    // / Set given extended mark (regular mark + file name)
    // / Free and set given extended mark (regular mark + file name)
    // / Convert mark name to the offset
    // / Convert local mark name to the offset
    // / Return true if position a and b are equal.
    #[inline(always)]
    #[c2rust::src_loc = "101:1"]
    pub unsafe extern "C" fn equalpos(mut a: pos_T, mut b: pos_T) -> bool {
        return a.lnum == b.lnum && a.col == b.col && a.coladd == b.coladd;
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
    use super::ascii_h::ascii_isdigit;
    use super::mark_defs_h::NMARKS;
    use super::pos_h::{pos_T, linenr_T, colnr_T};
    // NVIM_MARK_H
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/buffer.h.generated.h:16"]
pub mod buffer_h_generated_h {
    use super::pos_h::linenr_T;
    use super::nvim_types_h::char_u;
    use super::buffer_defs_h::buf_T;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "24:1"]
        pub fn buflist_getfile(n: libc::c_int, lnum: linenr_T,
                               options: libc::c_int, forceit: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "21:1"]
        pub fn buflist_new(ffname: *mut char_u, sfname: *mut char_u,
                           lnum: linenr_T, flags: libc::c_int) -> *mut buf_T;
        #[no_mangle]
        #[c2rust::src_loc = "31:1"]
        pub fn buflist_nr2name(n: libc::c_int, fullname: libc::c_int,
                               helptail: libc::c_int) -> *mut char_u;
        #[no_mangle]
        #[c2rust::src_loc = "30:1"]
        pub fn buflist_findnr(nr: libc::c_int) -> *mut buf_T;
    }
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
        #[c2rust::src_loc = "48:1"]
        pub fn ptr2cells(p: *const char_u) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "63:1"]
        pub fn vim_isprintc(c: libc::c_int) -> bool;
        #[no_mangle]
        #[c2rust::src_loc = "76:1"]
        pub fn skipwhite(q: *const char_u) -> *mut char_u;
    }
    // NVIM_CHARSET_H
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/diff.h.generated.h:18"]
pub mod diff_h_generated_h {
    use super::pos_h::linenr_T;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "8:1"]
        pub fn diff_mark_adjust(line1: linenr_T, line2: linenr_T,
                                amount: libc::c_long,
                                amount_after: libc::c_long);
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/os/env.h.generated.h:21"]
pub mod env_h_generated_h {
    use super::nvim_types_h::char_u;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "20:1"]
        pub fn expand_env(src: *mut char_u, dst: *mut char_u,
                          dstlen: libc::c_int);
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/os/fs.h.generated.h:21"]
pub mod fs_h_generated_h {
    use super::nvim_types_h::char_u;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "6:1"]
        pub fn os_dirname(buf: *mut char_u, len: size_t) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/fold.h.generated.h:22"]
pub mod fold_h_generated_h {
    use super::buffer_defs_h::win_T;
    use super::pos_h::linenr_T;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn foldMarkAdjust(wp: *mut win_T, line1: linenr_T,
                              line2: linenr_T, amount: libc::c_long,
                              amount_after: libc::c_long);
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/extmark.h.generated.h:23"]
pub mod extmark_h_generated_h {
    use super::buffer_defs_h::buf_T;
    use super::pos_h::linenr_T;
    use super::extmark_defs_h::{ExtmarkOp, kExtmarkNOOP};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "12:1"]
        pub fn extmark_adjust(buf: *mut buf_T, line1: linenr_T,
                              line2: linenr_T, amount: libc::c_long,
                              amount_after: libc::c_long, undo: ExtmarkOp);
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/memline.h.generated.h:25"]
pub mod memline_h_generated_h {
    use super::pos_h::linenr_T;
    use super::nvim_types_h::char_u;
    use super::buffer_defs_h::buf_T;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "19:1"]
        pub fn ml_get(lnum: linenr_T) -> *mut char_u;
        #[no_mangle]
        #[c2rust::src_loc = "21:1"]
        pub fn ml_get_buf(buf: *mut buf_T, lnum: linenr_T, will_change: bool)
         -> *mut char_u;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/path.h.generated.h:30"]
pub mod path_h_generated_h {
    use super::nvim_types_h::char_u;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "15:1"]
        pub fn path_fnamecmp(fname1: *const libc::c_char,
                             fname2: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn path_shorten_fname(full_path: *mut char_u,
                                  dir_name: *mut char_u) -> *mut char_u;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/quickfix.h.generated.h:31"]
pub mod quickfix_h_generated_h {
    use super::buffer_defs_h::win_T;
    use super::pos_h::linenr_T;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "11:1"]
        pub fn qf_mark_adjust(wp: *mut win_T, line1: linenr_T,
                              line2: linenr_T, amount: libc::c_long,
                              amount_after: libc::c_long) -> bool;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/search.h.generated.h:32"]
pub mod search_h_generated_h {
    use super::vim_h::{Direction, kDirectionNotSet};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "34:1"]
        pub fn findsent(dir: Direction, count: libc::c_long) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "35:1"]
        pub fn findpar(pincl: *mut bool, dir: libc::c_int,
                       count: libc::c_long, what: libc::c_int,
                       both: libc::c_int) -> bool;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/sign.h.generated.h:33"]
pub mod sign_h_generated_h {
    use super::pos_h::linenr_T;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "17:1"]
        pub fn sign_mark_adjust(line1: linenr_T, line2: linenr_T,
                                amount: libc::c_long,
                                amount_after: libc::c_long);
    }
}
#[c2rust::header_src = "/home/vole/neovim/build/include/ui.h.generated.h:35"]
pub mod ui_h_generated_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "26:1"]
        pub fn ui_flush();
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/os/input.h.generated.h:38"]
pub mod input_h_generated_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "11:1"]
        pub fn os_breakcheck();
    }
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::types_h::{__int8_t, __uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __int64_t, __uint64_t, __uid_t, __gid_t,
                        __time_t, __ssize_t};
pub use self::stdint_intn_h::{int8_t, int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
pub use self::stddef_h::{size_t, ptrdiff_t, NULL_1, NULL, NULL_0};
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
                         kCallbackNone, Callback, C2RustUnnamed_1,
                         dict_watcher, DictWatcher, ScopeDictDictItem,
                         dictitem_T, C2RustUnnamed_2, DI_FLAGS_ALLOC,
                         DI_FLAGS_LOCK, DI_FLAGS_FIX, DI_FLAGS_RO_SBX,
                         DI_FLAGS_RO, tv_list_ref, tv_list_set_ret,
                         tv_list_locked, tv_list_set_lock, tv_list_set_copyid,
                         tv_list_len, tv_list_copyid, tv_list_latest_copy,
                         tv_list_uidx, tv_list_has_watchers, tv_list_first,
                         tv_list_last, tv_dict_set_ret, tv_dict_len,
                         tv_dict_is_watched, tv_init, tv_get_float_chk,
                         tv_dict_watcher_node_data, tv_is_func, funccall_S};
pub use self::pos_h::{linenr_T, colnr_T, C2RustUnnamed, MAXLNUM,
                      C2RustUnnamed_0, MAXCOL, pos_T, lpos_T};
pub use self::stdarg_h::va_list;
pub use self::stdio_h::{ssize_t, sprintf, snprintf};
pub use self::sys_types_h::{gid_t, uid_t};
pub use self::time_t_h::time_t;
pub use self::pthreadtypes_arch_h::__pthread_rwlock_arch_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::{pthread_mutex_t, pthread_rwlock_t};
pub use self::hashtab_h::{hash_T, hashitem_S, hashitem_T, hashtable_S,
                          hashtab_T};
pub use self::garray_h::{growarray, garray_T, ga_append_via_ptr, ga_grow};
pub use self::queue_h::{_queue, QUEUE, QUEUE_EMPTY, QUEUE_INIT, QUEUE_ADD,
                        QUEUE_INSERT_HEAD, QUEUE_INSERT_TAIL, QUEUE_REMOVE};
pub use self::profile_h::proftime_T;
pub use self::grid_defs_h::{schar_T, sattr_T, ScreenGrid};
pub use self::vim_h::{Direction, BACKWARD_FILE, FORWARD_FILE, BACKWARD,
                      FORWARD, kDirectionNotSet, TRUE, FALSE, OK, FAIL};
pub use self::buffer_defs_h::{file_buffer, C2RustUnnamed_3,
                              BufUpdateCallbacks, C2RustUnnamed_4, synblock_T,
                              buf_T, win_T, window_S, qf_info_T, FloatConfig,
                              WinStyle, kWinStyleMinimal, kWinStyleUnused,
                              FloatRelative, kFloatRelativeCursor,
                              kFloatRelativeWindow, kFloatRelativeEditor,
                              FloatAnchor, taggy_T, taggy, matchitem_T,
                              matchitem, match_T, posmatch_T, posmatch,
                              llpos_T, winopt_T, alist_T, arglist, wline_T,
                              w_line, pos_save_T, C2RustUnnamed_5,
                              C2RustUnnamed_6, frame_T, frame_S, disptick_T,
                              syn_time_T, mapblock_T, mapblock, wininfo_T,
                              wininfo_S, ChangedtickDictItem, diffblock_S,
                              diff_T, tabpage_S, tabpage_T, BUF_HAS_QF_ENTRY,
                              BUF_HAS_LL_ENTRY, win_hl_attr, qf_info_S};
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
                               ExtmarkOp, kExtmarkUndoNoRedo, kExtmarkNoUndo,
                               kExtmarkUndo, kExtmarkNOOP};
pub use self::marktree_h::{MarkTree, mtnode_t, mtnode_s, mtkey_t, mtpos_t};
pub use self::map_defs_h::{ptr_t, cstr_t};
pub use self::terminal_h::{Terminal, terminal};
pub use self::sign_defs_h::{signlist_T, signlist, signgroup_T, signgroup_S};
pub use self::regexp_defs_h::{regprog_T, regprog, regengine_T, regengine,
                              regmmatch_T, regmatch_T, reg_extmatch_T,
                              reg_extmatch};
pub use self::defs_h::{Window, Boolean, Integer, Float, String_0, object,
                       C2RustUnnamed_14, Dictionary, KeyValuePair,
                       key_value_pair, Object, Array, ObjectType,
                       kObjectTypeTabpage, kObjectTypeWindow,
                       kObjectTypeBuffer, kObjectTypeLuaRef,
                       kObjectTypeDictionary, kObjectTypeArray,
                       kObjectTypeString, kObjectTypeFloat,
                       kObjectTypeInteger, kObjectTypeBoolean, kObjectTypeNil,
                       is_internal_call};
pub use self::mark_defs_h::{fmark_T, filemark, xfmark_T, xfilemark,
                            NGLOBALMARKS, EXTRA_MARKS, JUMPLISTSIZE, NMARKS};
pub use self::time_h::{Timestamp, os_time};
pub use self::option_defs_h::{LastSet, JOP_STACK, breakat_flags, jop_flags};
pub use self::syntax_defs_h::{synstate_T, syn_state, C2RustUnnamed_7,
                              bufstate_T, buf_state};
pub use self::undo_defs_h::{u_header_T, u_header, visualinfo_T, u_entry_T,
                            u_entry, C2RustUnnamed_9, C2RustUnnamed_10,
                            C2RustUnnamed_11, C2RustUnnamed_12};
pub use self::extmark_h::{undo_object, C2RustUnnamed_8, ExtmarkSavePos,
                          ExtmarkMove, ExtmarkSplice, UndoObjectType,
                          kExtmarkClear, kExtmarkSavePos, kExtmarkUpdate,
                          kExtmarkMove, kExtmarkSplice};
pub use self::fs_defs_h::FileID;
pub use self::memline_defs_h::{memline_T, memline, chunksize_T, ml_chunksize,
                               infoptr_T, info_pointer};
pub use self::memfile_defs_h::{bhdr_T, bhdr, mf_hashitem_T, mf_hashitem,
                               blocknr_T, memfile_T, memfile, mf_hashtab_T,
                               mf_hashtab};
pub use self::highlight_defs_h::{C2RustUnnamed_13, HLF_COUNT, HLF_MSG,
                                 HLF_NFLOAT, HLF_MSGSEP, HLF_INACTIVE, HLF_0,
                                 HLF_QFL, HLF_MC, HLF_CUL, HLF_CUC, HLF_TPF,
                                 HLF_TPS, HLF_TP, HLF_PST, HLF_PSB, HLF_PSI,
                                 HLF_PNI, HLF_SPL, HLF_SPR, HLF_SPC, HLF_SPB,
                                 HLF_CONCEAL, HLF_SC, HLF_TXD, HLF_DED,
                                 HLF_CHD, HLF_ADD, HLF_FC, HLF_FL, HLF_WM,
                                 HLF_W, HLF_VNC, HLF_V, HLF_T, HLF_C, HLF_SNC,
                                 HLF_S, HLF_R, HLF_CLN, HLF_N, HLF_CM, HLF_M,
                                 HLF_L, HLF_I, HLF_E, HLF_D, HLF_AT,
                                 HLF_TERMNC, HLF_TERM, HLF_EOB, HLF_8,
                                 highlight_attr};
pub use self::struct_iovec_h::iovec;
pub use self::uv_h::{uv_loop_s, uv_signal_t, uv_signal_s, C2RustUnnamed_15,
                     uv_signal_cb, uv_handle_t, uv_handle_s, C2RustUnnamed_16,
                     uv_close_cb, uv_handle_type, UV_HANDLE_TYPE_MAX, UV_FILE,
                     UV_SIGNAL, UV_UDP, UV_TTY, UV_TIMER, UV_TCP, UV_STREAM,
                     UV_PROCESS, UV_PREPARE, UV_POLL, UV_NAMED_PIPE, UV_IDLE,
                     UV_HANDLE, UV_FS_POLL, UV_FS_EVENT, UV_CHECK, UV_ASYNC,
                     UV_UNKNOWN_HANDLE, uv_loop_t, C2RustUnnamed_17,
                     C2RustUnnamed_18, uv_async_t, uv_async_s, uv_async_cb,
                     C2RustUnnamed_19, C2RustUnnamed_20, uv_req_type,
                     UV_REQ_TYPE_MAX, UV_GETNAMEINFO, UV_GETADDRINFO, UV_WORK,
                     UV_FS, UV_UDP_SEND, UV_SHUTDOWN, UV_WRITE, UV_CONNECT,
                     UV_REQ, UV_UNKNOWN_REQ, uv_stream_s, uv_connection_cb,
                     uv_stream_t, uv_shutdown_t, uv_shutdown_s,
                     uv_shutdown_cb, uv_connect_t, uv_connect_s,
                     uv_connect_cb, uv_read_cb, uv_alloc_cb, C2RustUnnamed_21,
                     uv_tcp_s, C2RustUnnamed_22, uv_tcp_t, uv_pipe_s,
                     C2RustUnnamed_23, uv_pipe_t, uv_timer_s, uv_timer_cb,
                     uv_timer_t, C2RustUnnamed_24, uv_idle_s, uv_idle_cb,
                     uv_idle_t, C2RustUnnamed_25, uv_process_s, uv_exit_cb,
                     uv_process_t, C2RustUnnamed_26, uv_stdio_flags,
                     UV_OVERLAPPED_PIPE, UV_WRITABLE_PIPE, UV_READABLE_PIPE,
                     UV_INHERIT_STREAM, UV_INHERIT_FD, UV_CREATE_PIPE,
                     UV_IGNORE, uv_stdio_container_s, C2RustUnnamed_27,
                     uv_stdio_container_t, uv_process_options_s,
                     uv_process_options_t};
pub use self::unix_h::{uv__io_t, uv__io_s, uv__io_cb, uv_rwlock_t, uv_mutex_t,
                       uv_buf_t, uv_file, uv_gid_t, uv_uid_t};
pub use self::normal_h::{MotionType, kMTUnknown, kMTBlockWise, kMTLineWise,
                         kMTCharWise, oparg_S, oparg_T};
pub use self::ex_cmds_enum_generated_h::{CMD_index, CMD_USER_BUF, CMD_USER,
                                         CMD_SIZE, CMD_tilde, CMD_Next,
                                         CMD_at, CMD_rshift, CMD_equal,
                                         CMD_lshift, CMD_and, CMD_pound,
                                         CMD_bang, CMD_z, CMD_yank,
                                         CMD_xunmenu, CMD_xunmap,
                                         CMD_xnoremenu, CMD_xnoremap,
                                         CMD_xmenu, CMD_xmapclear, CMD_xmap,
                                         CMD_xall, CMD_xit, CMD_wviminfo,
                                         CMD_wundo, CMD_wshada, CMD_wqall,
                                         CMD_wq, CMD_wprevious, CMD_wnext,
                                         CMD_winpos, CMD_windo, CMD_wincmd,
                                         CMD_winsize, CMD_while, CMD_wall,
                                         CMD_wNext, CMD_write, CMD_vunmenu,
                                         CMD_vunmap, CMD_vsplit,
                                         CMD_vnoremenu, CMD_vnew,
                                         CMD_vnoremap, CMD_vmenu,
                                         CMD_vmapclear, CMD_vmap, CMD_viusage,
                                         CMD_vimgrepadd, CMD_vimgrep,
                                         CMD_view, CMD_visual, CMD_vertical,
                                         CMD_verbose, CMD_version,
                                         CMD_vglobal, CMD_update,
                                         CMD_unsilent, CMD_unmenu, CMD_unmap,
                                         CMD_unlockvar, CMD_unlet, CMD_unhide,
                                         CMD_unabbreviate, CMD_undolist,
                                         CMD_undojoin, CMD_undo, CMD_tunmap,
                                         CMD_tunmenu, CMD_tselect, CMD_try,
                                         CMD_trewind, CMD_tprevious,
                                         CMD_topleft, CMD_tnoremap, CMD_tnext,
                                         CMD_tmapclear, CMD_tmap, CMD_tmenu,
                                         CMD_tlast, CMD_tjump, CMD_throw,
                                         CMD_tfirst, CMD_terminal,
                                         CMD_tclfile, CMD_tcldo, CMD_tcl,
                                         CMD_tabs, CMD_tabrewind, CMD_tabNext,
                                         CMD_tabprevious, CMD_tabonly,
                                         CMD_tabnew, CMD_tabnext, CMD_tablast,
                                         CMD_tabmove, CMD_tabfirst,
                                         CMD_tabfind, CMD_tabedit, CMD_tabdo,
                                         CMD_tabclose, CMD_tab, CMD_tags,
                                         CMD_tag, CMD_tNext, CMD_tchdir,
                                         CMD_tcd, CMD_t, CMD_syncbind,
                                         CMD_syntime, CMD_syntax,
                                         CMD_swapname, CMD_sview, CMD_suspend,
                                         CMD_sunmenu, CMD_sunmap, CMD_sunhide,
                                         CMD_stselect, CMD_stjump,
                                         CMD_stopinsert, CMD_startreplace,
                                         CMD_startgreplace, CMD_startinsert,
                                         CMD_stag, CMD_stop, CMD_srewind,
                                         CMD_sprevious, CMD_spellwrong,
                                         CMD_spellundo, CMD_spellrepall,
                                         CMD_spellinfo, CMD_spelldump,
                                         CMD_spellgood, CMD_split, CMD_sort,
                                         CMD_source, CMD_snoremenu,
                                         CMD_snoremap, CMD_snomagic,
                                         CMD_snext, CMD_smenu, CMD_smapclear,
                                         CMD_smap, CMD_smagic, CMD_slast,
                                         CMD_sleep, CMD_silent, CMD_sign,
                                         CMD_simalt, CMD_sfirst, CMD_sfind,
                                         CMD_setlocal, CMD_setglobal,
                                         CMD_setfiletype, CMD_set,
                                         CMD_scscope, CMD_scriptencoding,
                                         CMD_scriptnames, CMD_sbrewind,
                                         CMD_sbprevious, CMD_sbnext,
                                         CMD_sbmodified, CMD_sblast,
                                         CMD_sbfirst, CMD_sball, CMD_sbNext,
                                         CMD_sbuffer, CMD_saveas, CMD_sandbox,
                                         CMD_sall, CMD_sargument, CMD_sNext,
                                         CMD_substitute, CMD_rviminfo,
                                         CMD_rubyfile, CMD_rubydo, CMD_ruby,
                                         CMD_rundo, CMD_runtime, CMD_rshada,
                                         CMD_rightbelow, CMD_right,
                                         CMD_rewind, CMD_return, CMD_retab,
                                         CMD_resize, CMD_registers,
                                         CMD_redrawtabline, CMD_redrawstatus,
                                         CMD_redraw, CMD_redir, CMD_redo,
                                         CMD_recover, CMD_read, CMD_qall,
                                         CMD_quitall, CMD_quit, CMD_pyxfile,
                                         CMD_pythonx, CMD_pyxdo, CMD_pyx,
                                         CMD_py3file, CMD_python3, CMD_py3do,
                                         CMD_py3, CMD_pyfile, CMD_pydo,
                                         CMD_python, CMD_pwd, CMD_put,
                                         CMD_ptselect, CMD_ptrewind,
                                         CMD_ptprevious, CMD_ptnext,
                                         CMD_ptlast, CMD_ptjump, CMD_ptfirst,
                                         CMD_ptNext, CMD_ptag, CMD_psearch,
                                         CMD_profdel, CMD_profile,
                                         CMD_previous, CMD_preserve, CMD_ppop,
                                         CMD_popup, CMD_pop, CMD_pedit,
                                         CMD_perldo, CMD_perl, CMD_pclose,
                                         CMD_packloadall, CMD_packadd,
                                         CMD_print, CMD_ownsyntax,
                                         CMD_ounmenu, CMD_ounmap, CMD_options,
                                         CMD_onoremenu, CMD_onoremap,
                                         CMD_only, CMD_omenu, CMD_omapclear,
                                         CMD_omap, CMD_oldfiles, CMD_nunmenu,
                                         CMD_nunmap, CMD_number, CMD_normal,
                                         CMD_noswapfile, CMD_noremenu,
                                         CMD_noreabbrev, CMD_nohlsearch,
                                         CMD_noautocmd, CMD_noremap,
                                         CMD_nnoremenu, CMD_nnoremap,
                                         CMD_nmenu, CMD_nmapclear, CMD_nmap,
                                         CMD_new, CMD_next, CMD_mzfile,
                                         CMD_mzscheme, CMD_mode, CMD_mkview,
                                         CMD_mkvimrc, CMD_mkspell,
                                         CMD_mksession, CMD_mkexrc,
                                         CMD_messages, CMD_menutranslate,
                                         CMD_menu, CMD_match, CMD_marks,
                                         CMD_mapclear, CMD_map, CMD_make,
                                         CMD_mark, CMD_move, CMD_ls,
                                         CMD_lwindow, CMD_lvimgrepadd,
                                         CMD_lvimgrep, CMD_luafile, CMD_luado,
                                         CMD_lua, CMD_lunmap, CMD_ltag,
                                         CMD_lrewind, CMD_lpfile,
                                         CMD_lprevious, CMD_lopen, CMD_lolder,
                                         CMD_lockvar, CMD_lockmarks,
                                         CMD_loadkeymap, CMD_loadview,
                                         CMD_lnfile, CMD_lnewer, CMD_lnext,
                                         CMD_lnoremap, CMD_lmake,
                                         CMD_lmapclear, CMD_lmap, CMD_llist,
                                         CMD_llast, CMD_ll, CMD_lhistory,
                                         CMD_lhelpgrep, CMD_lgrepadd,
                                         CMD_lgrep, CMD_lgetexpr,
                                         CMD_lgetbuffer, CMD_lgetfile,
                                         CMD_lfirst, CMD_lfdo, CMD_lfile,
                                         CMD_lexpr, CMD_let, CMD_leftabove,
                                         CMD_left, CMD_ldo, CMD_lcscope,
                                         CMD_lclose, CMD_lchdir, CMD_lcd,
                                         CMD_lbottom, CMD_lbelow, CMD_lbuffer,
                                         CMD_later, CMD_laddfile,
                                         CMD_laddbuffer, CMD_laddexpr,
                                         CMD_language, CMD_labove, CMD_last,
                                         CMD_lNfile, CMD_lNext, CMD_list,
                                         CMD_keepalt, CMD_keeppatterns,
                                         CMD_keepjumps, CMD_keepmarks, CMD_k,
                                         CMD_jumps, CMD_join, CMD_iunmenu,
                                         CMD_iunabbrev, CMD_iunmap,
                                         CMD_isplit, CMD_isearch, CMD_intro,
                                         CMD_inoremenu, CMD_inoreabbrev,
                                         CMD_inoremap, CMD_imenu,
                                         CMD_imapclear, CMD_imap, CMD_ilist,
                                         CMD_ijump, CMD_if, CMD_iabclear,
                                         CMD_iabbrev, CMD_insert, CMD_history,
                                         CMD_hide, CMD_highlight,
                                         CMD_hardcopy, CMD_helptags,
                                         CMD_helpgrep, CMD_helpclose,
                                         CMD_help, CMD_gvim, CMD_gui,
                                         CMD_grepadd, CMD_grep, CMD_goto,
                                         CMD_global, CMD_function, CMD_for,
                                         CMD_foldopen, CMD_folddoclosed,
                                         CMD_folddoopen, CMD_foldclose,
                                         CMD_fold, CMD_first, CMD_finish,
                                         CMD_finally, CMD_find, CMD_filter,
                                         CMD_filetype, CMD_files, CMD_file,
                                         CMD_exusage, CMD_exit, CMD_execute,
                                         CMD_ex, CMD_enew, CMD_endwhile,
                                         CMD_endtry, CMD_endfor,
                                         CMD_endfunction, CMD_endif,
                                         CMD_emenu, CMD_elseif, CMD_else,
                                         CMD_echon, CMD_echomsg, CMD_echohl,
                                         CMD_echoerr, CMD_echo, CMD_earlier,
                                         CMD_edit, CMD_dsplit, CMD_dsearch,
                                         CMD_drop, CMD_doautoall,
                                         CMD_doautocmd, CMD_dlist, CMD_djump,
                                         CMD_digraphs, CMD_diffthis,
                                         CMD_diffsplit, CMD_diffput,
                                         CMD_diffpatch, CMD_diffoff,
                                         CMD_diffget, CMD_diffupdate,
                                         CMD_display, CMD_delfunction,
                                         CMD_delcommand, CMD_debuggreedy,
                                         CMD_debug, CMD_delmarks, CMD_delete,
                                         CMD_cwindow, CMD_cunmenu,
                                         CMD_cunabbrev, CMD_cunmap, CMD_cstag,
                                         CMD_cscope, CMD_crewind, CMD_cquit,
                                         CMD_cpfile, CMD_cprevious, CMD_copen,
                                         CMD_const, CMD_confirm, CMD_continue,
                                         CMD_compiler, CMD_comclear,
                                         CMD_command, CMD_colorscheme,
                                         CMD_colder, CMD_copy, CMD_cnoremenu,
                                         CMD_cnoreabbrev, CMD_cnoremap,
                                         CMD_cnfile, CMD_cnewer, CMD_cnext,
                                         CMD_cmenu, CMD_cmapclear, CMD_cmap,
                                         CMD_clearjumps, CMD_close, CMD_clast,
                                         CMD_clist, CMD_chistory,
                                         CMD_checktime, CMD_checkpath,
                                         CMD_checkhealth, CMD_changes,
                                         CMD_chdir, CMD_cgetexpr,
                                         CMD_cgetbuffer, CMD_cgetfile,
                                         CMD_cfirst, CMD_cfdo, CMD_cfile,
                                         CMD_cexpr, CMD_center, CMD_cdo,
                                         CMD_cd, CMD_cclose, CMD_cc,
                                         CMD_cbottom, CMD_cbelow, CMD_cbuffer,
                                         CMD_catch, CMD_call, CMD_caddfile,
                                         CMD_caddexpr, CMD_caddbuffer,
                                         CMD_cabove, CMD_cabclear,
                                         CMD_cabbrev, CMD_cNfile, CMD_cNext,
                                         CMD_change, CMD_bwipeout,
                                         CMD_bunload, CMD_bufdo, CMD_buffers,
                                         CMD_browse, CMD_breaklist,
                                         CMD_breakdel, CMD_breakadd,
                                         CMD_break, CMD_brewind,
                                         CMD_bprevious, CMD_botright,
                                         CMD_bnext, CMD_bmodified, CMD_blast,
                                         CMD_bfirst, CMD_belowright,
                                         CMD_behave, CMD_bdelete, CMD_badd,
                                         CMD_ball, CMD_bNext, CMD_buffer,
                                         CMD_aunmenu, CMD_augroup,
                                         CMD_autocmd, CMD_ascii, CMD_argument,
                                         CMD_arglocal, CMD_argglobal,
                                         CMD_argedit, CMD_argdo,
                                         CMD_argdelete, CMD_argadd, CMD_args,
                                         CMD_anoremenu, CMD_amenu, CMD_all,
                                         CMD_aboveleft, CMD_abclear,
                                         CMD_abbreviate, CMD_append,
                                         cmdidx_T};
pub use self::ex_cmds_defs_h::{exarg, cstack_T, eslist_T, eslist_elem,
                               C2RustUnnamed_28, LineGetter, exarg_T,
                               cmdmod_T};
pub use self::event_defs_h::{argv_callback, message, Event, event_create};
pub use self::multiqueue_h::{MultiQueue, multiqueue};
pub use self::loop_h::{WatcherPtr, __kl1_WatcherPtr, kl1_WatcherPtr,
                       kmp_WatcherPtr_t, kl_WatcherPtr_t, loop_0, Loop,
                       kmp_init_WatcherPtr, kmp_destroy_WatcherPtr,
                       kmp_alloc_WatcherPtr, kmp_free_WatcherPtr,
                       kl_init_WatcherPtr, kl_destroy_WatcherPtr,
                       kl_push_WatcherPtr, kl_shift_at_WatcherPtr};
pub use self::buffer_h::{GETF_SETMARK, getf_values, GETF_SWITCH, GETF_ALT,
                         buf_inc_changedtick, buf_get_changedtick,
                         buf_set_changedtick};
pub use self::rbuffer_h::{rbuffer, rbuffer_callback, RBuffer, rbuffer_size};
pub use self::stream_h::{stream, stream_close_cb, Stream, stream_write_cb,
                         stream_read_cb, C2RustUnnamed_29};
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
pub use self::pack_template_h::{C2RustUnnamed_30, C2RustUnnamed_31,
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
                             msgpack_vrefbuffer_init,
                             msgpack_vrefbuffer_destroy,
                             msgpack_vrefbuffer_append_ref,
                             msgpack_vrefbuffer_append_copy};
pub use self::channel_h::{Channel, CallbackReader, C2RustUnnamed_33,
                          StderrState, StdioPair, ChannelStreamType,
                          kChannelStreamInternal, kChannelStreamStderr,
                          kChannelStreamStdio, kChannelStreamSocket,
                          kChannelStreamProc, callback_reader_set,
                          find_channel, channel_instream, channel_outstream,
                          channels};
pub use self::channel_defs_h::{RpcState, C2RustUnnamed_32, ChannelCallFrame};
pub use self::fileio_h::{FileDescriptor, file_eof, file_fd};
pub use self::assert_h::{__ASSERT_FUNCTION, __assert_fail};
use self::string_h::{memcpy, memmove, memset, strcmp, strlen};
pub use self::limits_h::PATH_MAX;
use self::stdlib_h::{malloc, calloc, realloc, free, abort};
use self::log_h_generated_h::logmsg;
pub use self::stdbool_h::{true_0, false_0};
pub use self::log_h::WARN_LOG_LEVEL;
use self::mbyte_h_generated_h::{utf_ptr2char, utfc_ptr2len, mb_stricmp,
                                utf_head_off};
pub use self::mbyte_h::{mb_strcmp_ic, mb_ptr2len};
use self::libintl_h::gettext;
use self::message_h_generated_h::{msg, emsg, emsgf, msg_putchar, msg_outtrans,
                                  msg_outtrans_attr, msg_puts, msg_puts_title,
                                  message_filtered};
use self::typval_h_generated_h::{tv_dict_watcher_notify, tv_dict_unref,
                                 tv_dict_find};
pub use self::strings_h::strappend;
use self::strings_h_generated_h::{vim_strsave, vim_strnsave, vim_strchr};
pub use self::os_defs_h::MAXPATHL;
pub use self::globals_h::{IOSIZE, e_umark, e_marknotset, e_markinval,
                          e_invarg2, e_invarg, e_argreq, Columns, firstwin,
                          curwin, first_tabpage, curtab, curbuf, saved_cursor,
                          cmdmod, IObuff, NameBuff, got_int, global_busy,
                          listcmd_busy};
use self::memory_h_generated_h::{xfree, xcalloc, xrealloc, xstrlcpy};
pub use self::kvec_h::_memcpy_free;
use self::in_h::{ntohl, ntohs};
pub use self::uintn_identity_h::{__uint64_identity, __uint32_identity,
                                 __uint16_identity};
pub use self::byteswap_h::{__bswap_64, __bswap_32, __bswap_16};
pub use self::ascii_h::{NUL, TAB, ascii_iswhite, ascii_isdigit,
                        ascii_isxdigit, ascii_isident, ascii_isbdigit,
                        ascii_isspace};
pub use self::mark_h::{mark_global_index, mark_local_index, lt, equalpos,
                       ltoreq, clearpos};
use self::buffer_h_generated_h::{buflist_getfile, buflist_new,
                                 buflist_nr2name, buflist_findnr};
pub use self::charset_h::{vim_isbreak, ptr2cells, vim_isprintc, skipwhite};
use self::diff_h_generated_h::diff_mark_adjust;
use self::env_h_generated_h::expand_env;
use self::fs_h_generated_h::os_dirname;
use self::fold_h_generated_h::foldMarkAdjust;
use self::extmark_h_generated_h::extmark_adjust;
use self::memline_h_generated_h::{ml_get, ml_get_buf};
use self::path_h_generated_h::{path_fnamecmp, path_shorten_fname};
use self::quickfix_h_generated_h::qf_mark_adjust;
use self::search_h_generated_h::{findsent, findpar};
use self::sign_h_generated_h::sign_mark_adjust;
use self::ui_h_generated_h::ui_flush;
use self::input_h_generated_h::os_breakcheck;
// This is an open source non-commercial project. Dear PVS-Studio, please check
// it. PVS-Studio Static Code Analyzer for C, C++ and C#: http://www.viva64.com
/*
 * mark.c: functions for setting marks and jumping to them
 */
/*
 * This file contains routines to maintain and manipulate marks.
 */
/*
 * If a named file mark's lnum is non-zero, it is valid.
 * If a named file mark's fnum is non-zero, it is for an existing buffer,
 * otherwise it is from .shada and namedfm[n].fname is the file name.
 * There are marks 'A - 'Z (set by user) and '0 to '9 (set when writing
 * shada).
 */
// / Global marks (marks with file number or name)
#[c2rust::src_loc = "53:17"]
static mut namedfm: [xfmark_T; 36] =
    [xfmark_T{fmark:
                  fmark_T{mark: pos_T{lnum: 0, col: 0, coladd: 0,},
                          fnum: 0,
                          timestamp: 0,
                          additional_data:
                              0 as *const dict_T as *mut dict_T,},
              fname: 0 as *const char_u as *mut char_u,}; 36];
/*
 * Set named mark "c" at current cursor position.
 * Returns OK on success, FAIL if bad name given.
 */
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn setmark(mut c: libc::c_int) -> libc::c_int {
    return setmark_pos(c, &mut (*curwin).w_cursor, (*curbuf).handle);
}
// / Free fmark_T item
#[no_mangle]
#[c2rust::src_loc = "68:1"]
pub unsafe extern "C" fn free_fmark(mut fm: fmark_T) {
    tv_dict_unref(fm.additional_data);
}
// / Free xfmark_T item
#[no_mangle]
#[c2rust::src_loc = "74:1"]
pub unsafe extern "C" fn free_xfmark(mut fm: xfmark_T) {
    xfree(fm.fname as *mut libc::c_void);
    free_fmark(fm.fmark);
}
// / Free and clear fmark_T item
#[no_mangle]
#[c2rust::src_loc = "81:1"]
pub unsafe extern "C" fn clear_fmark(mut fm: *mut fmark_T) {
    free_fmark(*fm);
    memset(fm as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<fmark_T>() as libc::c_ulong);
}
/*
 * Set named mark "c" to position "pos".
 * When "c" is upper case use file "fnum".
 * Returns OK on success, FAIL if bad name given.
 */
#[no_mangle]
#[c2rust::src_loc = "93:1"]
pub unsafe extern "C" fn setmark_pos(mut c: libc::c_int, mut pos: *mut pos_T,
                                     mut fnum: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* Check for a special key (may cause islower() to crash). */
    if c < 0 as libc::c_int { return FAIL }
    if c == '\'' as i32 || c == '`' as i32 {
        if pos == &mut (*curwin).w_cursor as *mut pos_T {
            setpcmark();
            /* keep it even when the cursor doesn't move */
            (*curwin).w_prev_pcmark = (*curwin).w_pcmark
        } else { (*curwin).w_pcmark = *pos }
        return OK
    }
    // Can't set a mark in a non-existant buffer.
    let mut buf = buflist_findnr(fnum);
    if buf.is_null() { return FAIL }
    if c == '\"' as i32 {
        let fmarkp___: *mut fmark_T = &mut (*buf).b_last_cursor;
        free_fmark(*fmarkp___);
        let fmarkp__ = fmarkp___;
        (*fmarkp__).mark = *pos;
        (*fmarkp__).fnum = (*buf).handle;
        (*fmarkp__).timestamp = os_time();
        (*fmarkp__).additional_data = NULL_1 as *mut dict_T;
        return OK
    }
    /* Allow setting '[ and '] for an autocommand that simulates reading a
   * file. */
    if c == '[' as i32 { (*buf).b_op_start = *pos; return OK }
    if c == ']' as i32 { (*buf).b_op_end = *pos; return OK }
    if c == '<' as i32 || c == '>' as i32 {
        if c == '<' as i32 {
            (*buf).b_visual.vi_start = *pos
        } else { (*buf).b_visual.vi_end = *pos }
        if (*buf).b_visual.vi_mode == NUL {
            // Visual_mode has not yet been set, use a sane default.
            (*buf).b_visual.vi_mode = 'v' as i32
        }
        return OK
    }
    if c as libc::c_uint >= 'a' as i32 as libc::c_uint &&
           c as libc::c_uint <= 'z' as i32 as libc::c_uint {
        i = c - 'a' as i32;
        let fmarkp____0 = (*buf).b_namedm.as_mut_ptr().offset(i as isize);
        free_fmark(*fmarkp____0);
        let fmarkp___0 = fmarkp____0;
        (*fmarkp___0).mark = *pos;
        (*fmarkp___0).fnum = fnum;
        (*fmarkp___0).timestamp = os_time();
        (*fmarkp___0).additional_data = NULL_1 as *mut dict_T;
        return OK
    }
    if c as libc::c_uint >= 'A' as i32 as libc::c_uint &&
           c as libc::c_uint <= 'Z' as i32 as libc::c_uint ||
           ascii_isdigit(c) as libc::c_int != 0 {
        if ascii_isdigit(c) {
            i = c - '0' as i32 + NMARKS
        } else { i = c - 'A' as i32 }
        let xfmarkp__ = namedfm.as_mut_ptr().offset(i as isize);
        free_xfmark(*xfmarkp__);
        (*xfmarkp__).fname = 0 as *mut char_u;
        let fmarkp___1: *mut fmark_T = &mut (*xfmarkp__).fmark;
        (*fmarkp___1).mark = *pos;
        (*fmarkp___1).fnum = fnum;
        (*fmarkp___1).timestamp = os_time();
        (*fmarkp___1).additional_data = NULL_1 as *mut dict_T;
        return OK
    }
    return FAIL;
}
/*
 * Set the previous context mark to the current position and add it to the
 * jump list.
 */
#[no_mangle]
#[c2rust::src_loc = "167:1"]
pub unsafe extern "C" fn setpcmark() {
    let mut fm = 0 as *mut xfmark_T;
    /* for :global the mark is set only once */
    if global_busy != 0 || listcmd_busy != 0 ||
           cmdmod.keepjumps as libc::c_int != 0 {
        return
    }
    (*curwin).w_prev_pcmark = (*curwin).w_pcmark;
    (*curwin).w_pcmark = (*curwin).w_cursor;
    if (*curwin).w_pcmark.lnum == 0 as libc::c_int as libc::c_long {
        (*curwin).w_pcmark.lnum = 1 as libc::c_int as linenr_T
    }
    if jop_flags & JOP_STACK as libc::c_uint != 0 {
        // jumpoptions=stack: if we're somewhere in the middle of the jumplist
    // discard everything after the current index.
        if (*curwin).w_jumplistidx <
               (*curwin).w_jumplistlen - 1 as libc::c_int {
            // Discard the rest of the jumplist by cutting the length down to
      // contain nothing beyond the current index.
            (*curwin).w_jumplistlen =
                (*curwin).w_jumplistidx + 1 as libc::c_int
        }
    }
    /* If jumplist is full: remove oldest entry */
    (*curwin).w_jumplistlen += 1;
    if (*curwin).w_jumplistlen > JUMPLISTSIZE {
        (*curwin).w_jumplistlen = JUMPLISTSIZE;
        free_xfmark((*curwin).w_jumplist[0 as libc::c_int as usize]);
        memmove(&mut *(*curwin).w_jumplist.as_mut_ptr().offset(0 as
                                                                   libc::c_int
                                                                   as isize)
                    as *mut xfmark_T as *mut libc::c_void,
                &mut *(*curwin).w_jumplist.as_mut_ptr().offset(1 as
                                                                   libc::c_int
                                                                   as isize)
                    as *mut xfmark_T as *const libc::c_void,
                ((JUMPLISTSIZE - 1 as libc::c_int) as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<xfmark_T>()
                                                     as libc::c_ulong));
    }
    (*curwin).w_jumplistidx = (*curwin).w_jumplistlen;
    fm =
        &mut *(*curwin).w_jumplist.as_mut_ptr().offset(((*curwin).w_jumplistlen
                                                            -
                                                            1 as libc::c_int)
                                                           as isize) as
            *mut xfmark_T;
    let xfmarkp__ = fm;
    (*xfmarkp__).fname = 0 as *mut char_u;
    let fmarkp__: *mut fmark_T = &mut (*xfmarkp__).fmark;
    (*fmarkp__).mark = (*curwin).w_pcmark;
    (*fmarkp__).fnum = (*curbuf).handle;
    (*fmarkp__).timestamp = os_time();
    (*fmarkp__).additional_data = NULL_1 as *mut dict_T;
}
/*
 * To change context, call setpcmark(), then move the current position to
 * where ever, then call checkpcmark().  This ensures that the previous
 * context will only be changed if the cursor moved to a different line.
 * If pcmark was deleted (with "dG") the previous mark is restored.
 */
#[no_mangle]
#[c2rust::src_loc = "211:1"]
pub unsafe extern "C" fn checkpcmark() {
    if (*curwin).w_prev_pcmark.lnum != 0 as libc::c_int as libc::c_long &&
           (equalpos((*curwin).w_pcmark, (*curwin).w_cursor) as libc::c_int !=
                0 ||
                (*curwin).w_pcmark.lnum == 0 as libc::c_int as libc::c_long) {
        (*curwin).w_pcmark = (*curwin).w_prev_pcmark;
        (*curwin).w_prev_pcmark.lnum = 0 as libc::c_int as linenr_T
        /* Show it has been checked */
    };
}
/*
 * move "count" positions in the jump list (count may be negative)
 */
#[no_mangle]
#[c2rust::src_loc = "224:1"]
pub unsafe extern "C" fn movemark(mut count: libc::c_int) -> *mut pos_T {
    let mut pos = 0 as *mut pos_T;
    let mut jmp = 0 as *mut xfmark_T;
    cleanup_jumplist(curwin, true_0 != 0);
    if (*curwin).w_jumplistlen == 0 as libc::c_int {
        /* nothing to jump to */
        return NULL_1 as *mut libc::c_void as *mut pos_T
    }
    loop  {
        if (*curwin).w_jumplistidx + count < 0 as libc::c_int ||
               (*curwin).w_jumplistidx + count >= (*curwin).w_jumplistlen {
            return NULL_1 as *mut libc::c_void as *mut pos_T
        }
        /*
     * if first CTRL-O or CTRL-I command after a jump, add cursor position
     * to list.  Careful: If there are duplicates (CTRL-O immediately after
     * starting Vim on a file), another entry may have been removed.
     */
        if (*curwin).w_jumplistidx == (*curwin).w_jumplistlen {
            setpcmark(); /* skip the new entry */
            (*curwin).w_jumplistidx -= 1;
            if (*curwin).w_jumplistidx + count < 0 as libc::c_int {
                return NULL_1 as *mut libc::c_void as *mut pos_T
            }
        }
        (*curwin).w_jumplistidx += count;
        jmp =
            (*curwin).w_jumplist.as_mut_ptr().offset((*curwin).w_jumplistidx
                                                         as isize);
        if (*jmp).fmark.fnum == 0 as libc::c_int { fname2fnum(jmp); }
        if (*jmp).fmark.fnum != (*curbuf).handle {
            /* jump to other file */
            if buflist_findnr((*jmp).fmark.fnum).is_null() {
                /* Skip this one .. */
                count +=
                    if count < 0 as libc::c_int {
                        -(1 as libc::c_int)
                    } else { 1 as libc::c_int };
                continue ;
            } else {
                if buflist_getfile((*jmp).fmark.fnum, (*jmp).fmark.mark.lnum,
                                   0 as libc::c_int, FALSE) == FAIL {
                    return NULL_1 as *mut libc::c_void as *mut pos_T
                }
                /* Set lnum again, autocommands my have changed it */
                (*curwin).w_cursor = (*jmp).fmark.mark;
                pos = -(1 as libc::c_int) as *mut pos_T
            }
        } else { pos = &mut (*jmp).fmark.mark }
        return pos
    };
}
/*
 * Move "count" positions in the changelist (count may be negative).
 */
#[no_mangle]
#[c2rust::src_loc = "277:1"]
pub unsafe extern "C" fn movechangelist(mut count: libc::c_int)
 -> *mut pos_T {
    let mut n: libc::c_int = 0;
    if (*curbuf).b_changelistlen == 0 as libc::c_int {
        /* nothing to jump to */
        return NULL_1 as *mut libc::c_void as *mut pos_T
    }
    n = (*curwin).w_changelistidx;
    if n + count < 0 as libc::c_int {
        if n == 0 as libc::c_int {
            return NULL_1 as *mut libc::c_void as *mut pos_T
        }
        n = 0 as libc::c_int
    } else if n + count >= (*curbuf).b_changelistlen {
        if n == (*curbuf).b_changelistlen - 1 as libc::c_int {
            return NULL_1 as *mut libc::c_void as *mut pos_T
        }
        n = (*curbuf).b_changelistlen - 1 as libc::c_int
    } else { n += count }
    (*curwin).w_changelistidx = n;
    return &mut (*(*curbuf).b_changelist.as_mut_ptr().offset(n as
                                                                 isize)).mark;
}
/*
 * Find mark "c" in buffer pointed to by "buf".
 * If "changefile" is TRUE it's allowed to edit another file for '0, 'A, etc.
 * If "fnum" is not NULL store the fnum there for '0, 'A etc., don't edit
 * another file.
 * Returns:
 * - pointer to pos_T if found.  lnum is 0 when mark not set, -1 when mark is
 *   in another file which can't be gotten. (caller needs to check lnum!)
 * - NULL if there is no mark called 'c'.
 * - -1 if mark is in other file and jumped there (only if changefile is TRUE)
 */
#[no_mangle]
#[c2rust::src_loc = "310:1"]
pub unsafe extern "C" fn getmark_buf(mut buf: *mut buf_T, mut c: libc::c_int,
                                     mut changefile: bool) -> *mut pos_T {
    return getmark_buf_fnum(buf, c, changefile, NULL_1 as *mut libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "315:1"]
pub unsafe extern "C" fn getmark(mut c: libc::c_int, mut changefile: bool)
 -> *mut pos_T {
    return getmark_buf_fnum(curbuf, c, changefile,
                            NULL_1 as *mut libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "320:1"]
pub unsafe extern "C" fn getmark_buf_fnum(mut buf: *mut buf_T,
                                          mut c: libc::c_int,
                                          mut changefile: bool,
                                          mut fnum: *mut libc::c_int)
 -> *mut pos_T {
    let mut posp = 0 as *mut pos_T;
    let mut startp = 0 as *mut pos_T;
    let mut endp = 0 as *mut pos_T;
    static mut pos_copy: pos_T = pos_T{lnum: 0, col: 0, coladd: 0,};
    posp = NULL_1 as *mut pos_T;
    /* Check for special key, can't be a mark name and might cause islower()
   * to crash. */
    if c < 0 as libc::c_int { return posp }
    if !(c > '~' as i32) {
        if c == '\'' as i32 || c == '`' as i32 {
            // previous context mark
            pos_copy = (*curwin).w_pcmark;
            posp = &mut pos_copy // need to make a copy because
            //   w_pcmark may be changed soon
        } else if c == '\"' as i32 {
            // to pos when leaving buffer
            posp = &mut (*buf).b_last_cursor.mark
        } else if c == '^' as i32 {
            // to where Insert mode stopped
            posp = &mut (*buf).b_last_insert.mark
        } else if c == '.' as i32 {
            // to where last change was made
            posp = &mut (*buf).b_last_change.mark
        } else if c == '[' as i32 {
            // to start of previous operator
            posp = &mut (*buf).b_op_start
        } else if c == ']' as i32 {
            // to end of previous operator
            posp = &mut (*buf).b_op_end
        } else if c == '{' as i32 || c == '}' as i32 {
            // to previous/next paragraph
            let mut pos =
                pos_T{lnum: 0,
                      col: 0,
                      coladd: 0,}; /* avoid that '' is changed */
            let mut oa =
                oparg_T{op_type: 0,
                        regname: 0,
                        motion_type: kMTCharWise,
                        motion_force: 0,
                        use_reg_one: false,
                        inclusive: false,
                        end_adjusted: false,
                        start: pos_T{lnum: 0, col: 0, coladd: 0,},
                        end: pos_T{lnum: 0, col: 0, coladd: 0,},
                        cursor_start: pos_T{lnum: 0, col: 0, coladd: 0,},
                        line_count: 0,
                        empty: false,
                        is_VIsual: false,
                        start_vcol: 0,
                        end_vcol: 0,
                        prev_opcount: 0,
                        prev_count0: 0,};
            let mut slcb = listcmd_busy;
            pos = (*curwin).w_cursor;
            listcmd_busy = TRUE;
            if findpar(&mut oa.inclusive,
                       if c == '}' as i32 {
                           FORWARD as libc::c_int
                       } else { BACKWARD as libc::c_int }, 1 as libc::c_long,
                       NUL, FALSE) {
                pos_copy = (*curwin).w_cursor;
                posp = &mut pos_copy
            }
            (*curwin).w_cursor = pos;
            listcmd_busy = slcb
        } else if c == '(' as i32 || c == ')' as i32 {
            /* to previous/next sentence */
            let mut pos_0 =
                pos_T{lnum: 0,
                      col: 0,
                      coladd: 0,}; /* avoid that '' is changed */
            let mut slcb_0 = listcmd_busy;
            pos_0 = (*curwin).w_cursor;
            listcmd_busy = TRUE;
            if findsent(if c == ')' as i32 {
                            FORWARD as libc::c_int
                        } else { BACKWARD as libc::c_int } as Direction,
                        1 as libc::c_long) != 0 {
                pos_copy = (*curwin).w_cursor;
                posp = &mut pos_copy
            }
            (*curwin).w_cursor = pos_0;
            listcmd_busy = slcb_0
        } else if c == '<' as i32 || c == '>' as i32 {
            /* start/end of visual area */
            startp = &mut (*buf).b_visual.vi_start;
            endp = &mut (*buf).b_visual.vi_end;
            if ((c == '<' as i32) as libc::c_int ==
                    lt(*startp, *endp) as libc::c_int ||
                    (*endp).lnum == 0 as libc::c_int as libc::c_long) &&
                   (*startp).lnum != 0 as libc::c_int as libc::c_long {
                posp = startp
            } else { posp = endp }
            // For Visual line mode, set mark at begin or end of line
            if (*buf).b_visual.vi_mode == 'V' as i32 {
                pos_copy = *posp;
                posp = &mut pos_copy;
                if c == '<' as i32 {
                    pos_copy.col = 0 as libc::c_int
                } else { pos_copy.col = MAXCOL as libc::c_int }
                pos_copy.coladd = 0 as libc::c_int
            }
        } else if c as libc::c_uint >= 'a' as i32 as libc::c_uint &&
                      c as libc::c_uint <= 'z' as i32 as libc::c_uint {
            /* normal named mark */
            posp =
                &mut (*(*buf).b_namedm.as_mut_ptr().offset((c - 'a' as i32) as
                                                               isize)).mark
        } else if c as libc::c_uint >= 'A' as i32 as libc::c_uint &&
                      c as libc::c_uint <= 'Z' as i32 as libc::c_uint ||
                      ascii_isdigit(c) as libc::c_int != 0 {
            /* named file mark */
            if ascii_isdigit(c) {
                c = c - '0' as i32 + NMARKS
            } else { c -= 'A' as i32 }
            posp = &mut (*namedfm.as_mut_ptr().offset(c as isize)).fmark.mark;
            if namedfm[c as usize].fmark.fnum == 0 as libc::c_int {
                fname2fnum(&mut *namedfm.as_mut_ptr().offset(c as isize));
            }
            if !fnum.is_null() {
                *fnum = namedfm[c as usize].fmark.fnum
            } else if namedfm[c as usize].fmark.fnum != (*buf).handle {
                /* mark is in another file */
                posp = &mut pos_copy;
                if namedfm[c as usize].fmark.mark.lnum !=
                       0 as libc::c_int as libc::c_long &&
                       changefile as libc::c_int != 0 &&
                       namedfm[c as usize].fmark.fnum != 0 {
                    if buflist_getfile(namedfm[c as usize].fmark.fnum,
                                       1 as libc::c_int as linenr_T,
                                       GETF_SETMARK as libc::c_int, FALSE) ==
                           OK {
                        /* mark exists, but is not valid in
                                           current buffer */
                        /* Set the lnum now, autocommands could have changed it */
                        (*curwin).w_cursor = namedfm[c as usize].fmark.mark;
                        return -(1 as libc::c_int) as *mut pos_T
                    }
                    pos_copy.lnum = -(1 as libc::c_int) as linenr_T
                    /* can't get file */
                } else { pos_copy.lnum = 0 as libc::c_int as linenr_T }
            }
        }
    }
    return posp;
}
/*
 * Search for the next named mark in the current file.
 *
 * Returns pointer to pos_T of the next mark or NULL if no mark is found.
 */
#[no_mangle]
#[c2rust::src_loc = "434:1"]
pub unsafe extern "C" fn getnextmark(mut startpos: *mut pos_T,
                                     mut dir: libc::c_int,
                                     mut begin_line: libc::c_int)
 -> *mut pos_T {
    let mut i: libc::c_int = 0;
    let mut result = NULL_1 as *mut pos_T;
    let mut pos = pos_T{lnum: 0, col: 0, coladd: 0,};
    pos = *startpos;
    /* When searching backward and leaving the cursor on the first non-blank,
   * position must be in a previous line.
   * When searching forward and leaving the cursor on the first non-blank,
   * position must be in a next line. */
    if dir == BACKWARD as libc::c_int && begin_line != 0 {
        pos.col = 0 as libc::c_int
    } else if dir == FORWARD as libc::c_int && begin_line != 0 {
        pos.col = MAXCOL as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < NMARKS {
        if (*curbuf).b_namedm[i as usize].mark.lnum >
               0 as libc::c_int as libc::c_long {
            if dir == FORWARD as libc::c_int {
                if (result.is_null() ||
                        lt((*curbuf).b_namedm[i as usize].mark, *result) as
                            libc::c_int != 0) &&
                       lt(pos, (*curbuf).b_namedm[i as usize].mark) as
                           libc::c_int != 0 {
                    result =
                        &mut (*(*curbuf).b_namedm.as_mut_ptr().offset(i as
                                                                          isize)).mark
                }
            } else if (result.is_null() ||
                           lt(*result, (*curbuf).b_namedm[i as usize].mark) as
                               libc::c_int != 0) &&
                          lt((*curbuf).b_namedm[i as usize].mark, pos) as
                              libc::c_int != 0 {
                result =
                    &mut (*(*curbuf).b_namedm.as_mut_ptr().offset(i as
                                                                      isize)).mark
            }
        }
        i += 1
    }
    return result;
}
/*
 * For an xtended filemark: set the fnum from the fname.
 * This is used for marks obtained from the .shada file.  It's postponed
 * until the mark is used to avoid a long startup delay.
 */
#[c2rust::src_loc = "478:1"]
unsafe extern "C" fn fname2fnum(mut fm: *mut xfmark_T) {
    let mut p = 0 as *mut char_u;
    if !(*fm).fname.is_null() {
        /*
     * First expand "~/" in the file name to the home directory.
     * Don't expand the whole name, it may contain other '~' chars.
     */
        if *(*fm).fname.offset(0 as libc::c_int as isize) as libc::c_int ==
               '~' as i32 &&
               *(*fm).fname.offset(1 as libc::c_int as isize) as libc::c_int
                   == '/' as i32 {
            let mut len: libc::c_int = 0;
            expand_env(b"~/\x00" as *const u8 as *const libc::c_char as
                           *mut char_u, NameBuff.as_mut_ptr(), MAXPATHL);
            len =
                strlen(NameBuff.as_mut_ptr() as *mut libc::c_char) as
                    libc::c_int;
            xstrlcpy(NameBuff.as_mut_ptr().offset(len as isize) as
                         *mut libc::c_char,
                     (*fm).fname.offset(2 as libc::c_int as isize) as
                         *mut libc::c_char,
                     (4096 as libc::c_int - len) as size_t);
        } else {
            xstrlcpy(NameBuff.as_mut_ptr() as *mut libc::c_char,
                     (*fm).fname as *mut libc::c_char,
                     4096 as libc::c_int as size_t);
        }
        /* Try to shorten the file name. */
        os_dirname(IObuff.as_mut_ptr(), IOSIZE as size_t);
        p = path_shorten_fname(NameBuff.as_mut_ptr(), IObuff.as_mut_ptr());
        // buflist_new() will call fmarks_check_names()
        buflist_new(NameBuff.as_mut_ptr(), p, 1 as libc::c_int as linenr_T,
                    0 as libc::c_int);
    };
}
/*
 * Check all file marks for a name that matches the file name in buf.
 * May replace the name with an fnum.
 * Used for marks that come from the .shada file.
 */
#[no_mangle]
#[c2rust::src_loc = "514:1"]
pub unsafe extern "C" fn fmarks_check_names(mut buf: *mut buf_T) {
    let mut name = (*buf).b_ffname;
    let mut i: libc::c_int = 0;
    if (*buf).b_ffname.is_null() { return }
    i = 0 as libc::c_int;
    while i < NGLOBALMARKS {
        fmarks_check_one(&mut *namedfm.as_mut_ptr().offset(i as isize), name,
                         buf);
        i += 1
    }
    let mut wp =
        if curtab == curtab { firstwin } else { (*curtab).tp_firstwin };
    while !wp.is_null() {
        i = 0 as libc::c_int;
        while i < (*wp).w_jumplistlen {
            fmarks_check_one(&mut *(*wp).w_jumplist.as_mut_ptr().offset(i as
                                                                            isize),
                             name, buf);
            i += 1
        }
        wp = (*wp).w_next
    };
}
#[c2rust::src_loc = "532:1"]
unsafe extern "C" fn fmarks_check_one(mut fm: *mut xfmark_T,
                                      mut name: *mut char_u,
                                      mut buf: *mut buf_T) {
    if (*fm).fmark.fnum == 0 as libc::c_int && !(*fm).fname.is_null() &&
           path_fnamecmp(name as *const libc::c_char,
                         (*fm).fname as *const libc::c_char) ==
               0 as libc::c_int {
        (*fm).fmark.fnum = (*buf).handle;
        let mut ptr_ =
            &mut (*fm).fname as *mut *mut char_u as *mut *mut libc::c_void;
        xfree(*ptr_);
        *ptr_ = NULL_1 as *mut libc::c_void
    };
}
/*
 * Check a if a position from a mark is valid.
 * Give and error message and return FAIL if not.
 */
#[no_mangle]
#[c2rust::src_loc = "546:1"]
pub unsafe extern "C" fn check_mark(mut pos: *mut pos_T) -> libc::c_int {
    if pos.is_null() {
        emsg(gettext(e_umark.as_mut_ptr() as *mut libc::c_char) as
                 *mut char_u);
        return FAIL
    }
    if (*pos).lnum <= 0 as libc::c_int as libc::c_long {
        /* lnum is negative if mark is in another file can can't get that
     * file, error message already give then. */
        if (*pos).lnum == 0 as libc::c_int as libc::c_long {
            emsg(gettext(e_marknotset.as_mut_ptr() as *mut libc::c_char) as
                     *mut char_u);
        }
        return FAIL
    }
    if (*pos).lnum > (*curbuf).b_ml.ml_line_count {
        emsg(gettext(e_markinval.as_mut_ptr() as *mut libc::c_char) as
                 *mut char_u);
        return FAIL
    }
    return OK;
}
// / Clear all marks and change list in the given buffer
// /
// / Used mainly when trashing the entire buffer during ":e" type commands.
// /
// / @param[out]  buf  Buffer to clear marks in.
#[no_mangle]
#[c2rust::src_loc = "571:1"]
pub unsafe extern "C" fn clrallmarks(buf: *mut buf_T) {
    let mut i = 0 as libc::c_int as size_t; // start/end op mark cleared
    while i < NMARKS as libc::c_ulong {
        clear_fmark(&mut *(*buf).b_namedm.as_mut_ptr().offset(i as isize));
        i = i.wrapping_add(1)
    }
    clear_fmark(&mut (*buf).b_last_cursor);
    (*buf).b_last_cursor.mark.lnum = 1 as libc::c_int as linenr_T;
    clear_fmark(&mut (*buf).b_last_insert);
    clear_fmark(&mut (*buf).b_last_change);
    (*buf).b_op_start.lnum = 0 as libc::c_int as linenr_T;
    (*buf).b_op_end.lnum = 0 as libc::c_int as linenr_T;
    let mut i_0 = 0 as libc::c_int;
    while i_0 < (*buf).b_changelistlen {
        clear_fmark(&mut *(*buf).b_changelist.as_mut_ptr().offset(i_0 as
                                                                      isize));
        i_0 += 1
    }
    (*buf).b_changelistlen = 0 as libc::c_int;
}
/*
 * Get name of file from a filemark.
 * When it's in the current buffer, return the text at the mark.
 * Returns an allocated string.
 */
#[no_mangle]
#[c2rust::src_loc = "594:1"]
pub unsafe extern "C" fn fm_getname(mut fmark: *mut fmark_T,
                                    mut lead_len: libc::c_int)
 -> *mut char_u {
    if (*fmark).fnum == (*curbuf).handle {
        /* current buffer */
        return mark_line(&mut (*fmark).mark, lead_len)
    }
    return buflist_nr2name((*fmark).fnum, FALSE, TRUE);
}
/*
 * Return the line at mark "mp".  Truncate to fit in window.
 * The returned string has been allocated.
 */
#[c2rust::src_loc = "605:1"]
unsafe extern "C" fn mark_line(mut mp: *mut pos_T, mut lead_len: libc::c_int)
 -> *mut char_u {
    let mut s = 0 as *mut char_u;
    let mut p = 0 as *mut char_u;
    let mut len: libc::c_int = 0;
    if (*mp).lnum == 0 as libc::c_int as libc::c_long ||
           (*mp).lnum > (*curbuf).b_ml.ml_line_count {
        return vim_strsave(b"-invalid-\x00" as *const u8 as
                               *const libc::c_char as *mut char_u)
    }
    if Columns >= 0 as libc::c_int &&
           Columns as size_t <= 18446744073709551615 as libc::c_ulong {
    } else {
        __assert_fail(b"Columns >= 0 && (size_t)Columns <= SIZE_MAX\x00" as
                          *const u8 as *const libc::c_char,
                      b"/home/vole/neovim/src/nvim/mark.c\x00" as *const u8 as
                          *const libc::c_char,
                      612 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"char_u *mark_line(pos_T *, int)\x00")).as_ptr());
    }
    // Allow for up to 5 bytes per character.
    s =
        vim_strnsave(skipwhite(ml_get((*mp).lnum)),
                     (Columns as
                          size_t).wrapping_mul(5 as libc::c_int as
                                                   libc::c_ulong));
    // Truncate the line to fit it in the window
    len = 0 as libc::c_int;
    p = s;
    while *p as libc::c_int != NUL {
        len += ptr2cells(p);
        if len >= Columns - lead_len { break ; }
        p = p.offset(utfc_ptr2len(p) as isize)
    }
    *p = NUL as char_u;
    return s;
}
/*
 * print the marks
 */
#[no_mangle]
#[c2rust::src_loc = "630:1"]
pub unsafe extern "C" fn ex_marks(mut eap: *mut exarg_T) {
    let mut arg = (*eap).arg;
    let mut i: libc::c_int = 0;
    let mut name = 0 as *mut char_u;
    if !arg.is_null() && *arg as libc::c_int == NUL {
        arg = NULL_1 as *mut char_u
    }
    show_one_mark('\'' as i32, arg, &mut (*curwin).w_pcmark,
                  NULL_1 as *mut char_u, true_0);
    i = 0 as libc::c_int;
    while i < NMARKS {
        show_one_mark(i + 'a' as i32, arg,
                      &mut (*(*curbuf).b_namedm.as_mut_ptr().offset(i as
                                                                        isize)).mark,
                      NULL_1 as *mut char_u, true_0);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < NGLOBALMARKS {
        if namedfm[i as usize].fmark.fnum != 0 as libc::c_int {
            name =
                fm_getname(&mut (*namedfm.as_mut_ptr().offset(i as
                                                                  isize)).fmark,
                           15 as libc::c_int)
        } else { name = namedfm[i as usize].fname }
        if !name.is_null() {
            show_one_mark(if i >= NMARKS {
                              (i - NMARKS) + '0' as i32
                          } else { (i) + 'A' as i32 }, arg,
                          &mut (*namedfm.as_mut_ptr().offset(i as
                                                                 isize)).fmark.mark,
                          name,
                          (namedfm[i as usize].fmark.fnum == (*curbuf).handle)
                              as libc::c_int);
            if namedfm[i as usize].fmark.fnum != 0 as libc::c_int {
                xfree(name as *mut libc::c_void);
            }
        }
        i += 1
    }
    show_one_mark('\"' as i32, arg, &mut (*curbuf).b_last_cursor.mark,
                  NULL_1 as *mut char_u, true_0);
    show_one_mark('[' as i32, arg, &mut (*curbuf).b_op_start,
                  NULL_1 as *mut char_u, true_0);
    show_one_mark(']' as i32, arg, &mut (*curbuf).b_op_end,
                  NULL_1 as *mut char_u, true_0);
    show_one_mark('^' as i32, arg, &mut (*curbuf).b_last_insert.mark,
                  NULL_1 as *mut char_u, true_0);
    show_one_mark('.' as i32, arg, &mut (*curbuf).b_last_change.mark,
                  NULL_1 as *mut char_u, true_0);
    show_one_mark('<' as i32, arg, &mut (*curbuf).b_visual.vi_start,
                  NULL_1 as *mut char_u, true_0);
    show_one_mark('>' as i32, arg, &mut (*curbuf).b_visual.vi_end,
                  NULL_1 as *mut char_u, true_0);
    show_one_mark(-(1 as libc::c_int), arg, NULL_1 as *mut pos_T,
                  NULL_1 as *mut char_u, false_0);
}
#[c2rust::src_loc = "665:1"]
unsafe extern "C" fn show_one_mark(mut c: libc::c_int, mut arg: *mut char_u,
                                   mut p: *mut pos_T,
                                   mut name_arg: *mut char_u,
                                   mut current: libc::c_int) 
 // in current file
 {
    static mut did_title: bool = false_0 != 0;
    let mut mustfree = false_0 != 0;
    let mut name = name_arg;
    if c == -(1 as libc::c_int) {
        // finish up
        if did_title {
            did_title = false_0 != 0
        } else if arg.is_null() {
            msg(gettext(b"No marks set\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char) as
                    *mut char_u);
        } else {
            emsgf(gettext(b"E283: No marks matching \"%s\"\x00" as *const u8
                              as *const libc::c_char as *mut libc::c_char) as
                      *const libc::c_char, arg);
        }
    } else if got_int == 0 && (arg.is_null() || !vim_strchr(arg, c).is_null())
                  && (*p).lnum != 0 as libc::c_int as libc::c_long {
        // don't output anything if 'q' typed at --more-- prompt
        if name.is_null() && current != 0 {
            name = mark_line(p, 15 as libc::c_int);
            mustfree = true_0 != 0
        }
        if !message_filtered(name) {
            if !did_title {
                // Highlight title
                msg_puts_title(gettext(b"\nmark line  col file/text\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char));
                did_title = true_0 != 0
            }
            msg_putchar('\n' as i32);
            if got_int == 0 {
                snprintf(IObuff.as_mut_ptr() as *mut libc::c_char,
                         IOSIZE as libc::c_ulong,
                         b" %c %6ld %4d \x00" as *const u8 as
                             *const libc::c_char, c, (*p).lnum, (*p).col);
                msg_outtrans(IObuff.as_mut_ptr());
                if !name.is_null() {
                    msg_outtrans_attr(name,
                                      if current != 0 {
                                          highlight_attr[HLF_D as libc::c_int
                                                             as usize]
                                      } else { 0 as libc::c_int });
                }
            }
            ui_flush();
            // show one line at a time
        }
        if mustfree { xfree(name as *mut libc::c_void); }
    };
}
/*
 * ":delmarks[!] [marks]"
 */
#[no_mangle]
#[c2rust::src_loc = "721:1"]
pub unsafe extern "C" fn ex_delmarks(mut eap: *mut exarg_T) {
    let mut p = 0 as *mut char_u;
    let mut from: libc::c_int = 0;
    let mut to: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut lower: libc::c_int = 0;
    let mut digit: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if *(*eap).arg as libc::c_int == NUL && (*eap).forceit != 0 {
        /* clear all marks */
        clrallmarks(curbuf);
    } else if (*eap).forceit != 0 {
        emsg(gettext(e_invarg.as_mut_ptr() as *mut libc::c_char) as
                 *mut char_u);
    } else if *(*eap).arg as libc::c_int == NUL {
        emsg(gettext(e_argreq.as_mut_ptr() as *mut libc::c_char) as
                 *mut char_u);
    } else {
        /* clear specified marks only */
        p = (*eap).arg;
        while *p as libc::c_int != NUL {
            lower =
                (*p as libc::c_uint >= 'a' as i32 as libc::c_uint &&
                     *p as libc::c_uint <= 'z' as i32 as libc::c_uint) as
                    libc::c_int;
            digit = ascii_isdigit(*p as libc::c_int) as libc::c_int;
            if lower != 0 || digit != 0 ||
                   *p as libc::c_uint >= 'A' as i32 as libc::c_uint &&
                       *p as libc::c_uint <= 'Z' as i32 as libc::c_uint {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int ==
                       '-' as i32 {
                    /* clear range of marks */
                    from = *p as libc::c_int;
                    to = *p.offset(2 as libc::c_int as isize) as libc::c_int;
                    if (if lower != 0 {
                            (*p.offset(2 as libc::c_int as isize) as
                                 libc::c_uint >= 'a' as i32 as libc::c_uint &&
                                 *p.offset(2 as libc::c_int as isize) as
                                     libc::c_uint <=
                                     'z' as i32 as libc::c_uint) as
                                libc::c_int
                        } else {
                            (if digit != 0 {
                                 ascii_isdigit(*p.offset(2 as libc::c_int as
                                                             isize) as
                                                   libc::c_int) as libc::c_int
                             } else {
                                 (*p.offset(2 as libc::c_int as isize) as
                                      libc::c_uint >=
                                      'A' as i32 as libc::c_uint &&
                                      *p.offset(2 as libc::c_int as isize) as
                                          libc::c_uint <=
                                          'Z' as i32 as libc::c_uint) as
                                     libc::c_int
                             })
                        }) == 0 || to < from {
                        emsgf(gettext(e_invarg2.as_mut_ptr() as
                                          *mut libc::c_char) as
                                  *const libc::c_char, p);
                        return
                    }
                    p = p.offset(2 as libc::c_int as isize)
                } else {
                    /* clear one lower case mark */
                    to = *p as libc::c_int;
                    from = to
                }
                i = from;
                while i <= to {
                    if lower != 0 {
                        (*curbuf).b_namedm[(i - 'a' as i32) as
                                               usize].mark.lnum =
                            0 as libc::c_int as linenr_T
                    } else {
                        if digit != 0 {
                            n = i - '0' as i32 + NMARKS
                        } else { n = i - 'A' as i32 }
                        namedfm[n as usize].fmark.mark.lnum =
                            0 as libc::c_int as linenr_T;
                        let mut ptr_ =
                            &mut (*namedfm.as_mut_ptr().offset(n as
                                                                   isize)).fname
                                as *mut *mut char_u as *mut *mut libc::c_void;
                        xfree(*ptr_);
                        *ptr_ = NULL_1 as *mut libc::c_void
                    }
                    i += 1
                }
            } else {
                match *p as libc::c_int {
                    34 => {
                        let fmarkp___: *mut fmark_T =
                            &mut (*curbuf).b_last_cursor;
                        free_fmark(*fmarkp___);
                        let fmarkp__ = fmarkp___;
                        (*fmarkp__).mark =
                            {
                                let mut init =
                                    pos_T{lnum: 0 as libc::c_int as linenr_T,
                                          col: 0 as libc::c_int,
                                          coladd: 0 as libc::c_int,};
                                init
                            };
                        (*fmarkp__).fnum = 0 as libc::c_int;
                        (*fmarkp__).timestamp = os_time();
                        (*fmarkp__).additional_data = NULL_1 as *mut dict_T
                    }
                    94 => {
                        let fmarkp____0: *mut fmark_T =
                            &mut (*curbuf).b_last_insert;
                        free_fmark(*fmarkp____0);
                        let fmarkp___0 = fmarkp____0;
                        (*fmarkp___0).mark =
                            {
                                let mut init =
                                    pos_T{lnum: 0 as libc::c_int as linenr_T,
                                          col: 0 as libc::c_int,
                                          coladd: 0 as libc::c_int,};
                                init
                            };
                        (*fmarkp___0).fnum = 0 as libc::c_int;
                        (*fmarkp___0).timestamp = os_time();
                        (*fmarkp___0).additional_data = NULL_1 as *mut dict_T
                    }
                    46 => {
                        let fmarkp____1: *mut fmark_T =
                            &mut (*curbuf).b_last_change;
                        free_fmark(*fmarkp____1);
                        let fmarkp___1 = fmarkp____1;
                        (*fmarkp___1).mark =
                            {
                                let mut init =
                                    pos_T{lnum: 0 as libc::c_int as linenr_T,
                                          col: 0 as libc::c_int,
                                          coladd: 0 as libc::c_int,};
                                init
                            };
                        (*fmarkp___1).fnum = 0 as libc::c_int;
                        (*fmarkp___1).timestamp = os_time();
                        (*fmarkp___1).additional_data = NULL_1 as *mut dict_T
                    }
                    91 => {
                        (*curbuf).b_op_start.lnum =
                            0 as libc::c_int as linenr_T
                    }
                    93 => {
                        (*curbuf).b_op_end.lnum = 0 as libc::c_int as linenr_T
                    }
                    60 => {
                        (*curbuf).b_visual.vi_start.lnum =
                            0 as libc::c_int as linenr_T
                    }
                    62 => {
                        (*curbuf).b_visual.vi_end.lnum =
                            0 as libc::c_int as linenr_T
                    }
                    32 => { }
                    _ => {
                        emsgf(gettext(e_invarg2.as_mut_ptr() as
                                          *mut libc::c_char) as
                                  *const libc::c_char, p);
                        return
                    }
                }
            }
            p = p.offset(1)
        }
    };
}
/*
 * print the jumplist
 */
#[no_mangle]
#[c2rust::src_loc = "792:1"]
pub unsafe extern "C" fn ex_jumps(mut eap: *mut exarg_T) {
    let mut i: libc::c_int = 0;
    let mut name = 0 as *mut char_u;
    cleanup_jumplist(curwin, true_0 != 0);
    // Highlight title
    msg_puts_title(gettext(b"\n jump line  col file/text\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char) as
                       *const libc::c_char);
    let mut current_block_12: u64;
    i = 0 as libc::c_int;
    while i < (*curwin).w_jumplistlen && got_int == 0 {
        if (*curwin).w_jumplist[i as usize].fmark.mark.lnum !=
               0 as libc::c_int as libc::c_long {
            name =
                fm_getname(&mut (*(*curwin).w_jumplist.as_mut_ptr().offset(i
                                                                               as
                                                                               isize)).fmark,
                           16 as libc::c_int);
            // apply :filter /pat/ or file name not available
            if name.is_null() || message_filtered(name) as libc::c_int != 0 {
                xfree(name as *mut libc::c_void);
                current_block_12 = 10680521327981672866;
            } else {
                msg_putchar('\n' as i32);
                if got_int != 0 {
                    xfree(name as *mut libc::c_void);
                    break ;
                } else {
                    sprintf(IObuff.as_mut_ptr() as *mut libc::c_char,
                            b"%c %2d %5ld %4d \x00" as *const u8 as
                                *const libc::c_char,
                            if i == (*curwin).w_jumplistidx {
                                '>' as i32
                            } else { ' ' as i32 },
                            if i > (*curwin).w_jumplistidx {
                                (i) - (*curwin).w_jumplistidx
                            } else { ((*curwin).w_jumplistidx) - i },
                            (*curwin).w_jumplist[i as usize].fmark.mark.lnum,
                            (*curwin).w_jumplist[i as usize].fmark.mark.col);
                    msg_outtrans(IObuff.as_mut_ptr());
                    msg_outtrans_attr(name,
                                      if (*curwin).w_jumplist[i as
                                                                  usize].fmark.fnum
                                             == (*curbuf).handle {
                                          highlight_attr[HLF_D as libc::c_int
                                                             as usize]
                                      } else { 0 as libc::c_int });
                    xfree(name as *mut libc::c_void);
                    os_breakcheck();
                }
                current_block_12 = 15976848397966268834;
            }
        } else { current_block_12 = 15976848397966268834; }
        match current_block_12 {
            15976848397966268834 => { ui_flush(); }
            _ => { }
        }
        i += 1
    }
    if (*curwin).w_jumplistidx == (*curwin).w_jumplistlen {
        msg_puts(b"\n>\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
#[c2rust::src_loc = "834:1"]
pub unsafe extern "C" fn ex_clearjumps(mut eap: *mut exarg_T) {
    free_jumplist(curwin);
    (*curwin).w_jumplistlen = 0 as libc::c_int;
    (*curwin).w_jumplistidx = 0 as libc::c_int;
}
/*
 * print the changelist
 */
#[no_mangle]
#[c2rust::src_loc = "844:1"]
pub unsafe extern "C" fn ex_changes(mut eap: *mut exarg_T) {
    let mut i: libc::c_int = 0;
    let mut name = 0 as *mut char_u;
    // Highlight title
    msg_puts_title(gettext(b"\nchange line  col text\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char) as
                       *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*curbuf).b_changelistlen && got_int == 0 {
        if (*curbuf).b_changelist[i as usize].mark.lnum !=
               0 as libc::c_int as libc::c_long {
            msg_putchar('\n' as i32);
            if got_int != 0 { break ; }
            sprintf(IObuff.as_mut_ptr() as *mut libc::c_char,
                    b"%c %3d %5ld %4d \x00" as *const u8 as
                        *const libc::c_char,
                    if i == (*curwin).w_changelistidx {
                        '>' as i32
                    } else { ' ' as i32 },
                    if i > (*curwin).w_changelistidx {
                        (i) - (*curwin).w_changelistidx
                    } else { ((*curwin).w_changelistidx) - i },
                    (*curbuf).b_changelist[i as usize].mark.lnum,
                    (*curbuf).b_changelist[i as usize].mark.col);
            msg_outtrans(IObuff.as_mut_ptr());
            name =
                mark_line(&mut (*(*curbuf).b_changelist.as_mut_ptr().offset(i
                                                                                as
                                                                                isize)).mark,
                          17 as libc::c_int);
            msg_outtrans_attr(name,
                              highlight_attr[HLF_D as libc::c_int as usize]);
            xfree(name as *mut libc::c_void);
            os_breakcheck();
        }
        ui_flush();
        i += 1
    }
    if (*curwin).w_changelistidx == (*curbuf).b_changelistlen {
        msg_puts(b"\n>\x00" as *const u8 as *const libc::c_char);
    };
}
/* don't delete the line, just put at first deleted line */
/*
 * Adjust marks between line1 and line2 (inclusive) to move 'amount' lines.
 * Must be called before changed_*(), appended_lines() or deleted_lines().
 * May be called before or after changing the text.
 * When deleting lines line1 to line2, use an 'amount' of MAXLNUM: The marks
 * within this range are made invalid.
 * If 'amount_after' is non-zero adjust marks after line2.
 * Example: Delete lines 34 and 35: mark_adjust(34, 35, MAXLNUM, -2);
 * Example: Insert two lines below 55: mark_adjust(56, MAXLNUM, 2, 0);
 *				   or: mark_adjust(56, 55, MAXLNUM, 2);
 */
#[no_mangle]
#[c2rust::src_loc = "915:1"]
pub unsafe extern "C" fn mark_adjust(mut line1: linenr_T, mut line2: linenr_T,
                                     mut amount: libc::c_long,
                                     mut amount_after: libc::c_long,
                                     mut op: ExtmarkOp) {
    mark_adjust_internal(line1, line2, amount, amount_after, true_0 != 0, op);
}
// mark_adjust_nofold() does the same as mark_adjust() but without adjusting
// folds in any way. Folds must be adjusted manually by the caller.
// This is only useful when folds need to be moved in a way different to
// calling foldMarkAdjust() with arguments line1, line2, amount, amount_after,
// for an example of why this may be necessary, see do_move().
#[no_mangle]
#[c2rust::src_loc = "929:1"]
pub unsafe extern "C" fn mark_adjust_nofold(mut line1: linenr_T,
                                            mut line2: linenr_T,
                                            mut amount: libc::c_long,
                                            mut amount_after: libc::c_long,
                                            mut op: ExtmarkOp) {
    mark_adjust_internal(line1, line2, amount, amount_after, false_0 != 0,
                         op);
}
#[c2rust::src_loc = "936:1"]
unsafe extern "C" fn mark_adjust_internal(mut line1: linenr_T,
                                          mut line2: linenr_T,
                                          mut amount: libc::c_long,
                                          mut amount_after: libc::c_long,
                                          mut adjust_folds: bool,
                                          mut op: ExtmarkOp) {
    let mut i: libc::c_int = 0;
    let mut fnum = (*curbuf).handle;
    let mut lp = 0 as *mut linenr_T;
    static mut initpos: pos_T =
        {
            let mut init =
                pos_T{lnum: 1 as libc::c_int as linenr_T,
                      col: 0 as libc::c_int,
                      coladd: 0 as libc::c_int,};
            init
        };
    if line2 < line1 && amount_after == 0 as libc::c_long {
        /* nothing to do */
        return
    }
    if !cmdmod.lockmarks {
        /* named marks, lower case and upper case */
        i = 0 as libc::c_int;
        while i < NMARKS {
            lp =
                &mut (*(*curbuf).b_namedm.as_mut_ptr().offset(i as
                                                                  isize)).mark.lnum;
            if *lp >= line1 && *lp <= line2 {
                if amount == MAXLNUM as libc::c_int as libc::c_long {
                    *lp = 0 as libc::c_int as linenr_T
                } else { *lp += amount }
            } else if amount_after != 0 && *lp > line2 { *lp += amount_after }
            if namedfm[i as usize].fmark.fnum == fnum {
                lp =
                    &mut (*namedfm.as_mut_ptr().offset(i as
                                                           isize)).fmark.mark.lnum;
                if *lp >= line1 && *lp <= line2 {
                    if amount == MAXLNUM as libc::c_int as libc::c_long {
                        *lp = line1
                    } else { *lp += amount }
                } else if amount_after != 0 && *lp > line2 {
                    *lp += amount_after
                }
            }
            i += 1
        }
        i = NMARKS;
        while i < NGLOBALMARKS {
            if namedfm[i as usize].fmark.fnum == fnum {
                lp =
                    &mut (*namedfm.as_mut_ptr().offset(i as
                                                           isize)).fmark.mark.lnum;
                if *lp >= line1 && *lp <= line2 {
                    if amount == MAXLNUM as libc::c_int as libc::c_long {
                        *lp = line1
                    } else { *lp += amount }
                } else if amount_after != 0 && *lp > line2 {
                    *lp += amount_after
                }
            }
            i += 1
        }
        /* last Insert position */
        lp = &mut (*curbuf).b_last_insert.mark.lnum;
        if *lp >= line1 && *lp <= line2 {
            if amount == MAXLNUM as libc::c_int as libc::c_long {
                *lp = 0 as libc::c_int as linenr_T
            } else { *lp += amount }
        } else if amount_after != 0 && *lp > line2 { *lp += amount_after }
        /* last change position */
        lp = &mut (*curbuf).b_last_change.mark.lnum;
        if *lp >= line1 && *lp <= line2 {
            if amount == MAXLNUM as libc::c_int as libc::c_long {
                *lp = 0 as libc::c_int as linenr_T
            } else { *lp += amount }
        } else if amount_after != 0 && *lp > line2 { *lp += amount_after }
        /* last cursor position, if it was set */
        if !equalpos((*curbuf).b_last_cursor.mark, initpos) {
            lp = &mut (*curbuf).b_last_cursor.mark.lnum;
            if *lp >= line1 && *lp <= line2 {
                if amount == MAXLNUM as libc::c_int as libc::c_long {
                    *lp = 0 as libc::c_int as linenr_T
                } else { *lp += amount }
            } else if amount_after != 0 && *lp > line2 { *lp += amount_after }
        }
        /* list of change positions */
        i = 0 as libc::c_int;
        while i < (*curbuf).b_changelistlen {
            lp =
                &mut (*(*curbuf).b_changelist.as_mut_ptr().offset(i as
                                                                      isize)).mark.lnum;
            if *lp >= line1 && *lp <= line2 {
                if amount == MAXLNUM as libc::c_int as libc::c_long {
                    *lp = line1
                } else { *lp += amount }
            } else if amount_after != 0 && *lp > line2 { *lp += amount_after }
            i += 1
        }
        /* Visual area */
        lp = &mut (*curbuf).b_visual.vi_start.lnum;
        if *lp >= line1 && *lp <= line2 {
            if amount == MAXLNUM as libc::c_int as libc::c_long {
                *lp = line1
            } else { *lp += amount }
        } else if amount_after != 0 && *lp > line2 { *lp += amount_after }
        lp = &mut (*curbuf).b_visual.vi_end.lnum;
        if *lp >= line1 && *lp <= line2 {
            if amount == MAXLNUM as libc::c_int as libc::c_long {
                *lp = line1
            } else { *lp += amount }
        } else if amount_after != 0 && *lp > line2 { *lp += amount_after }
        // quickfix marks
        if !qf_mark_adjust(NULL_1 as *mut win_T, line1, line2, amount,
                           amount_after) {
            (*curbuf).b_has_qf_entry &= !BUF_HAS_QF_ENTRY
        }
        // location lists
        let mut found_one = false_0 != 0;
        let mut tab = first_tabpage;
        while !tab.is_null() {
            let mut win =
                if tab == curtab { firstwin } else { (*tab).tp_firstwin };
            while !win.is_null() {
                found_one =
                    (found_one as libc::c_int |
                         qf_mark_adjust(win, line1, line2, amount,
                                        amount_after) as libc::c_int) as bool;
                win = (*win).w_next
            }
            tab = (*tab).tp_next
        }
        if !found_one { (*curbuf).b_has_qf_entry &= !BUF_HAS_LL_ENTRY }
        sign_mark_adjust(line1, line2, amount, amount_after);
        if op as libc::c_uint != kExtmarkNOOP as libc::c_int as libc::c_uint {
            extmark_adjust(curbuf, line1, line2, amount, amount_after, op);
        }
    }
    /* previous context mark */
    lp = &mut (*curwin).w_pcmark.lnum;
    if *lp >= line1 && *lp <= line2 {
        if amount == MAXLNUM as libc::c_int as libc::c_long {
            *lp = 0 as libc::c_int as linenr_T
        } else { *lp += amount }
    } else if amount_after != 0 && *lp > line2 { *lp += amount_after }
    /* previous pcmark */
    lp = &mut (*curwin).w_prev_pcmark.lnum;
    if *lp >= line1 && *lp <= line2 {
        if amount == MAXLNUM as libc::c_int as libc::c_long {
            *lp = 0 as libc::c_int as linenr_T
        } else { *lp += amount }
    } else if amount_after != 0 && *lp > line2 { *lp += amount_after }
    /* saved cursor for formatting */
    if saved_cursor.lnum != 0 as libc::c_int as libc::c_long {
        lp = &mut saved_cursor.lnum;
        if *lp >= line1 && *lp <= line2 {
            if amount == MAXLNUM as libc::c_int as libc::c_long {
                *lp = line1
            } else { *lp += amount }
        } else if amount_after != 0 && *lp > line2 { *lp += amount_after }
    }
    /*
   * Adjust items in all windows related to the current buffer.
   */
    let mut tab_0 = first_tabpage;
    while !tab_0.is_null() {
        let mut win_0 =
            if tab_0 == curtab { firstwin } else { (*tab_0).tp_firstwin };
        while !win_0.is_null() {
            if !cmdmod.lockmarks {
                /* Marks in the jumplist.  When deleting lines, this may create
       * duplicate marks in the jumplist, they will be removed later. */
                i = 0 as libc::c_int;
                while i < (*win_0).w_jumplistlen {
                    if (*win_0).w_jumplist[i as usize].fmark.fnum == fnum {
                        lp =
                            &mut (*(*win_0).w_jumplist.as_mut_ptr().offset(i
                                                                               as
                                                                               isize)).fmark.mark.lnum;
                        if *lp >= line1 && *lp <= line2 {
                            if amount ==
                                   MAXLNUM as libc::c_int as libc::c_long {
                                *lp = line1
                            } else { *lp += amount }
                        } else if amount_after != 0 && *lp > line2 {
                            *lp += amount_after
                        }
                    }
                    i += 1
                }
            }
            if (*win_0).w_buffer == curbuf {
                if !cmdmod.lockmarks {
                    /* marks in the tag stack */
                    i = 0 as libc::c_int;
                    while i < (*win_0).w_tagstacklen {
                        if (*win_0).w_tagstack[i as usize].fmark.fnum == fnum
                           {
                            lp =
                                &mut (*(*win_0).w_tagstack.as_mut_ptr().offset(i
                                                                                   as
                                                                                   isize)).fmark.mark.lnum;
                            if *lp >= line1 && *lp <= line2 {
                                if amount ==
                                       MAXLNUM as libc::c_int as libc::c_long
                                   {
                                    *lp = line1
                                } else { *lp += amount }
                            } else if amount_after != 0 && *lp > line2 {
                                *lp += amount_after
                            }
                        }
                        i += 1
                    }
                }
                /* the displayed Visual area */
                if (*win_0).w_old_cursor_lnum !=
                       0 as libc::c_int as libc::c_long {
                    lp = &mut (*win_0).w_old_cursor_lnum;
                    if *lp >= line1 && *lp <= line2 {
                        if amount == MAXLNUM as libc::c_int as libc::c_long {
                            *lp = line1
                        } else { *lp += amount }
                    } else if amount_after != 0 && *lp > line2 {
                        *lp += amount_after
                    }
                    lp = &mut (*win_0).w_old_visual_lnum;
                    if *lp >= line1 && *lp <= line2 {
                        if amount == MAXLNUM as libc::c_int as libc::c_long {
                            *lp = line1
                        } else { *lp += amount }
                    } else if amount_after != 0 && *lp > line2 {
                        *lp += amount_after
                    }
                }
                /* topline and cursor position for windows with the same buffer
       * other than the current window */
                if win_0 != curwin {
                    if (*win_0).w_topline >= line1 &&
                           (*win_0).w_topline <= line2 {
                        if amount == MAXLNUM as libc::c_int as libc::c_long {
                            if line1 <= 1 as libc::c_int as libc::c_long {
                                (*win_0).w_topline =
                                    1 as libc::c_int as linenr_T
                            } else {
                                (*win_0).w_topline =
                                    line1 - 1 as libc::c_int as libc::c_long
                            }
                        } else {
                            (*win_0).w_topline += amount
                        } /* keep topline on the same line */
                        (*win_0).w_topfill = 0 as libc::c_int
                    } else if amount_after != 0 && (*win_0).w_topline > line2
                     {
                        (*win_0).w_topline += amount_after;
                        (*win_0).w_topfill = 0 as libc::c_int
                    }
                    if (*win_0).w_cursor.lnum >= line1 &&
                           (*win_0).w_cursor.lnum <= line2 {
                        if amount == MAXLNUM as libc::c_int as libc::c_long {
                            /* topline is deleted */
                            if line1 <= 1 as libc::c_int as libc::c_long {
                                (*win_0).w_cursor.lnum =
                                    1 as libc::c_int as linenr_T
                            } else {
                                (*win_0).w_cursor.lnum =
                                    line1 - 1 as libc::c_int as libc::c_long
                            } /* keep cursor on the same line */
                            (*win_0).w_cursor.col = 0 as libc::c_int
                        } else { (*win_0).w_cursor.lnum += amount }
                    } else if amount_after != 0 &&
                                  (*win_0).w_cursor.lnum > line2 {
                        (*win_0).w_cursor.lnum += amount_after
                    }
                }
                if adjust_folds {
                    foldMarkAdjust(win_0, line1, line2, amount, amount_after);
                }
            }
            win_0 = (*win_0).w_next
        }
        tab_0 = (*tab_0).tp_next
    }
    /* line with cursor is deleted */
    /* adjust diffs */
    diff_mark_adjust(line1, line2, amount, amount_after);
}
/* This code is used often, needs to be fast. */
// Adjust marks in line "lnum" at column "mincol" and further: add
// "lnum_amount" to the line number and add "col_amount" to the column
// position.
// "spaces_removed" is the number of spaces that were removed, matters when the
// cursor is inside them.
#[no_mangle]
#[c2rust::src_loc = "1106:1"]
pub unsafe extern "C" fn mark_col_adjust(mut lnum: linenr_T,
                                         mut mincol: colnr_T,
                                         mut lnum_amount: libc::c_long,
                                         mut col_amount: libc::c_long,
                                         mut spaces_removed: libc::c_int) {
    let mut i: libc::c_int = 0; /* nothing to do */
    let mut fnum = (*curbuf).handle;
    let mut posp = 0 as *mut pos_T;
    if col_amount == 0 as libc::c_long && lnum_amount == 0 as libc::c_long ||
           cmdmod.lockmarks as libc::c_int != 0 {
        return
    }
    /* named marks, lower case and upper case */
    i = 0 as libc::c_int;
    while i < NMARKS {
        posp =
            &mut (*(*curbuf).b_namedm.as_mut_ptr().offset(i as isize)).mark;
        if (*posp).lnum == lnum && (*posp).col >= mincol {
            (*posp).lnum += lnum_amount;
            if col_amount >
                   (-(2147483647 as libc::c_int) - 1 as libc::c_int) as
                       libc::c_long &&
                   col_amount <= 2147483647 as libc::c_int as libc::c_long {
            } else {
                __assert_fail(b"col_amount > INT_MIN && col_amount <= INT_MAX\x00"
                                  as *const u8 as *const libc::c_char,
                              b"/home/vole/neovim/src/nvim/mark.c\x00" as
                                  *const u8 as *const libc::c_char,
                              1119 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 57],
                                                        &[libc::c_char; 57]>(b"void mark_col_adjust(linenr_T, colnr_T, long, long, int)\x00")).as_ptr());
            }
            if col_amount < 0 as libc::c_int as libc::c_long &&
                   (*posp).col <= -col_amount as colnr_T {
                (*posp).col = 0 as libc::c_int
            } else if (*posp).col < spaces_removed {
                (*posp).col = col_amount as libc::c_int + spaces_removed
            } else { (*posp).col += col_amount as colnr_T }
        }
        if namedfm[i as usize].fmark.fnum == fnum {
            posp = &mut (*namedfm.as_mut_ptr().offset(i as isize)).fmark.mark;
            if (*posp).lnum == lnum && (*posp).col >= mincol {
                (*posp).lnum += lnum_amount;
                if col_amount >
                       (-(2147483647 as libc::c_int) - 1 as libc::c_int) as
                           libc::c_long &&
                       col_amount <= 2147483647 as libc::c_int as libc::c_long
                   {
                } else {
                    __assert_fail(b"col_amount > INT_MIN && col_amount <= INT_MAX\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"/home/vole/neovim/src/nvim/mark.c\x00" as
                                      *const u8 as *const libc::c_char,
                                  1121 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 57],
                                                            &[libc::c_char; 57]>(b"void mark_col_adjust(linenr_T, colnr_T, long, long, int)\x00")).as_ptr());
                }
                if col_amount < 0 as libc::c_int as libc::c_long &&
                       (*posp).col <= -col_amount as colnr_T {
                    (*posp).col = 0 as libc::c_int
                } else if (*posp).col < spaces_removed {
                    (*posp).col = col_amount as libc::c_int + spaces_removed
                } else { (*posp).col += col_amount as colnr_T }
            }
        }
        i += 1
    }
    i = NMARKS;
    while i < NGLOBALMARKS {
        if namedfm[i as usize].fmark.fnum == fnum {
            posp = &mut (*namedfm.as_mut_ptr().offset(i as isize)).fmark.mark;
            if (*posp).lnum == lnum && (*posp).col >= mincol {
                (*posp).lnum += lnum_amount;
                if col_amount >
                       (-(2147483647 as libc::c_int) - 1 as libc::c_int) as
                           libc::c_long &&
                       col_amount <= 2147483647 as libc::c_int as libc::c_long
                   {
                } else {
                    __assert_fail(b"col_amount > INT_MIN && col_amount <= INT_MAX\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"/home/vole/neovim/src/nvim/mark.c\x00" as
                                      *const u8 as *const libc::c_char,
                                  1125 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 57],
                                                            &[libc::c_char; 57]>(b"void mark_col_adjust(linenr_T, colnr_T, long, long, int)\x00")).as_ptr());
                }
                if col_amount < 0 as libc::c_int as libc::c_long &&
                       (*posp).col <= -col_amount as colnr_T {
                    (*posp).col = 0 as libc::c_int
                } else if (*posp).col < spaces_removed {
                    (*posp).col = col_amount as libc::c_int + spaces_removed
                } else { (*posp).col += col_amount as colnr_T }
            }
        }
        i += 1
    }
    /* last Insert position */
    posp = &mut (*curbuf).b_last_insert.mark;
    if (*posp).lnum == lnum && (*posp).col >= mincol {
        (*posp).lnum += lnum_amount;
        if col_amount >
               (-(2147483647 as libc::c_int) - 1 as libc::c_int) as
                   libc::c_long &&
               col_amount <= 2147483647 as libc::c_int as libc::c_long {
        } else {
            __assert_fail(b"col_amount > INT_MIN && col_amount <= INT_MAX\x00"
                              as *const u8 as *const libc::c_char,
                          b"/home/vole/neovim/src/nvim/mark.c\x00" as
                              *const u8 as *const libc::c_char,
                          1129 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 57],
                                                    &[libc::c_char; 57]>(b"void mark_col_adjust(linenr_T, colnr_T, long, long, int)\x00")).as_ptr());
        }
        if col_amount < 0 as libc::c_int as libc::c_long &&
               (*posp).col <= -col_amount as colnr_T {
            (*posp).col = 0 as libc::c_int
        } else if (*posp).col < spaces_removed {
            (*posp).col = col_amount as libc::c_int + spaces_removed
        } else { (*posp).col += col_amount as colnr_T }
    }
    /* last change position */
    posp = &mut (*curbuf).b_last_change.mark;
    if (*posp).lnum == lnum && (*posp).col >= mincol {
        (*posp).lnum += lnum_amount;
        if col_amount >
               (-(2147483647 as libc::c_int) - 1 as libc::c_int) as
                   libc::c_long &&
               col_amount <= 2147483647 as libc::c_int as libc::c_long {
        } else {
            __assert_fail(b"col_amount > INT_MIN && col_amount <= INT_MAX\x00"
                              as *const u8 as *const libc::c_char,
                          b"/home/vole/neovim/src/nvim/mark.c\x00" as
                              *const u8 as *const libc::c_char,
                          1132 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 57],
                                                    &[libc::c_char; 57]>(b"void mark_col_adjust(linenr_T, colnr_T, long, long, int)\x00")).as_ptr());
        }
        if col_amount < 0 as libc::c_int as libc::c_long &&
               (*posp).col <= -col_amount as colnr_T {
            (*posp).col = 0 as libc::c_int
        } else if (*posp).col < spaces_removed {
            (*posp).col = col_amount as libc::c_int + spaces_removed
        } else { (*posp).col += col_amount as colnr_T }
    }
    /* list of change positions */
    i = 0 as libc::c_int;
    while i < (*curbuf).b_changelistlen {
        posp =
            &mut (*(*curbuf).b_changelist.as_mut_ptr().offset(i as
                                                                  isize)).mark;
        if (*posp).lnum == lnum && (*posp).col >= mincol {
            (*posp).lnum += lnum_amount;
            if col_amount >
                   (-(2147483647 as libc::c_int) - 1 as libc::c_int) as
                       libc::c_long &&
                   col_amount <= 2147483647 as libc::c_int as libc::c_long {
            } else {
                __assert_fail(b"col_amount > INT_MIN && col_amount <= INT_MAX\x00"
                                  as *const u8 as *const libc::c_char,
                              b"/home/vole/neovim/src/nvim/mark.c\x00" as
                                  *const u8 as *const libc::c_char,
                              1136 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 57],
                                                        &[libc::c_char; 57]>(b"void mark_col_adjust(linenr_T, colnr_T, long, long, int)\x00")).as_ptr());
            }
            if col_amount < 0 as libc::c_int as libc::c_long &&
                   (*posp).col <= -col_amount as colnr_T {
                (*posp).col = 0 as libc::c_int
            } else if (*posp).col < spaces_removed {
                (*posp).col = col_amount as libc::c_int + spaces_removed
            } else { (*posp).col += col_amount as colnr_T }
        }
        i += 1
    }
    /* Visual area */
    posp = &mut (*curbuf).b_visual.vi_start;
    if (*posp).lnum == lnum && (*posp).col >= mincol {
        (*posp).lnum += lnum_amount;
        if col_amount >
               (-(2147483647 as libc::c_int) - 1 as libc::c_int) as
                   libc::c_long &&
               col_amount <= 2147483647 as libc::c_int as libc::c_long {
        } else {
            __assert_fail(b"col_amount > INT_MIN && col_amount <= INT_MAX\x00"
                              as *const u8 as *const libc::c_char,
                          b"/home/vole/neovim/src/nvim/mark.c\x00" as
                              *const u8 as *const libc::c_char,
                          1139 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 57],
                                                    &[libc::c_char; 57]>(b"void mark_col_adjust(linenr_T, colnr_T, long, long, int)\x00")).as_ptr());
        }
        if col_amount < 0 as libc::c_int as libc::c_long &&
               (*posp).col <= -col_amount as colnr_T {
            (*posp).col = 0 as libc::c_int
        } else if (*posp).col < spaces_removed {
            (*posp).col = col_amount as libc::c_int + spaces_removed
        } else { (*posp).col += col_amount as colnr_T }
    }
    posp = &mut (*curbuf).b_visual.vi_end;
    if (*posp).lnum == lnum && (*posp).col >= mincol {
        (*posp).lnum += lnum_amount;
        if col_amount >
               (-(2147483647 as libc::c_int) - 1 as libc::c_int) as
                   libc::c_long &&
               col_amount <= 2147483647 as libc::c_int as libc::c_long {
        } else {
            __assert_fail(b"col_amount > INT_MIN && col_amount <= INT_MAX\x00"
                              as *const u8 as *const libc::c_char,
                          b"/home/vole/neovim/src/nvim/mark.c\x00" as
                              *const u8 as *const libc::c_char,
                          1140 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 57],
                                                    &[libc::c_char; 57]>(b"void mark_col_adjust(linenr_T, colnr_T, long, long, int)\x00")).as_ptr());
        }
        if col_amount < 0 as libc::c_int as libc::c_long &&
               (*posp).col <= -col_amount as colnr_T {
            (*posp).col = 0 as libc::c_int
        } else if (*posp).col < spaces_removed {
            (*posp).col = col_amount as libc::c_int + spaces_removed
        } else { (*posp).col += col_amount as colnr_T }
    }
    /* previous context mark */
    posp = &mut (*curwin).w_pcmark;
    if (*posp).lnum == lnum && (*posp).col >= mincol {
        (*posp).lnum += lnum_amount;
        if col_amount >
               (-(2147483647 as libc::c_int) - 1 as libc::c_int) as
                   libc::c_long &&
               col_amount <= 2147483647 as libc::c_int as libc::c_long {
        } else {
            __assert_fail(b"col_amount > INT_MIN && col_amount <= INT_MAX\x00"
                              as *const u8 as *const libc::c_char,
                          b"/home/vole/neovim/src/nvim/mark.c\x00" as
                              *const u8 as *const libc::c_char,
                          1143 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 57],
                                                    &[libc::c_char; 57]>(b"void mark_col_adjust(linenr_T, colnr_T, long, long, int)\x00")).as_ptr());
        }
        if col_amount < 0 as libc::c_int as libc::c_long &&
               (*posp).col <= -col_amount as colnr_T {
            (*posp).col = 0 as libc::c_int
        } else if (*posp).col < spaces_removed {
            (*posp).col = col_amount as libc::c_int + spaces_removed
        } else { (*posp).col += col_amount as colnr_T }
    }
    /* previous pcmark */
    posp = &mut (*curwin).w_prev_pcmark;
    if (*posp).lnum == lnum && (*posp).col >= mincol {
        (*posp).lnum += lnum_amount;
        if col_amount >
               (-(2147483647 as libc::c_int) - 1 as libc::c_int) as
                   libc::c_long &&
               col_amount <= 2147483647 as libc::c_int as libc::c_long {
        } else {
            __assert_fail(b"col_amount > INT_MIN && col_amount <= INT_MAX\x00"
                              as *const u8 as *const libc::c_char,
                          b"/home/vole/neovim/src/nvim/mark.c\x00" as
                              *const u8 as *const libc::c_char,
                          1146 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 57],
                                                    &[libc::c_char; 57]>(b"void mark_col_adjust(linenr_T, colnr_T, long, long, int)\x00")).as_ptr());
        }
        if col_amount < 0 as libc::c_int as libc::c_long &&
               (*posp).col <= -col_amount as colnr_T {
            (*posp).col = 0 as libc::c_int
        } else if (*posp).col < spaces_removed {
            (*posp).col = col_amount as libc::c_int + spaces_removed
        } else { (*posp).col += col_amount as colnr_T }
    }
    /* saved cursor for formatting */
    posp = &mut saved_cursor;
    if (*posp).lnum == lnum && (*posp).col >= mincol {
        (*posp).lnum += lnum_amount;
        if col_amount >
               (-(2147483647 as libc::c_int) - 1 as libc::c_int) as
                   libc::c_long &&
               col_amount <= 2147483647 as libc::c_int as libc::c_long {
        } else {
            __assert_fail(b"col_amount > INT_MIN && col_amount <= INT_MAX\x00"
                              as *const u8 as *const libc::c_char,
                          b"/home/vole/neovim/src/nvim/mark.c\x00" as
                              *const u8 as *const libc::c_char,
                          1149 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 57],
                                                    &[libc::c_char; 57]>(b"void mark_col_adjust(linenr_T, colnr_T, long, long, int)\x00")).as_ptr());
        }
        if col_amount < 0 as libc::c_int as libc::c_long &&
               (*posp).col <= -col_amount as colnr_T {
            (*posp).col = 0 as libc::c_int
        } else if (*posp).col < spaces_removed {
            (*posp).col = col_amount as libc::c_int + spaces_removed
        } else { (*posp).col += col_amount as colnr_T }
    }
    /*
   * Adjust items in all windows related to the current buffer.
   */
    let mut win =
        if curtab == curtab { firstwin } else { (*curtab).tp_firstwin };
    while !win.is_null() {
        /* marks in the jumplist */
        i = 0 as libc::c_int;
        while i < (*win).w_jumplistlen {
            if (*win).w_jumplist[i as usize].fmark.fnum == fnum {
                posp =
                    &mut (*(*win).w_jumplist.as_mut_ptr().offset(i as
                                                                     isize)).fmark.mark;
                if (*posp).lnum == lnum && (*posp).col >= mincol {
                    (*posp).lnum += lnum_amount;
                    if col_amount >
                           (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                               as libc::c_long &&
                           col_amount <=
                               2147483647 as libc::c_int as libc::c_long {
                    } else {
                        __assert_fail(b"col_amount > INT_MIN && col_amount <= INT_MAX\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"/home/vole/neovim/src/nvim/mark.c\x00"
                                          as *const u8 as *const libc::c_char,
                                      1158 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 57],
                                                                &[libc::c_char; 57]>(b"void mark_col_adjust(linenr_T, colnr_T, long, long, int)\x00")).as_ptr());
                    }
                    if col_amount < 0 as libc::c_int as libc::c_long &&
                           (*posp).col <= -col_amount as colnr_T {
                        (*posp).col = 0 as libc::c_int
                    } else if (*posp).col < spaces_removed {
                        (*posp).col =
                            col_amount as libc::c_int + spaces_removed
                    } else { (*posp).col += col_amount as colnr_T }
                }
            }
            i += 1
        }
        if (*win).w_buffer == curbuf {
            /* marks in the tag stack */
            i = 0 as libc::c_int;
            while i < (*win).w_tagstacklen {
                if (*win).w_tagstack[i as usize].fmark.fnum == fnum {
                    posp =
                        &mut (*(*win).w_tagstack.as_mut_ptr().offset(i as
                                                                         isize)).fmark.mark;
                    if (*posp).lnum == lnum && (*posp).col >= mincol {
                        (*posp).lnum += lnum_amount;
                        if col_amount >
                               (-(2147483647 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_long &&
                               col_amount <=
                                   2147483647 as libc::c_int as libc::c_long {
                        } else {
                            __assert_fail(b"col_amount > INT_MIN && col_amount <= INT_MAX\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"/home/vole/neovim/src/nvim/mark.c\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          1166 as libc::c_int as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 57],
                                                                    &[libc::c_char; 57]>(b"void mark_col_adjust(linenr_T, colnr_T, long, long, int)\x00")).as_ptr());
                        }
                        if col_amount < 0 as libc::c_int as libc::c_long &&
                               (*posp).col <= -col_amount as colnr_T {
                            (*posp).col = 0 as libc::c_int
                        } else if (*posp).col < spaces_removed {
                            (*posp).col =
                                col_amount as libc::c_int + spaces_removed
                        } else { (*posp).col += col_amount as colnr_T }
                    }
                }
                i += 1
            }
            /* cursor position for other windows with the same buffer */
            if win != curwin {
                posp = &mut (*win).w_cursor;
                if (*posp).lnum == lnum && (*posp).col >= mincol {
                    (*posp).lnum += lnum_amount;
                    if col_amount >
                           (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                               as libc::c_long &&
                           col_amount <=
                               2147483647 as libc::c_int as libc::c_long {
                    } else {
                        __assert_fail(b"col_amount > INT_MIN && col_amount <= INT_MAX\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"/home/vole/neovim/src/nvim/mark.c\x00"
                                          as *const u8 as *const libc::c_char,
                                      1172 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 57],
                                                                &[libc::c_char; 57]>(b"void mark_col_adjust(linenr_T, colnr_T, long, long, int)\x00")).as_ptr());
                    }
                    if col_amount < 0 as libc::c_int as libc::c_long &&
                           (*posp).col <= -col_amount as colnr_T {
                        (*posp).col = 0 as libc::c_int
                    } else if (*posp).col < spaces_removed {
                        (*posp).col =
                            col_amount as libc::c_int + spaces_removed
                    } else { (*posp).col += col_amount as colnr_T }
                }
            }
        }
        win = (*win).w_next
    };
}
// When deleting lines, this may create duplicate marks in the
// jumplist. They will be removed here for the specified window.
// When "checktail" is true, removes tail jump if it matches current position.
#[no_mangle]
#[c2rust::src_loc = "1181:1"]
pub unsafe extern "C" fn cleanup_jumplist(mut wp: *mut win_T,
                                          mut checktail: bool) {
    let mut i: libc::c_int = 0;
    // Load all the files from the jump list. This is
  // needed to properly clean up duplicate entries, but will take some
  // time.
    i = 0 as libc::c_int;
    while i < (*wp).w_jumplistlen {
        if (*wp).w_jumplist[i as usize].fmark.fnum == 0 as libc::c_int &&
               (*wp).w_jumplist[i as usize].fmark.mark.lnum !=
                   0 as libc::c_int as libc::c_long {
            fname2fnum(&mut *(*wp).w_jumplist.as_mut_ptr().offset(i as
                                                                      isize));
        }
        i += 1
    }
    let mut to = 0 as libc::c_int;
    let mut from = 0 as libc::c_int;
    while from < (*wp).w_jumplistlen {
        if (*wp).w_jumplistidx == from { (*wp).w_jumplistidx = to }
        i = from + 1 as libc::c_int;
        while i < (*wp).w_jumplistlen {
            if (*wp).w_jumplist[i as usize].fmark.fnum ==
                   (*wp).w_jumplist[from as usize].fmark.fnum &&
                   (*wp).w_jumplist[from as usize].fmark.fnum !=
                       0 as libc::c_int &&
                   (*wp).w_jumplist[i as usize].fmark.mark.lnum ==
                       (*wp).w_jumplist[from as usize].fmark.mark.lnum {
                break ;
            }
            i += 1
        }
        let mut mustfree: bool = false;
        if i >= (*wp).w_jumplistlen {
            // not duplicate
            mustfree = false_0 != 0
        } else if i > from + 1 as libc::c_int {
            mustfree = jop_flags & JOP_STACK as libc::c_uint == 0
        } else { mustfree = true_0 != 0 } // adjacent duplicate
        if mustfree {
            xfree((*wp).w_jumplist[from as usize].fname as *mut libc::c_void);
        } else {
            if to != from {
                // non-adjacent duplicate
                // jumpoptions=stack: remove duplicates only when adjacent.
                // Not using wp->w_jumplist[to++] = wp->w_jumplist[from] because
        // this way valgrind complains about overlapping source and destination
        // in memcpy() call. (clang-3.6.0, debug build with -DEXITFREE).
                (*wp).w_jumplist[to as usize] =
                    (*wp).w_jumplist[from as usize]
            }
            to += 1
        }
        from += 1
    }
    if (*wp).w_jumplistidx == (*wp).w_jumplistlen { (*wp).w_jumplistidx = to }
    (*wp).w_jumplistlen = to;
    // When pointer is below last jump, remove the jump if it matches the current
  // line.  This avoids useless/phantom jumps. #9805
    if checktail as libc::c_int != 0 && (*wp).w_jumplistlen != 0 &&
           (*wp).w_jumplistidx == (*wp).w_jumplistlen {
        let mut fm_last: *const xfmark_T =
            &mut *(*wp).w_jumplist.as_mut_ptr().offset(((*wp).w_jumplistlen -
                                                            1 as libc::c_int)
                                                           as isize) as
                *mut xfmark_T;
        if (*fm_last).fmark.fnum == (*curbuf).handle &&
               (*fm_last).fmark.mark.lnum == (*wp).w_cursor.lnum {
            xfree((*fm_last).fname as *mut libc::c_void);
            (*wp).w_jumplistlen -= 1;
            (*wp).w_jumplistidx -= 1
        }
    };
}
/*
 * Copy the jumplist from window "from" to window "to".
 */
#[no_mangle]
#[c2rust::src_loc = "1254:1"]
pub unsafe extern "C" fn copy_jumplist(mut from: *mut win_T,
                                       mut to: *mut win_T) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*from).w_jumplistlen {
        (*to).w_jumplist[i as usize] = (*from).w_jumplist[i as usize];
        if !(*from).w_jumplist[i as usize].fname.is_null() {
            (*to).w_jumplist[i as usize].fname =
                vim_strsave((*from).w_jumplist[i as usize].fname)
        }
        i += 1
    }
    (*to).w_jumplistlen = (*from).w_jumplistlen;
    (*to).w_jumplistidx = (*from).w_jumplistidx;
}
// / Iterate over jumplist items
// /
// / @warning No jumplist-editing functions must be run while iteration is in
// /          progress.
// /
// / @param[in]   iter  Iterator. Pass NULL to start iteration.
// / @param[in]   win   Window for which jump list is processed.
// / @param[out]  fm    Item definition.
// /
// / @return Pointer that needs to be passed to next `mark_jumplist_iter` call or
// /         NULL if iteration is over.
#[no_mangle]
#[c2rust::src_loc = "1278:1"]
pub unsafe extern "C" fn mark_jumplist_iter(iter: *const libc::c_void,
                                            win: *const win_T,
                                            fm: *mut xfmark_T)
 -> *const libc::c_void {
    if iter == NULL_1 as *const libc::c_void &&
           (*win).w_jumplistlen == 0 as libc::c_int {
        *fm =
            {
                let mut init =
                    xfilemark{fmark:
                                  {
                                      let mut init =
                                          filemark{mark:
                                                       {
                                                           let mut init =
                                                               pos_T{lnum:
                                                                         0 as
                                                                             libc::c_int
                                                                             as
                                                                             linenr_T,
                                                                     col:
                                                                         0 as
                                                                             libc::c_int,
                                                                     coladd:
                                                                         0 as
                                                                             libc::c_int,};
                                                           init
                                                       },
                                                   fnum: 0 as libc::c_int,
                                                   timestamp:
                                                       0 as libc::c_int as
                                                           Timestamp,
                                                   additional_data:
                                                       NULL_1 as
                                                           *mut dict_T,};
                                      init
                                  },
                              fname: NULL_1 as *mut char_u,};
                init
            };
        return NULL_1 as *const libc::c_void
    }
    let iter_mark =
        if iter == NULL_1 as *const libc::c_void {
            &*(*win).w_jumplist.as_ptr().offset(0 as libc::c_int as isize) as
                *const xfmark_T
        } else { iter as *const xfmark_T };
    *fm = *iter_mark;
    if iter_mark ==
           &*(*win).w_jumplist.as_ptr().offset(((*win).w_jumplistlen -
                                                    1 as libc::c_int) as
                                                   isize) as *const xfmark_T {
        return NULL_1 as *const libc::c_void
    } else {
        return iter_mark.offset(1 as libc::c_int as isize) as
                   *const libc::c_void
    };
}
// / Iterate over global marks
// /
// / @warning No mark-editing functions must be run while iteration is in
// /          progress.
// /
// / @param[in]   iter  Iterator. Pass NULL to start iteration.
// / @param[out]  name  Mark name.
// / @param[out]  fm    Mark definition.
// /
// / @return Pointer that needs to be passed to next `mark_global_iter` call or
// /         NULL if iteration is over.
#[no_mangle]
#[c2rust::src_loc = "1309:1"]
pub unsafe extern "C" fn mark_global_iter(iter: *const libc::c_void,
                                          name: *mut libc::c_char,
                                          fm: *mut xfmark_T)
 -> *const libc::c_void {
    *name = NUL as libc::c_char;
    let mut iter_mark =
        if iter == NULL_1 as *const libc::c_void {
            &mut *namedfm.as_mut_ptr().offset(0 as libc::c_int as isize) as
                *mut xfmark_T
        } else { iter as *const xfmark_T };
    while (iter_mark.wrapping_offset_from(&mut *namedfm.as_mut_ptr().offset(0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                              as *mut xfmark_T) as
               libc::c_long as size_t) <
              (::std::mem::size_of::<[xfmark_T; 36]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<xfmark_T>()
                                                   as
                                                   libc::c_ulong).wrapping_div(((::std::mem::size_of::<[xfmark_T; 36]>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_rem(::std::mem::size_of::<xfmark_T>()
                                                                                                                     as
                                                                                                                     libc::c_ulong)
                                                                                    ==
                                                                                    0)
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   size_t)
              && (*iter_mark).fmark.mark.lnum == 0 {
        iter_mark = iter_mark.offset(1)
    }
    if iter_mark.wrapping_offset_from(&mut *namedfm.as_mut_ptr().offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                          as *mut xfmark_T) as libc::c_long as
           size_t ==
           (::std::mem::size_of::<[xfmark_T; 36]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<xfmark_T>()
                                                as
                                                libc::c_ulong).wrapping_div(((::std::mem::size_of::<[xfmark_T; 36]>()
                                                                                  as
                                                                                  libc::c_ulong).wrapping_rem(::std::mem::size_of::<xfmark_T>()
                                                                                                                  as
                                                                                                                  libc::c_ulong)
                                                                                 ==
                                                                                 0)
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                size_t)
           || (*iter_mark).fmark.mark.lnum == 0 {
        return NULL_1 as *const libc::c_void
    }
    let mut iter_off =
        iter_mark.wrapping_offset_from(&mut *namedfm.as_mut_ptr().offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                           as *mut xfmark_T) as libc::c_long
            as size_t;
    *name =
        if iter_off < NMARKS as libc::c_ulong {
            ('A' as i32) + iter_off as libc::c_char as libc::c_int
        } else {
            ('0' as i32) +
                iter_off.wrapping_sub(NMARKS as libc::c_ulong) as libc::c_char
                    as libc::c_int
        } as libc::c_char;
    *fm = *iter_mark;
    loop  {
        iter_mark = iter_mark.offset(1);
        if !((iter_mark.wrapping_offset_from(&mut *namedfm.as_mut_ptr().offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                                 as *mut xfmark_T) as
                  libc::c_long as size_t) <
                 (::std::mem::size_of::<[xfmark_T; 36]>() as
                      libc::c_ulong).wrapping_div(::std::mem::size_of::<xfmark_T>()
                                                      as
                                                      libc::c_ulong).wrapping_div(((::std::mem::size_of::<[xfmark_T; 36]>()
                                                                                        as
                                                                                        libc::c_ulong).wrapping_rem(::std::mem::size_of::<xfmark_T>()
                                                                                                                        as
                                                                                                                        libc::c_ulong)
                                                                                       ==
                                                                                       0)
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      size_t))
           {
            break ;
        }
        if (*iter_mark).fmark.mark.lnum != 0 {
            return iter_mark as *const libc::c_void
        }
    }
    return NULL_1 as *const libc::c_void;
}
// / Get next mark and its name
// /
// / @param[in]      buf        Buffer for which next mark is taken.
// / @param[in,out]  mark_name  Pointer to the current mark name. Next mark name
// /                            will be saved at this address as well.
// /
// /                            Current mark name must either be NUL, '"', '^',
// /                            '.' or 'a' .. 'z'. If it is neither of these
// /                            behaviour is undefined.
// /
// / @return Pointer to the next mark or NULL.
#[inline]
#[c2rust::src_loc = "1349:1"]
unsafe extern "C" fn next_buffer_mark(buf: *const buf_T,
                                      mark_name: *mut libc::c_char)
 -> *const fmark_T {
    match *mark_name as libc::c_int {
        NUL => {
            *mark_name = '\"' as i32 as libc::c_char;
            return &(*buf).b_last_cursor
        }
        34 => {
            *mark_name = '^' as i32 as libc::c_char;
            return &(*buf).b_last_insert
        }
        94 => {
            *mark_name = '.' as i32 as libc::c_char;
            return &(*buf).b_last_change
        }
        46 => {
            *mark_name = 'a' as i32 as libc::c_char;
            return &*(*buf).b_namedm.as_ptr().offset(0 as libc::c_int as
                                                         isize) as
                       *const fmark_T
        }
        122 => { return NULL_1 as *const fmark_T }
        _ => {
            *mark_name += 1;
            return &*(*buf).b_namedm.as_ptr().offset((*mark_name as
                                                          libc::c_int -
                                                          'a' as i32) as
                                                         isize) as
                       *const fmark_T
        }
    };
}
// / Iterate over buffer marks
// /
// / @warning No mark-editing functions must be run while iteration is in
// /          progress.
// /
// / @param[in]   iter  Iterator. Pass NULL to start iteration.
// / @param[in]   buf   Buffer.
// / @param[out]  name  Mark name.
// / @param[out]  fm    Mark definition.
// /
// / @return Pointer that needs to be passed to next `mark_buffer_iter` call or
// /         NULL if iteration is over.
#[no_mangle]
#[c2rust::src_loc = "1392:1"]
pub unsafe extern "C" fn mark_buffer_iter(iter: *const libc::c_void,
                                          buf: *const buf_T,
                                          name: *mut libc::c_char,
                                          fm: *mut fmark_T)
 -> *const libc::c_void {
    *name = NUL as libc::c_char;
    let mut mark_name =
        if iter == NULL_1 as *const libc::c_void {
            NUL
        } else if iter ==
                      &(*buf).b_last_cursor as *const fmark_T as
                          *const libc::c_void {
            '\"' as i32
        } else if iter ==
                      &(*buf).b_last_insert as *const fmark_T as
                          *const libc::c_void {
            '^' as i32
        } else if iter ==
                      &(*buf).b_last_change as *const fmark_T as
                          *const libc::c_void {
            '.' as i32
        } else {
            ('a' as i32) +
                (iter as
                     *const fmark_T).wrapping_offset_from(&*(*buf).b_namedm.as_ptr().offset(0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize)
                                                              as
                                                              *const fmark_T)
                    as libc::c_long as libc::c_char as libc::c_int
        } as libc::c_char;
    let mut iter_mark = next_buffer_mark(buf, &mut mark_name);
    while !iter_mark.is_null() &&
              (*iter_mark).mark.lnum == 0 as libc::c_int as libc::c_long {
        iter_mark = next_buffer_mark(buf, &mut mark_name)
    }
    if iter_mark.is_null() { return NULL_1 as *const libc::c_void }
    let mut iter_off =
        iter_mark.wrapping_offset_from(&*(*buf).b_namedm.as_ptr().offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                           as *const fmark_T) as libc::c_long
            as size_t;
    if mark_name != 0 {
        *name = mark_name
    } else {
        *name =
            ('a' as i32 + iter_off as libc::c_char as libc::c_int) as
                libc::c_char
    }
    *fm = *iter_mark;
    return iter_mark as *const libc::c_void;
}
// / Set global mark
// /
// / @param[in]  name    Mark name.
// / @param[in]  fm      Mark to be set.
// / @param[in]  update  If true then only set global mark if it was created
// /                     later then existing one.
// /
// / @return true on success, false on failure.
#[no_mangle]
#[c2rust::src_loc = "1432:1"]
pub unsafe extern "C" fn mark_set_global(name: libc::c_char, fm: xfmark_T,
                                         update: bool) -> bool {
    let idx = mark_global_index(name);
    if idx == -(1 as libc::c_int) { return false_0 != 0 }
    let fm_tgt: *mut xfmark_T =
        &mut *namedfm.as_mut_ptr().offset(idx as isize) as *mut xfmark_T;
    if update as libc::c_int != 0 &&
           fm.fmark.timestamp <= (*fm_tgt).fmark.timestamp {
        return false_0 != 0
    }
    if (*fm_tgt).fmark.mark.lnum != 0 as libc::c_int as libc::c_long {
        free_xfmark(*fm_tgt);
    }
    *fm_tgt = fm;
    return true_0 != 0;
}
// / Set local mark
// /
// / @param[in]  name    Mark name.
// / @param[in]  buf     Pointer to the buffer to set mark in.
// / @param[in]  fm      Mark to be set.
// / @param[in]  update  If true then only set global mark if it was created
// /                     later then existing one.
// /
// / @return true on success, false on failure.
#[no_mangle]
#[c2rust::src_loc = "1458:1"]
pub unsafe extern "C" fn mark_set_local(name: libc::c_char, buf: *mut buf_T,
                                        fm: fmark_T, update: bool) -> bool {
    let mut fm_tgt = NULL_1 as *mut fmark_T;
    if name as libc::c_uint >= 'a' as i32 as libc::c_uint &&
           name as libc::c_uint <= 'z' as i32 as libc::c_uint {
        fm_tgt =
            &mut *(*buf).b_namedm.as_mut_ptr().offset((name as libc::c_int -
                                                           'a' as i32) as
                                                          isize) as
                *mut fmark_T
    } else if name as libc::c_int == '\"' as i32 {
        fm_tgt = &mut (*buf).b_last_cursor
    } else if name as libc::c_int == '^' as i32 {
        fm_tgt = &mut (*buf).b_last_insert
    } else if name as libc::c_int == '.' as i32 {
        fm_tgt = &mut (*buf).b_last_change
    } else { return false_0 != 0 }
    if update as libc::c_int != 0 && fm.timestamp <= (*fm_tgt).timestamp {
        return false_0 != 0
    }
    if (*fm_tgt).mark.lnum != 0 as libc::c_int as libc::c_long {
        free_fmark(*fm_tgt);
    }
    *fm_tgt = fm;
    return true_0 != 0;
}
/*
 * Free items in the jumplist of window "wp".
 */
#[no_mangle]
#[c2rust::src_loc = "1487:1"]
pub unsafe extern "C" fn free_jumplist(mut wp: *mut win_T) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*wp).w_jumplistlen {
        free_xfmark((*wp).w_jumplist[i as usize]);
        i += 1
    }
    (*wp).w_jumplistlen = 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1497:1"]
pub unsafe extern "C" fn set_last_cursor(mut win: *mut win_T) {
    if !(*win).w_buffer.is_null() {
        let fmarkp___: *mut fmark_T = &mut (*(*win).w_buffer).b_last_cursor;
        free_fmark(*fmarkp___);
        let fmarkp__ = fmarkp___;
        (*fmarkp__).mark = (*win).w_cursor;
        (*fmarkp__).fnum = 0 as libc::c_int;
        (*fmarkp__).timestamp = os_time();
        (*fmarkp__).additional_data = NULL_1 as *mut dict_T
    };
}
// / Adjust position to point to the first byte of a multi-byte character
// /
// / If it points to a tail byte it is move backwards to the head byte.
// /
// / @param[in]  buf  Buffer to adjust position in.
// / @param[out]  lp  Position to adjust.
#[no_mangle]
#[c2rust::src_loc = "1524:1"]
pub unsafe extern "C" fn mark_mb_adjustpos(mut buf: *mut buf_T,
                                           mut lp: *mut pos_T) {
    if (*lp).col > 0 as libc::c_int || (*lp).coladd > 1 as libc::c_int {
        let p: *const char_u = ml_get_buf(buf, (*lp).lnum, false_0 != 0);
        if *p as libc::c_int == NUL ||
               (strlen(p as *mut libc::c_char) as libc::c_int) < (*lp).col {
            (*lp).col = 0 as libc::c_int
        } else { (*lp).col -= utf_head_off(p, p.offset((*lp).col as isize)) }
        // Reset "coladd" when the cursor would be on the right half of a
    // double-wide character.
        if (*lp).coladd == 1 as libc::c_int &&
               *p.offset((*lp).col as isize) as libc::c_int != TAB &&
               vim_isprintc(utf_ptr2char(p.offset((*lp).col as isize))) as
                   libc::c_int != 0 &&
               ptr2cells(p.offset((*lp).col as isize)) > 1 as libc::c_int {
            (*lp).coladd = 0 as libc::c_int
        }
    };
}
