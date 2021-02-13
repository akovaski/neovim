use crate::*;

pub mod internal {
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: u32,
        pub fp_offset: u32,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
pub mod types_h {
    pub type __int8_t = libc::c_schar;
    pub type __uint8_t = libc::c_uchar;
    pub type __int16_t = i16;
    pub type __uint16_t = u16;
    pub type __int32_t = i32;
    pub type __uint32_t = u32;
    pub type __int64_t = i64;
    pub type __uint64_t = u64;
    pub type __uid_t = u32;
    pub type __gid_t = u32;
    pub type __time_t = i64;
    pub type __ssize_t = i64;
}
pub mod stdint_intn_h {
    pub type i8 = __int8_t;
    pub type int16_t = __int16_t;
    pub type i32 = __int32_t;
    pub type i64 = __int64_t;
}
pub mod stdint_uintn_h {
    pub type u8 = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type u32 = __uint32_t;
    pub type u64 = __uint64_t;
}
pub mod stddef_h {
    pub type size_t = u64;
    pub const NULL: i32 = 0;
}
pub mod stdarg_h {
    pub type va_list = __builtin_va_list;
}
pub mod nvim_types_h {
    // dummy to pass an ACL to a function
    // Shorthand for unsigned variables. Many systems, but not all, have u_char
    // already defined, so we use u8 to avoid trouble.
    pub type u8 = libc::c_uchar;
    // Opaque handle used by API clients to refer to various objects in vim
    pub type handle_T = i32;
    // Opaque handle to a lua value. Must be free with `executor_free_luaref` when
    // not needed anymore! LUA_NOREF represents missing reference, i e to indicate
    // absent callback etc.
    pub type LuaRef = i32;
    // NVIM_TYPES_H
}
pub mod typval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct sctx_T {
        pub sc_sid: scid_T,
        pub sc_seq: i32,
        pub sc_lnum: linenr_T,
    }
    pub type scid_T = i32;
    // for linenr_T
    // / Type used for VimL VAR_NUMBER values
    // / Type used for VimL VAR_FLOAT values
    // / Refcount for dict or list that should not be freed
    // / Additional values for tv_list_alloc() len argument
    // / List length is not known in advance
    // /
    // / To be used when there is neither a way to know how many elements will be
    // / needed nor are any educated guesses.
    // / List length *should* be known, but is actually not
    // /
    // / All occurrences of this value should be eventually removed. This is for
    // / the case when the only reason why list length is not known is that it
    // / would be hard to code without refactoring, but refactoring is needed.
    // / List length may be known in advance, but it requires too much effort
    // /
    // / To be used when it looks impractical to determine list length.
    // / Maximal possible value of varnumber_T variable
    // / Mimimal possible value of varnumber_T variable
    // / %d printf format specifier for varnumber_T
    pub type dict_T = dictvar_S;
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
    // /< First item, NULL if none.
    // /< Last item, NULL if none.
    // /< First watcher, NULL if none.
    // /< When not NULL item at index "lv_idx".
    // /< Copied list used by deepcopy().
    // /< next list in used lists list.
    // /< Previous list in used lists list.
    // /< Reference count.
    // /< Number of items.
    // /< Index of a cached item, used for optimising repeated l[idx].
    // /< ID used by deepcopy().
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct dictvar_S {
        pub dv_lock: VarLockStatus,
        pub dv_scope: ScopeType,
        pub dv_refcount: i32,
        pub dv_copyID: i32,
        pub dv_hashtab: hashtab_T,
        pub dv_copydict: *mut dict_T,
        pub dv_used_next: *mut dict_T,
        pub dv_used_prev: *mut dict_T,
        pub watchers: QUEUE,
    }
    pub type ScopeType = u32;
    pub const VAR_DEF_SCOPE: ScopeType = 2;
    pub const VAR_SCOPE: ScopeType = 1;
    pub const VAR_NO_SCOPE: ScopeType = 0;
    pub type VarLockStatus = u32;
    pub const VAR_FIXED: VarLockStatus = 2;
    pub const VAR_LOCKED: VarLockStatus = 1;
    pub const VAR_UNLOCKED: VarLockStatus = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ScopeDictDictItem {
        pub di_tv: typval_T,
        pub di_flags: u8,
        pub di_key: [u8; 1],
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
        pub v_string: *mut u8,
        pub v_list: *mut list_T,
        pub v_dict: *mut dict_T,
        pub v_partial: *mut partial_T,
    }
    pub type partial_T = partial_S;
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
    pub struct partial_S {
        pub pt_refcount: i32,
        pub pt_name: *mut u8,
        pub pt_func: *mut ufunc_T,
        pub pt_auto: bool,
        pub pt_argc: i32,
        pub pt_argv: *mut typval_T,
        pub pt_dict: *mut dict_T,
    }
    pub type ufunc_T = ufunc;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ufunc {
        pub uf_varargs: i32,
        pub uf_flags: i32,
        pub uf_calls: i32,
        pub uf_cleared: bool,
        pub uf_args: garray_T,
        pub uf_lines: garray_T,
        pub uf_profiling: i32,
        pub uf_prof_initialized: i32,
        pub uf_tm_count: i32,
        pub uf_tm_total: proftime_T,
        pub uf_tm_self: proftime_T,
        pub uf_tm_children: proftime_T,
        pub uf_tml_count: *mut i32,
        pub uf_tml_total: *mut proftime_T,
        pub uf_tml_self: *mut proftime_T,
        pub uf_tml_start: proftime_T,
        pub uf_tml_children: proftime_T,
        pub uf_tml_wait: proftime_T,
        pub uf_tml_idx: i32,
        pub uf_tml_execed: i32,
        pub uf_script_ctx: sctx_T,
        pub uf_refcount: i32,
        pub uf_scoped: *mut funccall_T,
        pub uf_name: [u8; 0],
    }
    pub type funccall_T = funccall_S;
    pub type list_T = listvar_S;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct listvar_S {
        pub lv_first: *mut listitem_T,
        pub lv_last: *mut listitem_T,
        pub lv_watch: *mut listwatch_T,
        pub lv_idx_item: *mut listitem_T,
        pub lv_copylist: *mut list_T,
        pub lv_used_next: *mut list_T,
        pub lv_used_prev: *mut list_T,
        pub lv_refcount: i32,
        pub lv_len: i32,
        pub lv_idx: i32,
        pub lv_copyID: i32,
        pub lv_lock: VarLockStatus,
    }
    pub type listitem_T = listitem_S;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct listitem_S {
        pub li_next: *mut listitem_T,
        pub li_prev: *mut listitem_T,
        pub li_tv: typval_T,
    }
    pub type listwatch_T = listwatch_S;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct listwatch_S {
        pub lw_item: *mut listitem_T,
        pub lw_next: *mut listwatch_T,
    }
    pub type float_T = libc::c_double;
    pub type SpecialVarValue = u32;
    pub const kSpecialVarNull: SpecialVarValue = 2;
    pub const kSpecialVarTrue: SpecialVarValue = 1;
    pub const kSpecialVarFalse: SpecialVarValue = 0;
    pub type varnumber_T = i64;
    pub type VarType = u32;
    pub const VAR_PARTIAL: VarType = 8;
    pub const VAR_SPECIAL: VarType = 7;
    pub const VAR_FLOAT: VarType = 6;
    pub const VAR_DICT: VarType = 5;
    pub const VAR_LIST: VarType = 4;
    pub const VAR_FUNC: VarType = 3;
    pub const VAR_STRING: VarType = 2;
    pub const VAR_NUMBER: VarType = 1;
    pub const VAR_UNKNOWN: VarType = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Callback {
        pub data: C2RustUnnamed_8,
        pub type_0: CallbackType,
    }
    pub type CallbackType = u32;
    pub const kCallbackPartial: CallbackType = 2;
    pub const kCallbackFuncref: CallbackType = 1;
    pub const kCallbackNone: CallbackType = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_8 {
        pub funcref: *mut u8,
        pub partial: *mut partial_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct dict_watcher {
        pub callback: Callback,
        pub key_pattern: *mut i8,
        pub key_pattern_len: size_t,
        pub node: QUEUE,
        pub busy: bool,
    }
    pub type DictWatcher = dict_watcher;
    // In a hashtab item "hi_key" points to "di_key" in a dictitem.
    // This avoids adding a pointer to the hashtab item.
    // / Convert a hashitem pointer to a dictitem pointer
    // / Increase reference count for a given list
    // /
    // / Does nothing for NULL lists.
    // /
    // / @param[in,out]  l  List to modify.
    #[inline(always)]
    pub unsafe extern "C" fn tv_list_ref(l: *mut list_T) {
        if l.is_null() {
            return;
        }
        (*l).lv_refcount += 1;
    }
    // / Set a list as the return value
    // /
    // / @param[out]  tv  Object to receive the list
    // / @param[in,out]  l  List to pass to the object
    #[inline(always)]
    pub unsafe extern "C" fn tv_list_set_ret(tv: *mut typval_T, l: *mut list_T) {
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
    pub unsafe extern "C" fn tv_list_locked(l: *const list_T) -> VarLockStatus {
        if l.is_null() {
            return VAR_FIXED;
        }
        return (*l).lv_lock;
    }
    // / Set list lock status
    // /
    // / May only “set” VAR_FIXED for NULL lists.
    // /
    // / @param[out]  l  List to modify.
    // / @param[in]  lock  New lock status.
    #[inline]
    pub unsafe extern "C" fn tv_list_set_lock(l: *mut list_T, lock: VarLockStatus) {
        if l.is_null() {
            if lock as u32 == VAR_FIXED as i32 as u32 {
            } else {
                assert!(false, "lock == VAR_FIXED");
            }
            return;
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
    pub unsafe extern "C" fn tv_list_set_copyid(l: *mut list_T, copyid: i32) {
        (*l).lv_copyID = copyid;
    }
    // / Get the number of items in a list
    // /
    // / @param[in]  l  List to check.
    #[inline]
    pub unsafe extern "C" fn tv_list_len(l: *const list_T) -> i32 {
        if l.is_null() {
            return 0;
        }
        return (*l).lv_len;
    }
    // / Get list copyID
    // /
    // / Does not expect NULL list, be careful.
    // /
    // / @param[in]  l  List to check.
    #[inline]
    pub unsafe extern "C" fn tv_list_copyid(l: *const list_T) -> i32 {
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
    pub unsafe extern "C" fn tv_list_latest_copy(l: *const list_T) -> *mut list_T {
        return (*l).lv_copylist;
    }
    // / Normalize index: that is, return either -1 or non-negative index
    // /
    // / @param[in]  l  List to index. Used to get length.
    // / @param[in]  n  List index, possibly negative.
    // /
    // / @return -1 or list index in range [0, tv_list_len(l)).
    #[inline]
    pub unsafe extern "C" fn tv_list_uidx(l: *const list_T, mut n: i32) -> i32 {
        // Negative index is relative to the end.
        if n < 0 {
            n += tv_list_len(l)
        }
        // Check for index out of range.
        if n < 0 || n >= tv_list_len(l) {
            return -(1);
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
    pub unsafe extern "C" fn tv_list_has_watchers(l: *const list_T) -> bool {
        return !l.is_null() && !(*l).lv_watch.is_null();
    }
    // / Get first list item
    // /
    // / @param[in]  l  List to get item from.
    // /
    // / @return List item or NULL in case of an empty list.
    #[inline]
    pub unsafe extern "C" fn tv_list_first(l: *const list_T) -> *mut listitem_T {
        if l.is_null() {
            return NULL_1 as *mut listitem_T;
        }
        return (*l).lv_first;
    }
    // / Get last list item
    // /
    // / @param[in]  l  List to get item from.
    // /
    // / @return List item or NULL in case of an empty list.
    #[inline]
    pub unsafe extern "C" fn tv_list_last(l: *const list_T) -> *mut listitem_T {
        if l.is_null() {
            return NULL_1 as *mut listitem_T;
        }
        return (*l).lv_last;
    }
    // / Set a dictionary as the return value
    // /
    // / @param[out]  tv  Object to receive the dictionary
    // / @param[in,out]  d  Dictionary to pass to the object
    #[inline(always)]
    pub unsafe extern "C" fn tv_dict_set_ret(tv: *mut typval_T, d: *mut dict_T) {
        (*tv).v_type = VAR_DICT;
        (*tv).vval.v_dict = d;
        if !d.is_null() {
            (*d).dv_refcount += 1
        };
    }
    // / Get the number of items in a Dictionary
    // /
    // / @param[in]  d  Dictionary to check.
    #[inline]
    pub unsafe extern "C" fn tv_dict_len(d: *const dict_T) -> i64 {
        if d.is_null() {
            return 0;
        }
        return (*d).dv_hashtab.ht_used as i64;
    }
    // / Check if dictionary is watched
    // /
    // / @param[in]  d  Dictionary to check.
    // /
    // / @return true if there is at least one watcher.
    #[inline]
    pub unsafe extern "C" fn tv_dict_is_watched(d: *const dict_T) -> bool {
        return !d.is_null() && QUEUE_EMPTY(&(*d).watchers) == 0;
    }
    // / Initialize VimL object
    // /
    // / Initializes to unlocked VAR_UNKNOWN object.
    // /
    // / @param[out]  tv  Object to initialize.
    #[inline]
    pub unsafe extern "C" fn tv_init(tv: *mut typval_T) {
        if !tv.is_null() {
            memset(tv as *mut libc::c_void, 0, ::std::mem::size_of::<typval_T>() as u64);
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
    pub unsafe extern "C" fn tv_get_float_chk(tv: *const typval_T, ret_f: *mut float_T) -> bool {
        if (*tv).v_type as u32 == VAR_FLOAT as i32 as u32 {
            *ret_f = (*tv).vval.v_float;
            return true;
        }
        if (*tv).v_type as u32 == VAR_NUMBER as i32 as u32 {
            *ret_f = (*tv).vval.v_number as float_T;
            return true;
        }
        emsgf(b"%s\x00" as *const u8 as *const i8, gettext("E808: Number or Float required"));
        return false;
    }
    // / Compute the `DictWatcher` address from a QUEUE node.
    // /
    // / This only exists for .asan-blacklist (ASAN doesn't handle QUEUE_DATA pointer
    // / arithmetic).
    #[inline(always)]
    pub unsafe extern "C" fn tv_dict_watcher_node_data(mut q: *mut QUEUE) -> *mut DictWatcher {
        return (q as *mut i8).offset(-(32)) as *mut DictWatcher;
    }
    // / Check whether given typval_T contains a function
    // /
    // / That is, whether it contains VAR_FUNC or VAR_PARTIAL.
    // /
    // / @param[in]  tv  Typval to check.
    // /
    // / @return True if it is a function or a partial, false otherwise.
    #[inline]
    pub unsafe extern "C" fn tv_is_func(tv: typval_T) -> bool {
        return tv.v_type as u32 == VAR_FUNC as i32 as u32 || tv.v_type as u32 == VAR_PARTIAL as i32 as u32;
    }
    extern "C" {
        pub type funccall_S;
    }
    // NVIM_EVAL_TYPVAL_H
}
pub mod pos_h {
    pub type linenr_T = i64;
    // line number type
    // / Format used to print values which have linenr_T type
    // / Column number type
    pub type colnr_T = i32;
    // / Maximal column number, 31 bits
    pub type C2RustUnnamed = u32;
    pub const MAXCOL: C2RustUnnamed = 2147483647;
    /*
     * position in file or buffer
     */
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct pos_T {
        pub lnum: linenr_T,
        pub col: colnr_T,
        pub coladd: colnr_T,
    }
    /*
     * Same, but without coladd.
     */
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct lpos_T {
        pub lnum: linenr_T,
        pub col: colnr_T,
    }
    // NVIM_POS_H
}
pub mod ctype_h {
    pub type C2RustUnnamed_0 = u32;
    pub const _ISalnum: C2RustUnnamed_0 = 8;
    pub const _ISpunct: C2RustUnnamed_0 = 4;
    pub const _IScntrl: C2RustUnnamed_0 = 2;
    pub const _ISblank: C2RustUnnamed_0 = 1;
    pub const _ISgraph: C2RustUnnamed_0 = 32768;
    pub const _ISprint: C2RustUnnamed_0 = 16384;
    pub const _ISspace: C2RustUnnamed_0 = 8192;
    pub const _ISxdigit: C2RustUnnamed_0 = 4096;
    pub const _ISdigit: C2RustUnnamed_0 = 2048;
    pub const _ISalpha: C2RustUnnamed_0 = 1024;
    pub const _ISlower: C2RustUnnamed_0 = 512;
    pub const _ISupper: C2RustUnnamed_0 = 256;
}
pub mod sys_types_h {
    pub type gid_t = __gid_t;
    pub type uid_t = __uid_t;
}
pub mod time_t_h {
    pub type time_t = __time_t;
}
pub mod pthreadtypes_arch_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __pthread_rwlock_arch_t {
        pub __readers: u32,
        pub __writers: u32,
        pub __wrphase_futex: u32,
        pub __writers_futex: u32,
        pub __pad3: u32,
        pub __pad4: u32,
        pub __cur_writer: i32,
        pub __shared: i32,
        pub __rwelision: libc::c_schar,
        pub __pad1: [libc::c_uchar; 7],
        pub __pad2: u64,
        pub __flags: u32,
    }
}
pub mod thread_shared_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __pthread_internal_list {
        pub __prev: *mut __pthread_internal_list,
        pub __next: *mut __pthread_internal_list,
    }
    pub type __pthread_list_t = __pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __pthread_mutex_s {
        pub __lock: i32,
        pub __count: u32,
        pub __owner: i32,
        pub __nusers: u32,
        pub __kind: i32,
        pub __spins: i16,
        pub __elision: i16,
        pub __list: __pthread_list_t,
    }
}
pub mod pthreadtypes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union pthread_mutex_t {
        pub __data: __pthread_mutex_s,
        pub __size: [i8; 40],
        pub __align: i64,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union pthread_rwlock_t {
        pub __data: __pthread_rwlock_arch_t,
        pub __size: [i8; 56],
        pub __align: i64,
    }
}
pub mod hashtab_h {
    // / Type for hash number (hash calculation result).
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
    pub struct hashitem_S {
        pub hi_hash: hash_T,
        pub hi_key: *mut u8,
    }
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
    pub struct hashtable_S {
        pub ht_mask: hash_T,
        pub ht_used: size_t,
        pub ht_filled: size_t,
        pub ht_locked: i32,
        pub ht_array: *mut hashitem_T,
        pub ht_smallarray: [hashitem_T; 16],
    }
    pub type hashtab_T = hashtable_S;
    // / initial array
    // NVIM_HASHTAB_H
    // / Iterate over a hashtab
    // /
    // / @param[in]  ht  Hashtab to iterate over.
    // / @param  hi  Name of the variable with current hashtab entry.
    // / @param  code  Cycle body.
}
pub mod garray_h {
    // for size_t
    // for u8
    // / Structure used for growing arrays.
    // / This is used to store information that only grows, is deleted all at
    // / once, and needs to be accessed by index.  See ga_clear() and ga_grow().
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct growarray {
        pub ga_len: i32,
        pub ga_maxlen: i32,
        pub ga_itemsize: i32,
        pub ga_growsize: i32,
        pub ga_data: *mut libc::c_void,
    }
    pub type garray_T = growarray;
    #[inline]
    pub unsafe extern "C" fn ga_append_via_ptr(mut gap: *mut garray_T, mut item_size: size_t) -> *mut libc::c_void {
        if item_size as i32 != (*gap).ga_itemsize {
            logmsg(
                WARN_LOG_LEVEL,
                ptr::null_mut(),
                (*::std::mem::transmute::<&[u8; 18], &[i8; 18]>(b"ga_append_via_ptr\x00")).as_ptr(),
                50,
                true,
                b"wrong item size (%zu), should be %d\x00" as *const u8 as *const i8,
                item_size,
                (*gap).ga_itemsize,
            );
        }
        ga_grow(gap, 1);
        let fresh0 = (*gap).ga_len;
        (*gap).ga_len = (*gap).ga_len + 1;
        return ((*gap).ga_data as *mut i8).offset(item_size.wrapping_mul(fresh0 as size_t) as isize) as *mut libc::c_void;
    }
    extern "C" {
        #[no_mangle]
        pub fn ga_grow(gap: *mut garray_T, n: i32);
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
pub mod iconv_h {
    pub type iconv_t = *mut libc::c_void;
    extern "C" {
        #[no_mangle]
        pub fn iconv_open(__tocode: *const i8, __fromcode: *const i8) -> iconv_t;
        #[no_mangle]
        pub fn iconv(__cd: iconv_t, __inbuf: *mut *mut i8, __inbytesleft: *mut size_t, __outbuf: *mut *mut i8, __outbytesleft: *mut size_t) -> size_t;
        #[no_mangle]
        pub fn iconv_close(__cd: iconv_t) -> i32;
    }
}
pub mod buffer_defs_h {
    // for FILE
    pub type buf_T = file_buffer;
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
    pub struct file_buffer {
        pub handle: handle_T,
        pub b_ml: memline_T,
        pub b_next: *mut buf_T,
        pub b_prev: *mut buf_T,
        pub b_nwindows: i32,
        pub b_flags: i32,
        pub b_locked: i32,
        pub b_ffname: *mut u8,
        pub b_sfname: *mut u8,
        pub b_fname: *mut u8,
        pub file_id_valid: bool,
        pub file_id: FileID,
        pub b_changed: i32,
        pub changedtick_di: ChangedtickDictItem,
        pub b_last_changedtick: varnumber_T,
        pub b_last_changedtick_pum: varnumber_T,
        pub b_saving: bool,
        pub b_mod_set: bool,
        pub b_mod_top: linenr_T,
        pub b_mod_bot: linenr_T,
        pub b_mod_xlines: i64,
        pub b_wininfo: *mut wininfo_T,
        pub b_mtime: i64,
        pub b_mtime_read: i64,
        pub b_orig_size: u64,
        pub b_orig_mode: i32,
        pub b_namedm: [fmark_T; 26],
        pub b_visual: visualinfo_T,
        pub b_visual_mode_eval: i32,
        pub b_last_cursor: fmark_T,
        pub b_last_insert: fmark_T,
        pub b_last_change: fmark_T,
        pub b_changelist: [fmark_T; 100],
        pub b_changelistlen: i32,
        pub b_new_change: bool,
        pub b_chartab: [u64; 4],
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
        pub b_u_numhead: i32,
        pub b_u_synced: bool,
        pub b_u_seq_last: i64,
        pub b_u_save_nr_last: i64,
        pub b_u_seq_cur: i64,
        pub b_u_time_cur: time_t,
        pub b_u_save_nr_cur: i64,
        pub b_u_line_ptr: *mut u8,
        pub b_u_line_lnum: linenr_T,
        pub b_u_line_colnr: colnr_T,
        pub b_scanned: bool,
        pub b_p_iminsert: i64,
        pub b_p_imsearch: i64,
        pub b_kmap_state: i16,
        pub b_kmap_ga: garray_T,
        pub b_p_initialized: bool,
        pub b_p_script_ctx: [LastSet; 80],
        pub b_p_ai: i32,
        pub b_p_ai_nopaste: i32,
        pub b_p_bkc: *mut u8,
        pub b_bkc_flags: u32,
        pub b_p_ci: i32,
        pub b_p_bin: i32,
        pub b_p_bomb: i32,
        pub b_p_bh: *mut u8,
        pub b_p_bt: *mut u8,
        pub b_has_qf_entry: i32,
        pub b_p_bl: i32,
        pub b_p_channel: i64,
        pub b_p_cin: i32,
        pub b_p_cino: *mut u8,
        pub b_p_cink: *mut u8,
        pub b_p_cinw: *mut u8,
        pub b_p_com: *mut u8,
        pub b_p_cms: *mut u8,
        pub b_p_cpt: *mut u8,
        pub b_p_cfu: *mut u8,
        pub b_p_ofu: *mut u8,
        pub b_p_tfu: *mut u8,
        pub b_p_eol: i32,
        pub b_p_fixeol: i32,
        pub b_p_et: i32,
        pub b_p_et_nobin: i32,
        pub b_p_et_nopaste: i32,
        pub b_p_fenc: *mut u8,
        pub b_p_ff: *mut u8,
        pub b_p_ft: *mut u8,
        pub b_p_fo: *mut u8,
        pub b_p_flp: *mut u8,
        pub b_p_inf: i32,
        pub b_p_isk: *mut u8,
        pub b_p_def: *mut u8,
        pub b_p_inc: *mut u8,
        pub b_p_inex: *mut u8,
        pub b_p_inex_flags: u32,
        pub b_p_inde: *mut u8,
        pub b_p_inde_flags: u32,
        pub b_p_indk: *mut u8,
        pub b_p_fp: *mut u8,
        pub b_p_fex: *mut u8,
        pub b_p_fex_flags: u32,
        pub b_p_kp: *mut u8,
        pub b_p_lisp: i32,
        pub b_p_menc: *mut u8,
        pub b_p_mps: *mut u8,
        pub b_p_ml: i32,
        pub b_p_ml_nobin: i32,
        pub b_p_ma: i32,
        pub b_p_nf: *mut u8,
        pub b_p_pi: i32,
        pub b_p_qe: *mut u8,
        pub b_p_ro: i32,
        pub b_p_sw: i64,
        pub b_p_scbk: i64,
        pub b_p_si: i32,
        pub b_p_sts: i64,
        pub b_p_sts_nopaste: i64,
        pub b_p_sua: *mut u8,
        pub b_p_swf: i32,
        pub b_p_smc: i64,
        pub b_p_syn: *mut u8,
        pub b_p_ts: i64,
        pub b_p_tw: i64,
        pub b_p_tw_nobin: i64,
        pub b_p_tw_nopaste: i64,
        pub b_p_wm: i64,
        pub b_p_wm_nobin: i64,
        pub b_p_wm_nopaste: i64,
        pub b_p_keymap: *mut u8,
        pub b_p_gp: *mut u8,
        pub b_p_mp: *mut u8,
        pub b_p_efm: *mut u8,
        pub b_p_ep: *mut u8,
        pub b_p_path: *mut u8,
        pub b_p_ar: i32,
        pub b_p_tags: *mut u8,
        pub b_p_tc: *mut u8,
        pub b_tc_flags: u32,
        pub b_p_dict: *mut u8,
        pub b_p_tsr: *mut u8,
        pub b_p_ul: i64,
        pub b_p_udf: i32,
        pub b_p_lw: *mut u8,
        pub b_ind_level: i32,
        pub b_ind_open_imag: i32,
        pub b_ind_no_brace: i32,
        pub b_ind_first_open: i32,
        pub b_ind_open_extra: i32,
        pub b_ind_close_extra: i32,
        pub b_ind_open_left_imag: i32,
        pub b_ind_jump_label: i32,
        pub b_ind_case: i32,
        pub b_ind_case_code: i32,
        pub b_ind_case_break: i32,
        pub b_ind_param: i32,
        pub b_ind_func_type: i32,
        pub b_ind_comment: i32,
        pub b_ind_in_comment: i32,
        pub b_ind_in_comment2: i32,
        pub b_ind_cpp_baseclass: i32,
        pub b_ind_continuation: i32,
        pub b_ind_unclosed: i32,
        pub b_ind_unclosed2: i32,
        pub b_ind_unclosed_noignore: i32,
        pub b_ind_unclosed_wrapped: i32,
        pub b_ind_unclosed_whiteok: i32,
        pub b_ind_matching_paren: i32,
        pub b_ind_paren_prev: i32,
        pub b_ind_maxparen: i32,
        pub b_ind_maxcomment: i32,
        pub b_ind_scopedecl: i32,
        pub b_ind_scopedecl_code: i32,
        pub b_ind_java: i32,
        pub b_ind_js: i32,
        pub b_ind_keep_case_label: i32,
        pub b_ind_hash_comment: i32,
        pub b_ind_cpp_namespace: i32,
        pub b_ind_if_for_while: i32,
        pub b_ind_cpp_extern_c: i32,
        pub b_no_eol_lnum: linenr_T,
        pub b_start_eol: i32,
        pub b_start_ffc: i32,
        pub b_start_fenc: *mut u8,
        pub b_bad_char: i32,
        pub b_start_bomb: i32,
        pub b_bufvar: ScopeDictDictItem,
        pub b_vars: *mut dict_T,
        pub b_may_swap: bool,
        pub b_did_warn: bool,
        pub b_help: bool,
        pub b_spell: bool,
        pub b_prompt_text: *mut u8,
        pub b_prompt_callback: Callback,
        pub b_prompt_interrupt: Callback,
        pub b_prompt_insert: i32,
        pub b_s: synblock_T,
        pub b_signlist: *mut signlist_T,
        pub b_signcols_max: i32,
        pub b_signcols: i32,
        pub terminal: *mut Terminal,
        pub additional_data: *mut dict_T,
        pub b_mapped_ctrl_c: i32,
        pub b_marktree: [MarkTree; 1],
        pub b_extmark_index: *mut Map_uint64_t_ExtmarkItem,
        pub b_extmark_ns: *mut Map_uint64_t_ExtmarkNs,
        pub update_channels: C2RustUnnamed_4,
        pub update_callbacks: C2RustUnnamed_3,
        pub update_need_codepoints: bool,
        pub deleted_bytes: size_t,
        pub deleted_codepoints: size_t,
        pub deleted_codeunits: size_t,
        pub flush_count: i32,
        pub b_luahl: bool,
        pub b_luahl_start: LuaRef,
        pub b_luahl_window: LuaRef,
        pub b_luahl_line: LuaRef,
        pub b_luahl_end: LuaRef,
        pub b_diff_failed: i32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2RustUnnamed_3 {
        pub size: size_t,
        pub capacity: size_t,
        pub items: *mut BufUpdateCallbacks,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct BufUpdateCallbacks {
        pub on_lines: LuaRef,
        pub on_bytes: LuaRef,
        pub on_changedtick: LuaRef,
        pub on_detach: LuaRef,
        pub utf_sizes: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2RustUnnamed_4 {
        pub size: size_t,
        pub capacity: size_t,
        pub items: *mut u64,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct synblock_T {
        pub b_keywtab: hashtab_T,
        pub b_keywtab_ic: hashtab_T,
        pub b_syn_error: i32,
        pub b_syn_slow: bool,
        pub b_syn_ic: i32,
        pub b_syn_spell: i32,
        pub b_syn_patterns: garray_T,
        pub b_syn_clusters: garray_T,
        pub b_spell_cluster_id: i32,
        pub b_nospell_cluster_id: i32,
        pub b_syn_containedin: i32,
        pub b_syn_sync_flags: i32,
        pub b_syn_sync_id: int16_t,
        pub b_syn_sync_minlines: i64,
        pub b_syn_sync_maxlines: i64,
        pub b_syn_sync_linebreaks: i64,
        pub b_syn_linecont_pat: *mut u8,
        pub b_syn_linecont_prog: *mut regprog_T,
        pub b_syn_linecont_time: syn_time_T,
        pub b_syn_linecont_ic: i32,
        pub b_syn_topgrp: i32,
        pub b_syn_conceal: i32,
        pub b_syn_folditems: i32,
        pub b_sst_array: *mut synstate_T,
        pub b_sst_len: i32,
        pub b_sst_first: *mut synstate_T,
        pub b_sst_firstfree: *mut synstate_T,
        pub b_sst_freecount: i32,
        pub b_sst_check_lnum: linenr_T,
        pub b_sst_lasttick: disptick_T,
        pub b_langp: garray_T,
        pub b_spell_ismw: [bool; 256],
        pub b_spell_ismw_mb: *mut u8,
        pub b_p_spc: *mut u8,
        pub b_cap_prog: *mut regprog_T,
        pub b_p_spf: *mut u8,
        pub b_p_spl: *mut u8,
        pub b_cjk: i32,
        pub b_syn_chartab: [u8; 32],
        pub b_syn_isk: *mut u8,
    }
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
    pub struct window_S {
        pub handle: handle_T,
        pub w_buffer: *mut buf_T,
        pub w_s: *mut synblock_T,
        pub w_hl_id_normal: i32,
        pub w_hl_attr_normal: i32,
        pub w_hl_ids: [i32; 50],
        pub w_hl_attrs: [i32; 50],
        pub w_hl_needs_update: i32,
        pub w_prev: *mut win_T,
        pub w_next: *mut win_T,
        pub w_closing: bool,
        pub w_frame: *mut frame_T,
        pub w_cursor: pos_T,
        pub w_curswant: colnr_T,
        pub w_set_curswant: i32,
        pub w_last_cursorline: linenr_T,
        pub w_last_cursormoved: pos_T,
        pub w_old_visual_mode: i8,
        pub w_old_cursor_lnum: linenr_T,
        pub w_old_cursor_fcol: colnr_T,
        pub w_old_cursor_lcol: colnr_T,
        pub w_old_visual_lnum: linenr_T,
        pub w_old_visual_col: colnr_T,
        pub w_old_curswant: colnr_T,
        pub w_p_lcs_chars: C2RustUnnamed_6,
        pub w_p_fcs_chars: C2RustUnnamed_5,
        pub w_topline: linenr_T,
        pub w_topline_was_set: i8,
        pub w_topfill: i32,
        pub w_old_topfill: i32,
        pub w_botfill: bool,
        pub w_old_botfill: bool,
        pub w_leftcol: colnr_T,
        pub w_skipcol: colnr_T,
        pub w_winrow: i32,
        pub w_height: i32,
        pub w_status_height: i32,
        pub w_wincol: i32,
        pub w_width: i32,
        pub w_vsep_width: i32,
        pub w_save_cursor: pos_save_T,
        pub w_height_inner: i32,
        pub w_width_inner: i32,
        pub w_height_request: i32,
        pub w_width_request: i32,
        pub w_valid: i32,
        pub w_valid_cursor: pos_T,
        pub w_valid_leftcol: colnr_T,
        pub w_cline_height: i32,
        pub w_cline_folded: bool,
        pub w_cline_row: i32,
        pub w_virtcol: colnr_T,
        pub w_wrow: i32,
        pub w_wcol: i32,
        pub w_botline: linenr_T,
        pub w_empty_rows: i32,
        pub w_filler_rows: i32,
        pub w_lines_valid: i32,
        pub w_lines: *mut wline_T,
        pub w_folds: garray_T,
        pub w_fold_manual: bool,
        pub w_foldinvalid: bool,
        pub w_nrwidth: i32,
        pub w_redr_type: i32,
        pub w_upd_rows: i32,
        pub w_redraw_top: linenr_T,
        pub w_redraw_bot: linenr_T,
        pub w_redr_status: i32,
        pub w_ru_cursor: pos_T,
        pub w_ru_virtcol: colnr_T,
        pub w_ru_topline: linenr_T,
        pub w_ru_line_count: linenr_T,
        pub w_ru_topfill: i32,
        pub w_ru_empty: i8,
        pub w_alt_fnum: i32,
        pub w_alist: *mut alist_T,
        pub w_arg_idx: i32,
        pub w_arg_idx_invalid: i32,
        pub w_localdir: *mut u8,
        pub w_onebuf_opt: winopt_T,
        pub w_allbuf_opt: winopt_T,
        pub w_p_stl_flags: u32,
        pub w_p_fde_flags: u32,
        pub w_p_fdt_flags: u32,
        pub w_p_cc_cols: *mut i32,
        pub w_p_brimin: i32,
        pub w_p_brishift: i32,
        pub w_p_brisbr: bool,
        pub w_p_siso: i64,
        pub w_p_so: i64,
        pub w_scbind_pos: i64,
        pub w_winvar: ScopeDictDictItem,
        pub w_vars: *mut dict_T,
        pub w_pcmark: pos_T,
        pub w_prev_pcmark: pos_T,
        pub w_jumplist: [xfmark_T; 100],
        pub w_jumplistlen: i32,
        pub w_jumplistidx: i32,
        pub w_changelistidx: i32,
        pub w_match_head: *mut matchitem_T,
        pub w_next_match_id: i32,
        pub w_tagstack: [taggy_T; 20],
        pub w_tagstackidx: i32,
        pub w_tagstacklen: i32,
        pub w_grid: ScreenGrid,
        pub w_pos_changed: bool,
        pub w_floating: bool,
        pub w_float_config: FloatConfig,
        pub w_fraction: i32,
        pub w_prev_fraction_row: i32,
        pub w_nrwidth_line_count: linenr_T,
        pub w_nrwidth_width: i32,
        pub w_llist: *mut qf_info_T,
        pub w_llist_ref: *mut qf_info_T,
    }
    pub type qf_info_T = qf_info_S;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct FloatConfig {
        pub window: Window,
        pub bufpos: lpos_T,
        pub height: i32,
        pub width: i32,
        pub row: libc::c_double,
        pub col: libc::c_double,
        pub anchor: FloatAnchor,
        pub relative: FloatRelative,
        pub external: bool,
        pub focusable: bool,
        pub style: WinStyle,
    }
    pub type WinStyle = u32;
    pub const kWinStyleMinimal: WinStyle = 1;
    pub const kWinStyleUnused: WinStyle = 0;
    pub type FloatRelative = u32;
    pub const kFloatRelativeCursor: FloatRelative = 2;
    pub const kFloatRelativeWindow: FloatRelative = 1;
    pub const kFloatRelativeEditor: FloatRelative = 0;
    pub type FloatAnchor = i32;
    pub type taggy_T = taggy;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct taggy {
        pub tagname: *mut u8,
        pub fmark: fmark_T,
        pub cur_match: i32,
        pub cur_fnum: i32,
        pub user_data: *mut u8,
    }
    pub type matchitem_T = matchitem;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct matchitem {
        pub next: *mut matchitem_T,
        pub id: i32,
        pub priority: i32,
        pub pattern: *mut u8,
        pub hlg_id: i32,
        pub match_0: regmmatch_T,
        pub pos: posmatch_T,
        pub hl: match_T,
        pub conceal_char: i32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct match_T {
        pub rm: regmmatch_T,
        pub buf: *mut buf_T,
        pub lnum: linenr_T,
        pub attr: i32,
        pub attr_cur: i32,
        pub first_lnum: linenr_T,
        pub startcol: colnr_T,
        pub endcol: colnr_T,
        pub is_addpos: bool,
        pub tm: proftime_T,
    }
    pub type posmatch_T = posmatch;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct posmatch {
        pub pos: [llpos_T; 8],
        pub cur: i32,
        pub toplnum: linenr_T,
        pub botlnum: linenr_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct llpos_T {
        pub lnum: linenr_T,
        pub col: colnr_T,
        pub len: i32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct winopt_T {
        pub wo_arab: i32,
        pub wo_bri: i32,
        pub wo_briopt: *mut u8,
        pub wo_diff: i32,
        pub wo_fdc: *mut u8,
        pub wo_fdc_save: *mut u8,
        pub wo_fen: i32,
        pub wo_fen_save: i32,
        pub wo_fdi: *mut u8,
        pub wo_fdl: i64,
        pub wo_fdl_save: i32,
        pub wo_fdm: *mut u8,
        pub wo_fdm_save: *mut u8,
        pub wo_fml: i64,
        pub wo_fdn: i64,
        pub wo_fde: *mut u8,
        pub wo_fdt: *mut u8,
        pub wo_fmr: *mut u8,
        pub wo_lbr: i32,
        pub wo_list: i32,
        pub wo_nu: i32,
        pub wo_rnu: i32,
        pub wo_nuw: i64,
        pub wo_wfh: i32,
        pub wo_wfw: i32,
        pub wo_pvw: i32,
        pub wo_rl: i32,
        pub wo_rlc: *mut u8,
        pub wo_scr: i64,
        pub wo_spell: i32,
        pub wo_cuc: i32,
        pub wo_cul: i32,
        pub wo_cc: *mut u8,
        pub wo_stl: *mut u8,
        pub wo_scb: i32,
        pub wo_diff_saved: i32,
        pub wo_scb_save: i32,
        pub wo_wrap: i32,
        pub wo_wrap_save: i32,
        pub wo_cocu: *mut u8,
        pub wo_cole: i64,
        pub wo_crb: i32,
        pub wo_crb_save: i32,
        pub wo_scl: *mut u8,
        pub wo_winhl: *mut u8,
        pub wo_fcs: *mut u8,
        pub wo_lcs: *mut u8,
        pub wo_winbl: i64,
        pub wo_script_ctx: [LastSet; 42],
    }
    pub type alist_T = arglist;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct arglist {
        pub al_ga: garray_T,
        pub al_refcount: i32,
        pub id: i32,
    }
    pub type wline_T = w_line;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct w_line {
        pub wl_lnum: linenr_T,
        pub wl_size: uint16_t,
        pub wl_valid: i8,
        pub wl_folded: i8,
        pub wl_lastlnum: linenr_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct pos_save_T {
        pub w_topline_save: i32,
        pub w_topline_corr: i32,
        pub w_cursor_save: pos_T,
        pub w_cursor_corr: pos_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2RustUnnamed_5 {
        pub stl: i32,
        pub stlnc: i32,
        pub vert: i32,
        pub fold: i32,
        pub foldopen: i32,
        pub foldclosed: i32,
        pub foldsep: i32,
        pub diff: i32,
        pub msgsep: i32,
        pub eob: i32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2RustUnnamed_6 {
        pub eol: i32,
        pub ext: i32,
        pub prec: i32,
        pub nbsp: i32,
        pub space: i32,
        pub tab1: i32,
        pub tab2: i32,
        pub tab3: i32,
        pub trail: i32,
        pub conceal: i32,
    }
    pub type frame_T = frame_S;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct frame_S {
        pub fr_layout: i8,
        pub fr_width: i32,
        pub fr_newwidth: i32,
        pub fr_height: i32,
        pub fr_newheight: i32,
        pub fr_parent: *mut frame_T,
        pub fr_next: *mut frame_T,
        pub fr_prev: *mut frame_T,
        pub fr_child: *mut frame_T,
        pub fr_win: *mut win_T,
    }
    pub type disptick_T = uint16_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct syn_time_T {
        pub total: proftime_T,
        pub slowest: proftime_T,
        pub count: i64,
        pub match_0: i64,
    }
    pub type mapblock_T = mapblock;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct mapblock {
        pub m_next: *mut mapblock_T,
        pub m_keys: *mut u8,
        pub m_str: *mut u8,
        pub m_orig_str: *mut u8,
        pub m_keylen: i32,
        pub m_mode: i32,
        pub m_noremap: i32,
        pub m_silent: i8,
        pub m_nowait: i8,
        pub m_expr: i8,
        pub m_script_ctx: sctx_T,
    }
    pub type wininfo_T = wininfo_S;
    #[derive(Copy, Clone)]
    #[repr(C)]
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
    pub struct ChangedtickDictItem {
        pub di_tv: typval_T,
        pub di_flags: u8,
        pub di_key: [u8; 12],
    }
    #[inline]
    pub unsafe extern "C" fn win_hl_attr(mut wp: *mut win_T, mut hlf: i32) -> i32 {
        return (*wp).w_hl_attrs[hlf as usize];
    }
    extern "C" {
        pub type qf_info_S;
    }
    // NVIM_BUFFER_DEFS_H
    // / Macros defined in Vim, but not in Neovim
}
pub mod map_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Map_uint64_t_ExtmarkNs {
        pub table: *mut kh_uint64_t_ExtmarkNs_map_t,
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
    pub struct Map_uint64_t_ExtmarkItem {
        pub table: *mut kh_uint64_t_ExtmarkItem_map_t,
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
    extern "C" {
        #[no_mangle]
        pub fn map_uint64_t_ptr_t_get(map: *mut Map_uint64_t_ptr_t, key: u64) -> ptr_t;
    }
    // NVIM_MAP_H
}
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
    pub type khint32_t = u32;
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
    pub unsafe extern "C" fn __ac_X31_hash_string(mut s: *const i8) -> khint_t {
        let mut h = *s as khint_t;
        if h != 0 {
            s = s.offset(1);
            while *s != 0 {
                h = (h << 5).wrapping_sub(h).wrapping_add(*s as u8 as u32);
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
    pub unsafe extern "C" fn __ac_Wang_hash(mut key: khint_t) -> khint_t {
        key = (key as u32).wrapping_add(!(key << 15)) as khint_t as khint_t;
        key ^= key >> 10;
        key = (key as u32).wrapping_add(key << 3) as khint_t as khint_t;
        key ^= key >> 6;
        key = (key as u32).wrapping_add(!(key << 11)) as khint_t as khint_t;
        key ^= key >> 16;
        return key;
    }
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
pub mod extmark_defs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExtmarkItem {
        pub ns_id: u64,
        pub mark_id: u64,
        pub hl_id: i32,
        pub virt_text: VirtText,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct VirtText {
        pub size: size_t,
        pub capacity: size_t,
        pub items: *mut VirtTextChunk,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct VirtTextChunk {
        pub text: *mut i8,
        pub hl_id: i32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct extmark_undo_vec_t {
        pub size: size_t,
        pub capacity: size_t,
        pub items: *mut ExtmarkUndoObject,
    }
    pub type ExtmarkUndoObject = undo_object;
    extern "C" {
        pub type undo_object;
    }
    // NVIM_EXTMARK_DEFS_H
}
pub mod marktree_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct MarkTree {
        pub root: *mut mtnode_t,
        pub n_keys: size_t,
        pub n_nodes: size_t,
        pub next_id: u64,
        pub id2node: *mut Map_uint64_t_ptr_t,
    }
    pub type mtnode_t = mtnode_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct mtnode_s {
        pub n: i32,
        pub level: i32,
        pub parent: *mut mtnode_t,
        pub key: [mtkey_t; 19],
        pub ptr: [*mut mtnode_t; 0],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct mtkey_t {
        pub pos: mtpos_t,
        pub id: u64,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct mtpos_t {
        pub row: i32,
        pub col: i32,
    }
    // NVIM_MARKTREE_H
}
pub mod map_defs_h {
    pub type ptr_t = *mut libc::c_void;
    pub type cstr_t = *const i8;
    // NVIM_MAP_DEFS_H
}
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
    pub type QUEUE = _queue;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _queue {
        pub next: *mut _queue,
        pub prev: *mut _queue,
    }
    // Public macros.
    // Important note: mutating the list while QUEUE_FOREACH is
    // iterating over its elements results in undefined behavior.
    /* NOLINT(readability/braces) */
    // ffi.cdef is unable to swallow `bool` in place of `int` here.
    #[inline]
    pub unsafe extern "C" fn QUEUE_EMPTY(q: *const QUEUE) -> i32 {
        return (q == (*q).next) as i32;
    }
    #[inline]
    pub unsafe extern "C" fn QUEUE_INIT(q: *mut QUEUE) {
        (*q).next = q;
        (*q).prev = q;
    }
    #[inline]
    pub unsafe extern "C" fn QUEUE_ADD(h: *mut QUEUE, n: *mut QUEUE) {
        (*(*h).prev).next = (*n).next;
        (*(*n).next).prev = (*h).prev;
        (*h).prev = (*n).prev;
        (*(*h).prev).next = h;
    }
    #[inline]
    pub unsafe extern "C" fn QUEUE_INSERT_HEAD(h: *mut QUEUE, q: *mut QUEUE) {
        (*q).next = (*h).next;
        (*q).prev = h;
        (*(*q).next).prev = q;
        (*h).next = q;
    }
    #[inline]
    pub unsafe extern "C" fn QUEUE_INSERT_TAIL(h: *mut QUEUE, q: *mut QUEUE) {
        (*q).next = h;
        (*q).prev = (*h).prev;
        (*(*q).prev).next = q;
        (*h).prev = q;
    }
    #[inline]
    pub unsafe extern "C" fn QUEUE_REMOVE(q: *mut QUEUE) {
        (*(*q).prev).next = (*q).next;
        (*(*q).next).prev = (*q).prev;
    }
    // NVIM_LIB_QUEUE_H
}
pub mod terminal_h {
    pub type Terminal = terminal;
    extern "C" {
        pub type terminal;
    }
    // NVIM_TERMINAL_H
}
pub mod sign_defs_h {
    // signs: line annotations
    // Sign group
    // number of signs in this group
    // next sign id for this group
    // sign group name
    // Macros to get the sign group structure from the group name
    pub type signlist_T = signlist;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct signlist {
        pub id: i32,
        pub lnum: linenr_T,
        pub typenr: i32,
        pub group: *mut signgroup_T,
        pub priority: i32,
        pub next: *mut signlist_T,
        pub prev: *mut signlist_T,
    }
    pub type signgroup_T = signgroup_S;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct signgroup_S {
        pub refcount: uint16_t,
        pub next_sign_id: i32,
        pub sg_name: [u8; 1],
    }
    // NVIM_SIGN_DEFS_H
}
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
    pub struct regprog {
        pub engine: *mut regengine_T,
        pub regflags: u32,
        pub re_engine: u32,
        pub re_flags: u32,
    }
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
    pub struct regengine {
        pub regcomp: Option<unsafe extern "C" fn(_: *mut u8, _: i32) -> *mut regprog_T>,
        pub regfree: Option<unsafe extern "C" fn(_: *mut regprog_T) -> ()>,
        pub regexec_nl: Option<unsafe extern "C" fn(_: *mut regmatch_T, _: *mut u8, _: colnr_T, _: bool) -> i32>,
        pub regexec_multi: Option<unsafe extern "C" fn(_: *mut regmmatch_T, _: *mut win_T, _: *mut buf_T, _: linenr_T, _: colnr_T, _: *mut proftime_T, _: *mut i32) -> i64>,
        pub expr: *mut u8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct regmmatch_T {
        pub regprog: *mut regprog_T,
        pub startpos: [lpos_T; 10],
        pub endpos: [lpos_T; 10],
        pub rmm_ic: i32,
        pub rmm_maxcol: colnr_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct regmatch_T {
        pub regprog: *mut regprog_T,
        pub startp: [*mut u8; 10],
        pub endp: [*mut u8; 10],
        pub rm_ic: bool,
    }
    pub type reg_extmatch_T = reg_extmatch;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct reg_extmatch {
        pub refcnt: int16_t,
        pub matches: [*mut u8; 10],
    }
    // NVIM_REGEXP_DEFS_H
}
pub mod profile_h {
    pub type proftime_T = u64;
    // NVIM_PROFILE_H
}
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
    pub type Window = handle_T;
    pub type Boolean = bool;
    pub type Integer = i64;
    pub type Float = libc::c_double;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct String_0 {
        pub data: *mut i8,
        pub size: size_t,
    }
    // EXT types, cannot be split or reordered, see #EXT_OBJECT_TYPE_SHIFT
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct object {
        pub type_0: ObjectType,
        pub data: C2RustUnnamed_14,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
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
    pub struct Dictionary {
        pub items: *mut KeyValuePair,
        pub size: size_t,
        pub capacity: size_t,
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
        pub size: size_t,
        pub capacity: size_t,
    }
    pub type ObjectType = u32;
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
    #[inline(always)]
    pub unsafe extern "C" fn is_internal_call(channel_id: u64) -> bool {
        return channel_id & (1) << (::std::mem::size_of::<u64>() as u64).wrapping_mul(8).wrapping_sub(1) != 0;
    }
    // NVIM_API_PRIVATE_DEFS_H
}
pub mod grid_defs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ScreenGrid {
        pub handle: handle_T,
        pub chars: *mut schar_T,
        pub attrs: *mut sattr_T,
        pub line_offset: *mut u32,
        pub line_wraps: *mut u8,
        pub dirty_col: *mut i32,
        pub Rows: i32,
        pub Columns: i32,
        pub valid: bool,
        pub throttled: bool,
        pub row_offset: i32,
        pub col_offset: i32,
        pub blending: bool,
        pub focusable: bool,
        pub comp_row: i32,
        pub comp_col: i32,
        pub comp_index: size_t,
        pub comp_disabled: bool,
    }
    pub type sattr_T = int16_t;
    pub type schar_T = [u8; 29];
    pub const MAX_MCO: i32 = 6;
    // NVIM_GRID_DEFS_H
}
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
    pub type fmark_T = filemark;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct filemark {
        pub mark: pos_T,
        pub fnum: i32,
        pub timestamp: Timestamp,
        pub additional_data: *mut dict_T,
    }
    // /< Cursor position.
    // /< File number.
    // /< Time when this mark was last set.
    // /< Additional data from ShaDa file.
    // / Structure defining extended mark (mark with file name attached)
    pub type xfmark_T = xfilemark;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xfilemark {
        pub fmark: fmark_T,
        pub fname: *mut u8,
    }
    pub const NMARKS: i32 = 'z' as i32 - 'a' as i32 + 1;
    // /< Actual mark.
    // /< File name, used when fnum == 0.
    // NVIM_MARK_DEFS_H
}
pub mod time_h {
    pub type Timestamp = u64;
    // NVIM_OS_TIME_H
}
pub mod option_defs_h {
    // NVIM_OPTION_DEFS_H
}
pub mod syntax_defs_h {
    /* minimal size for state stack array */
    /* maximal size for state stack array */
    /* size of sst_stack[]. */
    /* normal distance between entries */
    /* invalid syn_state pointer */
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
    pub struct syn_state {
        pub sst_next: *mut synstate_T,
        pub sst_lnum: linenr_T,
        pub sst_union: C2RustUnnamed_7,
        pub sst_next_flags: i32,
        pub sst_stacksize: i32,
        pub sst_next_list: *mut int16_t,
        pub sst_tick: disptick_T,
        pub sst_change_lnum: linenr_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_7 {
        pub sst_stack: [bufstate_T; 7],
        pub sst_ga: garray_T,
    }
    pub type bufstate_T = buf_state;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct buf_state {
        pub bs_idx: i32,
        pub bs_flags: i32,
        pub bs_seqnr: i32,
        pub bs_cchar: i32,
        pub bs_extmatch: *mut reg_extmatch_T,
    }
    // NVIM_SYNTAX_DEFS_H
    // when non-zero, change in this line
    // may have made the state invalid
}
pub mod undo_defs_h {
    // for time_t
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
    pub struct u_header {
        pub uh_next: C2RustUnnamed_12,
        pub uh_prev: C2RustUnnamed_11,
        pub uh_alt_next: C2RustUnnamed_10,
        pub uh_alt_prev: C2RustUnnamed_9,
        pub uh_seq: i64,
        pub uh_walk: i32,
        pub uh_entry: *mut u_entry_T,
        pub uh_getbot_entry: *mut u_entry_T,
        pub uh_cursor: pos_T,
        pub uh_cursor_vcol: i64,
        pub uh_flags: i32,
        pub uh_namedm: [fmark_T; 26],
        pub uh_extmark: extmark_undo_vec_t,
        pub uh_visual: visualinfo_T,
        pub uh_time: time_t,
        pub uh_save_nr: i64,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct visualinfo_T {
        pub vi_start: pos_T,
        pub vi_end: pos_T,
        pub vi_mode: i32,
        pub vi_curswant: colnr_T,
    }
    pub type u_entry_T = u_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct u_entry {
        pub ue_next: *mut u_entry_T,
        pub ue_top: linenr_T,
        pub ue_bot: linenr_T,
        pub ue_lcount: linenr_T,
        pub ue_array: *mut *mut u8,
        pub ue_size: i64,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_9 {
        pub ptr: *mut u_header_T,
        pub seq: i64,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_10 {
        pub ptr: *mut u_header_T,
        pub seq: i64,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_11 {
        pub ptr: *mut u_header_T,
        pub seq: i64,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_12 {
        pub ptr: *mut u_header_T,
        pub seq: i64,
    }
    // NVIM_UNDO_DEFS_H
}
pub mod fs_defs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct FileID {
        pub inode: u64,
        pub device_id: u64,
    }
    // NVIM_OS_FS_DEFS_H
    // non-writable thing (e.g., block device)
    // something we can write to (character
    // device, fifo, socket, ..)
    // file or directory, check with os_isdir()
    // Values returned by os_nodetype()
}
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
    pub type memline_T = memline;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct memline {
        pub ml_line_count: linenr_T,
        pub ml_mfp: *mut memfile_T,
        pub ml_flags: i32,
        pub ml_stack: *mut infoptr_T,
        pub ml_stack_top: i32,
        pub ml_stack_size: i32,
        pub ml_line_lnum: linenr_T,
        pub ml_line_ptr: *mut u8,
        pub ml_locked: *mut bhdr_T,
        pub ml_locked_low: linenr_T,
        pub ml_locked_high: linenr_T,
        pub ml_locked_lineadd: i32,
        pub ml_chunksize: *mut chunksize_T,
        pub ml_numchunks: i32,
        pub ml_usedchunks: i32,
    }
    pub type chunksize_T = ml_chunksize;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ml_chunksize {
        pub mlcs_numlines: i32,
        pub mlcs_totalsize: i64,
    }
    pub type infoptr_T = info_pointer;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct info_pointer {
        pub ip_bnum: blocknr_T,
        pub ip_low: linenr_T,
        pub ip_high: linenr_T,
        pub ip_index: i32,
    }
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
    pub type bhdr_T = bhdr;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct bhdr {
        pub bh_hashitem: mf_hashitem_T,
        pub bh_next: *mut bhdr,
        pub bh_prev: *mut bhdr,
        pub bh_data: *mut libc::c_void,
        pub bh_page_count: u32,
        pub bh_flags: u32,
    }
    pub type mf_hashitem_T = mf_hashitem;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct mf_hashitem {
        pub mhi_next: *mut mf_hashitem,
        pub mhi_prev: *mut mf_hashitem,
        pub mhi_key: blocknr_T,
    }
    pub type blocknr_T = i64;
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
    pub type memfile_T = memfile;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct memfile {
        pub mf_fname: *mut u8,
        pub mf_ffname: *mut u8,
        pub mf_fd: i32,
        pub mf_free_first: *mut bhdr_T,
        pub mf_used_first: *mut bhdr_T,
        pub mf_used_last: *mut bhdr_T,
        pub mf_hash: mf_hashtab_T,
        pub mf_trans: mf_hashtab_T,
        pub mf_blocknr_max: blocknr_T,
        pub mf_blocknr_min: blocknr_T,
        pub mf_neg_count: blocknr_T,
        pub mf_infile_count: blocknr_T,
        pub mf_page_size: u32,
        pub mf_dirty: bool,
    }
    pub type mf_hashtab_T = mf_hashtab;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct mf_hashtab {
        pub mht_mask: size_t,
        pub mht_count: size_t,
        pub mht_buckets: *mut *mut mf_hashitem_T,
        pub mht_small_buckets: [*mut mf_hashitem_T; 64],
    }
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
pub mod globals_h {
    pub type WorkingStatus = u32;
    pub const kBroken: WorkingStatus = 2;
    pub const kWorking: WorkingStatus = 1;
    pub const kUnknown: WorkingStatus = 0;
    pub const IOSIZE: i32 = 1024 + 1;
    // These flags are set based upon 'fileencoding'.
    // Note that "enc_utf8" is also set for "unicode", because the characters are
    // internally stored as UTF-8 (to avoid trouble with NUL bytes).
    // euc-tw
    // 2byte-
    extern "C" {
        // previous window
        // NOLINT
        // When using this macro "break" only breaks out of the inner loop. Use "goto"
        // to break out of the tabpage loop.
        // -V:FOR_ALL_WINDOWS_IN_TAB:501
        #[no_mangle]
        pub static mut curwin: *mut win_T;
        // last buffer
        #[no_mangle]
        pub static mut curbuf: *mut buf_T;
        // mbyte flags that used to depend on 'encoding'. These are now deprecated, as
        // 'encoding' is always "utf-8". Code that use them can be refactored to
        // remove dead code.
        // / Encoding used when 'fencs' is set to "default"
        #[no_mangle]
        pub static mut fenc_default: *mut u8;
        // Selected "quit" at the dialog.
        #[no_mangle]
        pub static mut IObuff: [u8; 1025];
    }
    // NVIM_GLOBALS_H
}
pub mod nl_types_h {
    pub type nl_item = i32;
}
pub mod struct_iovec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct iovec {
        pub iov_base: *mut libc::c_void,
        pub iov_len: size_t,
    }
}
pub mod uv_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv_loop_s {
        pub data: *mut libc::c_void,
        pub active_handles: u32,
        pub handle_queue: [*mut libc::c_void; 2],
        pub active_reqs: C2RustUnnamed_20,
        pub stop_flag: u32,
        pub flags: u64,
        pub backend_fd: i32,
        pub pending_queue: [*mut libc::c_void; 2],
        pub watcher_queue: [*mut libc::c_void; 2],
        pub watchers: *mut *mut uv__io_t,
        pub nwatchers: u32,
        pub nfds: u32,
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
        pub async_wfd: i32,
        pub timer_heap: C2RustUnnamed_18,
        pub timer_counter: u64,
        pub time: u64,
        pub signal_pipefd: [i32; 2],
        pub signal_io_watcher: uv__io_t,
        pub child_watcher: uv_signal_t,
        pub emfile_fd: i32,
        pub inotify_read_watcher: uv__io_t,
        pub inotify_watchers: *mut libc::c_void,
        pub inotify_fd: i32,
    }
    pub type uv_signal_t = uv_signal_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv_signal_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_17,
        pub next_closing: *mut uv_handle_t,
        pub flags: u32,
        pub signal_cb: uv_signal_cb,
        pub signum: i32,
        pub tree_entry: C2RustUnnamed_15,
        pub caught_signals: u32,
        pub dispatched_signals: u32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2RustUnnamed_15 {
        pub rbe_left: *mut uv_signal_s,
        pub rbe_right: *mut uv_signal_s,
        pub rbe_parent: *mut uv_signal_s,
        pub rbe_color: i32,
    }
    pub type uv_signal_cb = Option<unsafe extern "C" fn(_: *mut uv_signal_t, _: i32) -> ()>;
    pub type uv_handle_t = uv_handle_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv_handle_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_16,
        pub next_closing: *mut uv_handle_t,
        pub flags: u32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_16 {
        pub fd: i32,
        pub reserved: [*mut libc::c_void; 4],
    }
    pub type uv_close_cb = Option<unsafe extern "C" fn(_: *mut uv_handle_t) -> ()>;
    pub type uv_handle_type = u32;
    pub const UV_HANDLE_TYPE_MAX: uv_handle_type = 18;
    pub const UV_FILE: uv_handle_type = 17;
    pub const UV_SIGNAL: uv_handle_type = 16;
    pub const UV_UDP: uv_handle_type = 15;
    pub const UV_TTY: uv_handle_type = 14;
    pub const UV_TIMER: uv_handle_type = 13;
    pub const UV_TCP: uv_handle_type = 12;
    pub const UV_STREAM: uv_handle_type = 11;
    pub const UV_PROCESS: uv_handle_type = 10;
    pub const UV_PREPARE: uv_handle_type = 9;
    pub const UV_POLL: uv_handle_type = 8;
    pub const UV_NAMED_PIPE: uv_handle_type = 7;
    pub const UV_IDLE: uv_handle_type = 6;
    pub const UV_HANDLE: uv_handle_type = 5;
    pub const UV_FS_POLL: uv_handle_type = 4;
    pub const UV_FS_EVENT: uv_handle_type = 3;
    pub const UV_CHECK: uv_handle_type = 2;
    pub const UV_ASYNC: uv_handle_type = 1;
    pub const UV_UNKNOWN_HANDLE: uv_handle_type = 0;
    pub type uv_loop_t = uv_loop_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_17 {
        pub fd: i32,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2RustUnnamed_18 {
        pub min: *mut libc::c_void,
        pub nelts: u32,
    }
    pub type uv_async_t = uv_async_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv_async_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_19,
        pub next_closing: *mut uv_handle_t,
        pub flags: u32,
        pub async_cb: uv_async_cb,
        pub queue: [*mut libc::c_void; 2],
        pub pending: i32,
    }
    pub type uv_async_cb = Option<unsafe extern "C" fn(_: *mut uv_async_t) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_19 {
        pub fd: i32,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_20 {
        pub unused: [*mut libc::c_void; 2],
        pub count: u32,
    }
    pub type uv_req_type = u32;
    pub const UV_REQ_TYPE_MAX: uv_req_type = 10;
    pub const UV_GETNAMEINFO: uv_req_type = 9;
    pub const UV_GETADDRINFO: uv_req_type = 8;
    pub const UV_WORK: uv_req_type = 7;
    pub const UV_FS: uv_req_type = 6;
    pub const UV_UDP_SEND: uv_req_type = 5;
    pub const UV_SHUTDOWN: uv_req_type = 4;
    pub const UV_WRITE: uv_req_type = 3;
    pub const UV_CONNECT: uv_req_type = 2;
    pub const UV_REQ: uv_req_type = 1;
    pub const UV_UNKNOWN_REQ: uv_req_type = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv_stream_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_21,
        pub next_closing: *mut uv_handle_t,
        pub flags: u32,
        pub write_queue_size: size_t,
        pub alloc_cb: uv_alloc_cb,
        pub read_cb: uv_read_cb,
        pub connect_req: *mut uv_connect_t,
        pub shutdown_req: *mut uv_shutdown_t,
        pub io_watcher: uv__io_t,
        pub write_queue: [*mut libc::c_void; 2],
        pub write_completed_queue: [*mut libc::c_void; 2],
        pub connection_cb: uv_connection_cb,
        pub delayed_error: i32,
        pub accepted_fd: i32,
        pub queued_fds: *mut libc::c_void,
    }
    pub type uv_connection_cb = Option<unsafe extern "C" fn(_: *mut uv_stream_t, _: i32) -> ()>;
    pub type uv_stream_t = uv_stream_s;
    pub type uv_shutdown_t = uv_shutdown_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv_shutdown_s {
        pub data: *mut libc::c_void,
        pub type_0: uv_req_type,
        pub reserved: [*mut libc::c_void; 6],
        pub handle: *mut uv_stream_t,
        pub cb: uv_shutdown_cb,
    }
    pub type uv_shutdown_cb = Option<unsafe extern "C" fn(_: *mut uv_shutdown_t, _: i32) -> ()>;
    pub type uv_connect_t = uv_connect_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv_connect_s {
        pub data: *mut libc::c_void,
        pub type_0: uv_req_type,
        pub reserved: [*mut libc::c_void; 6],
        pub cb: uv_connect_cb,
        pub handle: *mut uv_stream_t,
        pub queue: [*mut libc::c_void; 2],
    }
    pub type uv_connect_cb = Option<unsafe extern "C" fn(_: *mut uv_connect_t, _: i32) -> ()>;
    pub type uv_read_cb = Option<unsafe extern "C" fn(_: *mut uv_stream_t, _: ssize_t, _: *const uv_buf_t) -> ()>;
    pub type uv_alloc_cb = Option<unsafe extern "C" fn(_: *mut uv_handle_t, _: size_t, _: *mut uv_buf_t) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_21 {
        pub fd: i32,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv_tcp_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_22,
        pub next_closing: *mut uv_handle_t,
        pub flags: u32,
        pub write_queue_size: size_t,
        pub alloc_cb: uv_alloc_cb,
        pub read_cb: uv_read_cb,
        pub connect_req: *mut uv_connect_t,
        pub shutdown_req: *mut uv_shutdown_t,
        pub io_watcher: uv__io_t,
        pub write_queue: [*mut libc::c_void; 2],
        pub write_completed_queue: [*mut libc::c_void; 2],
        pub connection_cb: uv_connection_cb,
        pub delayed_error: i32,
        pub accepted_fd: i32,
        pub queued_fds: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_22 {
        pub fd: i32,
        pub reserved: [*mut libc::c_void; 4],
    }
    pub type uv_tcp_t = uv_tcp_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv_pipe_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_23,
        pub next_closing: *mut uv_handle_t,
        pub flags: u32,
        pub write_queue_size: size_t,
        pub alloc_cb: uv_alloc_cb,
        pub read_cb: uv_read_cb,
        pub connect_req: *mut uv_connect_t,
        pub shutdown_req: *mut uv_shutdown_t,
        pub io_watcher: uv__io_t,
        pub write_queue: [*mut libc::c_void; 2],
        pub write_completed_queue: [*mut libc::c_void; 2],
        pub connection_cb: uv_connection_cb,
        pub delayed_error: i32,
        pub accepted_fd: i32,
        pub queued_fds: *mut libc::c_void,
        pub ipc: i32,
        pub pipe_fname: *const i8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_23 {
        pub fd: i32,
        pub reserved: [*mut libc::c_void; 4],
    }
    pub type uv_pipe_t = uv_pipe_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv_timer_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_24,
        pub next_closing: *mut uv_handle_t,
        pub flags: u32,
        pub timer_cb: uv_timer_cb,
        pub heap_node: [*mut libc::c_void; 3],
        pub timeout: u64,
        pub repeat: u64,
        pub start_id: u64,
    }
    pub type uv_timer_cb = Option<unsafe extern "C" fn(_: *mut uv_timer_t) -> ()>;
    pub type uv_timer_t = uv_timer_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_24 {
        pub fd: i32,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv_idle_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_25,
        pub next_closing: *mut uv_handle_t,
        pub flags: u32,
        pub idle_cb: uv_idle_cb,
        pub queue: [*mut libc::c_void; 2],
    }
    pub type uv_idle_cb = Option<unsafe extern "C" fn(_: *mut uv_idle_t) -> ()>;
    pub type uv_idle_t = uv_idle_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_25 {
        pub fd: i32,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv_process_s {
        pub data: *mut libc::c_void,
        pub loop_0: *mut uv_loop_t,
        pub type_0: uv_handle_type,
        pub close_cb: uv_close_cb,
        pub handle_queue: [*mut libc::c_void; 2],
        pub u: C2RustUnnamed_26,
        pub next_closing: *mut uv_handle_t,
        pub flags: u32,
        pub exit_cb: uv_exit_cb,
        pub pid: i32,
        pub queue: [*mut libc::c_void; 2],
        pub status: i32,
    }
    pub type uv_exit_cb = Option<unsafe extern "C" fn(_: *mut uv_process_t, _: i64, _: i32) -> ()>;
    pub type uv_process_t = uv_process_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_26 {
        pub fd: i32,
        pub reserved: [*mut libc::c_void; 4],
    }
    pub type uv_stdio_flags = u32;
    pub const UV_OVERLAPPED_PIPE: uv_stdio_flags = 64;
    pub const UV_WRITABLE_PIPE: uv_stdio_flags = 32;
    pub const UV_READABLE_PIPE: uv_stdio_flags = 16;
    pub const UV_INHERIT_STREAM: uv_stdio_flags = 4;
    pub const UV_INHERIT_FD: uv_stdio_flags = 2;
    pub const UV_CREATE_PIPE: uv_stdio_flags = 1;
    pub const UV_IGNORE: uv_stdio_flags = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv_stdio_container_s {
        pub flags: uv_stdio_flags,
        pub data: C2RustUnnamed_27,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_27 {
        pub stream: *mut uv_stream_t,
        pub fd: i32,
    }
    pub type uv_stdio_container_t = uv_stdio_container_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv_process_options_s {
        pub exit_cb: uv_exit_cb,
        pub file: *const i8,
        pub args: *mut *mut i8,
        pub env: *mut *mut i8,
        pub cwd: *const i8,
        pub flags: u32,
        pub stdio_count: i32,
        pub stdio: *mut uv_stdio_container_t,
        pub uid: uv_uid_t,
        pub gid: uv_gid_t,
    }
    pub type uv_process_options_t = uv_process_options_s;
}
pub mod unix_h {
    pub type uv__io_t = uv__io_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv__io_s {
        pub cb: uv__io_cb,
        pub pending_queue: [*mut libc::c_void; 2],
        pub watcher_queue: [*mut libc::c_void; 2],
        pub pevents: u32,
        pub events: u32,
        pub fd: i32,
    }
    pub type uv__io_cb = Option<unsafe extern "C" fn(_: *mut uv_loop_s, _: *mut uv__io_s, _: u32) -> ()>;
    pub type uv_rwlock_t = pthread_rwlock_t;
    pub type uv_mutex_t = pthread_mutex_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct uv_buf_t {
        pub base: *mut i8,
        pub len: size_t,
    }
    pub type uv_file = i32;
    pub type uv_gid_t = gid_t;
    pub type uv_uid_t = uid_t;
}
pub mod event_defs_h {
    pub type argv_callback = Option<unsafe extern "C" fn(_: *mut *mut libc::c_void) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct message {
        pub handler: argv_callback,
        pub argv: [*mut libc::c_void; 10],
    }
    pub type Event = message;
    #[inline]
    pub unsafe extern "C" fn event_create(mut cb: argv_callback, mut argc: i32, mut args: ...) -> Event {
        if argc <= 10 {
        } else {
            assert!(false, "argc <= EVENT_HANDLER_MAX_ARGC");
        }
        let mut event = Event { handler: None, argv: [0 as *mut libc::c_void; 10] };
        if argc <= 10 {
        } else {
            assert!(false, "argc <= EVENT_HANDLER_MAX_ARGC");
        }
        event.handler = cb;
        if argc != 0 {
            let mut args_0: ::std::ffi::VaListImpl;
            args_0 = args.clone();
            let mut i = 0;
            while i < argc {
                event.argv[i as usize] = args_0.arg::<*mut libc::c_void>();
                i += 1
            }
        }
        return event;
    }
    // NVIM_EVENT_DEFS_H
}
pub mod multiqueue_h {
    pub type MultiQueue = multiqueue;
    extern "C" {
        pub type multiqueue;
    }
    // NVIM_EVENT_MULTIQUEUE_H
}
pub mod loop_h {
    pub type WatcherPtr = *mut libc::c_void;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __kl1_WatcherPtr {
        pub data: WatcherPtr,
        pub next: *mut __kl1_WatcherPtr,
    }
    pub type kl1_WatcherPtr = __kl1_WatcherPtr;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct kmp_WatcherPtr_t {
        pub cnt: size_t,
        pub n: size_t,
        pub max: size_t,
        pub buf: *mut *mut kl1_WatcherPtr,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct kl_WatcherPtr_t {
        pub head: *mut kl1_WatcherPtr,
        pub tail: *mut kl1_WatcherPtr,
        pub mp: *mut kmp_WatcherPtr_t,
        pub size: size_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
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
        pub recursive: i32,
    }
    pub type Loop = loop_0;
    #[inline]
    pub unsafe extern "C" fn kmp_init_WatcherPtr() -> *mut kmp_WatcherPtr_t {
        return xcalloc(1, ::std::mem::size_of::<kmp_WatcherPtr_t>() as u64) as *mut kmp_WatcherPtr_t;
    }
    #[inline]
    pub unsafe extern "C" fn kmp_destroy_WatcherPtr(mut mp: *mut kmp_WatcherPtr_t) {
        let mut k: size_t = 0;
        k = 0;
        while k < (*mp).n {
            let mut ptr_ = &mut *(*mp).buf.offset(k as isize) as *mut *mut kl1_WatcherPtr as *mut *mut libc::c_void;
            xfree(*ptr_);
            *ptr_ = NULL_2 as *mut libc::c_void;
            k = k.wrapping_add(1)
        }
        let mut ptr__0 = &mut (*mp).buf as *mut *mut *mut kl1_WatcherPtr as *mut *mut libc::c_void;
        xfree(*ptr__0);
        *ptr__0 = NULL_2 as *mut libc::c_void;
        let mut ptr__1 = &mut mp as *mut *mut kmp_WatcherPtr_t as *mut *mut libc::c_void;
        xfree(*ptr__1);
        *ptr__1 = NULL_2 as *mut libc::c_void;
    }
    #[inline]
    pub unsafe extern "C" fn kmp_alloc_WatcherPtr(mut mp: *mut kmp_WatcherPtr_t) -> *mut kl1_WatcherPtr {
        (*mp).cnt = (*mp).cnt.wrapping_add(1);
        if (*mp).n == 0 {
            return xcalloc(1, ::std::mem::size_of::<kl1_WatcherPtr>() as u64) as *mut kl1_WatcherPtr;
        }
        (*mp).n = (*mp).n.wrapping_sub(1);
        return *(*mp).buf.offset((*mp).n as isize);
    }
    #[inline]
    pub unsafe extern "C" fn kmp_free_WatcherPtr(mut mp: *mut kmp_WatcherPtr_t, mut p: *mut kl1_WatcherPtr) {
        (*mp).cnt = (*mp).cnt.wrapping_sub(1);
        if (*mp).n == (*mp).max {
            (*mp).max = if (*mp).max != 0 { ((*mp).max) << 1 } else { 16 };
            (*mp).buf = xrealloc((*mp).buf as *mut libc::c_void, (::std::mem::size_of::<*mut kl1_WatcherPtr>() as u64).wrapping_mul((*mp).max)) as *mut *mut kl1_WatcherPtr
        }
        let fresh1 = (*mp).n;
        (*mp).n = (*mp).n.wrapping_add(1);
        let ref mut fresh2 = *(*mp).buf.offset(fresh1 as isize);
        *fresh2 = p;
    }
    #[inline]
    pub unsafe extern "C" fn kl_init_WatcherPtr() -> *mut kl_WatcherPtr_t {
        let mut kl = xcalloc(1, ::std::mem::size_of::<kl_WatcherPtr_t>() as u64) as *mut kl_WatcherPtr_t;
        (*kl).mp = kmp_init_WatcherPtr();
        (*kl).tail = kmp_alloc_WatcherPtr((*kl).mp);
        (*kl).head = (*kl).tail;
        (*(*kl).head).next = 0 as *mut __kl1_WatcherPtr;
        return kl;
    }
    #[inline]
    pub unsafe extern "C" fn kl_destroy_WatcherPtr(mut kl: *mut kl_WatcherPtr_t) {
        let mut p = 0 as *mut kl1_WatcherPtr;
        p = (*kl).head;
        while p != (*kl).tail {
            kmp_free_WatcherPtr((*kl).mp, p);
            p = (*p).next
        }
        kmp_free_WatcherPtr((*kl).mp, p);
        kmp_destroy_WatcherPtr((*kl).mp);
        let mut ptr_ = &mut kl as *mut *mut kl_WatcherPtr_t as *mut *mut libc::c_void;
        xfree(*ptr_);
        *ptr_ = NULL_2 as *mut libc::c_void;
    }
    #[inline]
    pub unsafe extern "C" fn kl_push_WatcherPtr(mut kl: *mut kl_WatcherPtr_t, mut d: WatcherPtr) {
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
    pub unsafe extern "C" fn kl_shift_at_WatcherPtr(mut kl: *mut kl_WatcherPtr_t, mut n: *mut *mut kl1_WatcherPtr) -> WatcherPtr {
        if !(**n).next.is_null() {
        } else {
            assert!(false, "(*n)->next");
        }
        let mut p = 0 as *mut kl1_WatcherPtr;
        (*kl).size = (*kl).size.wrapping_sub(1);
        p = *n;
        *n = (**n).next;
        if p == (*kl).head {
            (*kl).head = *n
        }
        let mut d = (*p).data;
        kmp_free_WatcherPtr((*kl).mp, p);
        return d;
    }
    // NVIM_EVENT_LOOP_H
}
pub mod rbuffer_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct rbuffer {
        pub full_cb: rbuffer_callback,
        pub nonfull_cb: rbuffer_callback,
        pub data: *mut libc::c_void,
        pub size: size_t,
        pub temp: *mut i8,
        pub end_ptr: *mut i8,
        pub read_ptr: *mut i8,
        pub write_ptr: *mut i8,
        pub start_ptr: [i8; 0],
    }
    pub type rbuffer_callback = Option<unsafe extern "C" fn(_: *mut RBuffer, _: *mut libc::c_void) -> ()>;
    pub type RBuffer = rbuffer;
    extern "C" {
        #[no_mangle]
        pub fn rbuffer_size(buf: *mut RBuffer) -> size_t;
    }
    // NVIM_RBUFFER_H
}
pub mod stream_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct stream {
        pub closed: bool,
        pub did_eof: bool,
        pub uv: C2RustUnnamed_28,
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
    pub type stream_close_cb = Option<unsafe extern "C" fn(_: *mut Stream, _: *mut libc::c_void) -> ()>;
    pub type Stream = stream;
    pub type stream_write_cb = Option<unsafe extern "C" fn(_: *mut Stream, _: *mut libc::c_void, _: i32) -> ()>;
    pub type stream_read_cb = Option<unsafe extern "C" fn(_: *mut Stream, _: *mut RBuffer, _: size_t, _: *mut libc::c_void, _: bool) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_28 {
        pub pipe: uv_pipe_t,
        pub tcp: uv_tcp_t,
        pub idle: uv_idle_t,
    }
    // NVIM_EVENT_STREAM_H
}
pub mod process_h {
    pub type ProcessType = u32;
    pub const kProcessTypePty: ProcessType = 1;
    pub const kProcessTypeUv: ProcessType = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct process {
        pub type_0: ProcessType,
        pub loop_0: *mut Loop,
        pub data: *mut libc::c_void,
        pub pid: i32,
        pub status: i32,
        pub refcount: i32,
        pub exit_signal: u8,
        pub stopped_time: u64,
        pub cwd: *const i8,
        pub argv: *mut *mut i8,
        pub env: *mut *mut i8,
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
    pub type internal_process_cb = Option<unsafe extern "C" fn(_: *mut Process) -> ()>;
    pub type Process = process;
    pub type process_exit_cb = Option<unsafe extern "C" fn(_: *mut Process, _: i32, _: *mut libc::c_void) -> ()>;
    #[inline]
    pub unsafe extern "C" fn process_init(mut loop_0: *mut Loop, mut type_0: ProcessType, mut data: *mut libc::c_void) -> Process {
        return {
            let mut init = process {
                type_0: type_0,
                loop_0: loop_0,
                data: data,
                pid: 0,
                status: -(1),
                refcount: 0,
                exit_signal: 0,
                stopped_time: 0,
                cwd: NULL_0 as *const i8,
                argv: NULL_0 as *mut *mut i8,
                env: 0 as *mut *mut i8,
                in_0: {
                    let mut init = stream {
                        closed: false,
                        did_eof: false,
                        uv: C2RustUnnamed_28 {
                            pipe: uv_pipe_t {
                                data: 0 as *mut libc::c_void,
                                loop_0: 0 as *mut uv_loop_t,
                                type_0: UV_UNKNOWN_HANDLE,
                                close_cb: None,
                                handle_queue: [0 as *mut libc::c_void; 2],
                                u: C2RustUnnamed_23 { fd: 0 },
                                next_closing: 0 as *mut uv_handle_t,
                                flags: 0,
                                write_queue_size: 0,
                                alloc_cb: None,
                                read_cb: None,
                                connect_req: 0 as *mut uv_connect_t,
                                shutdown_req: 0 as *mut uv_shutdown_t,
                                io_watcher: uv__io_t {
                                    cb: None,
                                    pending_queue: [0 as *mut libc::c_void; 2],
                                    watcher_queue: [0 as *mut libc::c_void; 2],
                                    pevents: 0,
                                    events: 0,
                                    fd: 0,
                                },
                                write_queue: [0 as *mut libc::c_void; 2],
                                write_completed_queue: [0 as *mut libc::c_void; 2],
                                connection_cb: None,
                                delayed_error: 0,
                                accepted_fd: 0,
                                queued_fds: 0 as *mut libc::c_void,
                                ipc: 0,
                                pipe_fname: 0 as *const i8,
                            },
                        },
                        uvstream: 0 as *mut uv_stream_t,
                        uvbuf: uv_buf_t { base: 0 as *mut i8, len: 0 },
                        buffer: 0 as *mut RBuffer,
                        fd: 0,
                        read_cb: None,
                        write_cb: None,
                        cb_data: 0 as *mut libc::c_void,
                        close_cb: None,
                        internal_close_cb: None,
                        close_cb_data: 0 as *mut libc::c_void,
                        internal_data: 0 as *mut libc::c_void,
                        fpos: 0,
                        curmem: 0,
                        maxmem: 0,
                        pending_reqs: 0,
                        num_bytes: 0,
                        events: 0 as *mut MultiQueue,
                    };
                    init
                },
                out: {
                    let mut init = stream {
                        closed: false,
                        did_eof: false,
                        uv: C2RustUnnamed_28 {
                            pipe: uv_pipe_t {
                                data: 0 as *mut libc::c_void,
                                loop_0: 0 as *mut uv_loop_t,
                                type_0: UV_UNKNOWN_HANDLE,
                                close_cb: None,
                                handle_queue: [0 as *mut libc::c_void; 2],
                                u: C2RustUnnamed_23 { fd: 0 },
                                next_closing: 0 as *mut uv_handle_t,
                                flags: 0,
                                write_queue_size: 0,
                                alloc_cb: None,
                                read_cb: None,
                                connect_req: 0 as *mut uv_connect_t,
                                shutdown_req: 0 as *mut uv_shutdown_t,
                                io_watcher: uv__io_t {
                                    cb: None,
                                    pending_queue: [0 as *mut libc::c_void; 2],
                                    watcher_queue: [0 as *mut libc::c_void; 2],
                                    pevents: 0,
                                    events: 0,
                                    fd: 0,
                                },
                                write_queue: [0 as *mut libc::c_void; 2],
                                write_completed_queue: [0 as *mut libc::c_void; 2],
                                connection_cb: None,
                                delayed_error: 0,
                                accepted_fd: 0,
                                queued_fds: 0 as *mut libc::c_void,
                                ipc: 0,
                                pipe_fname: 0 as *const i8,
                            },
                        },
                        uvstream: 0 as *mut uv_stream_t,
                        uvbuf: uv_buf_t { base: 0 as *mut i8, len: 0 },
                        buffer: 0 as *mut RBuffer,
                        fd: 0,
                        read_cb: None,
                        write_cb: None,
                        cb_data: 0 as *mut libc::c_void,
                        close_cb: None,
                        internal_close_cb: None,
                        close_cb_data: 0 as *mut libc::c_void,
                        internal_data: 0 as *mut libc::c_void,
                        fpos: 0,
                        curmem: 0,
                        maxmem: 0,
                        pending_reqs: 0,
                        num_bytes: 0,
                        events: 0 as *mut MultiQueue,
                    };
                    init
                },
                err: {
                    let mut init = stream {
                        closed: false,
                        did_eof: false,
                        uv: C2RustUnnamed_28 {
                            pipe: uv_pipe_t {
                                data: 0 as *mut libc::c_void,
                                loop_0: 0 as *mut uv_loop_t,
                                type_0: UV_UNKNOWN_HANDLE,
                                close_cb: None,
                                handle_queue: [0 as *mut libc::c_void; 2],
                                u: C2RustUnnamed_23 { fd: 0 },
                                next_closing: 0 as *mut uv_handle_t,
                                flags: 0,
                                write_queue_size: 0,
                                alloc_cb: None,
                                read_cb: None,
                                connect_req: 0 as *mut uv_connect_t,
                                shutdown_req: 0 as *mut uv_shutdown_t,
                                io_watcher: uv__io_t {
                                    cb: None,
                                    pending_queue: [0 as *mut libc::c_void; 2],
                                    watcher_queue: [0 as *mut libc::c_void; 2],
                                    pevents: 0,
                                    events: 0,
                                    fd: 0,
                                },
                                write_queue: [0 as *mut libc::c_void; 2],
                                write_completed_queue: [0 as *mut libc::c_void; 2],
                                connection_cb: None,
                                delayed_error: 0,
                                accepted_fd: 0,
                                queued_fds: 0 as *mut libc::c_void,
                                ipc: 0,
                                pipe_fname: 0 as *const i8,
                            },
                        },
                        uvstream: 0 as *mut uv_stream_t,
                        uvbuf: uv_buf_t { base: 0 as *mut i8, len: 0 },
                        buffer: 0 as *mut RBuffer,
                        fd: 0,
                        read_cb: None,
                        write_cb: None,
                        cb_data: 0 as *mut libc::c_void,
                        close_cb: None,
                        internal_close_cb: None,
                        close_cb_data: 0 as *mut libc::c_void,
                        internal_data: 0 as *mut libc::c_void,
                        fpos: 0,
                        curmem: 0,
                        maxmem: 0,
                        pending_reqs: 0,
                        num_bytes: 0,
                        events: 0 as *mut MultiQueue,
                    };
                    init
                },
                cb: ::std::mem::transmute::<libc::intptr_t, process_exit_cb>(NULL_0 as libc::intptr_t),
                internal_exit_cb: ::std::mem::transmute::<libc::intptr_t, internal_process_cb>(NULL_0 as libc::intptr_t),
                internal_close_cb: ::std::mem::transmute::<libc::intptr_t, internal_process_cb>(NULL_0 as libc::intptr_t),
                closed: false,
                detach: false,
                events: NULL_0 as *mut MultiQueue,
            };
            init
        };
    }
    #[inline]
    pub unsafe extern "C" fn process_is_stopped(mut proc_0: *mut Process) -> bool {
        let mut exited = (*proc_0).status >= 0;
        return exited as i32 != 0 || (*proc_0).stopped_time != 0;
    }
    // NVIM_EVENT_PROCESS_H
}
pub mod ioctl_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct winsize {
        pub ws_row: u16,
        pub ws_col: u16,
        pub ws_xpixel: u16,
        pub ws_ypixel: u16,
    }
}
pub mod pty_process_unix_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct pty_process {
        pub process: Process,
        pub term_name: *mut i8,
        pub width: uint16_t,
        pub height: uint16_t,
        pub winsize: winsize,
        pub tty_fd: i32,
    }
    pub type PtyProcess = pty_process;
    #[inline]
    pub unsafe extern "C" fn pty_process_init(mut loop_0: *mut Loop, mut data: *mut libc::c_void) -> PtyProcess {
        let mut rv = PtyProcess {
            process: Process {
                type_0: kProcessTypeUv,
                loop_0: 0 as *mut Loop,
                data: 0 as *mut libc::c_void,
                pid: 0,
                status: 0,
                refcount: 0,
                exit_signal: 0,
                stopped_time: 0,
                cwd: 0 as *const i8,
                argv: 0 as *mut *mut i8,
                env: 0 as *mut *mut i8,
                in_0: Stream {
                    closed: false,
                    did_eof: false,
                    uv: C2RustUnnamed_28 {
                        pipe: uv_pipe_t {
                            data: 0 as *mut libc::c_void,
                            loop_0: 0 as *mut uv_loop_t,
                            type_0: UV_UNKNOWN_HANDLE,
                            close_cb: None,
                            handle_queue: [0 as *mut libc::c_void; 2],
                            u: C2RustUnnamed_23 { fd: 0 },
                            next_closing: 0 as *mut uv_handle_t,
                            flags: 0,
                            write_queue_size: 0,
                            alloc_cb: None,
                            read_cb: None,
                            connect_req: 0 as *mut uv_connect_t,
                            shutdown_req: 0 as *mut uv_shutdown_t,
                            io_watcher: uv__io_t {
                                cb: None,
                                pending_queue: [0 as *mut libc::c_void; 2],
                                watcher_queue: [0 as *mut libc::c_void; 2],
                                pevents: 0,
                                events: 0,
                                fd: 0,
                            },
                            write_queue: [0 as *mut libc::c_void; 2],
                            write_completed_queue: [0 as *mut libc::c_void; 2],
                            connection_cb: None,
                            delayed_error: 0,
                            accepted_fd: 0,
                            queued_fds: 0 as *mut libc::c_void,
                            ipc: 0,
                            pipe_fname: 0 as *const i8,
                        },
                    },
                    uvstream: 0 as *mut uv_stream_t,
                    uvbuf: uv_buf_t { base: 0 as *mut i8, len: 0 },
                    buffer: 0 as *mut RBuffer,
                    fd: 0,
                    read_cb: None,
                    write_cb: None,
                    cb_data: 0 as *mut libc::c_void,
                    close_cb: None,
                    internal_close_cb: None,
                    close_cb_data: 0 as *mut libc::c_void,
                    internal_data: 0 as *mut libc::c_void,
                    fpos: 0,
                    curmem: 0,
                    maxmem: 0,
                    pending_reqs: 0,
                    num_bytes: 0,
                    events: 0 as *mut MultiQueue,
                },
                out: Stream {
                    closed: false,
                    did_eof: false,
                    uv: C2RustUnnamed_28 {
                        pipe: uv_pipe_t {
                            data: 0 as *mut libc::c_void,
                            loop_0: 0 as *mut uv_loop_t,
                            type_0: UV_UNKNOWN_HANDLE,
                            close_cb: None,
                            handle_queue: [0 as *mut libc::c_void; 2],
                            u: C2RustUnnamed_23 { fd: 0 },
                            next_closing: 0 as *mut uv_handle_t,
                            flags: 0,
                            write_queue_size: 0,
                            alloc_cb: None,
                            read_cb: None,
                            connect_req: 0 as *mut uv_connect_t,
                            shutdown_req: 0 as *mut uv_shutdown_t,
                            io_watcher: uv__io_t {
                                cb: None,
                                pending_queue: [0 as *mut libc::c_void; 2],
                                watcher_queue: [0 as *mut libc::c_void; 2],
                                pevents: 0,
                                events: 0,
                                fd: 0,
                            },
                            write_queue: [0 as *mut libc::c_void; 2],
                            write_completed_queue: [0 as *mut libc::c_void; 2],
                            connection_cb: None,
                            delayed_error: 0,
                            accepted_fd: 0,
                            queued_fds: 0 as *mut libc::c_void,
                            ipc: 0,
                            pipe_fname: 0 as *const i8,
                        },
                    },
                    uvstream: 0 as *mut uv_stream_t,
                    uvbuf: uv_buf_t { base: 0 as *mut i8, len: 0 },
                    buffer: 0 as *mut RBuffer,
                    fd: 0,
                    read_cb: None,
                    write_cb: None,
                    cb_data: 0 as *mut libc::c_void,
                    close_cb: None,
                    internal_close_cb: None,
                    close_cb_data: 0 as *mut libc::c_void,
                    internal_data: 0 as *mut libc::c_void,
                    fpos: 0,
                    curmem: 0,
                    maxmem: 0,
                    pending_reqs: 0,
                    num_bytes: 0,
                    events: 0 as *mut MultiQueue,
                },
                err: Stream {
                    closed: false,
                    did_eof: false,
                    uv: C2RustUnnamed_28 {
                        pipe: uv_pipe_t {
                            data: 0 as *mut libc::c_void,
                            loop_0: 0 as *mut uv_loop_t,
                            type_0: UV_UNKNOWN_HANDLE,
                            close_cb: None,
                            handle_queue: [0 as *mut libc::c_void; 2],
                            u: C2RustUnnamed_23 { fd: 0 },
                            next_closing: 0 as *mut uv_handle_t,
                            flags: 0,
                            write_queue_size: 0,
                            alloc_cb: None,
                            read_cb: None,
                            connect_req: 0 as *mut uv_connect_t,
                            shutdown_req: 0 as *mut uv_shutdown_t,
                            io_watcher: uv__io_t {
                                cb: None,
                                pending_queue: [0 as *mut libc::c_void; 2],
                                watcher_queue: [0 as *mut libc::c_void; 2],
                                pevents: 0,
                                events: 0,
                                fd: 0,
                            },
                            write_queue: [0 as *mut libc::c_void; 2],
                            write_completed_queue: [0 as *mut libc::c_void; 2],
                            connection_cb: None,
                            delayed_error: 0,
                            accepted_fd: 0,
                            queued_fds: 0 as *mut libc::c_void,
                            ipc: 0,
                            pipe_fname: 0 as *const i8,
                        },
                    },
                    uvstream: 0 as *mut uv_stream_t,
                    uvbuf: uv_buf_t { base: 0 as *mut i8, len: 0 },
                    buffer: 0 as *mut RBuffer,
                    fd: 0,
                    read_cb: None,
                    write_cb: None,
                    cb_data: 0 as *mut libc::c_void,
                    close_cb: None,
                    internal_close_cb: None,
                    close_cb_data: 0 as *mut libc::c_void,
                    internal_data: 0 as *mut libc::c_void,
                    fpos: 0,
                    curmem: 0,
                    maxmem: 0,
                    pending_reqs: 0,
                    num_bytes: 0,
                    events: 0 as *mut MultiQueue,
                },
                cb: None,
                internal_exit_cb: None,
                internal_close_cb: None,
                closed: false,
                detach: false,
                events: 0 as *mut MultiQueue,
            },
            term_name: 0 as *mut i8,
            width: 0,
            height: 0,
            winsize: winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 },
            tty_fd: 0,
        };
        rv.process = process_init(loop_0, kProcessTypePty, data);
        rv.term_name = NULL_0 as *mut i8;
        rv.width = 80;
        rv.height = 24;
        rv.tty_fd = -(1);
        return rv;
    }
    // NVIM_OS_PTY_PROCESS_UNIX_H
}
pub mod libuv_process_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct libuv_process {
        pub process: Process,
        pub uv: uv_process_t,
        pub uvopts: uv_process_options_t,
        pub uvstdio: [uv_stdio_container_t; 3],
    }
    pub type LibuvProcess = libuv_process;
    #[inline]
    pub unsafe extern "C" fn libuv_process_init(mut loop_0: *mut Loop, mut data: *mut libc::c_void) -> LibuvProcess {
        let mut rv = {
            let mut init = libuv_process {
                process: process_init(loop_0, kProcessTypeUv, data),
                uv: uv_process_t {
                    data: 0 as *mut libc::c_void,
                    loop_0: 0 as *mut uv_loop_t,
                    type_0: UV_UNKNOWN_HANDLE,
                    close_cb: None,
                    handle_queue: [0 as *mut libc::c_void; 2],
                    u: C2RustUnnamed_26 { fd: 0 },
                    next_closing: 0 as *mut uv_handle_t,
                    flags: 0,
                    exit_cb: None,
                    pid: 0,
                    queue: [0 as *mut libc::c_void; 2],
                    status: 0,
                },
                uvopts: uv_process_options_t {
                    exit_cb: None,
                    file: 0 as *const i8,
                    args: 0 as *mut *mut i8,
                    env: 0 as *mut *mut i8,
                    cwd: 0 as *const i8,
                    flags: 0,
                    stdio_count: 0,
                    stdio: 0 as *mut uv_stdio_container_t,
                    uid: 0,
                    gid: 0,
                },
                uvstdio: [uv_stdio_container_t {
                    flags: UV_IGNORE,
                    data: C2RustUnnamed_27 { stream: 0 as *mut uv_stream_t },
                }; 3],
            };
            init
        };
        return rv;
    }
    // NVIM_EVENT_LIBUV_PROCESS_H
}
pub mod zone_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_zone_finalizer {
        pub func: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub data: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_zone_finalizer_array {
        pub tail: *mut msgpack_zone_finalizer,
        pub end: *mut msgpack_zone_finalizer,
        pub array: *mut msgpack_zone_finalizer,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_zone_chunk_list {
        pub free: size_t,
        pub ptr: *mut i8,
        pub head: *mut msgpack_zone_chunk,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_zone {
        pub chunk_list: msgpack_zone_chunk_list,
        pub finalizer_array: msgpack_zone_finalizer_array,
        pub chunk_size: size_t,
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_zone_malloc(mut zone: *mut msgpack_zone, mut size: size_t) -> *mut libc::c_void {
        let mut aligned = ((*zone).chunk_list.ptr.offset(MSGPACK_ZONE_ALIGN.wrapping_sub(1) as isize) as size_t).wrapping_div(MSGPACK_ZONE_ALIGN).wrapping_mul(MSGPACK_ZONE_ALIGN) as *mut i8;
        let mut adjusted_size = size.wrapping_add(aligned.offset_from((*zone).chunk_list.ptr) as i64 as u64);
        if (*zone).chunk_list.free >= adjusted_size {
            (*zone).chunk_list.free = ((*zone).chunk_list.free as u64).wrapping_sub(adjusted_size) as size_t as size_t;
            (*zone).chunk_list.ptr = (*zone).chunk_list.ptr.offset(adjusted_size as isize);
            return aligned as *mut libc::c_void;
        }
        let mut ptr = msgpack_zone_malloc_expand(zone, size.wrapping_add(MSGPACK_ZONE_ALIGN.wrapping_sub(1)));
        if !ptr.is_null() {
            return (ptr as size_t).wrapping_div(MSGPACK_ZONE_ALIGN).wrapping_mul(MSGPACK_ZONE_ALIGN) as *mut i8 as *mut libc::c_void;
        }
        return NULL_0 as *mut libc::c_void;
    }
    pub const MSGPACK_ZONE_ALIGN: u64 = ::std::mem::size_of::<*mut libc::c_void>() as u64;
    #[inline]
    pub unsafe extern "C" fn msgpack_zone_malloc_no_align(mut zone: *mut msgpack_zone, mut size: size_t) -> *mut libc::c_void {
        let mut ptr = 0 as *mut i8;
        let mut cl: *mut msgpack_zone_chunk_list = &mut (*zone).chunk_list;
        if (*zone).chunk_list.free < size {
            return msgpack_zone_malloc_expand(zone, size);
        }
        ptr = (*cl).ptr;
        (*cl).free = ((*cl).free as u64).wrapping_sub(size) as size_t as size_t;
        (*cl).ptr = (*cl).ptr.offset(size as isize);
        return ptr as *mut libc::c_void;
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_zone_push_finalizer(mut zone: *mut msgpack_zone, mut func: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>, mut data: *mut libc::c_void) -> bool {
        let fa: *mut msgpack_zone_finalizer_array = &mut (*zone).finalizer_array;
        let mut fin = (*fa).tail;
        if fin == (*fa).end {
            return msgpack_zone_push_finalizer_expand(zone, func, data);
        }
        (*fin).func = func;
        (*fin).data = data;
        (*fa).tail = (*fa).tail.offset(1);
        return true;
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_zone_swap(mut a: *mut msgpack_zone, mut b: *mut msgpack_zone) {
        let mut tmp = *a;
        *a = *b;
        *b = tmp;
    }
    extern "C" {
        pub type msgpack_zone_chunk;
        #[no_mangle]
        pub fn msgpack_zone_free(zone: *mut msgpack_zone);
        #[no_mangle]
        pub fn msgpack_zone_malloc_expand(zone: *mut msgpack_zone, size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        pub fn msgpack_zone_push_finalizer_expand(zone: *mut msgpack_zone, func: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>, data: *mut libc::c_void) -> bool;
    }
}
pub mod object_h {
    pub type msgpack_object_type = u32;
    pub const MSGPACK_OBJECT_EXT: msgpack_object_type = 9;
    pub const MSGPACK_OBJECT_BIN: msgpack_object_type = 8;
    pub const MSGPACK_OBJECT_MAP: msgpack_object_type = 7;
    pub const MSGPACK_OBJECT_ARRAY: msgpack_object_type = 6;
    pub const MSGPACK_OBJECT_STR: msgpack_object_type = 5;
    pub const MSGPACK_OBJECT_FLOAT: msgpack_object_type = 4;
    pub const MSGPACK_OBJECT_FLOAT64: msgpack_object_type = 4;
    pub const MSGPACK_OBJECT_FLOAT32: msgpack_object_type = 10;
    pub const MSGPACK_OBJECT_NEGATIVE_INTEGER: msgpack_object_type = 3;
    pub const MSGPACK_OBJECT_POSITIVE_INTEGER: msgpack_object_type = 2;
    pub const MSGPACK_OBJECT_BOOLEAN: msgpack_object_type = 1;
    pub const MSGPACK_OBJECT_NIL: msgpack_object_type = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_object {
        pub type_0: msgpack_object_type,
        pub via: msgpack_object_union,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union msgpack_object_union {
        pub boolean: bool,
        pub u64_0: u64,
        pub i64_0: i64,
        pub f64_0: libc::c_double,
        pub array: msgpack_object_array,
        pub map: msgpack_object_map,
        pub str_0: msgpack_object_str,
        pub bin: msgpack_object_bin,
        pub ext: msgpack_object_ext,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_object_ext {
        pub type_0: i8,
        pub size: u32,
        pub ptr: *const i8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_object_bin {
        pub size: u32,
        pub ptr: *const i8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_object_str {
        pub size: u32,
        pub ptr: *const i8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_object_map {
        pub size: u32,
        pub ptr: *mut msgpack_object_kv,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_object_kv {
        pub key: msgpack_object,
        pub val: msgpack_object,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_object_array {
        pub size: u32,
        pub ptr: *mut msgpack_object,
    }
}
pub mod pack_h {
    pub type msgpack_packer_write = Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const i8, _: size_t) -> i32>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_packer {
        pub data: *mut libc::c_void,
        pub callback: msgpack_packer_write,
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_packer_init(mut pk: *mut msgpack_packer, mut data: *mut libc::c_void, mut callback: msgpack_packer_write) {
        (*pk).data = data;
        (*pk).callback = callback;
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_packer_new(mut data: *mut libc::c_void, mut callback: msgpack_packer_write) -> *mut msgpack_packer {
        let mut pk = calloc(1, ::std::mem::size_of::<msgpack_packer>() as u64) as *mut msgpack_packer;
        if pk.is_null() {
            return NULL_0 as *mut msgpack_packer;
        }
        msgpack_packer_init(pk, data, callback);
        return pk;
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_packer_free(mut pk: *mut msgpack_packer) {
        free(pk as *mut libc::c_void);
    }
}
pub mod pack_template_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_29 {
        pub f: libc::c_float,
        pub i: u32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_30 {
        pub f: libc::c_double,
        pub i: u64,
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_char(mut x: *mut msgpack_packer, mut d: i8) -> i32 {
        if (d as i32) < -((1) << 5) {
            let mut buf: [libc::c_uchar; 2] = [0xd0 as i32 as libc::c_uchar, *(&mut d as *mut i8 as *mut u8).offset(0)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
        } else {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut i8 as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_signed_char(mut x: *mut msgpack_packer, mut d: libc::c_schar) -> i32 {
        if (d as i32) < -((1) << 5) {
            let mut buf: [libc::c_uchar; 2] = [0xd0 as i32 as libc::c_uchar, *(&mut d as *mut libc::c_schar as *mut u8).offset(0)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
        } else {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut libc::c_schar as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_short(mut x: *mut msgpack_packer, mut d: i16) -> i32 {
        if (d as i32) < -((1) << 5) {
            if (d as i32) < -((1) << 7) {
                let mut buf: [libc::c_uchar; 3] = [0; 3];
                buf[0 as i32 as usize] = 0xd1 as i32 as libc::c_uchar;
                let mut val = ntohs(d as uint16_t);
                memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 3);
            } else {
                let mut buf_0: [libc::c_uchar; 2] = [0xd0 as i32 as libc::c_uchar, *(&mut d as *mut i16 as *mut u8).offset(0)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 2);
            }
        } else if (d as i32) < (1) << 7 {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut i16 as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        } else if (d as i32) < (1) << 8 {
            let mut buf_1: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut i16 as *mut u8).offset(0)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_1.as_mut_ptr() as *const i8, 2);
        } else {
            let mut buf_2: [libc::c_uchar; 3] = [0; 3];
            buf_2[0 as i32 as usize] = 0xcd as i32 as libc::c_uchar;
            let mut val_0 = ntohs(d as uint16_t);
            memcpy(&mut *buf_2.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_2.as_mut_ptr() as *const i8, 3);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_int(mut x: *mut msgpack_packer, mut d: i32) -> i32 {
        if d < -((1) << 5) {
            if d < -((1) << 15) {
                let mut buf: [libc::c_uchar; 5] = [0; 5];
                buf[0 as i32 as usize] = 0xd2 as i32 as libc::c_uchar;
                let mut val = ntohl(d as u32);
                memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut u32 as *const libc::c_void, 4);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 5);
            } else if d < -((1) << 7) {
                let mut buf_0: [libc::c_uchar; 3] = [0; 3];
                buf_0[0 as i32 as usize] = 0xd1 as i32 as libc::c_uchar;
                let mut val_0 = ntohs(d as int16_t as uint16_t);
                memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut uint16_t as *const libc::c_void, 2);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 3);
            } else {
                let mut buf_1: [libc::c_uchar; 2] = [0xd0 as i32 as libc::c_uchar, *(&mut d as *mut i32 as *mut u8).offset(0)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_1.as_mut_ptr() as *const i8, 2);
            }
        } else if d < (1) << 7 {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut i32 as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        } else if d < (1) << 8 {
            let mut buf_2: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut i32 as *mut u8).offset(0)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_2.as_mut_ptr() as *const i8, 2);
        } else if d < (1) << 16 {
            let mut buf_3: [libc::c_uchar; 3] = [0; 3];
            buf_3[0 as i32 as usize] = 0xcd as i32 as libc::c_uchar;
            let mut val_1 = ntohs(d as uint16_t);
            memcpy(&mut *buf_3.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_1 as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_3.as_mut_ptr() as *const i8, 3);
        } else {
            let mut buf_4: [libc::c_uchar; 5] = [0; 5];
            buf_4[0 as i32 as usize] = 0xce as i32 as libc::c_uchar;
            let mut val_2 = ntohl(d as u32);
            memcpy(&mut *buf_4.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_2 as *mut u32 as *const libc::c_void, 4);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_4.as_mut_ptr() as *const i8, 5);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_long(mut x: *mut msgpack_packer, mut d: i64) -> i32 {
        if (d as libc::c_longlong) < -((1) << 5) {
            if (d as libc::c_longlong) < -((1) << 15) {
                if (d as libc::c_longlong) < -((1) << 31) {
                    let mut buf: [libc::c_uchar; 9] = [0; 9];
                    buf[0 as i32 as usize] = 0xd3 as i32 as libc::c_uchar;
                    let mut val = __bswap_64(d as __uint64_t);
                    memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut u64 as *const libc::c_void, 8);
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 9);
                } else {
                    let mut buf_0: [libc::c_uchar; 5] = [0; 5];
                    buf_0[0 as i32 as usize] = 0xd2 as i32 as libc::c_uchar;
                    let mut val_0 = ntohl(d as i32 as u32);
                    memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut u32 as *const libc::c_void, 4);
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 5);
                }
            } else if d < -((1) << 7) as i64 {
                let mut buf_1: [libc::c_uchar; 3] = [0; 3];
                buf_1[0 as i32 as usize] = 0xd1 as i32 as libc::c_uchar;
                let mut val_1 = ntohs(d as int16_t as uint16_t);
                memcpy(&mut *buf_1.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_1 as *mut uint16_t as *const libc::c_void, 2);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_1.as_mut_ptr() as *const i8, 3);
            } else {
                let mut buf_2: [libc::c_uchar; 2] = [0xd0 as i32 as libc::c_uchar, *(&mut d as *mut i64 as *mut u8).offset(0)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_2.as_mut_ptr() as *const i8, 2);
            }
        } else if d < ((1) << 7) as i64 {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut i64 as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        } else if (d as libc::c_longlong) < (1) << 16 {
            if d < ((1) << 8) as i64 {
                let mut buf_3: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut i64 as *mut u8).offset(0)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_3.as_mut_ptr() as *const i8, 2);
            } else {
                let mut buf_4: [libc::c_uchar; 3] = [0; 3];
                buf_4[0 as i32 as usize] = 0xcd as i32 as libc::c_uchar;
                let mut val_2 = ntohs(d as uint16_t);
                memcpy(&mut *buf_4.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_2 as *mut uint16_t as *const libc::c_void, 2);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_4.as_mut_ptr() as *const i8, 3);
            }
        } else if (d as libc::c_longlong) < (1) << 32 {
            let mut buf_5: [libc::c_uchar; 5] = [0; 5];
            buf_5[0 as i32 as usize] = 0xce as i32 as libc::c_uchar;
            let mut val_3 = ntohl(d as u32);
            memcpy(&mut *buf_5.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_3 as *mut u32 as *const libc::c_void, 4);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_5.as_mut_ptr() as *const i8, 5);
        } else {
            let mut buf_6: [libc::c_uchar; 9] = [0; 9];
            buf_6[0 as i32 as usize] = 0xcf as i32 as libc::c_uchar;
            let mut val_4 = __bswap_64(d as __uint64_t);
            memcpy(&mut *buf_6.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_4 as *mut u64 as *const libc::c_void, 8);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_6.as_mut_ptr() as *const i8, 9);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_long_long(mut x: *mut msgpack_packer, mut d: libc::c_longlong) -> i32 {
        if d < -((1) << 5) {
            if d < -((1) << 15) {
                if d < -((1) << 31) {
                    let mut buf: [libc::c_uchar; 9] = [0; 9];
                    buf[0 as i32 as usize] = 0xd3 as i32 as libc::c_uchar;
                    let mut val = __bswap_64(d as __uint64_t);
                    memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut u64 as *const libc::c_void, 8);
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 9);
                } else {
                    let mut buf_0: [libc::c_uchar; 5] = [0; 5];
                    buf_0[0 as i32 as usize] = 0xd2 as i32 as libc::c_uchar;
                    let mut val_0 = ntohl(d as i32 as u32);
                    memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut u32 as *const libc::c_void, 4);
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 5);
                }
            } else if d < -((1) << 7) as libc::c_longlong {
                let mut buf_1: [libc::c_uchar; 3] = [0; 3];
                buf_1[0 as i32 as usize] = 0xd1 as i32 as libc::c_uchar;
                let mut val_1 = ntohs(d as int16_t as uint16_t);
                memcpy(&mut *buf_1.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_1 as *mut uint16_t as *const libc::c_void, 2);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_1.as_mut_ptr() as *const i8, 3);
            } else {
                let mut buf_2: [libc::c_uchar; 2] = [0xd0 as i32 as libc::c_uchar, *(&mut d as *mut libc::c_longlong as *mut u8).offset(0)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_2.as_mut_ptr() as *const i8, 2);
            }
        } else if d < ((1) << 7) as libc::c_longlong {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut libc::c_longlong as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        } else if d < (1) << 16 {
            if d < ((1) << 8) as libc::c_longlong {
                let mut buf_3: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut libc::c_longlong as *mut u8).offset(0)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_3.as_mut_ptr() as *const i8, 2);
            } else {
                let mut buf_4: [libc::c_uchar; 3] = [0; 3];
                buf_4[0 as i32 as usize] = 0xcd as i32 as libc::c_uchar;
                let mut val_2 = ntohs(d as uint16_t);
                memcpy(&mut *buf_4.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_2 as *mut uint16_t as *const libc::c_void, 2);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_4.as_mut_ptr() as *const i8, 3);
            }
        } else if d < (1) << 32 {
            let mut buf_5: [libc::c_uchar; 5] = [0; 5];
            buf_5[0 as i32 as usize] = 0xce as i32 as libc::c_uchar;
            let mut val_3 = ntohl(d as u32);
            memcpy(&mut *buf_5.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_3 as *mut u32 as *const libc::c_void, 4);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_5.as_mut_ptr() as *const i8, 5);
        } else {
            let mut buf_6: [libc::c_uchar; 9] = [0; 9];
            buf_6[0 as i32 as usize] = 0xcf as i32 as libc::c_uchar;
            let mut val_4 = __bswap_64(d as __uint64_t);
            memcpy(&mut *buf_6.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_4 as *mut u64 as *const libc::c_void, 8);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_6.as_mut_ptr() as *const i8, 9);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_unsigned_char(mut x: *mut msgpack_packer, mut d: libc::c_uchar) -> i32 {
        if (d as i32) < (1) << 7 {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut libc::c_uchar as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        } else {
            let mut buf: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut libc::c_uchar as *mut u8).offset(0)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_unsigned_short(mut x: *mut msgpack_packer, mut d: u16) -> i32 {
        if (d as i32) < (1) << 7 {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut u16 as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        } else if (d as i32) < (1) << 8 {
            let mut buf: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut u16 as *mut u8).offset(0)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
        } else {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as i32 as usize] = 0xcd as i32 as libc::c_uchar;
            let mut val = ntohs(d);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 3);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_unsigned_int(mut x: *mut msgpack_packer, mut d: u32) -> i32 {
        if d < ((1) << 8) as u32 {
            if d < ((1) << 7) as u32 {
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut u32 as *mut u8).offset(0) as *mut u8 as *const i8, 1);
            } else {
                let mut buf: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut u32 as *mut u8).offset(0)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
            }
        } else if d < ((1) << 16) as u32 {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as i32 as usize] = 0xcd as i32 as libc::c_uchar;
            let mut val = ntohs(d as uint16_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 3);
        } else {
            let mut buf_1: [libc::c_uchar; 5] = [0; 5];
            buf_1[0 as i32 as usize] = 0xce as i32 as libc::c_uchar;
            let mut val_0 = ntohl(d);
            memcpy(&mut *buf_1.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut u32 as *const libc::c_void, 4);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_1.as_mut_ptr() as *const i8, 5);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_unsigned_long(mut x: *mut msgpack_packer, mut d: u64) -> i32 {
        if (d as libc::c_ulonglong) < (1) << 8 {
            if (d as libc::c_ulonglong) < (1) << 7 {
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut u64 as *mut u8).offset(0) as *mut u8 as *const i8, 1);
            } else {
                let mut buf: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut u64 as *mut u8).offset(0)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
            }
        } else if (d as libc::c_ulonglong) < (1) << 16 {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as i32 as usize] = 0xcd as i32 as libc::c_uchar;
            let mut val = ntohs(d as uint16_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 3);
        } else if (d as libc::c_ulonglong) < (1) << 32 {
            let mut buf_1: [libc::c_uchar; 5] = [0; 5];
            buf_1[0 as i32 as usize] = 0xce as i32 as libc::c_uchar;
            let mut val_0 = ntohl(d as u32);
            memcpy(&mut *buf_1.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut u32 as *const libc::c_void, 4);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_1.as_mut_ptr() as *const i8, 5);
        } else {
            let mut buf_2: [libc::c_uchar; 9] = [0; 9];
            buf_2[0 as i32 as usize] = 0xcf as i32 as libc::c_uchar;
            let mut val_1 = __bswap_64(d);
            memcpy(&mut *buf_2.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_1 as *mut u64 as *const libc::c_void, 8);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_2.as_mut_ptr() as *const i8, 9);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_unsigned_long_long(mut x: *mut msgpack_packer, mut d: libc::c_ulonglong) -> i32 {
        if d < (1) << 8 {
            if d < (1) << 7 {
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut libc::c_ulonglong as *mut u8).offset(0) as *mut u8 as *const i8, 1);
            } else {
                let mut buf: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut libc::c_ulonglong as *mut u8).offset(0)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
            }
        } else if d < (1) << 16 {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as i32 as usize] = 0xcd as i32 as libc::c_uchar;
            let mut val = ntohs(d as uint16_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 3);
        } else if d < (1) << 32 {
            let mut buf_1: [libc::c_uchar; 5] = [0; 5];
            buf_1[0 as i32 as usize] = 0xce as i32 as libc::c_uchar;
            let mut val_0 = ntohl(d as u32);
            memcpy(&mut *buf_1.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut u32 as *const libc::c_void, 4);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_1.as_mut_ptr() as *const i8, 5);
        } else {
            let mut buf_2: [libc::c_uchar; 9] = [0; 9];
            buf_2[0 as i32 as usize] = 0xcf as i32 as libc::c_uchar;
            let mut val_1 = __bswap_64(d as __uint64_t);
            memcpy(&mut *buf_2.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_1 as *mut u64 as *const libc::c_void, 8);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_2.as_mut_ptr() as *const i8, 9);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_uint8(mut x: *mut msgpack_packer, mut d: u8) -> i32 {
        if (d as i32) < (1) << 7 {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        } else {
            let mut buf: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut u8).offset(0)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_uint16(mut x: *mut msgpack_packer, mut d: uint16_t) -> i32 {
        if (d as i32) < (1) << 7 {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut uint16_t as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        } else if (d as i32) < (1) << 8 {
            let mut buf: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut uint16_t as *mut u8).offset(0)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
        } else {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as i32 as usize] = 0xcd as i32 as libc::c_uchar;
            let mut val = ntohs(d);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 3);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_uint32(mut x: *mut msgpack_packer, mut d: u32) -> i32 {
        if d < ((1) << 8) as u32 {
            if d < ((1) << 7) as u32 {
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut u32 as *mut u8).offset(0) as *mut u8 as *const i8, 1);
            } else {
                let mut buf: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut u32 as *mut u8).offset(0)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
            }
        } else if d < ((1) << 16) as u32 {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as i32 as usize] = 0xcd as i32 as libc::c_uchar;
            let mut val = ntohs(d as uint16_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 3);
        } else {
            let mut buf_1: [libc::c_uchar; 5] = [0; 5];
            buf_1[0 as i32 as usize] = 0xce as i32 as libc::c_uchar;
            let mut val_0 = ntohl(d);
            memcpy(&mut *buf_1.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut u32 as *const libc::c_void, 4);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_1.as_mut_ptr() as *const i8, 5);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_uint64(mut x: *mut msgpack_packer, mut d: u64) -> i32 {
        if (d as libc::c_ulonglong) < (1) << 8 {
            if (d as libc::c_ulonglong) < (1) << 7 {
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut u64 as *mut u8).offset(0) as *mut u8 as *const i8, 1);
            } else {
                let mut buf: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut u64 as *mut u8).offset(0)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
            }
        } else if (d as libc::c_ulonglong) < (1) << 16 {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as i32 as usize] = 0xcd as i32 as libc::c_uchar;
            let mut val = ntohs(d as uint16_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 3);
        } else if (d as libc::c_ulonglong) < (1) << 32 {
            let mut buf_1: [libc::c_uchar; 5] = [0; 5];
            buf_1[0 as i32 as usize] = 0xce as i32 as libc::c_uchar;
            let mut val_0 = ntohl(d as u32);
            memcpy(&mut *buf_1.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut u32 as *const libc::c_void, 4);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_1.as_mut_ptr() as *const i8, 5);
        } else {
            let mut buf_2: [libc::c_uchar; 9] = [0; 9];
            buf_2[0 as i32 as usize] = 0xcf as i32 as libc::c_uchar;
            let mut val_1 = __bswap_64(d);
            memcpy(&mut *buf_2.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_1 as *mut u64 as *const libc::c_void, 8);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_2.as_mut_ptr() as *const i8, 9);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_int8(mut x: *mut msgpack_packer, mut d: i8) -> i32 {
        if (d as i32) < -((1) << 5) {
            let mut buf: [libc::c_uchar; 2] = [0xd0 as i32 as libc::c_uchar, *(&mut d as *mut i8 as *mut u8).offset(0)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
        } else {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut i8 as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_int16(mut x: *mut msgpack_packer, mut d: int16_t) -> i32 {
        if (d as i32) < -((1) << 5) {
            if (d as i32) < -((1) << 7) {
                let mut buf: [libc::c_uchar; 3] = [0; 3];
                buf[0 as i32 as usize] = 0xd1 as i32 as libc::c_uchar;
                let mut val = ntohs(d as uint16_t);
                memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 3);
            } else {
                let mut buf_0: [libc::c_uchar; 2] = [0xd0 as i32 as libc::c_uchar, *(&mut d as *mut int16_t as *mut u8).offset(0)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 2);
            }
        } else if (d as i32) < (1) << 7 {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut int16_t as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        } else if (d as i32) < (1) << 8 {
            let mut buf_1: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut int16_t as *mut u8).offset(0)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_1.as_mut_ptr() as *const i8, 2);
        } else {
            let mut buf_2: [libc::c_uchar; 3] = [0; 3];
            buf_2[0 as i32 as usize] = 0xcd as i32 as libc::c_uchar;
            let mut val_0 = ntohs(d as uint16_t);
            memcpy(&mut *buf_2.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_2.as_mut_ptr() as *const i8, 3);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_int32(mut x: *mut msgpack_packer, mut d: i32) -> i32 {
        if d < -((1) << 5) {
            if d < -((1) << 15) {
                let mut buf: [libc::c_uchar; 5] = [0; 5];
                buf[0 as i32 as usize] = 0xd2 as i32 as libc::c_uchar;
                let mut val = ntohl(d as u32);
                memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut u32 as *const libc::c_void, 4);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 5);
            } else if d < -((1) << 7) {
                let mut buf_0: [libc::c_uchar; 3] = [0; 3];
                buf_0[0 as i32 as usize] = 0xd1 as i32 as libc::c_uchar;
                let mut val_0 = ntohs(d as int16_t as uint16_t);
                memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut uint16_t as *const libc::c_void, 2);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 3);
            } else {
                let mut buf_1: [libc::c_uchar; 2] = [0xd0 as i32 as libc::c_uchar, *(&mut d as *mut i32 as *mut u8).offset(0)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_1.as_mut_ptr() as *const i8, 2);
            }
        } else if d < (1) << 7 {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut i32 as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        } else if d < (1) << 8 {
            let mut buf_2: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut i32 as *mut u8).offset(0)];
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_2.as_mut_ptr() as *const i8, 2);
        } else if d < (1) << 16 {
            let mut buf_3: [libc::c_uchar; 3] = [0; 3];
            buf_3[0 as i32 as usize] = 0xcd as i32 as libc::c_uchar;
            let mut val_1 = ntohs(d as uint16_t);
            memcpy(&mut *buf_3.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_1 as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_3.as_mut_ptr() as *const i8, 3);
        } else {
            let mut buf_4: [libc::c_uchar; 5] = [0; 5];
            buf_4[0 as i32 as usize] = 0xce as i32 as libc::c_uchar;
            let mut val_2 = ntohl(d as u32);
            memcpy(&mut *buf_4.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_2 as *mut u32 as *const libc::c_void, 4);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_4.as_mut_ptr() as *const i8, 5);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_int64(mut x: *mut msgpack_packer, mut d: i64) -> i32 {
        if (d as libc::c_longlong) < -((1) << 5) {
            if (d as libc::c_longlong) < -((1) << 15) {
                if (d as libc::c_longlong) < -((1) << 31) {
                    let mut buf: [libc::c_uchar; 9] = [0; 9];
                    buf[0 as i32 as usize] = 0xd3 as i32 as libc::c_uchar;
                    let mut val = __bswap_64(d as __uint64_t);
                    memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut u64 as *const libc::c_void, 8);
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 9);
                } else {
                    let mut buf_0: [libc::c_uchar; 5] = [0; 5];
                    buf_0[0 as i32 as usize] = 0xd2 as i32 as libc::c_uchar;
                    let mut val_0 = ntohl(d as i32 as u32);
                    memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut u32 as *const libc::c_void, 4);
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 5);
                }
            } else if d < -((1) << 7) as i64 {
                let mut buf_1: [libc::c_uchar; 3] = [0; 3];
                buf_1[0 as i32 as usize] = 0xd1 as i32 as libc::c_uchar;
                let mut val_1 = ntohs(d as int16_t as uint16_t);
                memcpy(&mut *buf_1.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_1 as *mut uint16_t as *const libc::c_void, 2);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_1.as_mut_ptr() as *const i8, 3);
            } else {
                let mut buf_2: [libc::c_uchar; 2] = [0xd0 as i32 as libc::c_uchar, *(&mut d as *mut i64 as *mut u8).offset(0)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_2.as_mut_ptr() as *const i8, 2);
            }
        } else if d < ((1) << 7) as i64 {
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut i64 as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        } else if (d as libc::c_longlong) < (1) << 16 {
            if d < ((1) << 8) as i64 {
                let mut buf_3: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut i64 as *mut u8).offset(0)];
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_3.as_mut_ptr() as *const i8, 2);
            } else {
                let mut buf_4: [libc::c_uchar; 3] = [0; 3];
                buf_4[0 as i32 as usize] = 0xcd as i32 as libc::c_uchar;
                let mut val_2 = ntohs(d as uint16_t);
                memcpy(&mut *buf_4.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_2 as *mut uint16_t as *const libc::c_void, 2);
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_4.as_mut_ptr() as *const i8, 3);
            }
        } else if (d as libc::c_longlong) < (1) << 32 {
            let mut buf_5: [libc::c_uchar; 5] = [0; 5];
            buf_5[0 as i32 as usize] = 0xce as i32 as libc::c_uchar;
            let mut val_3 = ntohl(d as u32);
            memcpy(&mut *buf_5.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_3 as *mut u32 as *const libc::c_void, 4);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_5.as_mut_ptr() as *const i8, 5);
        } else {
            let mut buf_6: [libc::c_uchar; 9] = [0; 9];
            buf_6[0 as i32 as usize] = 0xcf as i32 as libc::c_uchar;
            let mut val_4 = __bswap_64(d as __uint64_t);
            memcpy(&mut *buf_6.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_4 as *mut u64 as *const libc::c_void, 8);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_6.as_mut_ptr() as *const i8, 9);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_fix_uint8(mut x: *mut msgpack_packer, mut d: u8) -> i32 {
        let mut buf: [libc::c_uchar; 2] = [0xcc as i32 as libc::c_uchar, *(&mut d as *mut u8).offset(0)];
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_fix_uint16(mut x: *mut msgpack_packer, mut d: uint16_t) -> i32 {
        let mut buf: [libc::c_uchar; 3] = [0; 3];
        buf[0 as i32 as usize] = 0xcd as i32 as libc::c_uchar;
        let mut val = ntohs(d);
        memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 3);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_fix_uint32(mut x: *mut msgpack_packer, mut d: u32) -> i32 {
        let mut buf: [libc::c_uchar; 5] = [0; 5];
        buf[0 as i32 as usize] = 0xce as i32 as libc::c_uchar;
        let mut val = ntohl(d);
        memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut u32 as *const libc::c_void, 4);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 5);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_fix_uint64(mut x: *mut msgpack_packer, mut d: u64) -> i32 {
        let mut buf: [libc::c_uchar; 9] = [0; 9];
        buf[0 as i32 as usize] = 0xcf as i32 as libc::c_uchar;
        let mut val = __bswap_64(d);
        memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut u64 as *const libc::c_void, 8);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 9);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_fix_int8(mut x: *mut msgpack_packer, mut d: i8) -> i32 {
        let mut buf: [libc::c_uchar; 2] = [0xd0 as i32 as libc::c_uchar, *(&mut d as *mut i8 as *mut u8).offset(0)];
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_fix_int16(mut x: *mut msgpack_packer, mut d: int16_t) -> i32 {
        let mut buf: [libc::c_uchar; 3] = [0; 3];
        buf[0 as i32 as usize] = 0xd1 as i32 as libc::c_uchar;
        let mut val = ntohs(d as uint16_t);
        memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 3);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_fix_int32(mut x: *mut msgpack_packer, mut d: i32) -> i32 {
        let mut buf: [libc::c_uchar; 5] = [0; 5];
        buf[0 as i32 as usize] = 0xd2 as i32 as libc::c_uchar;
        let mut val = ntohl(d as u32);
        memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut u32 as *const libc::c_void, 4);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 5);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_fix_int64(mut x: *mut msgpack_packer, mut d: i64) -> i32 {
        let mut buf: [libc::c_uchar; 9] = [0; 9];
        buf[0 as i32 as usize] = 0xd3 as i32 as libc::c_uchar;
        let mut val = __bswap_64(d as __uint64_t);
        memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut u64 as *const libc::c_void, 8);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 9);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_float(mut x: *mut msgpack_packer, mut d: libc::c_float) -> i32 {
        let mut buf: [libc::c_uchar; 5] = [0; 5];
        let mut mem = C2RustUnnamed_29 { f: 0. };
        mem.f = d;
        buf[0 as i32 as usize] = 0xca as i32 as libc::c_uchar;
        let mut val = ntohl(mem.i);
        memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut u32 as *const libc::c_void, 4);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 5);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_double(mut x: *mut msgpack_packer, mut d: libc::c_double) -> i32 {
        let mut buf: [libc::c_uchar; 9] = [0; 9];
        let mut mem = C2RustUnnamed_30 { f: 0. };
        mem.f = d;
        buf[0 as i32 as usize] = 0xcb as i32 as libc::c_uchar;
        let mut val = __bswap_64(mem.i);
        memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut u64 as *const libc::c_void, 8);
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 9);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_nil(mut x: *mut msgpack_packer) -> i32 {
        pub static mut d: libc::c_uchar = 0xc0 as i32 as libc::c_uchar;
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &d as *const libc::c_uchar as *const i8, 1);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_true(mut x: *mut msgpack_packer) -> i32 {
        pub static mut d: libc::c_uchar = 0xc3 as i32 as libc::c_uchar;
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &d as *const libc::c_uchar as *const i8, 1);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_false(mut x: *mut msgpack_packer) -> i32 {
        pub static mut d: libc::c_uchar = 0xc2 as i32 as libc::c_uchar;
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &d as *const libc::c_uchar as *const i8, 1);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_array(mut x: *mut msgpack_packer, mut n: size_t) -> i32 {
        if n < 16 {
            let mut d = (0x90 as i32 | n as u8 as i32) as libc::c_uchar;
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut d as *mut libc::c_uchar as *const i8, 1);
        } else if n < 65536 {
            let mut buf: [libc::c_uchar; 3] = [0; 3];
            buf[0 as i32 as usize] = 0xdc as i32 as libc::c_uchar;
            let mut val = ntohs(n as uint16_t);
            memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 3);
        } else {
            let mut buf_0: [libc::c_uchar; 5] = [0; 5];
            buf_0[0 as i32 as usize] = 0xdd as i32 as libc::c_uchar;
            let mut val_0 = ntohl(n as u32);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut u32 as *const libc::c_void, 4);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 5);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_map(mut x: *mut msgpack_packer, mut n: size_t) -> i32 {
        if n < 16 {
            let mut d = (0x80 as i32 | n as u8 as i32) as libc::c_uchar;
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut libc::c_uchar as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        } else if n < 65536 {
            let mut buf: [libc::c_uchar; 3] = [0; 3];
            buf[0 as i32 as usize] = 0xde as i32 as libc::c_uchar;
            let mut val = ntohs(n as uint16_t);
            memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 3);
        } else {
            let mut buf_0: [libc::c_uchar; 5] = [0; 5];
            buf_0[0 as i32 as usize] = 0xdf as i32 as libc::c_uchar;
            let mut val_0 = ntohl(n as u32);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut u32 as *const libc::c_void, 4);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 5);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_str(mut x: *mut msgpack_packer, mut l: size_t) -> i32 {
        if l < 32 {
            let mut d = (0xa0 as i32 | l as u8 as i32) as libc::c_uchar;
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut libc::c_uchar as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        } else if l < 256 {
            let mut buf: [libc::c_uchar; 2] = [0; 2];
            buf[0 as i32 as usize] = 0xd9 as i32 as libc::c_uchar;
            buf[1 as i32 as usize] = l as u8;
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
        } else if l < 65536 {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as i32 as usize] = 0xda as i32 as libc::c_uchar;
            let mut val = ntohs(l as uint16_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 3);
        } else {
            let mut buf_1: [libc::c_uchar; 5] = [0; 5];
            buf_1[0 as i32 as usize] = 0xdb as i32 as libc::c_uchar;
            let mut val_0 = ntohl(l as u32);
            memcpy(&mut *buf_1.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut u32 as *const libc::c_void, 4);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_1.as_mut_ptr() as *const i8, 5);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_str_body(mut x: *mut msgpack_packer, mut b: *const libc::c_void, mut l: size_t) -> i32 {
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, b as *const libc::c_uchar as *const i8, l);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_v4raw(mut x: *mut msgpack_packer, mut l: size_t) -> i32 {
        if l < 32 {
            let mut d = (0xa0 as i32 | l as u8 as i32) as libc::c_uchar;
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, &mut *(&mut d as *mut libc::c_uchar as *mut u8).offset(0) as *mut u8 as *const i8, 1);
        } else if l < 65536 {
            let mut buf: [libc::c_uchar; 3] = [0; 3];
            buf[0 as i32 as usize] = 0xda as i32 as libc::c_uchar;
            let mut val = ntohs(l as uint16_t);
            memcpy(&mut *buf.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 3);
        } else {
            let mut buf_0: [libc::c_uchar; 5] = [0; 5];
            buf_0[0 as i32 as usize] = 0xdb as i32 as libc::c_uchar;
            let mut val_0 = ntohl(l as u32);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut u32 as *const libc::c_void, 4);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 5);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_v4raw_body(mut x: *mut msgpack_packer, mut b: *const libc::c_void, mut l: size_t) -> i32 {
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, b as *const libc::c_uchar as *const i8, l);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_bin(mut x: *mut msgpack_packer, mut l: size_t) -> i32 {
        if l < 256 {
            let mut buf: [libc::c_uchar; 2] = [0; 2];
            buf[0 as i32 as usize] = 0xc4 as i32 as libc::c_uchar;
            buf[1 as i32 as usize] = l as u8;
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
        } else if l < 65536 {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0 as i32 as usize] = 0xc5 as i32 as libc::c_uchar;
            let mut val = ntohs(l as uint16_t);
            memcpy(&mut *buf_0.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 3);
        } else {
            let mut buf_1: [libc::c_uchar; 5] = [0; 5];
            buf_1[0 as i32 as usize] = 0xc6 as i32 as libc::c_uchar;
            let mut val_0 = ntohl(l as u32);
            memcpy(&mut *buf_1.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut u32 as *const libc::c_void, 4);
            return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_1.as_mut_ptr() as *const i8, 5);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_bin_body(mut x: *mut msgpack_packer, mut b: *const libc::c_void, mut l: size_t) -> i32 {
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, b as *const libc::c_uchar as *const i8, l);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_ext(mut x: *mut msgpack_packer, mut l: size_t, mut type_0: i8) -> i32 {
        match l {
            1 => {
                let mut buf: [libc::c_uchar; 2] = [0; 2];
                buf[0 as i32 as usize] = 0xd4 as i32 as libc::c_uchar;
                buf[1 as i32 as usize] = type_0 as libc::c_uchar;
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf.as_mut_ptr() as *const i8, 2);
            }
            2 => {
                let mut buf_0: [libc::c_uchar; 2] = [0; 2];
                buf_0[0 as i32 as usize] = 0xd5 as i32 as libc::c_uchar;
                buf_0[1 as i32 as usize] = type_0 as libc::c_uchar;
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_0.as_mut_ptr() as *const i8, 2);
            }
            4 => {
                let mut buf_1: [libc::c_uchar; 2] = [0; 2];
                buf_1[0 as i32 as usize] = 0xd6 as i32 as libc::c_uchar;
                buf_1[1 as i32 as usize] = type_0 as libc::c_uchar;
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_1.as_mut_ptr() as *const i8, 2);
            }
            8 => {
                let mut buf_2: [libc::c_uchar; 2] = [0; 2];
                buf_2[0 as i32 as usize] = 0xd7 as i32 as libc::c_uchar;
                buf_2[1 as i32 as usize] = type_0 as libc::c_uchar;
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_2.as_mut_ptr() as *const i8, 2);
            }
            16 => {
                let mut buf_3: [libc::c_uchar; 2] = [0; 2];
                buf_3[0 as i32 as usize] = 0xd8 as i32 as libc::c_uchar;
                buf_3[1 as i32 as usize] = type_0 as libc::c_uchar;
                return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_3.as_mut_ptr() as *const i8, 2);
            }
            _ => {
                if l < 256 {
                    let mut buf_4: [libc::c_uchar; 3] = [0; 3];
                    buf_4[0 as i32 as usize] = 0xc7 as i32 as libc::c_uchar;
                    buf_4[1 as i32 as usize] = l as libc::c_uchar;
                    buf_4[2 as i32 as usize] = type_0 as libc::c_uchar;
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_4.as_mut_ptr() as *const i8, 3);
                } else if l < 65536 {
                    let mut buf_5: [libc::c_uchar; 4] = [0; 4];
                    buf_5[0 as i32 as usize] = 0xc8 as i32 as libc::c_uchar;
                    let mut val = ntohs(l as uint16_t);
                    memcpy(&mut *buf_5.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val as *mut uint16_t as *const libc::c_void, 2);
                    buf_5[3 as i32 as usize] = type_0 as libc::c_uchar;
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_5.as_mut_ptr() as *const i8, 4);
                } else {
                    let mut buf_6: [libc::c_uchar; 6] = [0; 6];
                    buf_6[0 as i32 as usize] = 0xc9 as i32 as libc::c_uchar;
                    let mut val_0 = ntohl(l as u32);
                    memcpy(&mut *buf_6.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_void, &mut val_0 as *mut u32 as *const libc::c_void, 4);
                    buf_6[5 as i32 as usize] = type_0 as libc::c_uchar;
                    return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, buf_6.as_mut_ptr() as *const i8, 6);
                }
            }
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_pack_ext_body(mut x: *mut msgpack_packer, mut b: *const libc::c_void, mut l: size_t) -> i32 {
        return Some((*x).callback.expect("non-null function pointer")).expect("non-null function pointer")((*x).data, b as *const libc::c_uchar as *const i8, l);
    }
}
pub mod unpack_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_unpacked {
        pub zone: *mut msgpack_zone,
        pub data: msgpack_object,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_unpacker {
        pub buffer: *mut i8,
        pub used: size_t,
        pub free: size_t,
        pub off: size_t,
        pub parsed: size_t,
        pub z: *mut msgpack_zone,
        pub initial_buffer_size: size_t,
        pub ctx: *mut libc::c_void,
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_unpacker_reserve_buffer(mut mpac: *mut msgpack_unpacker, mut size: size_t) -> bool {
        if (*mpac).free >= size {
            return true;
        }
        return msgpack_unpacker_expand_buffer(mpac, size);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_unpacker_buffer(mut mpac: *mut msgpack_unpacker) -> *mut i8 {
        return (*mpac).buffer.offset((*mpac).used as isize);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_unpacker_buffer_capacity(mut mpac: *const msgpack_unpacker) -> size_t {
        return (*mpac).free;
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_unpacker_buffer_consumed(mut mpac: *mut msgpack_unpacker, mut size: size_t) {
        (*mpac).used = ((*mpac).used as u64).wrapping_add(size) as size_t as size_t;
        (*mpac).free = ((*mpac).free as u64).wrapping_sub(size) as size_t as size_t;
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_unpacked_init(mut result: *mut msgpack_unpacked) {
        memset(result as *mut libc::c_void, 0, ::std::mem::size_of::<msgpack_unpacked>() as u64);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_unpacked_destroy(mut result: *mut msgpack_unpacked) {
        if !(*result).zone.is_null() {
            msgpack_zone_free((*result).zone);
            (*result).zone = NULL_0 as *mut msgpack_zone;
            memset(&mut (*result).data as *mut msgpack_object as *mut libc::c_void, 0, ::std::mem::size_of::<msgpack_object>() as u64);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_unpacked_release_zone(mut result: *mut msgpack_unpacked) -> *mut msgpack_zone {
        if !(*result).zone.is_null() {
            let mut z = (*result).zone;
            (*result).zone = NULL_0 as *mut msgpack_zone;
            return z;
        }
        return NULL_0 as *mut msgpack_zone;
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_unpacker_message_size(mut mpac: *const msgpack_unpacker) -> size_t {
        return (*mpac).parsed.wrapping_sub((*mpac).off).wrapping_add((*mpac).used);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_unpacker_parsed_size(mut mpac: *const msgpack_unpacker) -> size_t {
        return (*mpac).parsed;
    }
    extern "C" {
        #[no_mangle]
        pub fn msgpack_unpacker_expand_buffer(mpac: *mut msgpack_unpacker, size: size_t) -> bool;
    }
}
pub mod sbuffer_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_sbuffer {
        pub size: size_t,
        pub data: *mut i8,
        pub alloc: size_t,
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_sbuffer_init(mut sbuf: *mut msgpack_sbuffer) {
        memset(sbuf as *mut libc::c_void, 0, ::std::mem::size_of::<msgpack_sbuffer>() as u64);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_sbuffer_destroy(mut sbuf: *mut msgpack_sbuffer) {
        free((*sbuf).data as *mut libc::c_void);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_sbuffer_new() -> *mut msgpack_sbuffer {
        return calloc(1, ::std::mem::size_of::<msgpack_sbuffer>() as u64) as *mut msgpack_sbuffer;
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_sbuffer_free(mut sbuf: *mut msgpack_sbuffer) {
        if sbuf.is_null() {
            return;
        }
        msgpack_sbuffer_destroy(sbuf);
        free(sbuf as *mut libc::c_void);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_sbuffer_write(mut data: *mut libc::c_void, mut buf: *const i8, mut len: size_t) -> i32 {
        let mut sbuf = data as *mut msgpack_sbuffer;
        if (*sbuf).alloc.wrapping_sub((*sbuf).size) < len {
            let mut tmp = 0 as *mut libc::c_void;
            let mut nsize = if (*sbuf).alloc != 0 { (*sbuf).alloc.wrapping_mul(2) } else { MSGPACK_SBUFFER_INIT_SIZE as u64 };
            while nsize < (*sbuf).size.wrapping_add(len) {
                let mut tmp_nsize = nsize.wrapping_mul(2);
                if tmp_nsize <= nsize {
                    nsize = (*sbuf).size.wrapping_add(len);
                    break;
                } else {
                    nsize = tmp_nsize
                }
            }
            tmp = realloc((*sbuf).data as *mut libc::c_void, nsize);
            if tmp.is_null() {
                return -(1);
            }
            (*sbuf).data = tmp as *mut i8;
            (*sbuf).alloc = nsize
        }
        memcpy((*sbuf).data.offset((*sbuf).size as isize) as *mut libc::c_void, buf as *const libc::c_void, len);
        (*sbuf).size = ((*sbuf).size as u64).wrapping_add(len) as size_t as size_t;
        return 0;
    }
    pub const MSGPACK_SBUFFER_INIT_SIZE: i32 = 8192;
    #[inline]
    pub unsafe extern "C" fn msgpack_sbuffer_release(mut sbuf: *mut msgpack_sbuffer) -> *mut i8 {
        let mut tmp = (*sbuf).data;
        (*sbuf).size = 0;
        (*sbuf).data = NULL_0 as *mut i8;
        (*sbuf).alloc = 0;
        return tmp;
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_sbuffer_clear(mut sbuf: *mut msgpack_sbuffer) {
        (*sbuf).size = 0;
    }
}
pub mod vrefbuffer_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_vrefbuffer_inner_buffer {
        pub free: size_t,
        pub ptr: *mut i8,
        pub head: *mut msgpack_vrefbuffer_chunk,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct msgpack_vrefbuffer {
        pub tail: *mut iovec,
        pub end: *mut iovec,
        pub array: *mut iovec,
        pub chunk_size: size_t,
        pub ref_size: size_t,
        pub inner_buffer: msgpack_vrefbuffer_inner_buffer,
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_vrefbuffer_new(mut ref_size: size_t, mut chunk_size: size_t) -> *mut msgpack_vrefbuffer {
        let mut vbuf = malloc(::std::mem::size_of::<msgpack_vrefbuffer>() as u64) as *mut msgpack_vrefbuffer;
        if vbuf.is_null() {
            return NULL_0 as *mut msgpack_vrefbuffer;
        }
        if !msgpack_vrefbuffer_init(vbuf, ref_size, chunk_size) {
            free(vbuf as *mut libc::c_void);
            return NULL_0 as *mut msgpack_vrefbuffer;
        }
        return vbuf;
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_vrefbuffer_free(mut vbuf: *mut msgpack_vrefbuffer) {
        if vbuf.is_null() {
            return;
        }
        msgpack_vrefbuffer_destroy(vbuf);
        free(vbuf as *mut libc::c_void);
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_vrefbuffer_write(mut data: *mut libc::c_void, mut buf: *const i8, mut len: size_t) -> i32 {
        let mut vbuf = data as *mut msgpack_vrefbuffer;
        if len < (*vbuf).ref_size {
            return msgpack_vrefbuffer_append_copy(vbuf, buf, len);
        } else {
            return msgpack_vrefbuffer_append_ref(vbuf, buf, len);
        };
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_vrefbuffer_vec(mut vref: *const msgpack_vrefbuffer) -> *const iovec {
        return (*vref).array;
    }
    #[inline]
    pub unsafe extern "C" fn msgpack_vrefbuffer_veclen(mut vref: *const msgpack_vrefbuffer) -> size_t {
        return (*vref).tail.offset_from((*vref).array) as i64 as size_t;
    }
    extern "C" {
        pub type msgpack_vrefbuffer_chunk;
        #[no_mangle]
        pub fn msgpack_vrefbuffer_init(vbuf: *mut msgpack_vrefbuffer, ref_size: size_t, chunk_size: size_t) -> bool;
        #[no_mangle]
        pub fn msgpack_vrefbuffer_destroy(vbuf: *mut msgpack_vrefbuffer);
        #[no_mangle]
        pub fn msgpack_vrefbuffer_append_ref(vbuf: *mut msgpack_vrefbuffer, buf: *const i8, len: size_t) -> i32;
        #[no_mangle]
        pub fn msgpack_vrefbuffer_append_copy(vbuf: *mut msgpack_vrefbuffer, buf: *const i8, len: size_t) -> i32;
    }
}
pub mod channel_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Channel {
        pub id: u64,
        pub refcount: size_t,
        pub events: *mut MultiQueue,
        pub streamtype: ChannelStreamType,
        pub stream: C2RustUnnamed_32,
        pub is_rpc: bool,
        pub rpc: RpcState,
        pub term: *mut Terminal,
        pub on_data: CallbackReader,
        pub on_stderr: CallbackReader,
        pub on_exit: Callback,
        pub exit_status: i32,
        pub callback_busy: bool,
        pub callback_scheduled: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct CallbackReader {
        pub cb: Callback,
        pub self_0: *mut dict_T,
        pub buffer: garray_T,
        pub eof: bool,
        pub buffered: bool,
        pub type_0: *const i8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed_32 {
        pub proc_0: Process,
        pub uv: LibuvProcess,
        pub pty: PtyProcess,
        pub socket: Stream,
        pub stdio: StdioPair,
        pub err: StderrState,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct StderrState {
        pub closed: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct StdioPair {
        pub in_0: Stream,
        pub out: Stream,
    }
    pub type ChannelStreamType = u32;
    pub const kChannelStreamInternal: ChannelStreamType = 4;
    pub const kChannelStreamStderr: ChannelStreamType = 3;
    pub const kChannelStreamStdio: ChannelStreamType = 2;
    pub const kChannelStreamSocket: ChannelStreamType = 1;
    pub const kChannelStreamProc: ChannelStreamType = 0;
    #[inline]
    pub unsafe extern "C" fn callback_reader_set(mut reader: CallbackReader) -> bool {
        return reader.cb.type_0 as u32 != kCallbackNone as i32 as u32 || !reader.self_0.is_null();
    }
    // / @returns Channel with the id or NULL if not found
    #[inline]
    pub unsafe extern "C" fn find_channel(mut id: u64) -> *mut Channel {
        return map_uint64_t_ptr_t_get(channels, id) as *mut Channel;
    }
    #[inline]
    pub unsafe extern "C" fn channel_instream(mut chan: *mut Channel) -> *mut Stream {
        match (*chan).streamtype as u32 {
            0 => return &mut (*chan).stream.proc_0.in_0,
            1 => return &mut (*chan).stream.socket,
            2 => return &mut (*chan).stream.stdio.out,
            4 | 3 => {
                abort();
            }
            _ => {}
        }
        abort();
    }
    #[inline]
    pub unsafe extern "C" fn channel_outstream(mut chan: *mut Channel) -> *mut Stream {
        match (*chan).streamtype as u32 {
            0 => return &mut (*chan).stream.proc_0.out,
            1 => return &mut (*chan).stream.socket,
            2 => return &mut (*chan).stream.stdio.in_0,
            4 | 3 => {
                abort();
            }
            _ => {}
        }
        abort();
    }
    extern "C" {
        #[no_mangle]
        pub static mut channels: *mut Map_uint64_t_ptr_t;
    }
    // NVIM_CHANNEL_H
}
pub mod channel_defs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct RpcState {
        pub subscribed_events: *mut Map_cstr_t_ptr_t,
        pub closed: bool,
        pub unpacker: *mut msgpack_unpacker,
        pub next_request_id: u32,
        pub call_stack: C2RustUnnamed_31,
        pub info: Dictionary,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2RustUnnamed_31 {
        pub size: size_t,
        pub capacity: size_t,
        pub items: *mut *mut ChannelCallFrame,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ChannelCallFrame {
        pub request_id: u32,
        pub returned: bool,
        pub errored: bool,
        pub result: Object,
    }
    // NVIM_MSGPACK_RPC_CHANNEL_DEFS_H
}
pub mod fileio_h {
    // / Structure used to read from/write to file
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct FileDescriptor {
        pub fd: i32,
        pub _error: i32,
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
    pub unsafe extern "C" fn file_eof(fp: *const FileDescriptor) -> bool {
        return (*fp).eof as i32 != 0 && rbuffer_size((*fp).rv) == 0;
    }
    // / Return the file descriptor associated with the FileDescriptor structure
    // /
    // / @param[in]  fp  File to check.
    // /
    // / @return File descriptor.
    #[inline]
    pub unsafe extern "C" fn file_fd(fp: *const FileDescriptor) -> i32 {
        return (*fp).fd;
    }
    // NVIM_OS_FILEIO_H
}
pub mod stdint_h {
    pub const SIZE_MAX: u64 = 18446744073709551615;
}
pub mod stdbool_h {
    pub const true_0: i32 = 1;
    pub const false_0: i32 = 0;
}
pub mod string_h {
    extern "C" {
        #[no_mangle]
        pub fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
        #[no_mangle]
        pub fn strcmp(_: *const i8, _: *const i8) -> i32;
        #[no_mangle]
        pub fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
        #[no_mangle]
        pub fn strchr(_: *const i8, _: i32) -> *mut i8;
        #[no_mangle]
        pub fn strlen(_: *const i8) -> u64;
        #[no_mangle]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    }
}
pub mod strings_h {
    extern "C" {
        #[no_mangle]
        pub fn strncasecmp(_: *const i8, _: *const i8, _: u64) -> i32;
    }
}
pub mod byteswap_h {
    #[inline]
    pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
        return (__bsx & 0xff000000 as u32) >> 24 | (__bsx & 0xff0000 as u32) >> 8 | (__bsx & 0xff00 as u32) << 8 | (__bsx & 0xff as u32) << 24;
    }
    #[inline]
    pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
        return (__bsx as i32 >> 8 & 0xff as i32 | (__bsx as i32 & 0xff as i32) << 8) as __uint16_t;
    }
    #[inline]
    pub unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
        return ((__bsx as libc::c_ulonglong & 0xff00000000000000 as libc::c_ulonglong) >> 56
            | (__bsx as libc::c_ulonglong & 0xff000000000000 as libc::c_ulonglong) >> 40
            | (__bsx as libc::c_ulonglong & 0xff0000000000 as libc::c_ulonglong) >> 24
            | (__bsx as libc::c_ulonglong & 0xff00000000 as libc::c_ulonglong) >> 8
            | (__bsx as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong) << 8
            | (__bsx as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong) << 24
            | (__bsx as libc::c_ulonglong & 0xff00 as libc::c_ulonglong) << 40
            | (__bsx as libc::c_ulonglong & 0xff as libc::c_ulonglong) << 56) as __uint64_t;
    }
}
pub mod uintn_identity_h {
    #[inline]
    pub unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
        return __x;
    }
    #[inline]
    pub unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t) -> __uint32_t {
        return __x;
    }
    #[inline]
    pub unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t) -> __uint16_t {
        return __x;
    }
}
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        pub fn malloc(_: u64) -> *mut libc::c_void;
        #[no_mangle]
        pub fn calloc(_: u64, _: u64) -> *mut libc::c_void;
        #[no_mangle]
        pub fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
        #[no_mangle]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        pub fn abort() -> !;
    }
}
pub mod assert_h {
    pub const __ASSERT_FUNCTION: [i8; 50] = unsafe { *::std::mem::transmute::<&[u8; 50], &[i8; 50]>(b"int utfc_ptr2char_len(const u8 *, int *, int)\x00") };
    pub const __ASSERT_FUNCTION_0: [i8; 58] = unsafe { *::std::mem::transmute::<&[u8; 58], &[i8; 58]>(b"void tv_list_set_lock(list_T *const, const VarLockStatus)\x00") };
    extern "C" {
        #[no_mangle]
        pub fn __assert_fail(__assertion: *const i8, __file: *const i8, __line: u32, __function: *const i8) -> !;
    }
}
pub mod log_h {
    pub const WARN_LOG_LEVEL: i32 = 2;
    // NVIM_LOG_H
}
pub mod errno_h {
    pub const errno: i32 = *__errno_location();
    extern "C" {
        #[no_mangle]
        pub fn __errno_location() -> *mut i32;
    }
}
pub mod macros_h {
    pub const TOUPPER_LOC: unsafe extern "C" fn(_: i32) -> i32 = toupper;
    pub const TOLOWER_LOC: unsafe extern "C" fn(_: i32) -> i32 = tolower;
    // NVIM_MACROS_H
    // /
    // / PRAGMA_DIAG_PUSH_IGNORE_MISSING_PROTOTYPES
    // /
    // Type of read()/write() `count` param is platform-dependent.
    // Type of uv_buf_t.len is platform-dependent.
    // Related: https://github.com/libuv/libuv/pull/1236
    // / Change type of structure pointers: cast `struct a *` to `struct b *`
    // /
    // / Used to silence PVS errors.
    // /
    // / @param  Type  Structure to cast to.
    // / @param  obj  Object to cast.
    // /
    // / @return ((Type *)obj).
    // -V:STRUCT_CAST:641
    // Duplicated in os/win_defs.h to avoid include-order sensitivity.
    // / Get last array entry
    // /
    // / This should be called with a real array. Calling this with a pointer is an
    // / error.
}
pub mod vim_h {
    pub const OK: i32 = 1;
    pub const FAIL: i32 = 0;
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
    // Enums need a typecast to be used as array index (for Ultrix).
    // / Compare file names
    // /
    // / On some systems case in a file name does not matter, on others it does.
    // /
    // / @note Does not account for maximum name lengths and things like "../dir",
    // /       thus it is not 100% accurate. OS may also use different algorythm for
    // /       case-insensitive comparison.
    // /
    // / @param[in]  x  First file name to compare.
    // / @param[in]  y  Second file name to compare.
    // /
    // / @return 0 for equal file names, non-zero otherwise.
    // max nr of %<flag> in statusline
    // columns needed by shown command
    // Prefer using emsgf(), because perror() may send the output to the wrong
    // destination and mess up the screen.
}
pub mod errno_base_h {
    pub const E2BIG: i32 = 7;
    pub const EINVAL: i32 = 22;
}
pub mod nvim_iconv_h {
    // define some missing constants if necessary
    pub const ICONV_E2BIG: i32 = E2BIG;
    pub const ICONV_ERRNO: i32 = *__errno_location();
    pub const ICONV_EINVAL: i32 = EINVAL;
    pub const ICONV_EILSEQ: i32 = EILSEQ;
    // NVIM_ICONV_H
}
pub mod asm_generic_errno_h {
    pub const EILSEQ: i32 = 84;
}
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        pub fn gettext(__msgid: *const i8) -> *mut i8;
    }
}
pub mod nvim_strings_h {
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
    pub unsafe extern "C" fn strappend(dst: *mut i8, src: *const i8) -> *mut i8 {
        let src_len = strlen(src);
        return (memmove(dst as *mut libc::c_void, src as *const libc::c_void, src_len) as *mut i8).offset(src_len as isize);
    }
    // NVIM_STRINGS_H
}
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
    pub unsafe extern "C" fn _memcpy_free(dest: *mut libc::c_void, src: *mut libc::c_void, size: size_t) -> *mut libc::c_void {
        memcpy(dest, src, size);
        let mut ptr_ = &src as *const *mut libc::c_void as *mut *mut libc::c_void;
        xfree(*ptr_);
        *ptr_ = NULL_1 as *mut libc::c_void;
        return dest;
    }
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
pub mod in_h {
    extern "C" {
        #[no_mangle]
        pub fn ntohl(__netlong: u32) -> u32;
        #[no_mangle]
        pub fn ntohs(__netshort: uint16_t) -> uint16_t;
    }
}
pub mod ascii_h {
    // Definitions of various common control characters.
    pub const NUL: i32 = '\u{0}' as i32;
    pub const TAB: i32 = '\t' as i32;
    pub const NL: i32 = '\n' as i32;
    /* CR is used by Mac OS X */
    pub const CSI: i32 = 0x9b as i32;
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
    pub unsafe extern "C" fn ascii_iswhite(mut c: i32) -> bool {
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
    pub unsafe extern "C" fn ascii_isdigit(mut c: i32) -> bool {
        return c >= '0' as i32 && c <= '9' as i32;
    }
    // / Checks if `c` is a hexadecimal digit, that is, one of 0-9, a-f, A-F.
    // /
    // / @see {ascii_isdigit}
    #[inline(always)]
    pub unsafe extern "C" fn ascii_isxdigit(mut c: i32) -> bool {
        return c >= '0' as i32 && c <= '9' as i32 || c >= 'a' as i32 && c <= 'f' as i32 || c >= 'A' as i32 && c <= 'F' as i32;
    }
    // / Checks if `c` is an “identifier” character
    // /
    // / That is, whether it is alphanumeric character or underscore.
    #[inline(always)]
    pub unsafe extern "C" fn ascii_isident(mut c: i32) -> bool {
        return c as u32 >= 'A' as i32 as u32 && c as u32 <= 'Z' as i32 as u32 || c as u32 >= 'a' as i32 as u32 && c as u32 <= 'z' as i32 as u32 || ascii_isdigit(c) as i32 != 0 || c == '_' as i32;
    }
    // / Checks if `c` is a binary digit, that is, 0-1.
    // /
    // / @see {ascii_isdigit}
    #[inline(always)]
    pub unsafe extern "C" fn ascii_isbdigit(mut c: i32) -> bool {
        return c == '0' as i32 || c == '1' as i32;
    }
    // / Checks if `c` is a white-space character, that is,
    // / one of \f, \n, \r, \t, \v.
    // /
    // / @see {ascii_isdigit}
    #[inline(always)]
    pub unsafe extern "C" fn ascii_isspace(mut c: i32) -> bool {
        return c >= 9 && c <= 13 || c == ' ' as i32;
    }
    /* NVIM_ASCII_H */
}
pub mod charset_h {
    // / Check if `c` is one of the characters in 'breakat'.
    // / Used very often if 'linebreak' is set
    #[inline(always)]
    pub unsafe extern "C" fn vim_isbreak(mut c: i32) -> bool {
        return breakat_flags[c as u8 as usize] != 0;
    }
    extern "C" {
        #[no_mangle]
        pub fn char2cells(c: i32) -> i32;
        #[no_mangle]
        pub fn ptr2cells(p: *const u8) -> i32;
        #[no_mangle]
        pub fn vim_iswordc_tab(c: i32, chartab: *const u64) -> bool;
        #[no_mangle]
        pub fn vim_isprintc(c: i32) -> bool;
    }
    // NVIM_CHARSET_H
}
pub mod arabic_h {
    // / Whether c belongs to the range of Arabic characters that might be shaped.
    #[inline]
    pub unsafe extern "C" fn arabic_char(mut c: i32) -> bool {
        // return c >= a_HAMZA && c <= a_MINI_ALEF;
        return c >= 0x621 as i32 && c <= 0x670 as i32;
    }
    // NVIM_ARABIC_H
}
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
    pub unsafe extern "C" fn mark_local_index(name: i8) -> i32 {
        return if name as u32 >= 'a' as i32 as u32 && name as u32 <= 'z' as i32 as u32 {
            (name as i32) - 'a' as i32
        } else if name as i32 == '\"' as i32 {
            NMARKS
        } else if name as i32 == '^' as i32 {
            (NMARKS) + 1
        } else if name as i32 == '.' as i32 {
            (NMARKS) + 2
        } else {
            -(1)
        };
    }
    // / Return true if position a is before (less than) position b.
    // / Return true if position a and b are equal.
    #[inline(always)]
    pub unsafe extern "C" fn equalpos(mut a: pos_T, mut b: pos_T) -> bool {
        return a.lnum == b.lnum && a.col == b.col && a.coladd == b.coladd;
    }
    // / Return true if position a is less than or equal to b.
    #[inline(always)]
    pub unsafe extern "C" fn ltoreq(mut a: pos_T, mut b: pos_T) -> bool {
        return lt(a, b) as i32 != 0 || equalpos(a, b) as i32 != 0;
    }
    // / Clear the pos_T structure pointed to by a.
    #[inline(always)]
    pub unsafe extern "C" fn clearpos(mut a: *mut pos_T) {
        (*a).lnum = 0;
        (*a).col = 0;
        (*a).coladd = 0;
    }
    #[inline]
    pub unsafe extern "C" fn mark_global_index(name: i8) -> i32 {
        return if name as u32 >= 'A' as i32 as u32 && name as u32 <= 'Z' as i32 as u32 {
            (name as i32) - 'A' as i32
        } else if ascii_isdigit(name as i32) as i32 != 0 {
            (NMARKS) + (name as i32 - '0' as i32)
        } else {
            -(1)
        };
    }
    #[inline(always)]
    pub unsafe extern "C" fn lt(mut a: pos_T, mut b: pos_T) -> bool {
        if a.lnum != b.lnum {
            return a.lnum < b.lnum;
        } else if a.col != b.col {
            return a.col < b.col;
        } else {
            return a.coladd < b.coladd;
        };
    }
    // NVIM_MARK_H
}
pub mod unicode_tables_generated_h {
extern "C" {
    pub static mut toLower: [convertStruct; 172];
    pub static mut toUpper: [convertStruct; 187];
    pub static mut combining: [interval; 280];
    pub static mut foldCase: [convertStruct; 192];
    pub static mut doublewidth: [interval; 113];
    pub static mut ambiguous: [interval; 179];
    pub static mut emoji_all: [interval; 151];
    pub static mut emoji_width: [interval; 39];
}
}
