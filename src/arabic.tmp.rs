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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:18"]
pub mod types_h {
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
    #[c2rust::src_loc = "158:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-intn.h:18"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/types.h:18"]
pub mod nvim_types_h {
    #[c2rust::src_loc = "11:1"]
    pub type char_u = libc::c_uchar;
    #[c2rust::src_loc = "17:1"]
    pub type handle_T = libc::c_int;
    #[c2rust::src_loc = "22:1"]
    pub type LuaRef = libc::c_int;
    // NVIM_TYPES_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/eval/typval.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/pos.h:18"]
pub mod pos_h {
    #[c2rust::src_loc = "4:1"]
    pub type linenr_T = libc::c_long;
    #[c2rust::src_loc = "9:1"]
    pub type colnr_T = libc::c_int;
    // line number type
    // / Format used to print values which have linenr_T type
    // / Column number type
    // / Format used to print values which have colnr_T type
    // / Maximal (invalid) line number
    // / Maximal column number, 31 bits
    // Minimum line number
    // minimum column number
    /*
 * position in file or buffer
 */
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
  "/usr/lib/llvm-6.0/lib/clang/6.0.1/include/stddef.h:18"]
pub mod stddef_h {
    #[c2rust::src_loc = "62:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "105:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "105:11"]
    pub const NULL_0: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "105:11"]
    pub const NULL_1: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src =
  "/usr/lib/llvm-6.0/lib/clang/6.0.1/include/stdarg.h:18"]
pub mod stdarg_h {
    #[c2rust::src_loc = "30:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/time_t.h:18"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/hashtab.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/garray.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/lib/queue.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/profile.h:18"]
pub mod profile_h {
    #[c2rust::src_loc = "7:1"]
    pub type proftime_T = uint64_t;
    use super::stdint_uintn_h::uint64_t;
    // NVIM_PROFILE_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/grid_defs.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/buffer_defs.h:18"]
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
        pub update_channels: C2RustUnnamed_1,
        pub update_callbacks: C2RustUnnamed_0,
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
    pub struct C2RustUnnamed_0 {
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
    pub struct C2RustUnnamed_1 {
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
        pub w_p_lcs_chars: C2RustUnnamed_3,
        pub w_p_fcs_chars: C2RustUnnamed_2,
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
    pub struct C2RustUnnamed_2 {
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
    pub struct C2RustUnnamed_3 {
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/map.h:18"]
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
    use super::khash_h::{khint_t, khint32_t};
    use super::stdint_uintn_h::uint64_t;
    use super::extmark_defs_h::ExtmarkItem;
    use super::map_defs_h::ptr_t;
    // NVIM_MAP_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/lib/khash.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/extmark_defs.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/marktree.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/map_defs.h:18"]
pub mod map_defs_h {
    #[c2rust::src_loc = "7:1"]
    pub type ptr_t = *mut libc::c_void;
    // NVIM_MAP_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/terminal.h:18"]
pub mod terminal_h {
    #[c2rust::src_loc = "8:1"]
    pub type Terminal = terminal;
    extern "C" {
        #[c2rust::src_loc = "8:16"]
        pub type terminal;
    }
    // NVIM_TERMINAL_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/sign_defs.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/regexp_defs.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/api/private/defs.h:18"]
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
    use super::nvim_types_h::handle_T;
    use super::stdint_uintn_h::uint64_t;
    // NVIM_API_PRIVATE_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/mark_defs.h:18"]
pub mod mark_defs_h {
    /*
 * marks: positions in a file
 * (a normal mark is a lnum/col pair, the same as a file position)
 */
    // / Number of possible numbered global marks
    // / Maximum possible number of letter marks
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
    use super::pos_h::pos_T;
    use super::time_h::Timestamp;
    use super::typval_h::dict_T;
    use super::nvim_types_h::char_u;
    // /< Actual mark.
    // /< File name, used when fnum == 0.
    // NVIM_MARK_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/os/time.h:18"]
pub mod time_h {
    #[c2rust::src_loc = "8:1"]
    pub type Timestamp = uint64_t;
    use super::stdint_uintn_h::uint64_t;
    // NVIM_OS_TIME_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/option_defs.h:18"]
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
        #[c2rust::src_loc = "466:12"]
        pub static mut p_arshape: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "646:12"]
        pub static mut p_tbidi: libc::c_int;
    }
    // NVIM_OPTION_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/syntax_defs.h:18"]
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
        pub sst_union: C2RustUnnamed_4,
        pub sst_next_flags: libc::c_int,
        pub sst_stacksize: libc::c_int,
        pub sst_next_list: *mut int16_t,
        pub sst_tick: disptick_T,
        pub sst_change_lnum: linenr_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:3"]
    pub union C2RustUnnamed_4 {
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/undo_defs.h:18"]
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
        pub uh_next: C2RustUnnamed_8,
        pub uh_prev: C2RustUnnamed_7,
        pub uh_alt_next: C2RustUnnamed_6,
        pub uh_alt_prev: C2RustUnnamed_5,
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
    pub union C2RustUnnamed_5 {
        pub ptr: *mut u_header_T,
        pub seq: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:3"]
    pub union C2RustUnnamed_6 {
        pub ptr: *mut u_header_T,
        pub seq: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:3"]
    pub union C2RustUnnamed_7 {
        pub ptr: *mut u_header_T,
        pub seq: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:3"]
    pub union C2RustUnnamed_8 {
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/os/fs_defs.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/memline_defs.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/memfile_defs.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/defs.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/loop.h:18"]
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
    use super::memory_h_generated_h::{xcalloc, xfree, xrealloc};
    use super::assert_h::__assert_fail;
    // NVIM_EVENT_LOOP_H
}
#[c2rust::header_src =
  "/usr/lib/llvm-6.0/lib/clang/6.0.1/include/stdbool.h:16"]
pub mod stdbool_h {
    #[c2rust::src_loc = "32:9"]
    pub const true_0: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "33:9"]
    pub const false_0: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/byteswap.h:18"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
        return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
                    |
                    (__bsx as libc::c_int & 0xff as libc::c_int) <<
                        8 as libc::c_int) as __uint16_t;
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
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/uintn-identity.h:18"]
pub mod uintn_identity_h {
    #[inline]
    #[c2rust::src_loc = "32:1"]
    pub unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t)
     -> __uint16_t {
        return __x;
    }
    #[inline]
    #[c2rust::src_loc = "38:1"]
    pub unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t)
     -> __uint32_t {
        return __x;
    }
    #[inline]
    #[c2rust::src_loc = "44:1"]
    pub unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t)
     -> __uint64_t {
        return __x;
    }
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/usr/include/string.h:18"]
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
#[c2rust::header_src = "/usr/include/assert.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/build/include/log.h.generated.h:18"]
pub mod log_h_generated_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "7:1"]
        pub fn logmsg(log_level: libc::c_int, context: *const libc::c_char,
                      func_name: *const libc::c_char, line_num: libc::c_int,
                      eol: bool, fmt: *const libc::c_char, _: ...) -> bool;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/log.h:18"]
pub mod log_h {
    #[c2rust::src_loc = "11:9"]
    pub const WARN_LOG_LEVEL: libc::c_int = 2 as libc::c_int;
    // NVIM_LOG_H
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/mbyte.h.generated.h:18"]
pub mod mbyte_h_generated_h {
    use super::nvim_types_h::char_u;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "26:1"]
        pub fn utf_char2bytes(c: libc::c_int, buf: *mut char_u)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "40:1"]
        pub fn mb_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/mbyte.h:18"]
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
    use super::mbyte_h_generated_h::mb_stricmp;
    use super::string_h::strcmp;
    // NVIM_MBYTE_H
}
#[c2rust::header_src = "/usr/include/libintl.h:18"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/message.h.generated.h:18"]
pub mod message_h_generated_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "22:1"]
        pub fn emsgf(fmt: *const libc::c_char, _: ...) -> bool;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/strings.h:18"]
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
  "/home/vole/neovim/build/include/memory.h.generated.h:18"]
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
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/lib/kvec.h:18"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/ascii.h:19"]
pub mod ascii_h {
    // Definitions of various common control characters.
    #[c2rust::src_loc = "19:9"]
    pub const NUL: libc::c_int = '\u{0}' as i32;
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/arabic.h:20"]
pub mod arabic_h {
    // / Whether c belongs to the range of Arabic characters that might be shaped.
    #[inline]
    #[c2rust::src_loc = "7:1"]
    pub unsafe extern "C" fn arabic_char(mut c: libc::c_int) -> bool {
        // return c >= a_HAMZA && c <= a_MINI_ALEF;
        return c >= 0x621 as libc::c_int && c <= 0x670 as libc::c_int;
    }
    // NVIM_ARABIC_H
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __int64_t, __uint64_t, __time_t};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
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
                         DictWatcher, ScopeDictDictItem, tv_list_ref,
                         tv_list_set_ret, tv_list_locked, tv_list_set_lock,
                         tv_list_set_copyid, tv_list_len, tv_list_copyid,
                         tv_list_latest_copy, tv_list_uidx,
                         tv_list_has_watchers, tv_list_first, tv_list_last,
                         tv_dict_set_ret, tv_dict_len, tv_dict_is_watched,
                         tv_init, tv_get_float_chk, tv_dict_watcher_node_data,
                         tv_is_func, funccall_S};
pub use self::pos_h::{linenr_T, colnr_T, pos_T, lpos_T};
pub use self::stddef_h::{size_t, NULL, NULL_0, NULL_1};
pub use self::stdarg_h::va_list;
pub use self::time_t_h::time_t;
pub use self::hashtab_h::{hash_T, hashitem_S, hashitem_T, hashtable_S,
                          hashtab_T};
pub use self::garray_h::{growarray, garray_T, ga_append_via_ptr, ga_grow};
pub use self::queue_h::{_queue, QUEUE, QUEUE_EMPTY, QUEUE_INIT, QUEUE_ADD,
                        QUEUE_INSERT_HEAD, QUEUE_INSERT_TAIL, QUEUE_REMOVE};
pub use self::profile_h::proftime_T;
pub use self::grid_defs_h::{schar_T, sattr_T, ScreenGrid};
pub use self::buffer_defs_h::{file_buffer, C2RustUnnamed_0,
                              BufUpdateCallbacks, C2RustUnnamed_1, synblock_T,
                              buf_T, win_T, window_S, qf_info_T, FloatConfig,
                              WinStyle, kWinStyleMinimal, kWinStyleUnused,
                              FloatRelative, kFloatRelativeCursor,
                              kFloatRelativeWindow, kFloatRelativeEditor,
                              FloatAnchor, taggy_T, taggy, matchitem_T,
                              matchitem, match_T, posmatch_T, posmatch,
                              llpos_T, winopt_T, alist_T, arglist, wline_T,
                              w_line, pos_save_T, C2RustUnnamed_2,
                              C2RustUnnamed_3, frame_T, frame_S, disptick_T,
                              syn_time_T, mapblock_T, mapblock, wininfo_T,
                              wininfo_S, ChangedtickDictItem, win_hl_attr,
                              qf_info_S};
pub use self::map_h::{Map_uint64_t_ExtmarkNs, kh_uint64_t_ExtmarkNs_map_t,
                      ExtmarkNs, Map_uint64_t_uint64_t,
                      kh_uint64_t_uint64_t_map_t, Map_uint64_t_ExtmarkItem,
                      kh_uint64_t_ExtmarkItem_map_t, Map_uint64_t_ptr_t,
                      kh_uint64_t_ptr_t_map_t};
pub use self::khash_h::{khint32_t, khint_t, __ac_X31_hash_string,
                        __ac_Wang_hash};
pub use self::extmark_defs_h::{ExtmarkItem, VirtText, VirtTextChunk,
                               extmark_undo_vec_t, ExtmarkUndoObject,
                               undo_object};
pub use self::marktree_h::{MarkTree, mtnode_t, mtnode_s, mtkey_t, mtpos_t};
pub use self::map_defs_h::ptr_t;
pub use self::terminal_h::{Terminal, terminal};
pub use self::sign_defs_h::{signlist_T, signlist, signgroup_T, signgroup_S};
pub use self::regexp_defs_h::{regprog_T, regprog, regengine_T, regengine,
                              regmmatch_T, regmatch_T, reg_extmatch_T,
                              reg_extmatch};
pub use self::defs_h::{Window, is_internal_call};
pub use self::mark_defs_h::{fmark_T, filemark, xfmark_T, xfilemark};
pub use self::time_h::Timestamp;
pub use self::option_defs_h::{LastSet, p_arshape, p_tbidi};
pub use self::syntax_defs_h::{synstate_T, syn_state, C2RustUnnamed_4,
                              bufstate_T, buf_state};
pub use self::undo_defs_h::{u_header_T, u_header, visualinfo_T, u_entry_T,
                            u_entry, C2RustUnnamed_5, C2RustUnnamed_6,
                            C2RustUnnamed_7, C2RustUnnamed_8};
pub use self::fs_defs_h::FileID;
pub use self::memline_defs_h::{memline_T, memline, chunksize_T, ml_chunksize,
                               infoptr_T, info_pointer};
pub use self::memfile_defs_h::{bhdr_T, bhdr, mf_hashitem_T, mf_hashitem,
                               blocknr_T, memfile_T, memfile, mf_hashtab_T,
                               mf_hashtab};
pub use self::event_defs_h::{argv_callback, message, Event, event_create};
pub use self::loop_h::{WatcherPtr, __kl1_WatcherPtr, kl1_WatcherPtr,
                       kmp_WatcherPtr_t, kl_WatcherPtr_t, kmp_init_WatcherPtr,
                       kmp_destroy_WatcherPtr, kmp_alloc_WatcherPtr,
                       kmp_free_WatcherPtr, kl_init_WatcherPtr,
                       kl_destroy_WatcherPtr, kl_push_WatcherPtr,
                       kl_shift_at_WatcherPtr};
pub use self::stdbool_h::{true_0, false_0};
pub use self::byteswap_h::{__bswap_16, __bswap_32, __bswap_64};
pub use self::uintn_identity_h::{__uint16_identity, __uint32_identity,
                                 __uint64_identity};
use self::string_h::{memcpy, memmove, memset, strcmp, strlen};
pub use self::assert_h::{__ASSERT_FUNCTION, __assert_fail};
use self::log_h_generated_h::logmsg;
pub use self::log_h::WARN_LOG_LEVEL;
use self::mbyte_h_generated_h::{utf_char2bytes, mb_stricmp};
pub use self::mbyte_h::mb_strcmp_ic;
use self::libintl_h::gettext;
use self::message_h_generated_h::emsgf;
pub use self::strings_h::strappend;
use self::memory_h_generated_h::{xfree, xcalloc, xrealloc};
pub use self::kvec_h::_memcpy_free;
pub use self::ascii_h::{NUL, ascii_iswhite, ascii_isdigit, ascii_isxdigit,
                        ascii_isident, ascii_isbdigit, ascii_isspace};
pub use self::arabic_h::arabic_char;
// This is an open source non-commercial project. Dear PVS-Studio, please check
// it. PVS-Studio Static Code Analyzer for C, C++ and C#: http://www.viva64.com
// / @file arabic.c
// /
// / Functions for Arabic language.
// /
// / Arabic characters are categorized into following types:
// /
// / Isolated    - iso-8859-6 form         char denoted with  a_*
// / Initial     - unicode form-B start    char denoted with  a_i_*
// / Medial      - unicode form-B middle   char denoted with  a_m_*
// / Final       - unicode form-B final    char denoted with  a_f_*
// / Stand-Alone - unicode form-B isolated char denoted with  a_s_* (NOT USED)
// Arabic ISO-10646-1 character set definition
// Arabic ISO-8859-6 (subset of 10646; 0600 - 06FF)
#[c2rust::src_loc = "28:9"]
pub const a_HAMZA: libc::c_int = 0x621 as libc::c_int;
#[c2rust::src_loc = "29:9"]
pub const a_ALEF_MADDA: libc::c_int = 0x622 as libc::c_int;
#[c2rust::src_loc = "30:9"]
pub const a_ALEF_HAMZA_ABOVE: libc::c_int = 0x623 as libc::c_int;
#[c2rust::src_loc = "31:9"]
pub const a_WAW_HAMZA: libc::c_int = 0x624 as libc::c_int;
#[c2rust::src_loc = "32:9"]
pub const a_ALEF_HAMZA_BELOW: libc::c_int = 0x625 as libc::c_int;
#[c2rust::src_loc = "33:9"]
pub const a_YEH_HAMZA: libc::c_int = 0x626 as libc::c_int;
#[c2rust::src_loc = "34:9"]
pub const a_ALEF: libc::c_int = 0x627 as libc::c_int;
#[c2rust::src_loc = "35:9"]
pub const a_BEH: libc::c_int = 0x628 as libc::c_int;
#[c2rust::src_loc = "36:9"]
pub const a_TEH_MARBUTA: libc::c_int = 0x629 as libc::c_int;
#[c2rust::src_loc = "37:9"]
pub const a_TEH: libc::c_int = 0x62a as libc::c_int;
#[c2rust::src_loc = "38:9"]
pub const a_THEH: libc::c_int = 0x62b as libc::c_int;
#[c2rust::src_loc = "39:9"]
pub const a_JEEM: libc::c_int = 0x62c as libc::c_int;
#[c2rust::src_loc = "40:9"]
pub const a_HAH: libc::c_int = 0x62d as libc::c_int;
#[c2rust::src_loc = "41:9"]
pub const a_KHAH: libc::c_int = 0x62e as libc::c_int;
#[c2rust::src_loc = "42:9"]
pub const a_DAL: libc::c_int = 0x62f as libc::c_int;
#[c2rust::src_loc = "43:9"]
pub const a_THAL: libc::c_int = 0x630 as libc::c_int;
#[c2rust::src_loc = "44:9"]
pub const a_REH: libc::c_int = 0x631 as libc::c_int;
#[c2rust::src_loc = "45:9"]
pub const a_ZAIN: libc::c_int = 0x632 as libc::c_int;
#[c2rust::src_loc = "46:9"]
pub const a_SEEN: libc::c_int = 0x633 as libc::c_int;
#[c2rust::src_loc = "47:9"]
pub const a_SHEEN: libc::c_int = 0x634 as libc::c_int;
#[c2rust::src_loc = "48:9"]
pub const a_SAD: libc::c_int = 0x635 as libc::c_int;
#[c2rust::src_loc = "49:9"]
pub const a_DAD: libc::c_int = 0x636 as libc::c_int;
#[c2rust::src_loc = "50:9"]
pub const a_TAH: libc::c_int = 0x637 as libc::c_int;
#[c2rust::src_loc = "51:9"]
pub const a_ZAH: libc::c_int = 0x638 as libc::c_int;
#[c2rust::src_loc = "52:9"]
pub const a_AIN: libc::c_int = 0x639 as libc::c_int;
#[c2rust::src_loc = "53:9"]
pub const a_GHAIN: libc::c_int = 0x63a as libc::c_int;
#[c2rust::src_loc = "54:9"]
pub const a_TATWEEL: libc::c_int = 0x640 as libc::c_int;
#[c2rust::src_loc = "55:9"]
pub const a_FEH: libc::c_int = 0x641 as libc::c_int;
#[c2rust::src_loc = "56:9"]
pub const a_QAF: libc::c_int = 0x642 as libc::c_int;
#[c2rust::src_loc = "57:9"]
pub const a_KAF: libc::c_int = 0x643 as libc::c_int;
#[c2rust::src_loc = "58:9"]
pub const a_LAM: libc::c_int = 0x644 as libc::c_int;
#[c2rust::src_loc = "59:9"]
pub const a_MEEM: libc::c_int = 0x645 as libc::c_int;
#[c2rust::src_loc = "60:9"]
pub const a_NOON: libc::c_int = 0x646 as libc::c_int;
#[c2rust::src_loc = "61:9"]
pub const a_HEH: libc::c_int = 0x647 as libc::c_int;
#[c2rust::src_loc = "62:9"]
pub const a_WAW: libc::c_int = 0x648 as libc::c_int;
#[c2rust::src_loc = "63:9"]
pub const a_ALEF_MAKSURA: libc::c_int = 0x649 as libc::c_int;
#[c2rust::src_loc = "64:9"]
pub const a_YEH: libc::c_int = 0x64a as libc::c_int;
#[c2rust::src_loc = "66:9"]
pub const a_FATHATAN: libc::c_int = 0x64b as libc::c_int;
#[c2rust::src_loc = "73:9"]
pub const a_SUKUN: libc::c_int = 0x652 as libc::c_int;
#[c2rust::src_loc = "77:9"]
pub const a_HAMZA_BELOW: libc::c_int = 0x655 as libc::c_int;
#[c2rust::src_loc = "93:9"]
pub const a_MINI_ALEF: libc::c_int = 0x670 as libc::c_int;
// Rest of 8859-6 does not relate to Arabic
// Arabic Presentation Form-B (subset of 10646; FE70 - FEFF)
//
//  s -> isolated
//  i -> initial
//  m -> medial
//  f -> final
#[c2rust::src_loc = "102:9"]
pub const a_s_FATHATAN: libc::c_int = 0xfe70 as libc::c_int;
#[c2rust::src_loc = "104:9"]
pub const a_s_DAMMATAN: libc::c_int = 0xfe72 as libc::c_int;
#[c2rust::src_loc = "106:9"]
pub const a_s_KASRATAN: libc::c_int = 0xfe74 as libc::c_int;
#[c2rust::src_loc = "108:9"]
pub const a_s_FATHA: libc::c_int = 0xfe76 as libc::c_int;
#[c2rust::src_loc = "119:9"]
pub const a_s_HAMZA: libc::c_int = 0xfe80 as libc::c_int;
#[c2rust::src_loc = "120:9"]
pub const a_s_ALEF_MADDA: libc::c_int = 0xfe81 as libc::c_int;
#[c2rust::src_loc = "121:9"]
pub const a_f_ALEF_MADDA: libc::c_int = 0xfe82 as libc::c_int;
#[c2rust::src_loc = "122:9"]
pub const a_s_ALEF_HAMZA_ABOVE: libc::c_int = 0xfe83 as libc::c_int;
#[c2rust::src_loc = "123:9"]
pub const a_f_ALEF_HAMZA_ABOVE: libc::c_int = 0xfe84 as libc::c_int;
#[c2rust::src_loc = "124:9"]
pub const a_s_WAW_HAMZA: libc::c_int = 0xfe85 as libc::c_int;
#[c2rust::src_loc = "125:9"]
pub const a_f_WAW_HAMZA: libc::c_int = 0xfe86 as libc::c_int;
#[c2rust::src_loc = "126:9"]
pub const a_s_ALEF_HAMZA_BELOW: libc::c_int = 0xfe87 as libc::c_int;
#[c2rust::src_loc = "127:9"]
pub const a_f_ALEF_HAMZA_BELOW: libc::c_int = 0xfe88 as libc::c_int;
#[c2rust::src_loc = "128:9"]
pub const a_s_YEH_HAMZA: libc::c_int = 0xfe89 as libc::c_int;
#[c2rust::src_loc = "129:9"]
pub const a_f_YEH_HAMZA: libc::c_int = 0xfe8a as libc::c_int;
#[c2rust::src_loc = "130:9"]
pub const a_i_YEH_HAMZA: libc::c_int = 0xfe8b as libc::c_int;
#[c2rust::src_loc = "131:9"]
pub const a_m_YEH_HAMZA: libc::c_int = 0xfe8c as libc::c_int;
#[c2rust::src_loc = "132:9"]
pub const a_s_ALEF: libc::c_int = 0xfe8d as libc::c_int;
#[c2rust::src_loc = "133:9"]
pub const a_f_ALEF: libc::c_int = 0xfe8e as libc::c_int;
#[c2rust::src_loc = "134:9"]
pub const a_s_BEH: libc::c_int = 0xfe8f as libc::c_int;
#[c2rust::src_loc = "135:9"]
pub const a_f_BEH: libc::c_int = 0xfe90 as libc::c_int;
#[c2rust::src_loc = "136:9"]
pub const a_i_BEH: libc::c_int = 0xfe91 as libc::c_int;
#[c2rust::src_loc = "137:9"]
pub const a_m_BEH: libc::c_int = 0xfe92 as libc::c_int;
#[c2rust::src_loc = "138:9"]
pub const a_s_TEH_MARBUTA: libc::c_int = 0xfe93 as libc::c_int;
#[c2rust::src_loc = "139:9"]
pub const a_f_TEH_MARBUTA: libc::c_int = 0xfe94 as libc::c_int;
#[c2rust::src_loc = "140:9"]
pub const a_s_TEH: libc::c_int = 0xfe95 as libc::c_int;
#[c2rust::src_loc = "141:9"]
pub const a_f_TEH: libc::c_int = 0xfe96 as libc::c_int;
#[c2rust::src_loc = "142:9"]
pub const a_i_TEH: libc::c_int = 0xfe97 as libc::c_int;
#[c2rust::src_loc = "143:9"]
pub const a_m_TEH: libc::c_int = 0xfe98 as libc::c_int;
#[c2rust::src_loc = "144:9"]
pub const a_s_THEH: libc::c_int = 0xfe99 as libc::c_int;
#[c2rust::src_loc = "145:9"]
pub const a_f_THEH: libc::c_int = 0xfe9a as libc::c_int;
#[c2rust::src_loc = "146:9"]
pub const a_i_THEH: libc::c_int = 0xfe9b as libc::c_int;
#[c2rust::src_loc = "147:9"]
pub const a_m_THEH: libc::c_int = 0xfe9c as libc::c_int;
#[c2rust::src_loc = "148:9"]
pub const a_s_JEEM: libc::c_int = 0xfe9d as libc::c_int;
#[c2rust::src_loc = "149:9"]
pub const a_f_JEEM: libc::c_int = 0xfe9e as libc::c_int;
#[c2rust::src_loc = "150:9"]
pub const a_i_JEEM: libc::c_int = 0xfe9f as libc::c_int;
#[c2rust::src_loc = "151:9"]
pub const a_m_JEEM: libc::c_int = 0xfea0 as libc::c_int;
#[c2rust::src_loc = "152:9"]
pub const a_s_HAH: libc::c_int = 0xfea1 as libc::c_int;
#[c2rust::src_loc = "153:9"]
pub const a_f_HAH: libc::c_int = 0xfea2 as libc::c_int;
#[c2rust::src_loc = "154:9"]
pub const a_i_HAH: libc::c_int = 0xfea3 as libc::c_int;
#[c2rust::src_loc = "155:9"]
pub const a_m_HAH: libc::c_int = 0xfea4 as libc::c_int;
#[c2rust::src_loc = "156:9"]
pub const a_s_KHAH: libc::c_int = 0xfea5 as libc::c_int;
#[c2rust::src_loc = "157:9"]
pub const a_f_KHAH: libc::c_int = 0xfea6 as libc::c_int;
#[c2rust::src_loc = "158:9"]
pub const a_i_KHAH: libc::c_int = 0xfea7 as libc::c_int;
#[c2rust::src_loc = "159:9"]
pub const a_m_KHAH: libc::c_int = 0xfea8 as libc::c_int;
#[c2rust::src_loc = "160:9"]
pub const a_s_DAL: libc::c_int = 0xfea9 as libc::c_int;
#[c2rust::src_loc = "161:9"]
pub const a_f_DAL: libc::c_int = 0xfeaa as libc::c_int;
#[c2rust::src_loc = "162:9"]
pub const a_s_THAL: libc::c_int = 0xfeab as libc::c_int;
#[c2rust::src_loc = "163:9"]
pub const a_f_THAL: libc::c_int = 0xfeac as libc::c_int;
#[c2rust::src_loc = "164:9"]
pub const a_s_REH: libc::c_int = 0xfead as libc::c_int;
#[c2rust::src_loc = "165:9"]
pub const a_f_REH: libc::c_int = 0xfeae as libc::c_int;
#[c2rust::src_loc = "166:9"]
pub const a_s_ZAIN: libc::c_int = 0xfeaf as libc::c_int;
#[c2rust::src_loc = "167:9"]
pub const a_f_ZAIN: libc::c_int = 0xfeb0 as libc::c_int;
#[c2rust::src_loc = "168:9"]
pub const a_s_SEEN: libc::c_int = 0xfeb1 as libc::c_int;
#[c2rust::src_loc = "169:9"]
pub const a_f_SEEN: libc::c_int = 0xfeb2 as libc::c_int;
#[c2rust::src_loc = "170:9"]
pub const a_i_SEEN: libc::c_int = 0xfeb3 as libc::c_int;
#[c2rust::src_loc = "171:9"]
pub const a_m_SEEN: libc::c_int = 0xfeb4 as libc::c_int;
#[c2rust::src_loc = "172:9"]
pub const a_s_SHEEN: libc::c_int = 0xfeb5 as libc::c_int;
#[c2rust::src_loc = "173:9"]
pub const a_f_SHEEN: libc::c_int = 0xfeb6 as libc::c_int;
#[c2rust::src_loc = "174:9"]
pub const a_i_SHEEN: libc::c_int = 0xfeb7 as libc::c_int;
#[c2rust::src_loc = "175:9"]
pub const a_m_SHEEN: libc::c_int = 0xfeb8 as libc::c_int;
#[c2rust::src_loc = "176:9"]
pub const a_s_SAD: libc::c_int = 0xfeb9 as libc::c_int;
#[c2rust::src_loc = "177:9"]
pub const a_f_SAD: libc::c_int = 0xfeba as libc::c_int;
#[c2rust::src_loc = "178:9"]
pub const a_i_SAD: libc::c_int = 0xfebb as libc::c_int;
#[c2rust::src_loc = "179:9"]
pub const a_m_SAD: libc::c_int = 0xfebc as libc::c_int;
#[c2rust::src_loc = "180:9"]
pub const a_s_DAD: libc::c_int = 0xfebd as libc::c_int;
#[c2rust::src_loc = "181:9"]
pub const a_f_DAD: libc::c_int = 0xfebe as libc::c_int;
#[c2rust::src_loc = "182:9"]
pub const a_i_DAD: libc::c_int = 0xfebf as libc::c_int;
#[c2rust::src_loc = "183:9"]
pub const a_m_DAD: libc::c_int = 0xfec0 as libc::c_int;
#[c2rust::src_loc = "184:9"]
pub const a_s_TAH: libc::c_int = 0xfec1 as libc::c_int;
#[c2rust::src_loc = "185:9"]
pub const a_f_TAH: libc::c_int = 0xfec2 as libc::c_int;
#[c2rust::src_loc = "186:9"]
pub const a_i_TAH: libc::c_int = 0xfec3 as libc::c_int;
#[c2rust::src_loc = "187:9"]
pub const a_m_TAH: libc::c_int = 0xfec4 as libc::c_int;
#[c2rust::src_loc = "188:9"]
pub const a_s_ZAH: libc::c_int = 0xfec5 as libc::c_int;
#[c2rust::src_loc = "189:9"]
pub const a_f_ZAH: libc::c_int = 0xfec6 as libc::c_int;
#[c2rust::src_loc = "190:9"]
pub const a_i_ZAH: libc::c_int = 0xfec7 as libc::c_int;
#[c2rust::src_loc = "191:9"]
pub const a_m_ZAH: libc::c_int = 0xfec8 as libc::c_int;
#[c2rust::src_loc = "192:9"]
pub const a_s_AIN: libc::c_int = 0xfec9 as libc::c_int;
#[c2rust::src_loc = "193:9"]
pub const a_f_AIN: libc::c_int = 0xfeca as libc::c_int;
#[c2rust::src_loc = "194:9"]
pub const a_i_AIN: libc::c_int = 0xfecb as libc::c_int;
#[c2rust::src_loc = "195:9"]
pub const a_m_AIN: libc::c_int = 0xfecc as libc::c_int;
#[c2rust::src_loc = "196:9"]
pub const a_s_GHAIN: libc::c_int = 0xfecd as libc::c_int;
#[c2rust::src_loc = "197:9"]
pub const a_f_GHAIN: libc::c_int = 0xfece as libc::c_int;
#[c2rust::src_loc = "198:9"]
pub const a_i_GHAIN: libc::c_int = 0xfecf as libc::c_int;
#[c2rust::src_loc = "199:9"]
pub const a_m_GHAIN: libc::c_int = 0xfed0 as libc::c_int;
#[c2rust::src_loc = "200:9"]
pub const a_s_FEH: libc::c_int = 0xfed1 as libc::c_int;
#[c2rust::src_loc = "201:9"]
pub const a_f_FEH: libc::c_int = 0xfed2 as libc::c_int;
#[c2rust::src_loc = "202:9"]
pub const a_i_FEH: libc::c_int = 0xfed3 as libc::c_int;
#[c2rust::src_loc = "203:9"]
pub const a_m_FEH: libc::c_int = 0xfed4 as libc::c_int;
#[c2rust::src_loc = "204:9"]
pub const a_s_QAF: libc::c_int = 0xfed5 as libc::c_int;
#[c2rust::src_loc = "205:9"]
pub const a_f_QAF: libc::c_int = 0xfed6 as libc::c_int;
#[c2rust::src_loc = "206:9"]
pub const a_i_QAF: libc::c_int = 0xfed7 as libc::c_int;
#[c2rust::src_loc = "207:9"]
pub const a_m_QAF: libc::c_int = 0xfed8 as libc::c_int;
#[c2rust::src_loc = "208:9"]
pub const a_s_KAF: libc::c_int = 0xfed9 as libc::c_int;
#[c2rust::src_loc = "209:9"]
pub const a_f_KAF: libc::c_int = 0xfeda as libc::c_int;
#[c2rust::src_loc = "210:9"]
pub const a_i_KAF: libc::c_int = 0xfedb as libc::c_int;
#[c2rust::src_loc = "211:9"]
pub const a_m_KAF: libc::c_int = 0xfedc as libc::c_int;
#[c2rust::src_loc = "212:9"]
pub const a_s_LAM: libc::c_int = 0xfedd as libc::c_int;
#[c2rust::src_loc = "213:9"]
pub const a_f_LAM: libc::c_int = 0xfede as libc::c_int;
#[c2rust::src_loc = "214:9"]
pub const a_i_LAM: libc::c_int = 0xfedf as libc::c_int;
#[c2rust::src_loc = "215:9"]
pub const a_m_LAM: libc::c_int = 0xfee0 as libc::c_int;
#[c2rust::src_loc = "216:9"]
pub const a_s_MEEM: libc::c_int = 0xfee1 as libc::c_int;
#[c2rust::src_loc = "217:9"]
pub const a_f_MEEM: libc::c_int = 0xfee2 as libc::c_int;
#[c2rust::src_loc = "218:9"]
pub const a_i_MEEM: libc::c_int = 0xfee3 as libc::c_int;
#[c2rust::src_loc = "219:9"]
pub const a_m_MEEM: libc::c_int = 0xfee4 as libc::c_int;
#[c2rust::src_loc = "220:9"]
pub const a_s_NOON: libc::c_int = 0xfee5 as libc::c_int;
#[c2rust::src_loc = "221:9"]
pub const a_f_NOON: libc::c_int = 0xfee6 as libc::c_int;
#[c2rust::src_loc = "222:9"]
pub const a_i_NOON: libc::c_int = 0xfee7 as libc::c_int;
#[c2rust::src_loc = "223:9"]
pub const a_m_NOON: libc::c_int = 0xfee8 as libc::c_int;
#[c2rust::src_loc = "224:9"]
pub const a_s_HEH: libc::c_int = 0xfee9 as libc::c_int;
#[c2rust::src_loc = "225:9"]
pub const a_f_HEH: libc::c_int = 0xfeea as libc::c_int;
#[c2rust::src_loc = "226:9"]
pub const a_i_HEH: libc::c_int = 0xfeeb as libc::c_int;
#[c2rust::src_loc = "227:9"]
pub const a_m_HEH: libc::c_int = 0xfeec as libc::c_int;
#[c2rust::src_loc = "228:9"]
pub const a_s_WAW: libc::c_int = 0xfeed as libc::c_int;
#[c2rust::src_loc = "229:9"]
pub const a_f_WAW: libc::c_int = 0xfeee as libc::c_int;
#[c2rust::src_loc = "230:9"]
pub const a_s_ALEF_MAKSURA: libc::c_int = 0xfeef as libc::c_int;
#[c2rust::src_loc = "231:9"]
pub const a_f_ALEF_MAKSURA: libc::c_int = 0xfef0 as libc::c_int;
#[c2rust::src_loc = "232:9"]
pub const a_s_YEH: libc::c_int = 0xfef1 as libc::c_int;
#[c2rust::src_loc = "233:9"]
pub const a_f_YEH: libc::c_int = 0xfef2 as libc::c_int;
#[c2rust::src_loc = "234:9"]
pub const a_i_YEH: libc::c_int = 0xfef3 as libc::c_int;
#[c2rust::src_loc = "235:9"]
pub const a_m_YEH: libc::c_int = 0xfef4 as libc::c_int;
#[c2rust::src_loc = "236:9"]
pub const a_s_LAM_ALEF_MADDA_ABOVE: libc::c_int = 0xfef5 as libc::c_int;
#[c2rust::src_loc = "237:9"]
pub const a_f_LAM_ALEF_MADDA_ABOVE: libc::c_int = 0xfef6 as libc::c_int;
#[c2rust::src_loc = "238:9"]
pub const a_s_LAM_ALEF_HAMZA_ABOVE: libc::c_int = 0xfef7 as libc::c_int;
#[c2rust::src_loc = "239:9"]
pub const a_f_LAM_ALEF_HAMZA_ABOVE: libc::c_int = 0xfef8 as libc::c_int;
#[c2rust::src_loc = "240:9"]
pub const a_s_LAM_ALEF_HAMZA_BELOW: libc::c_int = 0xfef9 as libc::c_int;
#[c2rust::src_loc = "241:9"]
pub const a_f_LAM_ALEF_HAMZA_BELOW: libc::c_int = 0xfefa as libc::c_int;
#[c2rust::src_loc = "242:9"]
pub const a_s_LAM_ALEF: libc::c_int = 0xfefb as libc::c_int;
#[c2rust::src_loc = "243:9"]
pub const a_f_LAM_ALEF: libc::c_int = 0xfefc as libc::c_int;
#[c2rust::src_loc = "245:9"]
pub const a_BYTE_ORDER_MARK: libc::c_int = 0xfeff as libc::c_int;
// Returns true if c is an ISO-8859-6 shaped ARABIC letter (user entered).
#[c2rust::src_loc = "253:1"]
unsafe extern "C" fn A_is_a(mut cur_c: libc::c_int) -> bool {
    match cur_c {
        a_HAMZA | a_ALEF_MADDA | a_ALEF_HAMZA_ABOVE | a_WAW_HAMZA |
        a_ALEF_HAMZA_BELOW | a_YEH_HAMZA | a_ALEF | a_BEH | a_TEH_MARBUTA |
        a_TEH | a_THEH | a_JEEM | a_HAH | a_KHAH | a_DAL | a_THAL | a_REH |
        a_ZAIN | a_SEEN | a_SHEEN | a_SAD | a_DAD | a_TAH | a_ZAH | a_AIN |
        a_GHAIN | a_TATWEEL | a_FEH | a_QAF | a_KAF | a_LAM | a_MEEM | a_NOON
        | a_HEH | a_WAW | a_ALEF_MAKSURA | a_YEH => {
            return true_0 != 0
        }
        _ => { }
    }
    return false_0 != 0;
}
// Returns true if c is an Isolated Form-B ARABIC letter
#[c2rust::src_loc = "300:1"]
unsafe extern "C" fn A_is_s(mut cur_c: libc::c_int) -> bool {
    match cur_c {
        a_s_HAMZA | a_s_ALEF_MADDA | a_s_ALEF_HAMZA_ABOVE | a_s_WAW_HAMZA |
        a_s_ALEF_HAMZA_BELOW | a_s_YEH_HAMZA | a_s_ALEF | a_s_BEH |
        a_s_TEH_MARBUTA | a_s_TEH | a_s_THEH | a_s_JEEM | a_s_HAH | a_s_KHAH |
        a_s_DAL | a_s_THAL | a_s_REH | a_s_ZAIN | a_s_SEEN | a_s_SHEEN |
        a_s_SAD | a_s_DAD | a_s_TAH | a_s_ZAH | a_s_AIN | a_s_GHAIN | a_s_FEH
        | a_s_QAF | a_s_KAF | a_s_LAM | a_s_MEEM | a_s_NOON | a_s_HEH |
        a_s_WAW | a_s_ALEF_MAKSURA | a_s_YEH => {
            return true_0 != 0
        }
        _ => { }
    }
    return false_0 != 0;
}
// Returns true if c is a Final shape of an ARABIC letter
#[c2rust::src_loc = "346:1"]
unsafe extern "C" fn A_is_f(mut cur_c: libc::c_int) -> bool {
    match cur_c {
        a_f_ALEF_MADDA | a_f_ALEF_HAMZA_ABOVE | a_f_WAW_HAMZA |
        a_f_ALEF_HAMZA_BELOW | a_f_YEH_HAMZA | a_f_ALEF | a_f_BEH |
        a_f_TEH_MARBUTA | a_f_TEH | a_f_THEH | a_f_JEEM | a_f_HAH | a_f_KHAH |
        a_f_DAL | a_f_THAL | a_f_REH | a_f_ZAIN | a_f_SEEN | a_f_SHEEN |
        a_f_SAD | a_f_DAD | a_f_TAH | a_f_ZAH | a_f_AIN | a_f_GHAIN | a_f_FEH
        | a_f_QAF | a_f_KAF | a_f_LAM | a_f_MEEM | a_f_NOON | a_f_HEH |
        a_f_WAW | a_f_ALEF_MAKSURA | a_f_YEH | a_f_LAM_ALEF_MADDA_ABOVE |
        a_f_LAM_ALEF_HAMZA_ABOVE | a_f_LAM_ALEF_HAMZA_BELOW | a_f_LAM_ALEF =>
        {
            return true_0 != 0
        }
        _ => { }
    }
    return false_0 != 0;
}
// Change shape - from ISO-8859-6/Isolated to Form-B Isolated
#[c2rust::src_loc = "394:1"]
unsafe extern "C" fn chg_c_a2s(mut cur_c: libc::c_int) -> libc::c_int {
    match cur_c {
        a_HAMZA => { return a_s_HAMZA }
        a_ALEF_MADDA => { return a_s_ALEF_MADDA }
        a_ALEF_HAMZA_ABOVE => { return a_s_ALEF_HAMZA_ABOVE }
        a_WAW_HAMZA => { return a_s_WAW_HAMZA }
        a_ALEF_HAMZA_BELOW => { return a_s_ALEF_HAMZA_BELOW }
        a_YEH_HAMZA => { return a_s_YEH_HAMZA }
        a_ALEF => { return a_s_ALEF }
        a_TEH_MARBUTA => { return a_s_TEH_MARBUTA }
        a_DAL => { return a_s_DAL }
        a_THAL => { return a_s_THAL }
        a_REH => { return a_s_REH }
        a_ZAIN => { return a_s_ZAIN }
        a_TATWEEL => { return cur_c }
        a_WAW => { return a_s_WAW }
        a_ALEF_MAKSURA => { return a_s_ALEF_MAKSURA }
        a_BEH => { return a_s_BEH }
        a_TEH => { return a_s_TEH }
        a_THEH => { return a_s_THEH }
        a_JEEM => { return a_s_JEEM }
        a_HAH => { return a_s_HAH }
        a_KHAH => { return a_s_KHAH }
        a_SEEN => { return a_s_SEEN }
        a_SHEEN => { return a_s_SHEEN }
        a_SAD => { return a_s_SAD }
        a_DAD => { return a_s_DAD }
        a_TAH => { return a_s_TAH }
        a_ZAH => { return a_s_ZAH }
        a_AIN => { return a_s_AIN }
        a_GHAIN => { return a_s_GHAIN }
        a_FEH => { return a_s_FEH }
        a_QAF => { return a_s_QAF }
        a_KAF => { return a_s_KAF }
        a_LAM => { return a_s_LAM }
        a_MEEM => { return a_s_MEEM }
        a_NOON => { return a_s_NOON }
        a_HEH => { return a_s_HEH }
        a_YEH => { return a_s_YEH }
        _ => { }
    } // exceptions
    return 0 as libc::c_int;
}
// Change shape - from ISO-8859-6/Isolated to Initial
#[c2rust::src_loc = "439:1"]
unsafe extern "C" fn chg_c_a2i(mut cur_c: libc::c_int) -> libc::c_int {
    match cur_c {
        a_YEH_HAMZA => { return a_i_YEH_HAMZA }
        a_HAMZA => { return a_s_HAMZA }
        a_ALEF_MADDA => { // exceptions
            return a_s_ALEF_MADDA
        }
        a_ALEF_HAMZA_ABOVE => { // exceptions
            return a_s_ALEF_HAMZA_ABOVE
        }
        a_WAW_HAMZA => { // exceptions
            return a_s_WAW_HAMZA
        }
        a_ALEF_HAMZA_BELOW => { // exceptions
            return a_s_ALEF_HAMZA_BELOW
        }
        a_ALEF => { // exceptions
            return a_s_ALEF
        }
        a_TEH_MARBUTA => { // exceptions
            return a_s_TEH_MARBUTA
        }
        a_DAL => { // exceptions
            return a_s_DAL
        }
        a_THAL => { // exceptions
            return a_s_THAL
        }
        a_REH => { // exceptions
            return a_s_REH
        }
        a_ZAIN => { // exceptions
            return a_s_ZAIN
        }
        a_TATWEEL => { // exceptions
            return cur_c
        }
        a_WAW => { // exceptions
            return a_s_WAW
        }
        a_ALEF_MAKSURA => { // exceptions
            return a_s_ALEF_MAKSURA
        }
        a_BEH => { return a_i_BEH }
        a_TEH => { return a_i_TEH }
        a_THEH => { return a_i_THEH }
        a_JEEM => { return a_i_JEEM }
        a_HAH => { return a_i_HAH }
        a_KHAH => { return a_i_KHAH }
        a_SEEN => { return a_i_SEEN }
        a_SHEEN => { return a_i_SHEEN }
        a_SAD => { return a_i_SAD }
        a_DAD => { return a_i_DAD }
        a_TAH => { return a_i_TAH }
        a_ZAH => { return a_i_ZAH }
        a_AIN => { return a_i_AIN }
        a_GHAIN => { return a_i_GHAIN }
        a_FEH => { return a_i_FEH }
        a_QAF => { return a_i_QAF }
        a_KAF => { return a_i_KAF }
        a_LAM => { return a_i_LAM }
        a_MEEM => { return a_i_MEEM }
        a_NOON => { return a_i_NOON }
        a_HEH => { return a_i_HEH }
        a_YEH => { return a_i_YEH }
        _ => { }
    } // exceptions
    return 0 as libc::c_int;
}
// Change shape - from ISO-8859-6/Isolated to Medial
#[c2rust::src_loc = "484:1"]
unsafe extern "C" fn chg_c_a2m(mut cur_c: libc::c_int) -> libc::c_int {
    match cur_c {
        a_HAMZA => { return a_s_HAMZA }
        a_ALEF_MADDA => { // exception
            return a_f_ALEF_MADDA
        }
        a_ALEF_HAMZA_ABOVE => { // exception
            return a_f_ALEF_HAMZA_ABOVE
        }
        a_WAW_HAMZA => { // exception
            return a_f_WAW_HAMZA
        }
        a_ALEF_HAMZA_BELOW => { // exception
            return a_f_ALEF_HAMZA_BELOW
        }
        a_YEH_HAMZA => { return a_m_YEH_HAMZA }
        a_ALEF => { // exception
            return a_f_ALEF
        }
        a_BEH => { return a_m_BEH }
        a_TEH_MARBUTA => { // exception
            return a_f_TEH_MARBUTA
        }
        a_TEH => { return a_m_TEH }
        a_THEH => { return a_m_THEH }
        a_JEEM => { return a_m_JEEM }
        a_HAH => { return a_m_HAH }
        a_KHAH => { return a_m_KHAH }
        a_DAL => { // exception
            return a_f_DAL
        }
        a_THAL => { // exception
            return a_f_THAL
        }
        a_REH => { // exception
            return a_f_REH
        }
        a_ZAIN => { // exception
            return a_f_ZAIN
        }
        a_SEEN => { return a_m_SEEN }
        a_SHEEN => { return a_m_SHEEN }
        a_SAD => { return a_m_SAD }
        a_DAD => { return a_m_DAD }
        a_TAH => { return a_m_TAH }
        a_ZAH => { return a_m_ZAH }
        a_AIN => { return a_m_AIN }
        a_GHAIN => { return a_m_GHAIN }
        a_TATWEEL => { // exception
            return cur_c
        }
        a_FEH => { return a_m_FEH }
        a_QAF => { return a_m_QAF }
        a_KAF => { return a_m_KAF }
        a_LAM => { return a_m_LAM }
        a_MEEM => { return a_m_MEEM }
        a_NOON => { return a_m_NOON }
        a_HEH => { return a_m_HEH }
        a_WAW => { // exception
            return a_f_WAW
        }
        a_ALEF_MAKSURA => { // exception
            return a_f_ALEF_MAKSURA
        }
        a_YEH => { return a_m_YEH }
        _ => { }
    } // exception
    return 0 as libc::c_int;
}
// Change shape - from ISO-8859-6/Isolated to final
#[c2rust::src_loc = "529:1"]
unsafe extern "C" fn chg_c_a2f(mut cur_c: libc::c_int) -> libc::c_int {
    // NOTE: these encodings need to be accounted for
  //
  // a_f_ALEF_MADDA;
  // a_f_ALEF_HAMZA_ABOVE;
  // a_f_ALEF_HAMZA_BELOW;
  // a_f_LAM_ALEF_MADDA_ABOVE;
  // a_f_LAM_ALEF_HAMZA_ABOVE;
  // a_f_LAM_ALEF_HAMZA_BELOW;
    match cur_c {
        a_HAMZA => { return a_s_HAMZA }
        a_ALEF_MADDA => { return a_f_ALEF_MADDA }
        a_ALEF_HAMZA_ABOVE => { return a_f_ALEF_HAMZA_ABOVE }
        a_WAW_HAMZA => { return a_f_WAW_HAMZA }
        a_ALEF_HAMZA_BELOW => { return a_f_ALEF_HAMZA_BELOW }
        a_YEH_HAMZA => { return a_f_YEH_HAMZA }
        a_ALEF => { return a_f_ALEF }
        a_BEH => { return a_f_BEH }
        a_TEH_MARBUTA => { return a_f_TEH_MARBUTA }
        a_TEH => { return a_f_TEH }
        a_THEH => { return a_f_THEH }
        a_JEEM => { return a_f_JEEM }
        a_HAH => { return a_f_HAH }
        a_KHAH => { return a_f_KHAH }
        a_DAL => { return a_f_DAL }
        a_THAL => { return a_f_THAL }
        a_REH => { return a_f_REH }
        a_ZAIN => { return a_f_ZAIN }
        a_SEEN => { return a_f_SEEN }
        a_SHEEN => { return a_f_SHEEN }
        a_SAD => { return a_f_SAD }
        a_DAD => { return a_f_DAD }
        a_TAH => { return a_f_TAH }
        a_ZAH => { return a_f_ZAH }
        a_AIN => { return a_f_AIN }
        a_GHAIN => { return a_f_GHAIN }
        a_TATWEEL => { // exception
            return cur_c
        }
        a_FEH => { return a_f_FEH }
        a_QAF => { return a_f_QAF }
        a_KAF => { return a_f_KAF }
        a_LAM => { return a_f_LAM }
        a_MEEM => { return a_f_MEEM }
        a_NOON => { return a_f_NOON }
        a_HEH => { return a_f_HEH }
        a_WAW => { return a_f_WAW }
        a_ALEF_MAKSURA => { return a_f_ALEF_MAKSURA }
        a_YEH => { return a_f_YEH }
        _ => { }
    } // exception
    return 0 as libc::c_int;
}
// Change shape - from Initial to Medial
// This code is unreachable, because for the relevant characters ARABIC_CHAR()
// is FALSE;
// Change shape - from Final to Medial
#[c2rust::src_loc = "618:1"]
unsafe extern "C" fn chg_c_f2m(mut cur_c: libc::c_int) -> libc::c_int {
    match cur_c {
        a_f_YEH_HAMZA => {
            return a_m_YEH_HAMZA
            // NOTE: these encodings are multi-positional, no ?
    //   case a_f_LAM_ALEF_MADDA_ABOVE:
    //   case a_f_LAM_ALEF_HAMZA_ABOVE:
    //   case a_f_LAM_ALEF_HAMZA_BELOW:
    //   case a_f_LAM_ALEF:
        }
        a_f_WAW_HAMZA | a_f_ALEF | a_f_TEH_MARBUTA | a_f_DAL | a_f_THAL |
        a_f_REH | a_f_ZAIN | a_f_WAW | a_f_ALEF_MAKSURA => {
            // exceptions
            return cur_c
        }
        a_f_BEH => { return a_m_BEH }
        a_f_TEH => { return a_m_TEH }
        a_f_THEH => { return a_m_THEH }
        a_f_JEEM => { return a_m_JEEM }
        a_f_HAH => { return a_m_HAH }
        a_f_KHAH => { return a_m_KHAH }
        a_f_SEEN => { return a_m_SEEN }
        a_f_SHEEN => { return a_m_SHEEN }
        a_f_SAD => { return a_m_SAD }
        a_f_DAD => { return a_m_DAD }
        a_f_TAH => { return a_m_TAH }
        a_f_ZAH => { return a_m_ZAH }
        a_f_AIN => { return a_m_AIN }
        a_f_GHAIN => { return a_m_GHAIN }
        a_f_FEH => { return a_m_FEH }
        a_f_QAF => { return a_m_QAF }
        a_f_KAF => { return a_m_KAF }
        a_f_LAM => { return a_m_LAM }
        a_f_MEEM => { return a_m_MEEM }
        a_f_NOON => { return a_m_NOON }
        a_f_HEH => { return a_m_HEH }
        a_f_YEH => { return a_m_YEH }
        _ => { }
    }
    return 0 as libc::c_int;
}
// Change shape - from Combination (2 char) to an Isolated.
#[c2rust::src_loc = "668:1"]
unsafe extern "C" fn chg_c_laa2i(mut hid_c: libc::c_int) -> libc::c_int {
    match hid_c {
        a_ALEF_MADDA => { return a_s_LAM_ALEF_MADDA_ABOVE }
        a_ALEF_HAMZA_ABOVE => { return a_s_LAM_ALEF_HAMZA_ABOVE }
        a_ALEF_HAMZA_BELOW => { return a_s_LAM_ALEF_HAMZA_BELOW }
        a_ALEF => { return a_s_LAM_ALEF }
        _ => { }
    }
    return 0 as libc::c_int;
}
// Change shape - from Combination-Isolated to Final.
#[c2rust::src_loc = "680:1"]
unsafe extern "C" fn chg_c_laa2f(mut hid_c: libc::c_int) -> libc::c_int {
    match hid_c {
        a_ALEF_MADDA => { return a_f_LAM_ALEF_MADDA_ABOVE }
        a_ALEF_HAMZA_ABOVE => { return a_f_LAM_ALEF_HAMZA_ABOVE }
        a_ALEF_HAMZA_BELOW => { return a_f_LAM_ALEF_HAMZA_BELOW }
        a_ALEF => { return a_f_LAM_ALEF }
        _ => { }
    }
    return 0 as libc::c_int;
}
// Do "half-shaping" on character "c". Return zero if no shaping.
#[c2rust::src_loc = "692:1"]
unsafe extern "C" fn half_shape(mut c: libc::c_int) -> libc::c_int {
    if A_is_a(c) { return chg_c_a2i(c) }
    if A_is_valid(c) as libc::c_int != 0 && A_is_f(c) as libc::c_int != 0 {
        return chg_c_f2m(c)
    }
    return 0 as libc::c_int;
}
// Do Arabic shaping on character "c".  Returns the shaped character.
// out:    "ccp" points to the first byte of the character to be shaped.
// in/out: "c1p" points to the first composing char for "c".
// in:     "prev_c"  is the previous character (not shaped)
// in:     "prev_c1" is the first composing char for the previous char
//          (not shaped)
// in:     "next_c"  is the next character (not shaped).
#[no_mangle]
#[c2rust::src_loc = "711:1"]
pub unsafe extern "C" fn arabic_shape(mut c: libc::c_int,
                                      mut ccp: *mut libc::c_int,
                                      mut c1p: *mut libc::c_int,
                                      mut prev_c: libc::c_int,
                                      mut prev_c1: libc::c_int,
                                      mut next_c: libc::c_int)
 -> libc::c_int {
    // Deal only with Arabic character, pass back all others
    if !A_is_ok(c) { return c }
    // half-shape current and previous character
    let mut shape_c = half_shape(prev_c);
    // Save away current character
    let mut curr_c = c;
    let mut curr_laa = A_firstc_laa(c, *c1p);
    let mut prev_laa = A_firstc_laa(prev_c, prev_c1);
    if curr_laa != 0 {
        if A_is_valid(prev_c) as libc::c_int != 0 && !A_is_f(shape_c) &&
               !A_is_s(shape_c) && prev_laa == 0 {
            curr_c = chg_c_laa2f(curr_laa)
        } else { curr_c = chg_c_laa2i(curr_laa) }
        // Remove the composing character
        *c1p = 0 as libc::c_int
    } else if !A_is_valid(prev_c) && A_is_valid(next_c) as libc::c_int != 0 {
        curr_c = chg_c_a2i(c)
    } else if shape_c == 0 || A_is_f(shape_c) as libc::c_int != 0 ||
                  A_is_s(shape_c) as libc::c_int != 0 || prev_laa != 0 {
        curr_c =
            if A_is_valid(next_c) as libc::c_int != 0 {
                chg_c_a2i(c)
            } else { chg_c_a2s(c) }
    } else if A_is_valid(next_c) {
        curr_c =
            if A_is_iso(c) as libc::c_int != 0 {
                chg_c_a2m(c)
            } else { 0 as libc::c_int }
    } else if A_is_valid(prev_c) {
        curr_c = chg_c_a2f(c)
    } else { curr_c = chg_c_a2s(c) }
    // Sanity check -- curr_c should, in the future, never be 0.
  // We should, in the future, insert a fatal error here.
    if curr_c == NUL { curr_c = c }
    if curr_c != c && !ccp.is_null() {
        let mut buf: [char_u; 22] = [0; 22];
        // Update the first byte of the character
        utf_char2bytes(curr_c, buf.as_mut_ptr());
        *ccp = buf[0 as libc::c_int as usize] as libc::c_int
    }
    // Return the shaped character
    return curr_c;
}
// / Check whether we are dealing with Arabic combining characters.
// / Note: these are NOT really composing characters!
// /
// / @param one First character.
// / @param two Character just after "one".
#[no_mangle]
#[c2rust::src_loc = "777:1"]
pub unsafe extern "C" fn arabic_combine(mut one: libc::c_int,
                                        mut two: libc::c_int) -> bool {
    if one == a_LAM { return arabic_maycombine(two) }
    return false_0 != 0;
}
// / Check whether we are dealing with a character that could be regarded as an
// / Arabic combining character, need to check the character before this.
#[no_mangle]
#[c2rust::src_loc = "787:1"]
pub unsafe extern "C" fn arabic_maycombine(mut two: libc::c_int) -> bool {
    if p_arshape != 0 && p_tbidi == 0 {
        return two == a_ALEF_MADDA || two == a_ALEF_HAMZA_ABOVE ||
                   two == a_ALEF_HAMZA_BELOW || two == a_ALEF
    }
    return false_0 != 0;
}
// A_firstc_laa returns first character of LAA combination if it ex.ists
// in: "c" base character
// in: "c1" first composing character
#[c2rust::src_loc = "801:1"]
unsafe extern "C" fn A_firstc_laa(mut c: libc::c_int, mut c1: libc::c_int)
 -> libc::c_int {
    if c1 != NUL && c == a_LAM && !A_is_harakat(c1) { return c1 }
    return 0 as libc::c_int;
}
// A_is_harakat returns true if 'c' is an Arabic Harakat character.
//     (harakat/tanween)
#[c2rust::src_loc = "811:1"]
unsafe extern "C" fn A_is_harakat(mut c: libc::c_int) -> bool {
    return c >= a_FATHATAN && c <= a_SUKUN;
}
// A_is_iso returns true if 'c' is an Arabic ISO-8859-6 character.
//     (alphabet/number/punctuation)
#[c2rust::src_loc = "818:1"]
unsafe extern "C" fn A_is_iso(mut c: libc::c_int) -> bool {
    return c >= a_HAMZA && c <= a_GHAIN ||
               c >= a_TATWEEL && c <= a_HAMZA_BELOW || c == a_MINI_ALEF;
}
// A_is_formb returns true if 'c' is an Arabic 10646-1 FormB character.
//     (alphabet/number/punctuation)
#[c2rust::src_loc = "827:1"]
unsafe extern "C" fn A_is_formb(mut c: libc::c_int) -> bool {
    return c >= a_s_FATHATAN && c <= a_s_DAMMATAN || c == a_s_KASRATAN ||
               c >= a_s_FATHA && c <= a_f_LAM_ALEF || c == a_BYTE_ORDER_MARK;
}
// A_is_ok returns true if 'c' is an Arabic 10646 (8859-6 or Form-B).
#[c2rust::src_loc = "836:1"]
unsafe extern "C" fn A_is_ok(mut c: libc::c_int) -> bool {
    return A_is_iso(c) as libc::c_int != 0 ||
               A_is_formb(c) as libc::c_int != 0;
}
// A_is_valid returns true if 'c' is an Arabic 10646 (8859-6 or Form-B),
// with some exceptions/exclusions.
#[c2rust::src_loc = "843:1"]
unsafe extern "C" fn A_is_valid(mut c: libc::c_int) -> bool {
    return A_is_ok(c) as libc::c_int != 0 && !A_is_special(c);
}
// A_is_special returns true if 'c' is not a special Arabic character.
// Specials don't adhere to most of the rules.
#[c2rust::src_loc = "850:1"]
unsafe extern "C" fn A_is_special(mut c: libc::c_int) -> bool {
    return c == a_HAMZA || c == a_s_HAMZA;
}
