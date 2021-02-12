use ::libc;
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
    use super::types_h::{__int16_t, __int32_t, __int64_t, __int8_t};
}
pub mod stdint_uintn_h {
    pub type u8 = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type u32 = __uint32_t;
    pub type u64 = __uint64_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t, __uint8_t};
}
pub mod stddef_h {
    pub type size_t = u64;
    pub const NULL_0: i32 = 0;
    pub const NULL: i32 = 0;
    pub const NULL_1: i32 = 0;
    pub const NULL_2: i32 = 0;
}
pub mod stdarg_h {
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
pub mod wint_t_h {
    pub type wint_t = u32;
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
    use super::assert_h::{__assert_fail, __ASSERT_FUNCTION_0};
    use super::garray_h::garray_T;
    use super::hashtab_h::hashtab_T;
    use super::libintl_h::gettext;
    use super::message_h_generated_h::emsgf;
    use super::pos_h::linenr_T;
    use super::profile_h::proftime_T;
    use super::queue_h::{QUEUE, QUEUE_EMPTY};
    use super::stdbool_h::{false_0, true_0};
    use super::stddef_h::{size_t, NULL_1};
    use super::string_h::memset;
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
    extern "C" {
        #[no_mangle]
        pub fn __ctype_b_loc() -> *mut *const u16;
        #[no_mangle]
        pub fn toupper(_: i32) -> i32;
        #[no_mangle]
        pub fn tolower(_: i32) -> i32;
    }
}
pub mod stdio_h {
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
    extern "C" {
        #[no_mangle]
        pub fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    }
}
pub mod sys_types_h {
    pub type gid_t = __gid_t;
    pub type uid_t = __uid_t;
    use super::types_h::{__gid_t, __uid_t};
}
pub mod time_t_h {
    pub type time_t = __time_t;
    use super::types_h::__time_t;
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
    use super::pthreadtypes_arch_h::__pthread_rwlock_arch_t;
    use super::thread_shared_types_h::__pthread_mutex_s;
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
    use super::stddef_h::size_t;
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
    use super::log_h::WARN_LOG_LEVEL;
    use super::log_h_generated_h::logmsg;
    use super::stdbool_h::true_0;
    use super::stddef_h::{size_t, NULL};
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
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        pub fn iconv_open(__tocode: *const i8, __fromcode: *const i8) -> iconv_t;
        #[no_mangle]
        pub fn iconv(__cd: iconv_t, __inbuf: *mut *mut i8, __inbytesleft: *mut size_t, __outbuf: *mut *mut i8, __outbytesleft: *mut size_t) -> size_t;
        #[no_mangle]
        pub fn iconv_close(__cd: iconv_t) -> i32;
    }
}
pub mod mbyte_h {
    // For indirect
    // for u8
    /*
     * Return byte length of character that starts with byte "b".
     * Returns 1 for a single-byte character.
     * MB_BYTE2LEN_CHECK() can be used to count a special key as one byte.
     * Don't call MB_BYTE2LEN(b) with b < 0 or b > 255!
     */
    // max length of an unicode char
    /* properties used in enc_canon_table[] (first three mutually exclusive) */
    /* Unicode: Big endian */
    /* Unicode: Little endian */
    /* Unicode: UCS-2 */
    /* Unicode: UCS-4 */
    /* Unicode: UTF-16 */
    /* Latin1 */
    /* Latin9 */
    /* Mac Roman (not Macro Man! :-) */
    // TODO(bfredl): eventually we should keep only one of the namings
    // / Flags for vimconv_T
    pub type C2RustUnnamed_1 = u32;
    pub const CONV_ICONV: C2RustUnnamed_1 = 5;
    pub const CONV_TO_LATIN9: C2RustUnnamed_1 = 4;
    pub const CONV_TO_LATIN1: C2RustUnnamed_1 = 3;
    pub const CONV_9_TO_UTF8: C2RustUnnamed_1 = 2;
    pub const CONV_TO_UTF8: C2RustUnnamed_1 = 1;
    pub const CONV_NONE: C2RustUnnamed_1 = 0;
    // / Structure used for string conversions
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct vimconv_T {
        pub vc_type: i32,
        pub vc_factor: i32,
        pub vc_fd: iconv_t,
        pub vc_fail: bool,
    }
    pub const ENC_8BIT: i32 = 0x1 as i32;
    pub const ENC_DBCS: i32 = 0x2 as i32;
    pub const ENC_MACROMAN: i32 = 0x800 as i32;
    pub const ENC_4BYTE: i32 = 0x80 as i32;
    pub const ENC_ENDIAN_L: i32 = 0x20 as i32;
    pub const ENC_UNICODE: i32 = 0x4 as i32;
    pub const ENC_ENDIAN_B: i32 = 0x10 as i32;
    pub const ENC_2WORD: i32 = 0x100 as i32;
    pub const ENC_2BYTE: i32 = 0x40 as i32;
    pub const ENC_LATIN9: i32 = 0x400 as i32;
    pub const ENC_LATIN1: i32 = 0x200 as i32;
    pub const mb_ptr2len: unsafe extern "C" fn(_: *const u8) -> i32 = utfc_ptr2len;
    // / Compare strings
    // /
    // / @param[in]  ic  True if case is to be ignored.
    // /
    // / @return 0 if s1 == s2, <0 if s1 < s2, >0 if s1 > s2.
    #[inline]
    pub unsafe extern "C" fn mb_strcmp_ic(mut ic: bool, mut s1: *const i8, mut s2: *const i8) -> i32 {
        return if ic as i32 != 0 { mb_stricmp(s1, s2) } else { strcmp(s1, s2) };
    }
    use super::iconv_h::iconv_t;
    use super::string_h::strcmp;
    use super::{mb_stricmp, utfc_ptr2len};
    // NVIM_MBYTE_H
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
    use super::defs_h::Window;
    use super::fs_defs_h::FileID;
    use super::garray_h::garray_T;
    use super::grid_defs_h::ScreenGrid;
    use super::hashtab_h::hashtab_T;
    use super::map_h::{Map_uint64_t_ExtmarkItem, Map_uint64_t_ExtmarkNs};
    use super::mark_defs_h::{fmark_T, xfmark_T};
    use super::marktree_h::MarkTree;
    use super::memline_defs_h::memline_T;
    use super::nvim_types_h::{handle_T, u8, LuaRef};
    use super::option_defs_h::LastSet;
    use super::pos_h::{colnr_T, linenr_T, lpos_T, pos_T};
    use super::profile_h::proftime_T;
    use super::regexp_defs_h::{regmmatch_T, regprog_T};
    use super::sign_defs_h::signlist_T;
    use super::stddef_h::size_t;
    use super::stdint_intn_h::int16_t;
    use super::stdint_uintn_h::{u32, u64, u8, uint16_t};
    use super::syntax_defs_h::synstate_T;
    use super::terminal_h::Terminal;
    use super::time_t_h::time_t;
    use super::typval_h::{dict_T, sctx_T, typval_T, varnumber_T, Callback, ScopeDictDictItem};
    use super::undo_defs_h::{u_header_T, visualinfo_T};
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
    use super::extmark_defs_h::ExtmarkItem;
    use super::khash_h::{khint32_t, khint_t};
    use super::map_defs_h::{cstr_t, ptr_t};
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
    use super::stddef_h::size_t;
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
    use super::map_h::Map_uint64_t_ptr_t;
    use super::stddef_h::size_t;
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
    use super::pos_h::linenr_T;
    use super::stdint_uintn_h::uint16_t;
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
    use super::buffer_defs_h::{buf_T, win_T};
    use super::pos_h::{colnr_T, linenr_T, lpos_T};
    use super::profile_h::proftime_T;
    use super::stdint_intn_h::int16_t;
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
    use super::nvim_types_h::{handle_T, LuaRef};
    use super::stddef_h::size_t;
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
    use super::nvim_types_h::{handle_T, u8};
    use super::stddef_h::size_t;
    use super::stdint_intn_h::int16_t;
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
    use super::pos_h::pos_T;
    use super::time_h::Timestamp;
    use super::typval_h::dict_T;
    // /< Actual mark.
    // /< File name, used when fnum == 0.
    // NVIM_MARK_DEFS_H
}
pub mod time_h {
    pub type Timestamp = u64;
    // NVIM_OS_TIME_H
}
pub mod option_defs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct LastSet {
        pub script_ctx: sctx_T,
        pub channel_id: u64,
    }
    pub const CMP_INTERNAL: i32 = 0x1 as i32;
    pub const CMP_KEEPASCII: i32 = 0x2 as i32;
    use super::typval_h::sctx_T;
    extern "C" {
        #[no_mangle]
        pub static mut p_ambw: *mut u8;
        #[no_mangle]
        pub static mut p_emoji: i32;
        #[no_mangle]
        pub static mut cmp_flags: u32;
        #[no_mangle]
        pub static mut p_enc: *mut u8;
        #[no_mangle]
        pub static mut breakat_flags: [i8; 256];
    }
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
    use super::buffer_defs_h::disptick_T;
    use super::garray_h::garray_T;
    use super::pos_h::linenr_T;
    use super::regexp_defs_h::reg_extmatch_T;
    use super::stdint_intn_h::int16_t;
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
    use super::extmark_defs_h::extmark_undo_vec_t;
    use super::mark_defs_h::fmark_T;
    use super::pos_h::{colnr_T, linenr_T, pos_T};
    use super::time_t_h::time_t;
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
    use super::memfile_defs_h::{bhdr_T, blocknr_T, memfile_T};
    use super::pos_h::linenr_T;
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
pub mod globals_h {
    pub type WorkingStatus = u32;
    pub const kBroken: WorkingStatus = 2;
    pub const kWorking: WorkingStatus = 1;
    pub const kUnknown: WorkingStatus = 0;
    pub const IOSIZE: i32 = 1024 + 1;
    // These flags are set based upon 'fileencoding'.
    // Note that "enc_utf8" is also set for "unicode", because the characters are
    // internally stored as UTF-8 (to avoid trouble with NUL bytes).
    pub const DBCS_JPN: i32 = 932;
    // japan
    pub const DBCS_JPNU: i32 = 9932;
    // euc-jp
    pub const DBCS_KOR: i32 = 949;
    // korea
    pub const DBCS_KORU: i32 = 9949;
    // euc-kr
    pub const DBCS_CHS: i32 = 936;
    // chinese
    pub const DBCS_CHSU: i32 = 9936;
    // euc-cn
    pub const DBCS_CHT: i32 = 950;
    // taiwan
    pub const DBCS_CHTU: i32 = 9950;
    // euc-tw
    // 2byte-
    pub const DBCS_DEBUG: i32 = -(1);
    use super::buffer_defs_h::{buf_T, win_T};
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
pub mod keymap_h {
    // keypad Delete key
    pub const KE_CSI: key_extra = 81;
    /*
     * translation of three byte code "K_SPECIAL a b" into int "K_xxx" and back
     */
    /*
     * get second or third byte when translating special key code into three bytes
     */
    /*
     * get single int code from second byte after K_SPECIAL
     */
    // Codes for keys that do not have a termcap name.
    // The numbers are fixed to make sure that recorded key sequences remain valid.
    // Add new entries at the end, not halfway.
    //
    // K_SPECIAL KS_EXTRA KE_xxx
    //
    // Entries must be in the range 0x02-0x7f (see comment at K_SPECIAL).
    pub type key_extra = u32;
    // <Cmd> special key
    // event
    pub const KE_COMMAND: key_extra = 104;
    // no-op: does nothing
    // , KE_FOCUSGAINED = 98    // focus gained
    // , KE_FOCUSLOST = 99      // focus lost
    // , KE_MOUSEMOVE = 100     // mouse moved with no button down
    // , KE_CANCEL = 101        // return from vgetc
    pub const KE_EVENT: key_extra = 102;
    // DnD data is available
    // , KE_CURSORHOLD = 96     // CursorHold event
    pub const KE_NOP: key_extra = 97;
    pub const KE_DROP: key_extra = 95;
    pub const KE_X2RELEASE: key_extra = 94;
    pub const KE_X2DRAG: key_extra = 93;
    pub const KE_X2MOUSE: key_extra = 92;
    pub const KE_X1RELEASE: key_extra = 91;
    // X1/X2 mouse-buttons
    pub const KE_X1DRAG: key_extra = 90;
    // control-end
    pub const KE_X1MOUSE: key_extra = 89;
    // control-home
    pub const KE_C_END: key_extra = 88;
    // control-right
    pub const KE_C_HOME: key_extra = 87;
    // control-left
    pub const KE_C_RIGHT: key_extra = 86;
    // open command-line window from Command-line Mode
    pub const KE_C_LEFT: key_extra = 85;
    // <Plug>
    pub const KE_CMDWIN: key_extra = 84;
    // <SNR>
    pub const KE_PLUG: key_extra = 83;
    // CSI typed directly
    pub const KE_SNR: key_extra = 82;
    // keypad Insert key
    pub const KE_KDEL: key_extra = 80;
    // scroll wheel pseudo-button Right
    pub const KE_KINS: key_extra = 79;
    // scroll wheel pseudo-button Left
    pub const KE_MOUSERIGHT: key_extra = 78;
    // scroll wheel pseudo-button Up
    pub const KE_MOUSELEFT: key_extra = 77;
    // scroll wheel pseudo-button Down
    pub const KE_MOUSEUP: key_extra = 76;
    // NOTE: The scroll wheel events are inverted: i.e. UP is the same as
    // moving the actual scroll wheel down, LEFT is the same as moving the
    // scroll wheel right.
    pub const KE_MOUSEDOWN: key_extra = 75;
    pub const KE_S_XF4: key_extra = 74;
    pub const KE_S_XF3: key_extra = 73;
    // vt100 shifted function keys for xterm
    pub const KE_S_XF2: key_extra = 72;
    // non-mappable left mouse button release
    pub const KE_S_XF1: key_extra = 71;
    // non-mappable Left mouse button click
    pub const KE_LEFTRELEASE_NM: key_extra = 70;
    pub const KE_LEFTMOUSE_NM: key_extra = 69;
    pub const KE_XRIGHT: key_extra = 68;
    pub const KE_XLEFT: key_extra = 67;
    // extra vt100 cursor keys for xterm
    pub const KE_XDOWN: key_extra = 66;
    // extra (vt100) home key for xterm
    pub const KE_XUP: key_extra = 65;
    // extra (vt100) home key for xterm
    pub const KE_ZHOME: key_extra = 64;
    // extra (vt100) end key for xterm
    pub const KE_XHOME: key_extra = 63;
    // extra (vt100) end key for xterm
    pub const KE_ZEND: key_extra = 62;
    pub const KE_XEND: key_extra = 61;
    pub const KE_XF4: key_extra = 60;
    pub const KE_XF3: key_extra = 59;
    // extra vt100 function keys for xterm
    pub const KE_XF2: key_extra = 58;
    // shifted TAB key (no longer used)
    // , KE_SNIFF_UNUSED = 56   // obsolete
    pub const KE_XF1: key_extra = 57;
    // unshifted TAB key
    pub const KE_S_TAB_OLD: key_extra = 55;
    // Ignored mouse drag/release
    pub const KE_TAB: key_extra = 54;
    // Right mouse button release
    pub const KE_IGNORE: key_extra = 53;
    // Drag with right mouse button down
    pub const KE_RIGHTRELEASE: key_extra = 52;
    // Right mouse button click
    pub const KE_RIGHTDRAG: key_extra = 51;
    // Middle mouse button release
    pub const KE_RIGHTMOUSE: key_extra = 50;
    // Drag with middle mouse button down
    pub const KE_MIDDLERELEASE: key_extra = 49;
    // Middle mouse button click
    pub const KE_MIDDLEDRAG: key_extra = 48;
    // Left mouse button release
    pub const KE_MIDDLEMOUSE: key_extra = 47;
    // Drag with left mouse button down
    pub const KE_LEFTRELEASE: key_extra = 46;
    // Left mouse button click
    pub const KE_LEFTDRAG: key_extra = 45;
    // mouse event start
    // Symbols for pseudo keys which are translated from the real key symbols
    // above.
    pub const KE_LEFTMOUSE: key_extra = 44;
    pub const KE_MOUSE: key_extra = 43;
    pub const KE_S_F37: key_extra = 42;
    pub const KE_S_F36: key_extra = 41;
    pub const KE_S_F35: key_extra = 40;
    pub const KE_S_F34: key_extra = 39;
    pub const KE_S_F33: key_extra = 38;
    pub const KE_S_F32: key_extra = 37;
    pub const KE_S_F31: key_extra = 36;
    pub const KE_S_F30: key_extra = 35;
    pub const KE_S_F29: key_extra = 34;
    pub const KE_S_F28: key_extra = 33;
    pub const KE_S_F27: key_extra = 32;
    pub const KE_S_F26: key_extra = 31;
    pub const KE_S_F25: key_extra = 30;
    pub const KE_S_F24: key_extra = 29;
    pub const KE_S_F23: key_extra = 28;
    pub const KE_S_F22: key_extra = 27;
    pub const KE_S_F21: key_extra = 26;
    pub const KE_S_F20: key_extra = 25;
    pub const KE_S_F19: key_extra = 24;
    pub const KE_S_F18: key_extra = 23;
    pub const KE_S_F17: key_extra = 22;
    pub const KE_S_F16: key_extra = 21;
    pub const KE_S_F15: key_extra = 20;
    pub const KE_S_F14: key_extra = 19;
    pub const KE_S_F13: key_extra = 18;
    pub const KE_S_F12: key_extra = 17;
    pub const KE_S_F11: key_extra = 16;
    pub const KE_S_F10: key_extra = 15;
    pub const KE_S_F9: key_extra = 14;
    pub const KE_S_F8: key_extra = 13;
    pub const KE_S_F7: key_extra = 12;
    pub const KE_S_F6: key_extra = 11;
    pub const KE_S_F5: key_extra = 10;
    pub const KE_S_F4: key_extra = 9;
    pub const KE_S_F3: key_extra = 8;
    // shifted function keys
    pub const KE_S_F2: key_extra = 7;
    // shift-down
    pub const KE_S_F1: key_extra = 6;
    // shift-up
    pub const KE_S_DOWN: key_extra = 5;
    // name of this terminal entry
    pub const KE_S_UP: key_extra = 4;
    pub const KE_NAME: key_extra = 3;
    /*
     * Keycode definitions for special keys.
     *
     * Any special key code sequences are replaced by these codes.
     */
    /*
     * For MSDOS some keys produce codes larger than 0xff. They are split into two
     * chars, the first one is K_NUL.
     */
    /* for MSDOS: special key follows */
    /*
     * K_SPECIAL is the first byte of a special key code and is always followed by
     * two bytes.
     * The second byte can have any value. ASCII is used for normal termcap
     * entries, 0x80 and higher for special keys, see below.
     * The third byte is guaranteed to be between 0x02 and 0x7f.
     */
    pub const K_SPECIAL: i32 = 0x80 as i32;
    /*
     * Positive characters are "normal" characters.
     * Negative characters are special key codes.  Only characters below -0x200
     * are used to so that the absolute value can't be mistaken for a single-byte
     * character.
     */
    /*
     * Characters 0x0100 - 0x01ff have a special meaning for abbreviations.
     * Multi-byte characters also have ABBR_OFF added, thus are above 0x0200.
     */
    /*
     * NUL cannot be in the input string, therefore it is replaced by
     *	K_SPECIAL   KS_ZERO	KE_FILLER
     */
    /*
     * K_SPECIAL cannot be in the input string, therefore it is replaced by
     *	K_SPECIAL   KS_SPECIAL	KE_FILLER
     */
    /*
     * KS_EXTRA is used for keys that have no termcap name
     *	K_SPECIAL   KS_EXTRA	KE_xxx
     */
    pub const KS_EXTRA: i32 = 253;
    /*
     * KS_MODIFIER is used when a modifier is given for a (special) key
     *	K_SPECIAL   KS_MODIFIER	bitmask
     */
    /*
     * These are used for the GUI
     *	K_SPECIAL   KS_xxx	KE_FILLER
     */
    /*
     * Used for switching Select mode back on after a mapping or menu.
     */
    /* Used a termcap entry that produces a normal character. */
    /* Used for click in a tab pages label. */
    /* Used for menu in a tab pages line. */
    /*
     * Filler used after KS_SPECIAL and others
     */
    pub const KE_FILLER: i32 = 'X' as i32;
    pub const KS_SPECIAL: i32 = 254;
    // NVIM_KEYMAP_H
    // Maximum length of a special key event as tokens.  This includes modifiers.
    // The longest event is something like <M-C-S-T-4-LeftDrag> which would be the
    // following string of tokens:
    //
    // <K_SPECIAL> <KS_MODIFIER> bitmask <K_SPECIAL> <KS_EXTRA> <KE_LEFTDRAG>.
    //
    // This is a total of 6 tokens, and is currently the longest one possible.
    /*
     * The length of the longest special key name, including modifiers.
     * Current longest is <M-C-S-T-D-A-4-ScrollWheelRight> (length includes '<' and
     * '>').
     */
    // "super" key (macOS: command-key)
    // use MOD_MASK_MULTI_CLICK
    // use MOD_MASK_MULTI_CLICK
    // use MOD_MASK_MULTI_CLICK
    // META when it's different from ALT
    // aka META
    /* Bits for modifier mask */
    /* 0x01 cannot be used, because the modifier must be 0x02 or higher */
    /*
     * Symbols for pseudo keys which are translated from the real key symbols
     * above.
     */
    // keypad equal
    // keypad comma
    // keypad 9
    // keypad 8
    // keypad 7
    // keypad 6
    // keypad 5
    // keypad 4
    // keypad 3
    // keypad 2
    // keypad 1
    // keypad 0
    // keypad . or ,
    // keypad Enter
    // keypad *
    // keypad /
    // keypad minus
    // keypad plus
    // keypad center
    // keypad pagedown (lower R.)
    // keypad pageup (upper R.)
    // keypad end (lower left)
    // keypad home (upper left)
    /* K_S_F13 to K_S_F37  are currently not used */
    // shifted func. keys
    /* extra set of shifted function keys F1-F4, for vt100 compatible xterm */
    // function keys
    /* extra set of cursor keys for vt100 compatible xterm */
    /* extra set of function keys F1-F4, for vt100 compatible xterm */
    // keypad right
    // keypad left
    // keypad down
    // keypad up
    /*
     * the three byte codes are replaced with the following int when using vgetc()
     */
}
pub mod langinfo_h {
    pub const CODESET: C2RustUnnamed_33 = 14;
    pub type C2RustUnnamed_33 = u32;
    pub const _NL_NUM: C2RustUnnamed_33 = 786449;
    pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed_33 = 786448;
    pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed_33 = 786447;
    pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed_33 = 786446;
    pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed_33 = 786445;
    pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed_33 = 786444;
    pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed_33 = 786443;
    pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed_33 = 786442;
    pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed_33 = 786441;
    pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed_33 = 786440;
    pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed_33 = 786439;
    pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed_33 = 786438;
    pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed_33 = 786437;
    pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed_33 = 786436;
    pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed_33 = 786435;
    pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed_33 = 786434;
    pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed_33 = 786433;
    pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed_33 = 786432;
    pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed_33 = 720898;
    pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed_33 = 720897;
    pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed_33 = 720896;
    pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed_33 = 655365;
    pub const _NL_TELEPHONE_CODESET: C2RustUnnamed_33 = 655364;
    pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed_33 = 655363;
    pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed_33 = 655362;
    pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed_33 = 655361;
    pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed_33 = 655360;
    pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed_33 = 589837;
    pub const _NL_ADDRESS_CODESET: C2RustUnnamed_33 = 589836;
    pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed_33 = 589835;
    pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed_33 = 589834;
    pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed_33 = 589833;
    pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed_33 = 589832;
    pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed_33 = 589831;
    pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed_33 = 589830;
    pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed_33 = 589829;
    pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed_33 = 589828;
    pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed_33 = 589827;
    pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed_33 = 589826;
    pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed_33 = 589825;
    pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed_33 = 589824;
    pub const _NL_NUM_LC_NAME: C2RustUnnamed_33 = 524295;
    pub const _NL_NAME_CODESET: C2RustUnnamed_33 = 524294;
    pub const _NL_NAME_NAME_MS: C2RustUnnamed_33 = 524293;
    pub const _NL_NAME_NAME_MISS: C2RustUnnamed_33 = 524292;
    pub const _NL_NAME_NAME_MRS: C2RustUnnamed_33 = 524291;
    pub const _NL_NAME_NAME_MR: C2RustUnnamed_33 = 524290;
    pub const _NL_NAME_NAME_GEN: C2RustUnnamed_33 = 524289;
    pub const _NL_NAME_NAME_FMT: C2RustUnnamed_33 = 524288;
    pub const _NL_NUM_LC_PAPER: C2RustUnnamed_33 = 458755;
    pub const _NL_PAPER_CODESET: C2RustUnnamed_33 = 458754;
    pub const _NL_PAPER_WIDTH: C2RustUnnamed_33 = 458753;
    pub const _NL_PAPER_HEIGHT: C2RustUnnamed_33 = 458752;
    pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed_33 = 327685;
    pub const _NL_MESSAGES_CODESET: C2RustUnnamed_33 = 327684;
    pub const __NOSTR: C2RustUnnamed_33 = 327683;
    pub const __YESSTR: C2RustUnnamed_33 = 327682;
    pub const __NOEXPR: C2RustUnnamed_33 = 327681;
    pub const __YESEXPR: C2RustUnnamed_33 = 327680;
    pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed_33 = 65542;
    pub const _NL_NUMERIC_CODESET: C2RustUnnamed_33 = 65541;
    pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed_33 = 65540;
    pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed_33 = 65539;
    pub const __GROUPING: C2RustUnnamed_33 = 65538;
    pub const THOUSEP: C2RustUnnamed_33 = 65537;
    pub const __THOUSANDS_SEP: C2RustUnnamed_33 = 65537;
    pub const RADIXCHAR: C2RustUnnamed_33 = 65536;
    pub const __DECIMAL_POINT: C2RustUnnamed_33 = 65536;
    pub const _NL_NUM_LC_MONETARY: C2RustUnnamed_33 = 262190;
    pub const _NL_MONETARY_CODESET: C2RustUnnamed_33 = 262189;
    pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed_33 = 262188;
    pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed_33 = 262187;
    pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed_33 = 262186;
    pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed_33 = 262185;
    pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed_33 = 262184;
    pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed_33 = 262183;
    pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed_33 = 262182;
    pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed_33 = 262181;
    pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed_33 = 262180;
    pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed_33 = 262179;
    pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed_33 = 262178;
    pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed_33 = 262177;
    pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed_33 = 262176;
    pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed_33 = 262175;
    pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed_33 = 262174;
    pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed_33 = 262173;
    pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed_33 = 262172;
    pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed_33 = 262171;
    pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed_33 = 262170;
    pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed_33 = 262169;
    pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed_33 = 262168;
    pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed_33 = 262167;
    pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed_33 = 262166;
    pub const __INT_N_SIGN_POSN: C2RustUnnamed_33 = 262165;
    pub const __INT_P_SIGN_POSN: C2RustUnnamed_33 = 262164;
    pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed_33 = 262163;
    pub const __INT_N_CS_PRECEDES: C2RustUnnamed_33 = 262162;
    pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed_33 = 262161;
    pub const __INT_P_CS_PRECEDES: C2RustUnnamed_33 = 262160;
    pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed_33 = 262159;
    pub const __N_SIGN_POSN: C2RustUnnamed_33 = 262158;
    pub const __P_SIGN_POSN: C2RustUnnamed_33 = 262157;
    pub const __N_SEP_BY_SPACE: C2RustUnnamed_33 = 262156;
    pub const __N_CS_PRECEDES: C2RustUnnamed_33 = 262155;
    pub const __P_SEP_BY_SPACE: C2RustUnnamed_33 = 262154;
    pub const __P_CS_PRECEDES: C2RustUnnamed_33 = 262153;
    pub const __FRAC_DIGITS: C2RustUnnamed_33 = 262152;
    pub const __INT_FRAC_DIGITS: C2RustUnnamed_33 = 262151;
    pub const __NEGATIVE_SIGN: C2RustUnnamed_33 = 262150;
    pub const __POSITIVE_SIGN: C2RustUnnamed_33 = 262149;
    pub const __MON_GROUPING: C2RustUnnamed_33 = 262148;
    pub const __MON_THOUSANDS_SEP: C2RustUnnamed_33 = 262147;
    pub const __MON_DECIMAL_POINT: C2RustUnnamed_33 = 262146;
    pub const __CURRENCY_SYMBOL: C2RustUnnamed_33 = 262145;
    pub const __INT_CURR_SYMBOL: C2RustUnnamed_33 = 262144;
    pub const _NL_NUM_LC_CTYPE: C2RustUnnamed_33 = 86;
    pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed_33 = 85;
    pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed_33 = 84;
    pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed_33 = 83;
    pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed_33 = 82;
    pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed_33 = 81;
    pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed_33 = 80;
    pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed_33 = 79;
    pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed_33 = 78;
    pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed_33 = 77;
    pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed_33 = 76;
    pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed_33 = 75;
    pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed_33 = 74;
    pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed_33 = 73;
    pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed_33 = 72;
    pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed_33 = 71;
    pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed_33 = 70;
    pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed_33 = 69;
    pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed_33 = 68;
    pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed_33 = 67;
    pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed_33 = 66;
    pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed_33 = 65;
    pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed_33 = 64;
    pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed_33 = 63;
    pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed_33 = 62;
    pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed_33 = 61;
    pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed_33 = 60;
    pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed_33 = 59;
    pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed_33 = 58;
    pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed_33 = 57;
    pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed_33 = 56;
    pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed_33 = 55;
    pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed_33 = 54;
    pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed_33 = 53;
    pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed_33 = 52;
    pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed_33 = 51;
    pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed_33 = 50;
    pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed_33 = 49;
    pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed_33 = 48;
    pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed_33 = 47;
    pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed_33 = 46;
    pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed_33 = 45;
    pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed_33 = 44;
    pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed_33 = 43;
    pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed_33 = 42;
    pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed_33 = 41;
    pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed_33 = 40;
    pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed_33 = 39;
    pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed_33 = 38;
    pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed_33 = 37;
    pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed_33 = 36;
    pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed_33 = 35;
    pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed_33 = 34;
    pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed_33 = 33;
    pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed_33 = 32;
    pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed_33 = 31;
    pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed_33 = 30;
    pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed_33 = 29;
    pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed_33 = 28;
    pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed_33 = 27;
    pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed_33 = 26;
    pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed_33 = 25;
    pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed_33 = 24;
    pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed_33 = 23;
    pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed_33 = 22;
    pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed_33 = 21;
    pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed_33 = 20;
    pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed_33 = 19;
    pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed_33 = 18;
    pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed_33 = 17;
    pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed_33 = 16;
    pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed_33 = 15;
    pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed_33 = 14;
    pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed_33 = 13;
    pub const _NL_CTYPE_WIDTH: C2RustUnnamed_33 = 12;
    pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed_33 = 11;
    pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed_33 = 10;
    pub const _NL_CTYPE_GAP6: C2RustUnnamed_33 = 9;
    pub const _NL_CTYPE_GAP5: C2RustUnnamed_33 = 8;
    pub const _NL_CTYPE_GAP4: C2RustUnnamed_33 = 7;
    pub const _NL_CTYPE_GAP3: C2RustUnnamed_33 = 6;
    pub const _NL_CTYPE_CLASS32: C2RustUnnamed_33 = 5;
    pub const _NL_CTYPE_GAP2: C2RustUnnamed_33 = 4;
    pub const _NL_CTYPE_TOLOWER: C2RustUnnamed_33 = 3;
    pub const _NL_CTYPE_GAP1: C2RustUnnamed_33 = 2;
    pub const _NL_CTYPE_TOUPPER: C2RustUnnamed_33 = 1;
    pub const _NL_CTYPE_CLASS: C2RustUnnamed_33 = 0;
    pub const _NL_NUM_LC_COLLATE: C2RustUnnamed_33 = 196627;
    pub const _NL_COLLATE_CODESET: C2RustUnnamed_33 = 196626;
    pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed_33 = 196625;
    pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed_33 = 196624;
    pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed_33 = 196623;
    pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed_33 = 196622;
    pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed_33 = 196621;
    pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed_33 = 196620;
    pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed_33 = 196619;
    pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed_33 = 196618;
    pub const _NL_COLLATE_TABLEWC: C2RustUnnamed_33 = 196617;
    pub const _NL_COLLATE_GAP3: C2RustUnnamed_33 = 196616;
    pub const _NL_COLLATE_GAP2: C2RustUnnamed_33 = 196615;
    pub const _NL_COLLATE_GAP1: C2RustUnnamed_33 = 196614;
    pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed_33 = 196613;
    pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed_33 = 196612;
    pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed_33 = 196611;
    pub const _NL_COLLATE_TABLEMB: C2RustUnnamed_33 = 196610;
    pub const _NL_COLLATE_RULESETS: C2RustUnnamed_33 = 196609;
    pub const _NL_COLLATE_NRULES: C2RustUnnamed_33 = 196608;
    pub const _NL_NUM_LC_TIME: C2RustUnnamed_33 = 131231;
    pub const _NL_WABALTMON_12: C2RustUnnamed_33 = 131230;
    pub const _NL_WABALTMON_11: C2RustUnnamed_33 = 131229;
    pub const _NL_WABALTMON_10: C2RustUnnamed_33 = 131228;
    pub const _NL_WABALTMON_9: C2RustUnnamed_33 = 131227;
    pub const _NL_WABALTMON_8: C2RustUnnamed_33 = 131226;
    pub const _NL_WABALTMON_7: C2RustUnnamed_33 = 131225;
    pub const _NL_WABALTMON_6: C2RustUnnamed_33 = 131224;
    pub const _NL_WABALTMON_5: C2RustUnnamed_33 = 131223;
    pub const _NL_WABALTMON_4: C2RustUnnamed_33 = 131222;
    pub const _NL_WABALTMON_3: C2RustUnnamed_33 = 131221;
    pub const _NL_WABALTMON_2: C2RustUnnamed_33 = 131220;
    pub const _NL_WABALTMON_1: C2RustUnnamed_33 = 131219;
    pub const _NL_ABALTMON_12: C2RustUnnamed_33 = 131218;
    pub const _NL_ABALTMON_11: C2RustUnnamed_33 = 131217;
    pub const _NL_ABALTMON_10: C2RustUnnamed_33 = 131216;
    pub const _NL_ABALTMON_9: C2RustUnnamed_33 = 131215;
    pub const _NL_ABALTMON_8: C2RustUnnamed_33 = 131214;
    pub const _NL_ABALTMON_7: C2RustUnnamed_33 = 131213;
    pub const _NL_ABALTMON_6: C2RustUnnamed_33 = 131212;
    pub const _NL_ABALTMON_5: C2RustUnnamed_33 = 131211;
    pub const _NL_ABALTMON_4: C2RustUnnamed_33 = 131210;
    pub const _NL_ABALTMON_3: C2RustUnnamed_33 = 131209;
    pub const _NL_ABALTMON_2: C2RustUnnamed_33 = 131208;
    pub const _NL_ABALTMON_1: C2RustUnnamed_33 = 131207;
    pub const _NL_WALTMON_12: C2RustUnnamed_33 = 131206;
    pub const _NL_WALTMON_11: C2RustUnnamed_33 = 131205;
    pub const _NL_WALTMON_10: C2RustUnnamed_33 = 131204;
    pub const _NL_WALTMON_9: C2RustUnnamed_33 = 131203;
    pub const _NL_WALTMON_8: C2RustUnnamed_33 = 131202;
    pub const _NL_WALTMON_7: C2RustUnnamed_33 = 131201;
    pub const _NL_WALTMON_6: C2RustUnnamed_33 = 131200;
    pub const _NL_WALTMON_5: C2RustUnnamed_33 = 131199;
    pub const _NL_WALTMON_4: C2RustUnnamed_33 = 131198;
    pub const _NL_WALTMON_3: C2RustUnnamed_33 = 131197;
    pub const _NL_WALTMON_2: C2RustUnnamed_33 = 131196;
    pub const _NL_WALTMON_1: C2RustUnnamed_33 = 131195;
    pub const __ALTMON_12: C2RustUnnamed_33 = 131194;
    pub const __ALTMON_11: C2RustUnnamed_33 = 131193;
    pub const __ALTMON_10: C2RustUnnamed_33 = 131192;
    pub const __ALTMON_9: C2RustUnnamed_33 = 131191;
    pub const __ALTMON_8: C2RustUnnamed_33 = 131190;
    pub const __ALTMON_7: C2RustUnnamed_33 = 131189;
    pub const __ALTMON_6: C2RustUnnamed_33 = 131188;
    pub const __ALTMON_5: C2RustUnnamed_33 = 131187;
    pub const __ALTMON_4: C2RustUnnamed_33 = 131186;
    pub const __ALTMON_3: C2RustUnnamed_33 = 131185;
    pub const __ALTMON_2: C2RustUnnamed_33 = 131184;
    pub const __ALTMON_1: C2RustUnnamed_33 = 131183;
    pub const _NL_TIME_CODESET: C2RustUnnamed_33 = 131182;
    pub const _NL_W_DATE_FMT: C2RustUnnamed_33 = 131181;
    pub const _DATE_FMT: C2RustUnnamed_33 = 131180;
    pub const _NL_TIME_TIMEZONE: C2RustUnnamed_33 = 131179;
    pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed_33 = 131178;
    pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed_33 = 131177;
    pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed_33 = 131176;
    pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed_33 = 131175;
    pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed_33 = 131174;
    pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed_33 = 131173;
    pub const _NL_WERA_T_FMT: C2RustUnnamed_33 = 131172;
    pub const _NL_WERA_D_T_FMT: C2RustUnnamed_33 = 131171;
    pub const _NL_WALT_DIGITS: C2RustUnnamed_33 = 131170;
    pub const _NL_WERA_D_FMT: C2RustUnnamed_33 = 131169;
    pub const _NL_WERA_YEAR: C2RustUnnamed_33 = 131168;
    pub const _NL_WT_FMT_AMPM: C2RustUnnamed_33 = 131167;
    pub const _NL_WT_FMT: C2RustUnnamed_33 = 131166;
    pub const _NL_WD_FMT: C2RustUnnamed_33 = 131165;
    pub const _NL_WD_T_FMT: C2RustUnnamed_33 = 131164;
    pub const _NL_WPM_STR: C2RustUnnamed_33 = 131163;
    pub const _NL_WAM_STR: C2RustUnnamed_33 = 131162;
    pub const _NL_WMON_12: C2RustUnnamed_33 = 131161;
    pub const _NL_WMON_11: C2RustUnnamed_33 = 131160;
    pub const _NL_WMON_10: C2RustUnnamed_33 = 131159;
    pub const _NL_WMON_9: C2RustUnnamed_33 = 131158;
    pub const _NL_WMON_8: C2RustUnnamed_33 = 131157;
    pub const _NL_WMON_7: C2RustUnnamed_33 = 131156;
    pub const _NL_WMON_6: C2RustUnnamed_33 = 131155;
    pub const _NL_WMON_5: C2RustUnnamed_33 = 131154;
    pub const _NL_WMON_4: C2RustUnnamed_33 = 131153;
    pub const _NL_WMON_3: C2RustUnnamed_33 = 131152;
    pub const _NL_WMON_2: C2RustUnnamed_33 = 131151;
    pub const _NL_WMON_1: C2RustUnnamed_33 = 131150;
    pub const _NL_WABMON_12: C2RustUnnamed_33 = 131149;
    pub const _NL_WABMON_11: C2RustUnnamed_33 = 131148;
    pub const _NL_WABMON_10: C2RustUnnamed_33 = 131147;
    pub const _NL_WABMON_9: C2RustUnnamed_33 = 131146;
    pub const _NL_WABMON_8: C2RustUnnamed_33 = 131145;
    pub const _NL_WABMON_7: C2RustUnnamed_33 = 131144;
    pub const _NL_WABMON_6: C2RustUnnamed_33 = 131143;
    pub const _NL_WABMON_5: C2RustUnnamed_33 = 131142;
    pub const _NL_WABMON_4: C2RustUnnamed_33 = 131141;
    pub const _NL_WABMON_3: C2RustUnnamed_33 = 131140;
    pub const _NL_WABMON_2: C2RustUnnamed_33 = 131139;
    pub const _NL_WABMON_1: C2RustUnnamed_33 = 131138;
    pub const _NL_WDAY_7: C2RustUnnamed_33 = 131137;
    pub const _NL_WDAY_6: C2RustUnnamed_33 = 131136;
    pub const _NL_WDAY_5: C2RustUnnamed_33 = 131135;
    pub const _NL_WDAY_4: C2RustUnnamed_33 = 131134;
    pub const _NL_WDAY_3: C2RustUnnamed_33 = 131133;
    pub const _NL_WDAY_2: C2RustUnnamed_33 = 131132;
    pub const _NL_WDAY_1: C2RustUnnamed_33 = 131131;
    pub const _NL_WABDAY_7: C2RustUnnamed_33 = 131130;
    pub const _NL_WABDAY_6: C2RustUnnamed_33 = 131129;
    pub const _NL_WABDAY_5: C2RustUnnamed_33 = 131128;
    pub const _NL_WABDAY_4: C2RustUnnamed_33 = 131127;
    pub const _NL_WABDAY_3: C2RustUnnamed_33 = 131126;
    pub const _NL_WABDAY_2: C2RustUnnamed_33 = 131125;
    pub const _NL_WABDAY_1: C2RustUnnamed_33 = 131124;
    pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed_33 = 131123;
    pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed_33 = 131122;
    pub const ERA_T_FMT: C2RustUnnamed_33 = 131121;
    pub const ERA_D_T_FMT: C2RustUnnamed_33 = 131120;
    pub const ALT_DIGITS: C2RustUnnamed_33 = 131119;
    pub const ERA_D_FMT: C2RustUnnamed_33 = 131118;
    pub const __ERA_YEAR: C2RustUnnamed_33 = 131117;
    pub const ERA: C2RustUnnamed_33 = 131116;
    pub const T_FMT_AMPM: C2RustUnnamed_33 = 131115;
    pub const T_FMT: C2RustUnnamed_33 = 131114;
    pub const D_FMT: C2RustUnnamed_33 = 131113;
    pub const D_T_FMT: C2RustUnnamed_33 = 131112;
    pub const PM_STR: C2RustUnnamed_33 = 131111;
    pub const AM_STR: C2RustUnnamed_33 = 131110;
    pub const MON_12: C2RustUnnamed_33 = 131109;
    pub const MON_11: C2RustUnnamed_33 = 131108;
    pub const MON_10: C2RustUnnamed_33 = 131107;
    pub const MON_9: C2RustUnnamed_33 = 131106;
    pub const MON_8: C2RustUnnamed_33 = 131105;
    pub const MON_7: C2RustUnnamed_33 = 131104;
    pub const MON_6: C2RustUnnamed_33 = 131103;
    pub const MON_5: C2RustUnnamed_33 = 131102;
    pub const MON_4: C2RustUnnamed_33 = 131101;
    pub const MON_3: C2RustUnnamed_33 = 131100;
    pub const MON_2: C2RustUnnamed_33 = 131099;
    pub const MON_1: C2RustUnnamed_33 = 131098;
    pub const ABMON_12: C2RustUnnamed_33 = 131097;
    pub const ABMON_11: C2RustUnnamed_33 = 131096;
    pub const ABMON_10: C2RustUnnamed_33 = 131095;
    pub const ABMON_9: C2RustUnnamed_33 = 131094;
    pub const ABMON_8: C2RustUnnamed_33 = 131093;
    pub const ABMON_7: C2RustUnnamed_33 = 131092;
    pub const ABMON_6: C2RustUnnamed_33 = 131091;
    pub const ABMON_5: C2RustUnnamed_33 = 131090;
    pub const ABMON_4: C2RustUnnamed_33 = 131089;
    pub const ABMON_3: C2RustUnnamed_33 = 131088;
    pub const ABMON_2: C2RustUnnamed_33 = 131087;
    pub const ABMON_1: C2RustUnnamed_33 = 131086;
    pub const DAY_7: C2RustUnnamed_33 = 131085;
    pub const DAY_6: C2RustUnnamed_33 = 131084;
    pub const DAY_5: C2RustUnnamed_33 = 131083;
    pub const DAY_4: C2RustUnnamed_33 = 131082;
    pub const DAY_3: C2RustUnnamed_33 = 131081;
    pub const DAY_2: C2RustUnnamed_33 = 131080;
    pub const DAY_1: C2RustUnnamed_33 = 131079;
    pub const ABDAY_7: C2RustUnnamed_33 = 131078;
    pub const ABDAY_6: C2RustUnnamed_33 = 131077;
    pub const ABDAY_5: C2RustUnnamed_33 = 131076;
    pub const ABDAY_4: C2RustUnnamed_33 = 131075;
    pub const ABDAY_3: C2RustUnnamed_33 = 131074;
    pub const ABDAY_2: C2RustUnnamed_33 = 131073;
    pub const ABDAY_1: C2RustUnnamed_33 = 131072;
    pub const CODESET_0: i32 = CODESET as i32;
    use super::nl_types_h::nl_item;
    extern "C" {
        #[no_mangle]
        pub fn nl_langinfo(__item: nl_item) -> *mut i8;
    }
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
    use super::stddef_h::size_t;
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
    use super::stddef_h::size_t;
    use super::stdio_h::ssize_t;
    use super::unix_h::{uv__io_t, uv_buf_t, uv_gid_t, uv_mutex_t, uv_rwlock_t, uv_uid_t};
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
    use super::pthreadtypes_h::{pthread_mutex_t, pthread_rwlock_t};
    use super::stddef_h::size_t;
    use super::sys_types_h::{gid_t, uid_t};
    use super::uv_h::uv_loop_s;
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
    use super::assert_h::__assert_fail;
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
    use super::assert_h::__assert_fail;
    use super::memory_h_generated_h::{xcalloc, xfree, xrealloc};
    use super::multiqueue_h::MultiQueue;
    use super::stddef_h::{size_t, NULL_2};
    use super::unix_h::uv_mutex_t;
    use super::uv_h::{uv_async_t, uv_loop_t, uv_signal_t, uv_timer_t};
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
    use super::stddef_h::size_t;
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
    use super::multiqueue_h::MultiQueue;
    use super::rbuffer_h::RBuffer;
    use super::stddef_h::size_t;
    use super::unix_h::{uv_buf_t, uv_file};
    use super::uv_h::{uv_idle_t, uv_pipe_t, uv_stream_t, uv_tcp_t};
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
    use super::loop_h::Loop;
    use super::multiqueue_h::MultiQueue;
    use super::rbuffer_h::RBuffer;
    use super::stdbool_h::false_0;
    use super::stddef_h::{size_t, NULL_0};
    use super::stdint_uintn_h::{u64, u8};
    use super::stream_h::{stream_close_cb, stream_read_cb, stream_write_cb, C2RustUnnamed_28, Stream};
    use super::unix_h::{uv__io_cb, uv__io_t, uv_buf_t, uv_file};
    use super::uv_h::{uv_alloc_cb, uv_close_cb, uv_connect_t, uv_connection_cb, uv_handle_t, uv_handle_type, uv_loop_t, uv_pipe_t, uv_read_cb, uv_shutdown_t, uv_stream_t, C2RustUnnamed_23, UV_UNKNOWN_HANDLE};
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
    use super::ioctl_types_h::winsize;
    use super::loop_h::Loop;
    use super::multiqueue_h::MultiQueue;
    use super::process_h::{internal_process_cb, kProcessTypePty, process_exit_cb, process_init, Process, ProcessType};
    use super::rbuffer_h::RBuffer;
    use super::stddef_h::{size_t, NULL_0};
    use super::stdint_uintn_h::{u64, u8, uint16_t};
    use super::stream_h::{stream_close_cb, stream_read_cb, stream_write_cb, C2RustUnnamed_28, Stream};
    use super::unix_h::{uv_buf_t, uv_file};
    use super::uv_h::uv_stream_t;
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
    use super::loop_h::Loop;
    use super::process_h::{kProcessTypeUv, process_init, Process, ProcessType};
    use super::unix_h::{uv_gid_t, uv_uid_t};
    use super::uv_h::{uv_close_cb, uv_exit_cb, uv_handle_t, uv_handle_type, uv_loop_t, uv_process_options_t, uv_process_t, uv_stdio_container_t, uv_stdio_flags, uv_stream_t, C2RustUnnamed_26, C2RustUnnamed_27, UV_IGNORE};
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
    use super::stdbool_h::true_0;
    use super::stddef_h::{size_t, NULL_0};
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
    use super::stdint_intn_h::{i64, i8};
    use super::stdint_uintn_h::{u32, u64};
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
    use super::stddef_h::{size_t, NULL_0};
    use super::stdlib_h::{calloc, free};
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
    use super::byteswap_h::__bswap_64;
    use super::in_h::{ntohl, ntohs};
    use super::pack_h::msgpack_packer;
    use super::stddef_h::size_t;
    use super::stdint_intn_h::{i32, i64, i8, int16_t};
    use super::stdint_uintn_h::{u32, u64, u8, uint16_t};
    use super::string_h::memcpy;
    use super::types_h::__uint64_t;
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
    use super::object_h::msgpack_object;
    use super::stdbool_h::true_0;
    use super::stddef_h::{size_t, NULL_0};
    use super::string_h::memset;
    use super::zone_h::{msgpack_zone, msgpack_zone_free};
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
    use super::stddef_h::{size_t, NULL_0};
    use super::stdlib_h::{calloc, free, realloc};
    use super::string_h::{memcpy, memset};
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
    use super::stddef_h::{size_t, NULL_0};
    use super::stdlib_h::{free, malloc};
    use super::struct_iovec_h::iovec;
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
    use super::channel_defs_h::RpcState;
    use super::garray_h::garray_T;
    use super::libuv_process_h::LibuvProcess;
    use super::map_h::{map_uint64_t_ptr_t_get, Map_uint64_t_ptr_t};
    use super::multiqueue_h::MultiQueue;
    use super::process_h::Process;
    use super::pty_process_unix_h::PtyProcess;
    use super::stddef_h::size_t;
    use super::stdlib_h::abort;
    use super::stream_h::Stream;
    use super::terminal_h::Terminal;
    use super::typval_h::{dict_T, kCallbackNone, C2RustUnnamed_8, Callback, CallbackType};
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
    use super::defs_h::{Dictionary, Object};
    use super::map_h::Map_cstr_t_ptr_t;
    use super::stddef_h::size_t;
    use super::unpack_h::msgpack_unpacker;
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
    use super::rbuffer_h::{rbuffer_size, RBuffer};
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
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t};
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
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t};
}
pub mod wctype_wchar_h {
    use super::wint_t_h::wint_t;
    extern "C" {
        #[no_mangle]
        pub fn towlower(__wc: wint_t) -> wint_t;
        #[no_mangle]
        pub fn towupper(__wc: wint_t) -> wint_t;
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
    use super::ctype_h::{tolower, toupper};
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
    use super::asm_generic_errno_h::EILSEQ;
    use super::errno_base_h::{E2BIG, EINVAL};
    use super::errno_h::__errno_location;
    // NVIM_ICONV_H
}
pub mod asm_generic_errno_h {
    pub const EILSEQ: i32 = 84;
}
pub mod strings_h_generated_h {
    extern "C" {
        #[no_mangle]
        pub fn vim_strsave(string: *const u8) -> *mut u8;
        #[no_mangle]
        pub fn vim_strchr(string: *const u8, c: i32) -> *mut u8;
    }
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
    use super::stddef_h::size_t;
    use super::string_h::{memmove, strlen};
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
    use super::memory_h_generated_h::xfree;
    use super::stddef_h::{size_t, NULL_1};
    use super::string_h::memcpy;
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
    use super::stdint_uintn_h::{u32, uint16_t};
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
pub mod locale_h {
    pub const __LC_CTYPE: i32 = 0;
}
pub mod include_locale_h {
    pub const LC_CTYPE: i32 = __LC_CTYPE;
    use super::locale_h::__LC_CTYPE;
    extern "C" {
        #[no_mangle]
        pub fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    }
}
pub mod charset_h {
    // / Check if `c` is one of the characters in 'breakat'.
    // / Used very often if 'linebreak' is set
    #[inline(always)]
    pub unsafe extern "C" fn vim_isbreak(mut c: i32) -> bool {
        return breakat_flags[c as u8 as usize] != 0;
    }
    use super::option_defs_h::breakat_flags;
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
pub mod cursor_h_generated_h {
    extern "C" {
        #[no_mangle]
        pub fn get_cursor_pos_ptr() -> *mut u8;
    }
}
pub mod memline_h_generated_h {
    use super::buffer_defs_h::buf_T;
    use super::pos_h::linenr_T;
    extern "C" {
        #[no_mangle]
        pub fn ml_get_buf(buf: *mut buf_T, lnum: linenr_T, will_change: bool) -> *mut u8;
    }
}
pub mod misc1_h_generated_h {
    extern "C" {
        #[no_mangle]
        pub fn beep_flush();
    }
}
pub mod arabic_h_generated_h {
    extern "C" {
        #[no_mangle]
        pub fn arabic_combine(one: i32, two: i32) -> bool;
        #[no_mangle]
        pub fn arabic_maycombine(two: i32) -> bool;
    }
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
    use super::ascii_h::ascii_isdigit;
    use super::mark_defs_h::NMARKS;
    use super::pos_h::{colnr_T, linenr_T, pos_T};
    // NVIM_MARK_H
}
pub mod mark_h_generated_h {
    use super::buffer_defs_h::buf_T;
    use super::pos_h::pos_T;
    extern "C" {
        #[no_mangle]
        pub fn mark_mb_adjustpos(buf: *mut buf_T, lp: *mut pos_T);
    }
}
pub mod unicode_tables_generated_h {
    pub static mut toLower: [convertStruct; 172] = [
        {
            let mut init = convertStruct {
                rangeStart: 0x41 as i32,
                rangeEnd: 0x5a as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xc0 as i32,
                rangeEnd: 0xd6 as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xd8 as i32,
                rangeEnd: 0xde as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x100 as i32,
                rangeEnd: 0x12e as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x130 as i32,
                rangeEnd: 0x130 as i32,
                step: -(1),
                offset: -(199),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x132 as i32,
                rangeEnd: 0x136 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x139 as i32,
                rangeEnd: 0x147 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x14a as i32,
                rangeEnd: 0x176 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x178 as i32,
                rangeEnd: 0x178 as i32,
                step: -(1),
                offset: -(121),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x179 as i32,
                rangeEnd: 0x17d as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x181 as i32,
                rangeEnd: 0x181 as i32,
                step: -(1),
                offset: 210,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x182 as i32,
                rangeEnd: 0x184 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x186 as i32,
                rangeEnd: 0x186 as i32,
                step: -(1),
                offset: 206,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x187 as i32,
                rangeEnd: 0x187 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x189 as i32,
                rangeEnd: 0x18a as i32,
                step: 1,
                offset: 205,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x18b as i32,
                rangeEnd: 0x18b as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x18e as i32,
                rangeEnd: 0x18e as i32,
                step: -(1),
                offset: 79,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x18f as i32,
                rangeEnd: 0x18f as i32,
                step: -(1),
                offset: 202,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x190 as i32,
                rangeEnd: 0x190 as i32,
                step: -(1),
                offset: 203,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x191 as i32,
                rangeEnd: 0x191 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x193 as i32,
                rangeEnd: 0x193 as i32,
                step: -(1),
                offset: 205,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x194 as i32,
                rangeEnd: 0x194 as i32,
                step: -(1),
                offset: 207,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x196 as i32,
                rangeEnd: 0x196 as i32,
                step: -(1),
                offset: 211,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x197 as i32,
                rangeEnd: 0x197 as i32,
                step: -(1),
                offset: 209,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x198 as i32,
                rangeEnd: 0x198 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x19c as i32,
                rangeEnd: 0x19c as i32,
                step: -(1),
                offset: 211,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x19d as i32,
                rangeEnd: 0x19d as i32,
                step: -(1),
                offset: 213,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x19f as i32,
                rangeEnd: 0x19f as i32,
                step: -(1),
                offset: 214,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1a0 as i32,
                rangeEnd: 0x1a4 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1a6 as i32,
                rangeEnd: 0x1a6 as i32,
                step: -(1),
                offset: 218,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1a7 as i32,
                rangeEnd: 0x1a7 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1a9 as i32,
                rangeEnd: 0x1a9 as i32,
                step: -(1),
                offset: 218,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ac as i32,
                rangeEnd: 0x1ac as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ae as i32,
                rangeEnd: 0x1ae as i32,
                step: -(1),
                offset: 218,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1af as i32,
                rangeEnd: 0x1af as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1b1 as i32,
                rangeEnd: 0x1b2 as i32,
                step: 1,
                offset: 217,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1b3 as i32,
                rangeEnd: 0x1b5 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1b7 as i32,
                rangeEnd: 0x1b7 as i32,
                step: -(1),
                offset: 219,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1b8 as i32,
                rangeEnd: 0x1bc as i32,
                step: 4,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c4 as i32,
                rangeEnd: 0x1c4 as i32,
                step: -(1),
                offset: 2,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c5 as i32,
                rangeEnd: 0x1c5 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c7 as i32,
                rangeEnd: 0x1c7 as i32,
                step: -(1),
                offset: 2,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c8 as i32,
                rangeEnd: 0x1c8 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ca as i32,
                rangeEnd: 0x1ca as i32,
                step: -(1),
                offset: 2,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1cb as i32,
                rangeEnd: 0x1db as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1de as i32,
                rangeEnd: 0x1ee as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f1 as i32,
                rangeEnd: 0x1f1 as i32,
                step: -(1),
                offset: 2,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f2 as i32,
                rangeEnd: 0x1f4 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f6 as i32,
                rangeEnd: 0x1f6 as i32,
                step: -(1),
                offset: -(97),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f7 as i32,
                rangeEnd: 0x1f7 as i32,
                step: -(1),
                offset: -(56),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f8 as i32,
                rangeEnd: 0x21e as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x220 as i32,
                rangeEnd: 0x220 as i32,
                step: -(1),
                offset: -(130),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x222 as i32,
                rangeEnd: 0x232 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x23a as i32,
                rangeEnd: 0x23a as i32,
                step: -(1),
                offset: 10795,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x23b as i32,
                rangeEnd: 0x23b as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x23d as i32,
                rangeEnd: 0x23d as i32,
                step: -(1),
                offset: -(163),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x23e as i32,
                rangeEnd: 0x23e as i32,
                step: -(1),
                offset: 10792,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x241 as i32,
                rangeEnd: 0x241 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x243 as i32,
                rangeEnd: 0x243 as i32,
                step: -(1),
                offset: -(195),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x244 as i32,
                rangeEnd: 0x244 as i32,
                step: -(1),
                offset: 69,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x245 as i32,
                rangeEnd: 0x245 as i32,
                step: -(1),
                offset: 71,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x246 as i32,
                rangeEnd: 0x24e as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x370 as i32,
                rangeEnd: 0x372 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x376 as i32,
                rangeEnd: 0x376 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x37f as i32,
                rangeEnd: 0x37f as i32,
                step: -(1),
                offset: 116,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x386 as i32,
                rangeEnd: 0x386 as i32,
                step: -(1),
                offset: 38,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x388 as i32,
                rangeEnd: 0x38a as i32,
                step: 1,
                offset: 37,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x38c as i32,
                rangeEnd: 0x38c as i32,
                step: -(1),
                offset: 64,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x38e as i32,
                rangeEnd: 0x38f as i32,
                step: 1,
                offset: 63,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x391 as i32,
                rangeEnd: 0x3a1 as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3a3 as i32,
                rangeEnd: 0x3ab as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3cf as i32,
                rangeEnd: 0x3cf as i32,
                step: -(1),
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3d8 as i32,
                rangeEnd: 0x3ee as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3f4 as i32,
                rangeEnd: 0x3f4 as i32,
                step: -(1),
                offset: -(60),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3f7 as i32,
                rangeEnd: 0x3f7 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3f9 as i32,
                rangeEnd: 0x3f9 as i32,
                step: -(1),
                offset: -(7),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3fa as i32,
                rangeEnd: 0x3fa as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3fd as i32,
                rangeEnd: 0x3ff as i32,
                step: 1,
                offset: -(130),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x400 as i32,
                rangeEnd: 0x40f as i32,
                step: 1,
                offset: 80,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x410 as i32,
                rangeEnd: 0x42f as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x460 as i32,
                rangeEnd: 0x480 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x48a as i32,
                rangeEnd: 0x4be as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x4c0 as i32,
                rangeEnd: 0x4c0 as i32,
                step: -(1),
                offset: 15,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x4c1 as i32,
                rangeEnd: 0x4cd as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x4d0 as i32,
                rangeEnd: 0x52e as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x531 as i32,
                rangeEnd: 0x556 as i32,
                step: 1,
                offset: 48,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x10a0 as i32,
                rangeEnd: 0x10c5 as i32,
                step: 1,
                offset: 7264,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x10c7 as i32,
                rangeEnd: 0x10cd as i32,
                step: 6,
                offset: 7264,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x13a0 as i32,
                rangeEnd: 0x13ef as i32,
                step: 1,
                offset: 38864,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x13f0 as i32,
                rangeEnd: 0x13f5 as i32,
                step: 1,
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c90 as i32,
                rangeEnd: 0x1cba as i32,
                step: 1,
                offset: -(3008),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1cbd as i32,
                rangeEnd: 0x1cbf as i32,
                step: 1,
                offset: -(3008),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1e00 as i32,
                rangeEnd: 0x1e94 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1e9e as i32,
                rangeEnd: 0x1e9e as i32,
                step: -(1),
                offset: -(7615),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ea0 as i32,
                rangeEnd: 0x1efe as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f08 as i32,
                rangeEnd: 0x1f0f as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f18 as i32,
                rangeEnd: 0x1f1d as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f28 as i32,
                rangeEnd: 0x1f2f as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f38 as i32,
                rangeEnd: 0x1f3f as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f48 as i32,
                rangeEnd: 0x1f4d as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f59 as i32,
                rangeEnd: 0x1f5f as i32,
                step: 2,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f68 as i32,
                rangeEnd: 0x1f6f as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f88 as i32,
                rangeEnd: 0x1f8f as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f98 as i32,
                rangeEnd: 0x1f9f as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fa8 as i32,
                rangeEnd: 0x1faf as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fb8 as i32,
                rangeEnd: 0x1fb9 as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fba as i32,
                rangeEnd: 0x1fbb as i32,
                step: 1,
                offset: -(74),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fbc as i32,
                rangeEnd: 0x1fbc as i32,
                step: -(1),
                offset: -(9),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fc8 as i32,
                rangeEnd: 0x1fcb as i32,
                step: 1,
                offset: -(86),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fcc as i32,
                rangeEnd: 0x1fcc as i32,
                step: -(1),
                offset: -(9),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fd8 as i32,
                rangeEnd: 0x1fd9 as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fda as i32,
                rangeEnd: 0x1fdb as i32,
                step: 1,
                offset: -(100),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fe8 as i32,
                rangeEnd: 0x1fe9 as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fea as i32,
                rangeEnd: 0x1feb as i32,
                step: 1,
                offset: -(112),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fec as i32,
                rangeEnd: 0x1fec as i32,
                step: -(1),
                offset: -(7),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ff8 as i32,
                rangeEnd: 0x1ff9 as i32,
                step: 1,
                offset: -(128),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ffa as i32,
                rangeEnd: 0x1ffb as i32,
                step: 1,
                offset: -(126),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ffc as i32,
                rangeEnd: 0x1ffc as i32,
                step: -(1),
                offset: -(9),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2126 as i32,
                rangeEnd: 0x2126 as i32,
                step: -(1),
                offset: -(7517),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x212a as i32,
                rangeEnd: 0x212a as i32,
                step: -(1),
                offset: -(8383),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x212b as i32,
                rangeEnd: 0x212b as i32,
                step: -(1),
                offset: -(8262),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2132 as i32,
                rangeEnd: 0x2132 as i32,
                step: -(1),
                offset: 28,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2160 as i32,
                rangeEnd: 0x216f as i32,
                step: 1,
                offset: 16,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2183 as i32,
                rangeEnd: 0x2183 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x24b6 as i32,
                rangeEnd: 0x24cf as i32,
                step: 1,
                offset: 26,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c00 as i32,
                rangeEnd: 0x2c2e as i32,
                step: 1,
                offset: 48,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c60 as i32,
                rangeEnd: 0x2c60 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c62 as i32,
                rangeEnd: 0x2c62 as i32,
                step: -(1),
                offset: -(10743),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c63 as i32,
                rangeEnd: 0x2c63 as i32,
                step: -(1),
                offset: -(3814),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c64 as i32,
                rangeEnd: 0x2c64 as i32,
                step: -(1),
                offset: -(10727),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c67 as i32,
                rangeEnd: 0x2c6b as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c6d as i32,
                rangeEnd: 0x2c6d as i32,
                step: -(1),
                offset: -(10780),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c6e as i32,
                rangeEnd: 0x2c6e as i32,
                step: -(1),
                offset: -(10749),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c6f as i32,
                rangeEnd: 0x2c6f as i32,
                step: -(1),
                offset: -(10783),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c70 as i32,
                rangeEnd: 0x2c70 as i32,
                step: -(1),
                offset: -(10782),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c72 as i32,
                rangeEnd: 0x2c75 as i32,
                step: 3,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c7e as i32,
                rangeEnd: 0x2c7f as i32,
                step: 1,
                offset: -(10815),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c80 as i32,
                rangeEnd: 0x2ce2 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2ceb as i32,
                rangeEnd: 0x2ced as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2cf2 as i32,
                rangeEnd: 0xa640 as i32,
                step: 31054,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa642 as i32,
                rangeEnd: 0xa66c as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa680 as i32,
                rangeEnd: 0xa69a as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa722 as i32,
                rangeEnd: 0xa72e as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa732 as i32,
                rangeEnd: 0xa76e as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa779 as i32,
                rangeEnd: 0xa77b as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa77d as i32,
                rangeEnd: 0xa77d as i32,
                step: -(1),
                offset: -(35332),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa77e as i32,
                rangeEnd: 0xa786 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa78b as i32,
                rangeEnd: 0xa78b as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa78d as i32,
                rangeEnd: 0xa78d as i32,
                step: -(1),
                offset: -(42280),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa790 as i32,
                rangeEnd: 0xa792 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa796 as i32,
                rangeEnd: 0xa7a8 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7aa as i32,
                rangeEnd: 0xa7aa as i32,
                step: -(1),
                offset: -(42308),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7ab as i32,
                rangeEnd: 0xa7ab as i32,
                step: -(1),
                offset: -(42319),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7ac as i32,
                rangeEnd: 0xa7ac as i32,
                step: -(1),
                offset: -(42315),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7ad as i32,
                rangeEnd: 0xa7ad as i32,
                step: -(1),
                offset: -(42305),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7ae as i32,
                rangeEnd: 0xa7ae as i32,
                step: -(1),
                offset: -(42308),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7b0 as i32,
                rangeEnd: 0xa7b0 as i32,
                step: -(1),
                offset: -(42258),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7b1 as i32,
                rangeEnd: 0xa7b1 as i32,
                step: -(1),
                offset: -(42282),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7b2 as i32,
                rangeEnd: 0xa7b2 as i32,
                step: -(1),
                offset: -(42261),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7b3 as i32,
                rangeEnd: 0xa7b3 as i32,
                step: -(1),
                offset: 928,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7b4 as i32,
                rangeEnd: 0xa7be as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7c2 as i32,
                rangeEnd: 0xa7c2 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7c4 as i32,
                rangeEnd: 0xa7c4 as i32,
                step: -(1),
                offset: -(48),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7c5 as i32,
                rangeEnd: 0xa7c5 as i32,
                step: -(1),
                offset: -(42307),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7c6 as i32,
                rangeEnd: 0xa7c6 as i32,
                step: -(1),
                offset: -(35384),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xff21 as i32,
                rangeEnd: 0xff3a as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x10400 as i32,
                rangeEnd: 0x10427 as i32,
                step: 1,
                offset: 40,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x104b0 as i32,
                rangeEnd: 0x104d3 as i32,
                step: 1,
                offset: 40,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x10c80 as i32,
                rangeEnd: 0x10cb2 as i32,
                step: 1,
                offset: 64,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x118a0 as i32,
                rangeEnd: 0x118bf as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x16e40 as i32,
                rangeEnd: 0x16e5f as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1e900 as i32,
                rangeEnd: 0x1e921 as i32,
                step: 1,
                offset: 34,
            };
            init
        },
    ];
    pub static mut toUpper: [convertStruct; 187] = [
        {
            let mut init = convertStruct {
                rangeStart: 0x61 as i32,
                rangeEnd: 0x7a as i32,
                step: 1,
                offset: -(32),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xb5 as i32,
                rangeEnd: 0xb5 as i32,
                step: -(1),
                offset: 743,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xe0 as i32,
                rangeEnd: 0xf6 as i32,
                step: 1,
                offset: -(32),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xf8 as i32,
                rangeEnd: 0xfe as i32,
                step: 1,
                offset: -(32),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xff as i32,
                rangeEnd: 0xff as i32,
                step: -(1),
                offset: 121,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x101 as i32,
                rangeEnd: 0x12f as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x131 as i32,
                rangeEnd: 0x131 as i32,
                step: -(1),
                offset: -(232),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x133 as i32,
                rangeEnd: 0x137 as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x13a as i32,
                rangeEnd: 0x148 as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x14b as i32,
                rangeEnd: 0x177 as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x17a as i32,
                rangeEnd: 0x17e as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x17f as i32,
                rangeEnd: 0x17f as i32,
                step: -(1),
                offset: -(300),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x180 as i32,
                rangeEnd: 0x180 as i32,
                step: -(1),
                offset: 195,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x183 as i32,
                rangeEnd: 0x185 as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x188 as i32,
                rangeEnd: 0x18c as i32,
                step: 4,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x192 as i32,
                rangeEnd: 0x192 as i32,
                step: -(1),
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x195 as i32,
                rangeEnd: 0x195 as i32,
                step: -(1),
                offset: 97,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x199 as i32,
                rangeEnd: 0x199 as i32,
                step: -(1),
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x19a as i32,
                rangeEnd: 0x19a as i32,
                step: -(1),
                offset: 163,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x19e as i32,
                rangeEnd: 0x19e as i32,
                step: -(1),
                offset: 130,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1a1 as i32,
                rangeEnd: 0x1a5 as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1a8 as i32,
                rangeEnd: 0x1ad as i32,
                step: 5,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1b0 as i32,
                rangeEnd: 0x1b4 as i32,
                step: 4,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1b6 as i32,
                rangeEnd: 0x1b9 as i32,
                step: 3,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1bd as i32,
                rangeEnd: 0x1bd as i32,
                step: -(1),
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1bf as i32,
                rangeEnd: 0x1bf as i32,
                step: -(1),
                offset: 56,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c5 as i32,
                rangeEnd: 0x1c5 as i32,
                step: -(1),
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c6 as i32,
                rangeEnd: 0x1c6 as i32,
                step: -(1),
                offset: -(2),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c8 as i32,
                rangeEnd: 0x1c8 as i32,
                step: -(1),
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c9 as i32,
                rangeEnd: 0x1c9 as i32,
                step: -(1),
                offset: -(2),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1cb as i32,
                rangeEnd: 0x1cb as i32,
                step: -(1),
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1cc as i32,
                rangeEnd: 0x1cc as i32,
                step: -(1),
                offset: -(2),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ce as i32,
                rangeEnd: 0x1dc as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1dd as i32,
                rangeEnd: 0x1dd as i32,
                step: -(1),
                offset: -(79),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1df as i32,
                rangeEnd: 0x1ef as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f2 as i32,
                rangeEnd: 0x1f2 as i32,
                step: -(1),
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f3 as i32,
                rangeEnd: 0x1f3 as i32,
                step: -(1),
                offset: -(2),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f5 as i32,
                rangeEnd: 0x1f9 as i32,
                step: 4,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fb as i32,
                rangeEnd: 0x21f as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x223 as i32,
                rangeEnd: 0x233 as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x23c as i32,
                rangeEnd: 0x23c as i32,
                step: -(1),
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x23f as i32,
                rangeEnd: 0x240 as i32,
                step: 1,
                offset: 10815,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x242 as i32,
                rangeEnd: 0x247 as i32,
                step: 5,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x249 as i32,
                rangeEnd: 0x24f as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x250 as i32,
                rangeEnd: 0x250 as i32,
                step: -(1),
                offset: 10783,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x251 as i32,
                rangeEnd: 0x251 as i32,
                step: -(1),
                offset: 10780,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x252 as i32,
                rangeEnd: 0x252 as i32,
                step: -(1),
                offset: 10782,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x253 as i32,
                rangeEnd: 0x253 as i32,
                step: -(1),
                offset: -(210),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x254 as i32,
                rangeEnd: 0x254 as i32,
                step: -(1),
                offset: -(206),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x256 as i32,
                rangeEnd: 0x257 as i32,
                step: 1,
                offset: -(205),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x259 as i32,
                rangeEnd: 0x259 as i32,
                step: -(1),
                offset: -(202),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x25b as i32,
                rangeEnd: 0x25b as i32,
                step: -(1),
                offset: -(203),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x25c as i32,
                rangeEnd: 0x25c as i32,
                step: -(1),
                offset: 42319,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x260 as i32,
                rangeEnd: 0x260 as i32,
                step: -(1),
                offset: -(205),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x261 as i32,
                rangeEnd: 0x261 as i32,
                step: -(1),
                offset: 42315,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x263 as i32,
                rangeEnd: 0x263 as i32,
                step: -(1),
                offset: -(207),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x265 as i32,
                rangeEnd: 0x265 as i32,
                step: -(1),
                offset: 42280,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x266 as i32,
                rangeEnd: 0x266 as i32,
                step: -(1),
                offset: 42308,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x268 as i32,
                rangeEnd: 0x268 as i32,
                step: -(1),
                offset: -(209),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x269 as i32,
                rangeEnd: 0x269 as i32,
                step: -(1),
                offset: -(211),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x26a as i32,
                rangeEnd: 0x26a as i32,
                step: -(1),
                offset: 42308,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x26b as i32,
                rangeEnd: 0x26b as i32,
                step: -(1),
                offset: 10743,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x26c as i32,
                rangeEnd: 0x26c as i32,
                step: -(1),
                offset: 42305,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x26f as i32,
                rangeEnd: 0x26f as i32,
                step: -(1),
                offset: -(211),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x271 as i32,
                rangeEnd: 0x271 as i32,
                step: -(1),
                offset: 10749,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x272 as i32,
                rangeEnd: 0x272 as i32,
                step: -(1),
                offset: -(213),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x275 as i32,
                rangeEnd: 0x275 as i32,
                step: -(1),
                offset: -(214),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x27d as i32,
                rangeEnd: 0x27d as i32,
                step: -(1),
                offset: 10727,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x280 as i32,
                rangeEnd: 0x280 as i32,
                step: -(1),
                offset: -(218),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x282 as i32,
                rangeEnd: 0x282 as i32,
                step: -(1),
                offset: 42307,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x283 as i32,
                rangeEnd: 0x283 as i32,
                step: -(1),
                offset: -(218),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x287 as i32,
                rangeEnd: 0x287 as i32,
                step: -(1),
                offset: 42282,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x288 as i32,
                rangeEnd: 0x288 as i32,
                step: -(1),
                offset: -(218),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x289 as i32,
                rangeEnd: 0x289 as i32,
                step: -(1),
                offset: -(69),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x28a as i32,
                rangeEnd: 0x28b as i32,
                step: 1,
                offset: -(217),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x28c as i32,
                rangeEnd: 0x28c as i32,
                step: -(1),
                offset: -(71),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x292 as i32,
                rangeEnd: 0x292 as i32,
                step: -(1),
                offset: -(219),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x29d as i32,
                rangeEnd: 0x29d as i32,
                step: -(1),
                offset: 42261,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x29e as i32,
                rangeEnd: 0x29e as i32,
                step: -(1),
                offset: 42258,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x345 as i32,
                rangeEnd: 0x345 as i32,
                step: -(1),
                offset: 84,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x371 as i32,
                rangeEnd: 0x373 as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x377 as i32,
                rangeEnd: 0x377 as i32,
                step: -(1),
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x37b as i32,
                rangeEnd: 0x37d as i32,
                step: 1,
                offset: 130,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3ac as i32,
                rangeEnd: 0x3ac as i32,
                step: -(1),
                offset: -(38),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3ad as i32,
                rangeEnd: 0x3af as i32,
                step: 1,
                offset: -(37),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3b1 as i32,
                rangeEnd: 0x3c1 as i32,
                step: 1,
                offset: -(32),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3c2 as i32,
                rangeEnd: 0x3c2 as i32,
                step: -(1),
                offset: -(31),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3c3 as i32,
                rangeEnd: 0x3cb as i32,
                step: 1,
                offset: -(32),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3cc as i32,
                rangeEnd: 0x3cc as i32,
                step: -(1),
                offset: -(64),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3cd as i32,
                rangeEnd: 0x3ce as i32,
                step: 1,
                offset: -(63),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3d0 as i32,
                rangeEnd: 0x3d0 as i32,
                step: -(1),
                offset: -(62),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3d1 as i32,
                rangeEnd: 0x3d1 as i32,
                step: -(1),
                offset: -(57),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3d5 as i32,
                rangeEnd: 0x3d5 as i32,
                step: -(1),
                offset: -(47),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3d6 as i32,
                rangeEnd: 0x3d6 as i32,
                step: -(1),
                offset: -(54),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3d7 as i32,
                rangeEnd: 0x3d7 as i32,
                step: -(1),
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3d9 as i32,
                rangeEnd: 0x3ef as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3f0 as i32,
                rangeEnd: 0x3f0 as i32,
                step: -(1),
                offset: -(86),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3f1 as i32,
                rangeEnd: 0x3f1 as i32,
                step: -(1),
                offset: -(80),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3f2 as i32,
                rangeEnd: 0x3f2 as i32,
                step: -(1),
                offset: 7,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3f3 as i32,
                rangeEnd: 0x3f3 as i32,
                step: -(1),
                offset: -(116),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3f5 as i32,
                rangeEnd: 0x3f5 as i32,
                step: -(1),
                offset: -(96),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3f8 as i32,
                rangeEnd: 0x3fb as i32,
                step: 3,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x430 as i32,
                rangeEnd: 0x44f as i32,
                step: 1,
                offset: -(32),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x450 as i32,
                rangeEnd: 0x45f as i32,
                step: 1,
                offset: -(80),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x461 as i32,
                rangeEnd: 0x481 as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x48b as i32,
                rangeEnd: 0x4bf as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x4c2 as i32,
                rangeEnd: 0x4ce as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x4cf as i32,
                rangeEnd: 0x4cf as i32,
                step: -(1),
                offset: -(15),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x4d1 as i32,
                rangeEnd: 0x52f as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x561 as i32,
                rangeEnd: 0x586 as i32,
                step: 1,
                offset: -(48),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x10d0 as i32,
                rangeEnd: 0x10fa as i32,
                step: 1,
                offset: 3008,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x10fd as i32,
                rangeEnd: 0x10ff as i32,
                step: 1,
                offset: 3008,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x13f8 as i32,
                rangeEnd: 0x13fd as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c80 as i32,
                rangeEnd: 0x1c80 as i32,
                step: -(1),
                offset: -(6254),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c81 as i32,
                rangeEnd: 0x1c81 as i32,
                step: -(1),
                offset: -(6253),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c82 as i32,
                rangeEnd: 0x1c82 as i32,
                step: -(1),
                offset: -(6244),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c83 as i32,
                rangeEnd: 0x1c84 as i32,
                step: 1,
                offset: -(6242),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c85 as i32,
                rangeEnd: 0x1c85 as i32,
                step: -(1),
                offset: -(6243),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c86 as i32,
                rangeEnd: 0x1c86 as i32,
                step: -(1),
                offset: -(6236),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c87 as i32,
                rangeEnd: 0x1c87 as i32,
                step: -(1),
                offset: -(6181),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c88 as i32,
                rangeEnd: 0x1c88 as i32,
                step: -(1),
                offset: 35266,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1d79 as i32,
                rangeEnd: 0x1d79 as i32,
                step: -(1),
                offset: 35332,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1d7d as i32,
                rangeEnd: 0x1d7d as i32,
                step: -(1),
                offset: 3814,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1d8e as i32,
                rangeEnd: 0x1d8e as i32,
                step: -(1),
                offset: 35384,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1e01 as i32,
                rangeEnd: 0x1e95 as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1e9b as i32,
                rangeEnd: 0x1e9b as i32,
                step: -(1),
                offset: -(59),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ea1 as i32,
                rangeEnd: 0x1eff as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f00 as i32,
                rangeEnd: 0x1f07 as i32,
                step: 1,
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f10 as i32,
                rangeEnd: 0x1f15 as i32,
                step: 1,
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f20 as i32,
                rangeEnd: 0x1f27 as i32,
                step: 1,
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f30 as i32,
                rangeEnd: 0x1f37 as i32,
                step: 1,
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f40 as i32,
                rangeEnd: 0x1f45 as i32,
                step: 1,
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f51 as i32,
                rangeEnd: 0x1f57 as i32,
                step: 2,
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f60 as i32,
                rangeEnd: 0x1f67 as i32,
                step: 1,
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f70 as i32,
                rangeEnd: 0x1f71 as i32,
                step: 1,
                offset: 74,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f72 as i32,
                rangeEnd: 0x1f75 as i32,
                step: 1,
                offset: 86,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f76 as i32,
                rangeEnd: 0x1f77 as i32,
                step: 1,
                offset: 100,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f78 as i32,
                rangeEnd: 0x1f79 as i32,
                step: 1,
                offset: 128,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f7a as i32,
                rangeEnd: 0x1f7b as i32,
                step: 1,
                offset: 112,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f7c as i32,
                rangeEnd: 0x1f7d as i32,
                step: 1,
                offset: 126,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f80 as i32,
                rangeEnd: 0x1f87 as i32,
                step: 1,
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f90 as i32,
                rangeEnd: 0x1f97 as i32,
                step: 1,
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fa0 as i32,
                rangeEnd: 0x1fa7 as i32,
                step: 1,
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fb0 as i32,
                rangeEnd: 0x1fb1 as i32,
                step: 1,
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fb3 as i32,
                rangeEnd: 0x1fb3 as i32,
                step: -(1),
                offset: 9,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fbe as i32,
                rangeEnd: 0x1fbe as i32,
                step: -(1),
                offset: -(7205),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fc3 as i32,
                rangeEnd: 0x1fc3 as i32,
                step: -(1),
                offset: 9,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fd0 as i32,
                rangeEnd: 0x1fd1 as i32,
                step: 1,
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fe0 as i32,
                rangeEnd: 0x1fe1 as i32,
                step: 1,
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fe5 as i32,
                rangeEnd: 0x1fe5 as i32,
                step: -(1),
                offset: 7,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ff3 as i32,
                rangeEnd: 0x1ff3 as i32,
                step: -(1),
                offset: 9,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x214e as i32,
                rangeEnd: 0x214e as i32,
                step: -(1),
                offset: -(28),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2170 as i32,
                rangeEnd: 0x217f as i32,
                step: 1,
                offset: -(16),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2184 as i32,
                rangeEnd: 0x2184 as i32,
                step: -(1),
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x24d0 as i32,
                rangeEnd: 0x24e9 as i32,
                step: 1,
                offset: -(26),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c30 as i32,
                rangeEnd: 0x2c5e as i32,
                step: 1,
                offset: -(48),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c61 as i32,
                rangeEnd: 0x2c61 as i32,
                step: -(1),
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c65 as i32,
                rangeEnd: 0x2c65 as i32,
                step: -(1),
                offset: -(10795),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c66 as i32,
                rangeEnd: 0x2c66 as i32,
                step: -(1),
                offset: -(10792),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c68 as i32,
                rangeEnd: 0x2c6c as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c73 as i32,
                rangeEnd: 0x2c76 as i32,
                step: 3,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c81 as i32,
                rangeEnd: 0x2ce3 as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2cec as i32,
                rangeEnd: 0x2cee as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2cf3 as i32,
                rangeEnd: 0x2cf3 as i32,
                step: -(1),
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2d00 as i32,
                rangeEnd: 0x2d25 as i32,
                step: 1,
                offset: -(7264),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2d27 as i32,
                rangeEnd: 0x2d2d as i32,
                step: 6,
                offset: -(7264),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa641 as i32,
                rangeEnd: 0xa66d as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa681 as i32,
                rangeEnd: 0xa69b as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa723 as i32,
                rangeEnd: 0xa72f as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa733 as i32,
                rangeEnd: 0xa76f as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa77a as i32,
                rangeEnd: 0xa77c as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa77f as i32,
                rangeEnd: 0xa787 as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa78c as i32,
                rangeEnd: 0xa791 as i32,
                step: 5,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa793 as i32,
                rangeEnd: 0xa793 as i32,
                step: -(1),
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa794 as i32,
                rangeEnd: 0xa794 as i32,
                step: -(1),
                offset: 48,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa797 as i32,
                rangeEnd: 0xa7a9 as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7b5 as i32,
                rangeEnd: 0xa7bf as i32,
                step: 2,
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7c3 as i32,
                rangeEnd: 0xa7c3 as i32,
                step: -(1),
                offset: -(1),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xab53 as i32,
                rangeEnd: 0xab53 as i32,
                step: -(1),
                offset: -(928),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xab70 as i32,
                rangeEnd: 0xabbf as i32,
                step: 1,
                offset: -(38864),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xff41 as i32,
                rangeEnd: 0xff5a as i32,
                step: 1,
                offset: -(32),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x10428 as i32,
                rangeEnd: 0x1044f as i32,
                step: 1,
                offset: -(40),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x104d8 as i32,
                rangeEnd: 0x104fb as i32,
                step: 1,
                offset: -(40),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x10cc0 as i32,
                rangeEnd: 0x10cf2 as i32,
                step: 1,
                offset: -(64),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x118c0 as i32,
                rangeEnd: 0x118df as i32,
                step: 1,
                offset: -(32),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x16e60 as i32,
                rangeEnd: 0x16e7f as i32,
                step: 1,
                offset: -(32),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1e922 as i32,
                rangeEnd: 0x1e943 as i32,
                step: 1,
                offset: -(34),
            };
            init
        },
    ];
    pub static mut combining: [interval; 280] = [
        {
            let mut init = interval { first: 0x300 as i32 as i64, last: 0x36f as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x483 as i32 as i64, last: 0x489 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x591 as i32 as i64, last: 0x5bd as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x5bf as i32 as i64, last: 0x5bf as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x5c1 as i32 as i64, last: 0x5c2 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x5c4 as i32 as i64, last: 0x5c5 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x5c7 as i32 as i64, last: 0x5c7 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x610 as i32 as i64, last: 0x61a as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x64b as i32 as i64, last: 0x65f as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x670 as i32 as i64, last: 0x670 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x6d6 as i32 as i64, last: 0x6dc as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x6df as i32 as i64, last: 0x6e4 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x6e7 as i32 as i64, last: 0x6e8 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x6ea as i32 as i64, last: 0x6ed as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x711 as i32 as i64, last: 0x711 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x730 as i32 as i64, last: 0x74a as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x7a6 as i32 as i64, last: 0x7b0 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x7eb as i32 as i64, last: 0x7f3 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x7fd as i32 as i64, last: 0x7fd as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x816 as i32 as i64, last: 0x819 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x81b as i32 as i64, last: 0x823 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x825 as i32 as i64, last: 0x827 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x829 as i32 as i64, last: 0x82d as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x859 as i32 as i64, last: 0x85b as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x8d3 as i32 as i64, last: 0x8e1 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x8e3 as i32 as i64, last: 0x903 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x93a as i32 as i64, last: 0x93c as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x93e as i32 as i64, last: 0x94f as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x951 as i32 as i64, last: 0x957 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x962 as i32 as i64, last: 0x963 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x981 as i32 as i64, last: 0x983 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x9bc as i32 as i64, last: 0x9bc as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x9be as i32 as i64, last: 0x9c4 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x9c7 as i32 as i64, last: 0x9c8 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x9cb as i32 as i64, last: 0x9cd as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x9d7 as i32 as i64, last: 0x9d7 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x9e2 as i32 as i64, last: 0x9e3 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x9fe as i32 as i64, last: 0x9fe as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xa01 as i32 as i64, last: 0xa03 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xa3c as i32 as i64, last: 0xa3c as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xa3e as i32 as i64, last: 0xa42 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xa47 as i32 as i64, last: 0xa48 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xa4b as i32 as i64, last: 0xa4d as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xa51 as i32 as i64, last: 0xa51 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xa70 as i32 as i64, last: 0xa71 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xa75 as i32 as i64, last: 0xa75 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xa81 as i32 as i64, last: 0xa83 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xabc as i32 as i64, last: 0xabc as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xabe as i32 as i64, last: 0xac5 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xac7 as i32 as i64, last: 0xac9 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xacb as i32 as i64, last: 0xacd as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xae2 as i32 as i64, last: 0xae3 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xafa as i32 as i64, last: 0xaff as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xb01 as i32 as i64, last: 0xb03 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xb3c as i32 as i64, last: 0xb3c as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xb3e as i32 as i64, last: 0xb44 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xb47 as i32 as i64, last: 0xb48 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xb4b as i32 as i64, last: 0xb4d as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xb56 as i32 as i64, last: 0xb57 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xb62 as i32 as i64, last: 0xb63 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xb82 as i32 as i64, last: 0xb82 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xbbe as i32 as i64, last: 0xbc2 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xbc6 as i32 as i64, last: 0xbc8 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xbca as i32 as i64, last: 0xbcd as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xbd7 as i32 as i64, last: 0xbd7 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xc00 as i32 as i64, last: 0xc04 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xc3e as i32 as i64, last: 0xc44 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xc46 as i32 as i64, last: 0xc48 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xc4a as i32 as i64, last: 0xc4d as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xc55 as i32 as i64, last: 0xc56 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xc62 as i32 as i64, last: 0xc63 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xc81 as i32 as i64, last: 0xc83 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xcbc as i32 as i64, last: 0xcbc as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xcbe as i32 as i64, last: 0xcc4 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xcc6 as i32 as i64, last: 0xcc8 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xcca as i32 as i64, last: 0xccd as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xcd5 as i32 as i64, last: 0xcd6 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xce2 as i32 as i64, last: 0xce3 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xd00 as i32 as i64, last: 0xd03 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xd3b as i32 as i64, last: 0xd3c as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xd3e as i32 as i64, last: 0xd44 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xd46 as i32 as i64, last: 0xd48 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xd4a as i32 as i64, last: 0xd4d as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xd57 as i32 as i64, last: 0xd57 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xd62 as i32 as i64, last: 0xd63 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xd82 as i32 as i64, last: 0xd83 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xdca as i32 as i64, last: 0xdca as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xdcf as i32 as i64, last: 0xdd4 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xdd6 as i32 as i64, last: 0xdd6 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xdd8 as i32 as i64, last: 0xddf as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xdf2 as i32 as i64, last: 0xdf3 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xe31 as i32 as i64, last: 0xe31 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xe34 as i32 as i64, last: 0xe3a as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xe47 as i32 as i64, last: 0xe4e as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xeb1 as i32 as i64, last: 0xeb1 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xeb4 as i32 as i64, last: 0xebc as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xec8 as i32 as i64, last: 0xecd as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xf18 as i32 as i64, last: 0xf19 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xf35 as i32 as i64, last: 0xf35 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xf37 as i32 as i64, last: 0xf37 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xf39 as i32 as i64, last: 0xf39 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xf3e as i32 as i64, last: 0xf3f as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xf71 as i32 as i64, last: 0xf84 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xf86 as i32 as i64, last: 0xf87 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xf8d as i32 as i64, last: 0xf97 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xf99 as i32 as i64, last: 0xfbc as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xfc6 as i32 as i64, last: 0xfc6 as i32 as i64 };
            init
        },
        {
            let mut init = interval {
                first: 0x102b as i32 as i64,
                last: 0x103e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1056 as i32 as i64,
                last: 0x1059 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x105e as i32 as i64,
                last: 0x1060 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1062 as i32 as i64,
                last: 0x1064 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1067 as i32 as i64,
                last: 0x106d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1071 as i32 as i64,
                last: 0x1074 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1082 as i32 as i64,
                last: 0x108d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x108f as i32 as i64,
                last: 0x108f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x109a as i32 as i64,
                last: 0x109d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x135d as i32 as i64,
                last: 0x135f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1712 as i32 as i64,
                last: 0x1714 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1732 as i32 as i64,
                last: 0x1734 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1752 as i32 as i64,
                last: 0x1753 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1772 as i32 as i64,
                last: 0x1773 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x17b4 as i32 as i64,
                last: 0x17d3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x17dd as i32 as i64,
                last: 0x17dd as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x180b as i32 as i64,
                last: 0x180d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1885 as i32 as i64,
                last: 0x1886 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x18a9 as i32 as i64,
                last: 0x18a9 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1920 as i32 as i64,
                last: 0x192b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1930 as i32 as i64,
                last: 0x193b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1a17 as i32 as i64,
                last: 0x1a1b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1a55 as i32 as i64,
                last: 0x1a5e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1a60 as i32 as i64,
                last: 0x1a7c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1a7f as i32 as i64,
                last: 0x1a7f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1ab0 as i32 as i64,
                last: 0x1abe as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b00 as i32 as i64,
                last: 0x1b04 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b34 as i32 as i64,
                last: 0x1b44 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b6b as i32 as i64,
                last: 0x1b73 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b80 as i32 as i64,
                last: 0x1b82 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1ba1 as i32 as i64,
                last: 0x1bad as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1be6 as i32 as i64,
                last: 0x1bf3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1c24 as i32 as i64,
                last: 0x1c37 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1cd0 as i32 as i64,
                last: 0x1cd2 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1cd4 as i32 as i64,
                last: 0x1ce8 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1ced as i32 as i64,
                last: 0x1ced as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1cf4 as i32 as i64,
                last: 0x1cf4 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1cf7 as i32 as i64,
                last: 0x1cf9 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1dc0 as i32 as i64,
                last: 0x1df9 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1dfb as i32 as i64,
                last: 0x1dff as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x20d0 as i32 as i64,
                last: 0x20f0 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2cef as i32 as i64,
                last: 0x2cf1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2d7f as i32 as i64,
                last: 0x2d7f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2de0 as i32 as i64,
                last: 0x2dff as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x302a as i32 as i64,
                last: 0x302f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3099 as i32 as i64,
                last: 0x309a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa66f as i32 as i64,
                last: 0xa672 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa674 as i32 as i64,
                last: 0xa67d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa69e as i32 as i64,
                last: 0xa69f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa6f0 as i32 as i64,
                last: 0xa6f1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa802 as i32 as i64,
                last: 0xa802 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa806 as i32 as i64,
                last: 0xa806 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa80b as i32 as i64,
                last: 0xa80b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa823 as i32 as i64,
                last: 0xa827 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa880 as i32 as i64,
                last: 0xa881 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa8b4 as i32 as i64,
                last: 0xa8c5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa8e0 as i32 as i64,
                last: 0xa8f1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa8ff as i32 as i64,
                last: 0xa8ff as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa926 as i32 as i64,
                last: 0xa92d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa947 as i32 as i64,
                last: 0xa953 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa980 as i32 as i64,
                last: 0xa983 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa9b3 as i32 as i64,
                last: 0xa9c0 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa9e5 as i32 as i64,
                last: 0xa9e5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xaa29 as i32 as i64,
                last: 0xaa36 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xaa43 as i32 as i64,
                last: 0xaa43 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xaa4c as i32 as i64,
                last: 0xaa4d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xaa7b as i32 as i64,
                last: 0xaa7d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xaab0 as i32 as i64,
                last: 0xaab0 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xaab2 as i32 as i64,
                last: 0xaab4 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xaab7 as i32 as i64,
                last: 0xaab8 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xaabe as i32 as i64,
                last: 0xaabf as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xaac1 as i32 as i64,
                last: 0xaac1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xaaeb as i32 as i64,
                last: 0xaaef as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xaaf5 as i32 as i64,
                last: 0xaaf6 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xabe3 as i32 as i64,
                last: 0xabea as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xabec as i32 as i64,
                last: 0xabed as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfb1e as i32 as i64,
                last: 0xfb1e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfe00 as i32 as i64,
                last: 0xfe0f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfe20 as i32 as i64,
                last: 0xfe2f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x101fd as i32 as i64,
                last: 0x101fd as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x102e0 as i32 as i64,
                last: 0x102e0 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x10376 as i32 as i64,
                last: 0x1037a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x10a01 as i32 as i64,
                last: 0x10a03 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x10a05 as i32 as i64,
                last: 0x10a06 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x10a0c as i32 as i64,
                last: 0x10a0f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x10a38 as i32 as i64,
                last: 0x10a3a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x10a3f as i32 as i64,
                last: 0x10a3f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x10ae5 as i32 as i64,
                last: 0x10ae6 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x10d24 as i32 as i64,
                last: 0x10d27 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x10f46 as i32 as i64,
                last: 0x10f50 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11000 as i32 as i64,
                last: 0x11002 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11038 as i32 as i64,
                last: 0x11046 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1107f as i32 as i64,
                last: 0x11082 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x110b0 as i32 as i64,
                last: 0x110ba as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11100 as i32 as i64,
                last: 0x11102 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11127 as i32 as i64,
                last: 0x11134 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11145 as i32 as i64,
                last: 0x11146 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11173 as i32 as i64,
                last: 0x11173 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11180 as i32 as i64,
                last: 0x11182 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x111b3 as i32 as i64,
                last: 0x111c0 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x111c9 as i32 as i64,
                last: 0x111cc as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1122c as i32 as i64,
                last: 0x11237 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1123e as i32 as i64,
                last: 0x1123e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x112df as i32 as i64,
                last: 0x112ea as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11300 as i32 as i64,
                last: 0x11303 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1133b as i32 as i64,
                last: 0x1133c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1133e as i32 as i64,
                last: 0x11344 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11347 as i32 as i64,
                last: 0x11348 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1134b as i32 as i64,
                last: 0x1134d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11357 as i32 as i64,
                last: 0x11357 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11362 as i32 as i64,
                last: 0x11363 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11366 as i32 as i64,
                last: 0x1136c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11370 as i32 as i64,
                last: 0x11374 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11435 as i32 as i64,
                last: 0x11446 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1145e as i32 as i64,
                last: 0x1145e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x114b0 as i32 as i64,
                last: 0x114c3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x115af as i32 as i64,
                last: 0x115b5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x115b8 as i32 as i64,
                last: 0x115c0 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x115dc as i32 as i64,
                last: 0x115dd as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11630 as i32 as i64,
                last: 0x11640 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x116ab as i32 as i64,
                last: 0x116b7 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1171d as i32 as i64,
                last: 0x1172b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1182c as i32 as i64,
                last: 0x1183a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x119d1 as i32 as i64,
                last: 0x119d7 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x119da as i32 as i64,
                last: 0x119e0 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x119e4 as i32 as i64,
                last: 0x119e4 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11a01 as i32 as i64,
                last: 0x11a0a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11a33 as i32 as i64,
                last: 0x11a39 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11a3b as i32 as i64,
                last: 0x11a3e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11a47 as i32 as i64,
                last: 0x11a47 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11a51 as i32 as i64,
                last: 0x11a5b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11a8a as i32 as i64,
                last: 0x11a99 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11c2f as i32 as i64,
                last: 0x11c36 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11c38 as i32 as i64,
                last: 0x11c3f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11c92 as i32 as i64,
                last: 0x11ca7 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11ca9 as i32 as i64,
                last: 0x11cb6 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11d31 as i32 as i64,
                last: 0x11d36 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11d3a as i32 as i64,
                last: 0x11d3a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11d3c as i32 as i64,
                last: 0x11d3d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11d3f as i32 as i64,
                last: 0x11d45 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11d47 as i32 as i64,
                last: 0x11d47 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11d8a as i32 as i64,
                last: 0x11d8e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11d90 as i32 as i64,
                last: 0x11d91 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11d93 as i32 as i64,
                last: 0x11d97 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11ef3 as i32 as i64,
                last: 0x11ef6 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x16af0 as i32 as i64,
                last: 0x16af4 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x16b30 as i32 as i64,
                last: 0x16b36 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x16f4f as i32 as i64,
                last: 0x16f4f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x16f51 as i32 as i64,
                last: 0x16f87 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x16f8f as i32 as i64,
                last: 0x16f92 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1bc9d as i32 as i64,
                last: 0x1bc9e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d165 as i32 as i64,
                last: 0x1d169 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d16d as i32 as i64,
                last: 0x1d172 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d17b as i32 as i64,
                last: 0x1d182 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d185 as i32 as i64,
                last: 0x1d18b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d1aa as i32 as i64,
                last: 0x1d1ad as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d242 as i32 as i64,
                last: 0x1d244 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1da00 as i32 as i64,
                last: 0x1da36 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1da3b as i32 as i64,
                last: 0x1da6c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1da75 as i32 as i64,
                last: 0x1da75 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1da84 as i32 as i64,
                last: 0x1da84 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1da9b as i32 as i64,
                last: 0x1da9f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1daa1 as i32 as i64,
                last: 0x1daaf as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1e000 as i32 as i64,
                last: 0x1e006 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1e008 as i32 as i64,
                last: 0x1e018 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1e01b as i32 as i64,
                last: 0x1e021 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1e023 as i32 as i64,
                last: 0x1e024 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1e026 as i32 as i64,
                last: 0x1e02a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1e130 as i32 as i64,
                last: 0x1e136 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1e2ec as i32 as i64,
                last: 0x1e2ef as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1e8d0 as i32 as i64,
                last: 0x1e8d6 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1e944 as i32 as i64,
                last: 0x1e94a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xe0100 as i32 as i64,
                last: 0xe01ef as i32 as i64,
            };
            init
        },
    ];
    pub static mut foldCase: [convertStruct; 192] = [
        {
            let mut init = convertStruct {
                rangeStart: 0x41 as i32,
                rangeEnd: 0x5a as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xb5 as i32,
                rangeEnd: 0xb5 as i32,
                step: -(1),
                offset: 775,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xc0 as i32,
                rangeEnd: 0xd6 as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xd8 as i32,
                rangeEnd: 0xde as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x100 as i32,
                rangeEnd: 0x12e as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x132 as i32,
                rangeEnd: 0x136 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x139 as i32,
                rangeEnd: 0x147 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x14a as i32,
                rangeEnd: 0x176 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x178 as i32,
                rangeEnd: 0x178 as i32,
                step: -(1),
                offset: -(121),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x179 as i32,
                rangeEnd: 0x17d as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x17f as i32,
                rangeEnd: 0x17f as i32,
                step: -(1),
                offset: -(268),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x181 as i32,
                rangeEnd: 0x181 as i32,
                step: -(1),
                offset: 210,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x182 as i32,
                rangeEnd: 0x184 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x186 as i32,
                rangeEnd: 0x186 as i32,
                step: -(1),
                offset: 206,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x187 as i32,
                rangeEnd: 0x187 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x189 as i32,
                rangeEnd: 0x18a as i32,
                step: 1,
                offset: 205,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x18b as i32,
                rangeEnd: 0x18b as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x18e as i32,
                rangeEnd: 0x18e as i32,
                step: -(1),
                offset: 79,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x18f as i32,
                rangeEnd: 0x18f as i32,
                step: -(1),
                offset: 202,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x190 as i32,
                rangeEnd: 0x190 as i32,
                step: -(1),
                offset: 203,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x191 as i32,
                rangeEnd: 0x191 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x193 as i32,
                rangeEnd: 0x193 as i32,
                step: -(1),
                offset: 205,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x194 as i32,
                rangeEnd: 0x194 as i32,
                step: -(1),
                offset: 207,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x196 as i32,
                rangeEnd: 0x196 as i32,
                step: -(1),
                offset: 211,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x197 as i32,
                rangeEnd: 0x197 as i32,
                step: -(1),
                offset: 209,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x198 as i32,
                rangeEnd: 0x198 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x19c as i32,
                rangeEnd: 0x19c as i32,
                step: -(1),
                offset: 211,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x19d as i32,
                rangeEnd: 0x19d as i32,
                step: -(1),
                offset: 213,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x19f as i32,
                rangeEnd: 0x19f as i32,
                step: -(1),
                offset: 214,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1a0 as i32,
                rangeEnd: 0x1a4 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1a6 as i32,
                rangeEnd: 0x1a6 as i32,
                step: -(1),
                offset: 218,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1a7 as i32,
                rangeEnd: 0x1a7 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1a9 as i32,
                rangeEnd: 0x1a9 as i32,
                step: -(1),
                offset: 218,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ac as i32,
                rangeEnd: 0x1ac as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ae as i32,
                rangeEnd: 0x1ae as i32,
                step: -(1),
                offset: 218,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1af as i32,
                rangeEnd: 0x1af as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1b1 as i32,
                rangeEnd: 0x1b2 as i32,
                step: 1,
                offset: 217,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1b3 as i32,
                rangeEnd: 0x1b5 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1b7 as i32,
                rangeEnd: 0x1b7 as i32,
                step: -(1),
                offset: 219,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1b8 as i32,
                rangeEnd: 0x1bc as i32,
                step: 4,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c4 as i32,
                rangeEnd: 0x1c4 as i32,
                step: -(1),
                offset: 2,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c5 as i32,
                rangeEnd: 0x1c5 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c7 as i32,
                rangeEnd: 0x1c7 as i32,
                step: -(1),
                offset: 2,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c8 as i32,
                rangeEnd: 0x1c8 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ca as i32,
                rangeEnd: 0x1ca as i32,
                step: -(1),
                offset: 2,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1cb as i32,
                rangeEnd: 0x1db as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1de as i32,
                rangeEnd: 0x1ee as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f1 as i32,
                rangeEnd: 0x1f1 as i32,
                step: -(1),
                offset: 2,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f2 as i32,
                rangeEnd: 0x1f4 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f6 as i32,
                rangeEnd: 0x1f6 as i32,
                step: -(1),
                offset: -(97),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f7 as i32,
                rangeEnd: 0x1f7 as i32,
                step: -(1),
                offset: -(56),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f8 as i32,
                rangeEnd: 0x21e as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x220 as i32,
                rangeEnd: 0x220 as i32,
                step: -(1),
                offset: -(130),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x222 as i32,
                rangeEnd: 0x232 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x23a as i32,
                rangeEnd: 0x23a as i32,
                step: -(1),
                offset: 10795,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x23b as i32,
                rangeEnd: 0x23b as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x23d as i32,
                rangeEnd: 0x23d as i32,
                step: -(1),
                offset: -(163),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x23e as i32,
                rangeEnd: 0x23e as i32,
                step: -(1),
                offset: 10792,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x241 as i32,
                rangeEnd: 0x241 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x243 as i32,
                rangeEnd: 0x243 as i32,
                step: -(1),
                offset: -(195),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x244 as i32,
                rangeEnd: 0x244 as i32,
                step: -(1),
                offset: 69,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x245 as i32,
                rangeEnd: 0x245 as i32,
                step: -(1),
                offset: 71,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x246 as i32,
                rangeEnd: 0x24e as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x345 as i32,
                rangeEnd: 0x345 as i32,
                step: -(1),
                offset: 116,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x370 as i32,
                rangeEnd: 0x372 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x376 as i32,
                rangeEnd: 0x376 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x37f as i32,
                rangeEnd: 0x37f as i32,
                step: -(1),
                offset: 116,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x386 as i32,
                rangeEnd: 0x386 as i32,
                step: -(1),
                offset: 38,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x388 as i32,
                rangeEnd: 0x38a as i32,
                step: 1,
                offset: 37,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x38c as i32,
                rangeEnd: 0x38c as i32,
                step: -(1),
                offset: 64,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x38e as i32,
                rangeEnd: 0x38f as i32,
                step: 1,
                offset: 63,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x391 as i32,
                rangeEnd: 0x3a1 as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3a3 as i32,
                rangeEnd: 0x3ab as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3c2 as i32,
                rangeEnd: 0x3c2 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3cf as i32,
                rangeEnd: 0x3cf as i32,
                step: -(1),
                offset: 8,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3d0 as i32,
                rangeEnd: 0x3d0 as i32,
                step: -(1),
                offset: -(30),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3d1 as i32,
                rangeEnd: 0x3d1 as i32,
                step: -(1),
                offset: -(25),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3d5 as i32,
                rangeEnd: 0x3d5 as i32,
                step: -(1),
                offset: -(15),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3d6 as i32,
                rangeEnd: 0x3d6 as i32,
                step: -(1),
                offset: -(22),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3d8 as i32,
                rangeEnd: 0x3ee as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3f0 as i32,
                rangeEnd: 0x3f0 as i32,
                step: -(1),
                offset: -(54),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3f1 as i32,
                rangeEnd: 0x3f1 as i32,
                step: -(1),
                offset: -(48),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3f4 as i32,
                rangeEnd: 0x3f4 as i32,
                step: -(1),
                offset: -(60),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3f5 as i32,
                rangeEnd: 0x3f5 as i32,
                step: -(1),
                offset: -(64),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3f7 as i32,
                rangeEnd: 0x3f7 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3f9 as i32,
                rangeEnd: 0x3f9 as i32,
                step: -(1),
                offset: -(7),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3fa as i32,
                rangeEnd: 0x3fa as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x3fd as i32,
                rangeEnd: 0x3ff as i32,
                step: 1,
                offset: -(130),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x400 as i32,
                rangeEnd: 0x40f as i32,
                step: 1,
                offset: 80,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x410 as i32,
                rangeEnd: 0x42f as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x460 as i32,
                rangeEnd: 0x480 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x48a as i32,
                rangeEnd: 0x4be as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x4c0 as i32,
                rangeEnd: 0x4c0 as i32,
                step: -(1),
                offset: 15,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x4c1 as i32,
                rangeEnd: 0x4cd as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x4d0 as i32,
                rangeEnd: 0x52e as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x531 as i32,
                rangeEnd: 0x556 as i32,
                step: 1,
                offset: 48,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x10a0 as i32,
                rangeEnd: 0x10c5 as i32,
                step: 1,
                offset: 7264,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x10c7 as i32,
                rangeEnd: 0x10cd as i32,
                step: 6,
                offset: 7264,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x13f8 as i32,
                rangeEnd: 0x13fd as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c80 as i32,
                rangeEnd: 0x1c80 as i32,
                step: -(1),
                offset: -(6222),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c81 as i32,
                rangeEnd: 0x1c81 as i32,
                step: -(1),
                offset: -(6221),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c82 as i32,
                rangeEnd: 0x1c82 as i32,
                step: -(1),
                offset: -(6212),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c83 as i32,
                rangeEnd: 0x1c84 as i32,
                step: 1,
                offset: -(6210),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c85 as i32,
                rangeEnd: 0x1c85 as i32,
                step: -(1),
                offset: -(6211),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c86 as i32,
                rangeEnd: 0x1c86 as i32,
                step: -(1),
                offset: -(6204),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c87 as i32,
                rangeEnd: 0x1c87 as i32,
                step: -(1),
                offset: -(6180),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c88 as i32,
                rangeEnd: 0x1c88 as i32,
                step: -(1),
                offset: 35267,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1c90 as i32,
                rangeEnd: 0x1cba as i32,
                step: 1,
                offset: -(3008),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1cbd as i32,
                rangeEnd: 0x1cbf as i32,
                step: 1,
                offset: -(3008),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1e00 as i32,
                rangeEnd: 0x1e94 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1e9b as i32,
                rangeEnd: 0x1e9b as i32,
                step: -(1),
                offset: -(58),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1e9e as i32,
                rangeEnd: 0x1e9e as i32,
                step: -(1),
                offset: -(7615),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ea0 as i32,
                rangeEnd: 0x1efe as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f08 as i32,
                rangeEnd: 0x1f0f as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f18 as i32,
                rangeEnd: 0x1f1d as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f28 as i32,
                rangeEnd: 0x1f2f as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f38 as i32,
                rangeEnd: 0x1f3f as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f48 as i32,
                rangeEnd: 0x1f4d as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f59 as i32,
                rangeEnd: 0x1f5f as i32,
                step: 2,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f68 as i32,
                rangeEnd: 0x1f6f as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f88 as i32,
                rangeEnd: 0x1f8f as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1f98 as i32,
                rangeEnd: 0x1f9f as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fa8 as i32,
                rangeEnd: 0x1faf as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fb8 as i32,
                rangeEnd: 0x1fb9 as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fba as i32,
                rangeEnd: 0x1fbb as i32,
                step: 1,
                offset: -(74),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fbc as i32,
                rangeEnd: 0x1fbc as i32,
                step: -(1),
                offset: -(9),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fbe as i32,
                rangeEnd: 0x1fbe as i32,
                step: -(1),
                offset: -(7173),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fc8 as i32,
                rangeEnd: 0x1fcb as i32,
                step: 1,
                offset: -(86),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fcc as i32,
                rangeEnd: 0x1fcc as i32,
                step: -(1),
                offset: -(9),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fd8 as i32,
                rangeEnd: 0x1fd9 as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fda as i32,
                rangeEnd: 0x1fdb as i32,
                step: 1,
                offset: -(100),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fe8 as i32,
                rangeEnd: 0x1fe9 as i32,
                step: 1,
                offset: -(8),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fea as i32,
                rangeEnd: 0x1feb as i32,
                step: 1,
                offset: -(112),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1fec as i32,
                rangeEnd: 0x1fec as i32,
                step: -(1),
                offset: -(7),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ff8 as i32,
                rangeEnd: 0x1ff9 as i32,
                step: 1,
                offset: -(128),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ffa as i32,
                rangeEnd: 0x1ffb as i32,
                step: 1,
                offset: -(126),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1ffc as i32,
                rangeEnd: 0x1ffc as i32,
                step: -(1),
                offset: -(9),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2126 as i32,
                rangeEnd: 0x2126 as i32,
                step: -(1),
                offset: -(7517),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x212a as i32,
                rangeEnd: 0x212a as i32,
                step: -(1),
                offset: -(8383),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x212b as i32,
                rangeEnd: 0x212b as i32,
                step: -(1),
                offset: -(8262),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2132 as i32,
                rangeEnd: 0x2132 as i32,
                step: -(1),
                offset: 28,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2160 as i32,
                rangeEnd: 0x216f as i32,
                step: 1,
                offset: 16,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2183 as i32,
                rangeEnd: 0x2183 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x24b6 as i32,
                rangeEnd: 0x24cf as i32,
                step: 1,
                offset: 26,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c00 as i32,
                rangeEnd: 0x2c2e as i32,
                step: 1,
                offset: 48,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c60 as i32,
                rangeEnd: 0x2c60 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c62 as i32,
                rangeEnd: 0x2c62 as i32,
                step: -(1),
                offset: -(10743),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c63 as i32,
                rangeEnd: 0x2c63 as i32,
                step: -(1),
                offset: -(3814),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c64 as i32,
                rangeEnd: 0x2c64 as i32,
                step: -(1),
                offset: -(10727),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c67 as i32,
                rangeEnd: 0x2c6b as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c6d as i32,
                rangeEnd: 0x2c6d as i32,
                step: -(1),
                offset: -(10780),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c6e as i32,
                rangeEnd: 0x2c6e as i32,
                step: -(1),
                offset: -(10749),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c6f as i32,
                rangeEnd: 0x2c6f as i32,
                step: -(1),
                offset: -(10783),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c70 as i32,
                rangeEnd: 0x2c70 as i32,
                step: -(1),
                offset: -(10782),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c72 as i32,
                rangeEnd: 0x2c75 as i32,
                step: 3,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c7e as i32,
                rangeEnd: 0x2c7f as i32,
                step: 1,
                offset: -(10815),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2c80 as i32,
                rangeEnd: 0x2ce2 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2ceb as i32,
                rangeEnd: 0x2ced as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x2cf2 as i32,
                rangeEnd: 0xa640 as i32,
                step: 31054,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa642 as i32,
                rangeEnd: 0xa66c as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa680 as i32,
                rangeEnd: 0xa69a as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa722 as i32,
                rangeEnd: 0xa72e as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa732 as i32,
                rangeEnd: 0xa76e as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa779 as i32,
                rangeEnd: 0xa77b as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa77d as i32,
                rangeEnd: 0xa77d as i32,
                step: -(1),
                offset: -(35332),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa77e as i32,
                rangeEnd: 0xa786 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa78b as i32,
                rangeEnd: 0xa78b as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa78d as i32,
                rangeEnd: 0xa78d as i32,
                step: -(1),
                offset: -(42280),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa790 as i32,
                rangeEnd: 0xa792 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa796 as i32,
                rangeEnd: 0xa7a8 as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7aa as i32,
                rangeEnd: 0xa7aa as i32,
                step: -(1),
                offset: -(42308),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7ab as i32,
                rangeEnd: 0xa7ab as i32,
                step: -(1),
                offset: -(42319),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7ac as i32,
                rangeEnd: 0xa7ac as i32,
                step: -(1),
                offset: -(42315),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7ad as i32,
                rangeEnd: 0xa7ad as i32,
                step: -(1),
                offset: -(42305),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7ae as i32,
                rangeEnd: 0xa7ae as i32,
                step: -(1),
                offset: -(42308),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7b0 as i32,
                rangeEnd: 0xa7b0 as i32,
                step: -(1),
                offset: -(42258),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7b1 as i32,
                rangeEnd: 0xa7b1 as i32,
                step: -(1),
                offset: -(42282),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7b2 as i32,
                rangeEnd: 0xa7b2 as i32,
                step: -(1),
                offset: -(42261),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7b3 as i32,
                rangeEnd: 0xa7b3 as i32,
                step: -(1),
                offset: 928,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7b4 as i32,
                rangeEnd: 0xa7be as i32,
                step: 2,
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7c2 as i32,
                rangeEnd: 0xa7c2 as i32,
                step: -(1),
                offset: 1,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7c4 as i32,
                rangeEnd: 0xa7c4 as i32,
                step: -(1),
                offset: -(48),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7c5 as i32,
                rangeEnd: 0xa7c5 as i32,
                step: -(1),
                offset: -(42307),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xa7c6 as i32,
                rangeEnd: 0xa7c6 as i32,
                step: -(1),
                offset: -(35384),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xab70 as i32,
                rangeEnd: 0xabbf as i32,
                step: 1,
                offset: -(38864),
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0xff21 as i32,
                rangeEnd: 0xff3a as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x10400 as i32,
                rangeEnd: 0x10427 as i32,
                step: 1,
                offset: 40,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x104b0 as i32,
                rangeEnd: 0x104d3 as i32,
                step: 1,
                offset: 40,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x10c80 as i32,
                rangeEnd: 0x10cb2 as i32,
                step: 1,
                offset: 64,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x118a0 as i32,
                rangeEnd: 0x118bf as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x16e40 as i32,
                rangeEnd: 0x16e5f as i32,
                step: 1,
                offset: 32,
            };
            init
        },
        {
            let mut init = convertStruct {
                rangeStart: 0x1e900 as i32,
                rangeEnd: 0x1e921 as i32,
                step: 1,
                offset: 34,
            };
            init
        },
    ];
    pub static mut doublewidth: [interval; 113] = [
        {
            let mut init = interval {
                first: 0x1100 as i32 as i64,
                last: 0x115f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x231a as i32 as i64,
                last: 0x231b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2329 as i32 as i64,
                last: 0x232a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x23e9 as i32 as i64,
                last: 0x23ec as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x23f0 as i32 as i64,
                last: 0x23f0 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x23f3 as i32 as i64,
                last: 0x23f3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25fd as i32 as i64,
                last: 0x25fe as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2614 as i32 as i64,
                last: 0x2615 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2648 as i32 as i64,
                last: 0x2653 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x267f as i32 as i64,
                last: 0x267f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2693 as i32 as i64,
                last: 0x2693 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26a1 as i32 as i64,
                last: 0x26a1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26aa as i32 as i64,
                last: 0x26ab as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26bd as i32 as i64,
                last: 0x26be as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26c4 as i32 as i64,
                last: 0x26c5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26ce as i32 as i64,
                last: 0x26ce as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26d4 as i32 as i64,
                last: 0x26d4 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26ea as i32 as i64,
                last: 0x26ea as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26f2 as i32 as i64,
                last: 0x26f3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26f5 as i32 as i64,
                last: 0x26f5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26fa as i32 as i64,
                last: 0x26fa as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26fd as i32 as i64,
                last: 0x26fd as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2705 as i32 as i64,
                last: 0x2705 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x270a as i32 as i64,
                last: 0x270b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2728 as i32 as i64,
                last: 0x2728 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x274c as i32 as i64,
                last: 0x274c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x274e as i32 as i64,
                last: 0x274e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2753 as i32 as i64,
                last: 0x2755 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2757 as i32 as i64,
                last: 0x2757 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2795 as i32 as i64,
                last: 0x2797 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x27b0 as i32 as i64,
                last: 0x27b0 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x27bf as i32 as i64,
                last: 0x27bf as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2b1b as i32 as i64,
                last: 0x2b1c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2b50 as i32 as i64,
                last: 0x2b50 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2b55 as i32 as i64,
                last: 0x2b55 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2e80 as i32 as i64,
                last: 0x2e99 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2e9b as i32 as i64,
                last: 0x2ef3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2f00 as i32 as i64,
                last: 0x2fd5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2ff0 as i32 as i64,
                last: 0x2ffb as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3000 as i32 as i64,
                last: 0x303e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3041 as i32 as i64,
                last: 0x3096 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3099 as i32 as i64,
                last: 0x30ff as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3105 as i32 as i64,
                last: 0x312f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3131 as i32 as i64,
                last: 0x318e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3190 as i32 as i64,
                last: 0x31ba as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x31c0 as i32 as i64,
                last: 0x31e3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x31f0 as i32 as i64,
                last: 0x321e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3220 as i32 as i64,
                last: 0x3247 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3250 as i32 as i64,
                last: 0x4dbf as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x4e00 as i32 as i64,
                last: 0xa48c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa490 as i32 as i64,
                last: 0xa4c6 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa960 as i32 as i64,
                last: 0xa97c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xac00 as i32 as i64,
                last: 0xd7a3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf900 as i32 as i64,
                last: 0xfaff as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfe10 as i32 as i64,
                last: 0xfe19 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfe30 as i32 as i64,
                last: 0xfe52 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfe54 as i32 as i64,
                last: 0xfe66 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfe68 as i32 as i64,
                last: 0xfe6b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xff01 as i32 as i64,
                last: 0xff60 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xffe0 as i32 as i64,
                last: 0xffe6 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x16fe0 as i32 as i64,
                last: 0x16fe3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x17000 as i32 as i64,
                last: 0x187f7 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x18800 as i32 as i64,
                last: 0x18af2 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b000 as i32 as i64,
                last: 0x1b11e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b150 as i32 as i64,
                last: 0x1b152 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b164 as i32 as i64,
                last: 0x1b167 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b170 as i32 as i64,
                last: 0x1b2fb as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f004 as i32 as i64,
                last: 0x1f004 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f0cf as i32 as i64,
                last: 0x1f0cf as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f18e as i32 as i64,
                last: 0x1f18e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f191 as i32 as i64,
                last: 0x1f19a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f200 as i32 as i64,
                last: 0x1f202 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f210 as i32 as i64,
                last: 0x1f23b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f240 as i32 as i64,
                last: 0x1f248 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f250 as i32 as i64,
                last: 0x1f251 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f260 as i32 as i64,
                last: 0x1f265 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f300 as i32 as i64,
                last: 0x1f320 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f32d as i32 as i64,
                last: 0x1f335 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f337 as i32 as i64,
                last: 0x1f37c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f37e as i32 as i64,
                last: 0x1f393 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3a0 as i32 as i64,
                last: 0x1f3ca as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3cf as i32 as i64,
                last: 0x1f3d3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3e0 as i32 as i64,
                last: 0x1f3f0 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3f4 as i32 as i64,
                last: 0x1f3f4 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3f8 as i32 as i64,
                last: 0x1f43e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f440 as i32 as i64,
                last: 0x1f440 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f442 as i32 as i64,
                last: 0x1f4fc as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f4ff as i32 as i64,
                last: 0x1f53d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f54b as i32 as i64,
                last: 0x1f54e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f550 as i32 as i64,
                last: 0x1f567 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f57a as i32 as i64,
                last: 0x1f57a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f595 as i32 as i64,
                last: 0x1f596 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5a4 as i32 as i64,
                last: 0x1f5a4 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5fb as i32 as i64,
                last: 0x1f64f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f680 as i32 as i64,
                last: 0x1f6c5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6cc as i32 as i64,
                last: 0x1f6cc as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6d0 as i32 as i64,
                last: 0x1f6d2 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6d5 as i32 as i64,
                last: 0x1f6d5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6eb as i32 as i64,
                last: 0x1f6ec as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6f4 as i32 as i64,
                last: 0x1f6fa as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f7e0 as i32 as i64,
                last: 0x1f7eb as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f90d as i32 as i64,
                last: 0x1f971 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f973 as i32 as i64,
                last: 0x1f976 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f97a as i32 as i64,
                last: 0x1f9a2 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f9a5 as i32 as i64,
                last: 0x1f9aa as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f9ae as i32 as i64,
                last: 0x1f9ca as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f9cd as i32 as i64,
                last: 0x1f9ff as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1fa70 as i32 as i64,
                last: 0x1fa73 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1fa78 as i32 as i64,
                last: 0x1fa7a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1fa80 as i32 as i64,
                last: 0x1fa82 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1fa90 as i32 as i64,
                last: 0x1fa95 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x20000 as i32 as i64,
                last: 0x2fffd as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x30000 as i32 as i64,
                last: 0x3fffd as i32 as i64,
            };
            init
        },
    ];
    pub static mut ambiguous: [interval; 179] = [
        {
            let mut init = interval { first: 0xa1 as i32 as i64, last: 0xa1 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xa4 as i32 as i64, last: 0xa4 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xa7 as i32 as i64, last: 0xa8 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xaa as i32 as i64, last: 0xaa as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xad as i32 as i64, last: 0xae as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xb0 as i32 as i64, last: 0xb4 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xb6 as i32 as i64, last: 0xba as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xbc as i32 as i64, last: 0xbf as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xc6 as i32 as i64, last: 0xc6 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xd0 as i32 as i64, last: 0xd0 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xd7 as i32 as i64, last: 0xd8 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xde as i32 as i64, last: 0xe1 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xe6 as i32 as i64, last: 0xe6 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xe8 as i32 as i64, last: 0xea as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xec as i32 as i64, last: 0xed as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xf0 as i32 as i64, last: 0xf0 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xf2 as i32 as i64, last: 0xf3 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xf7 as i32 as i64, last: 0xfa as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xfc as i32 as i64, last: 0xfc as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xfe as i32 as i64, last: 0xfe as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x101 as i32 as i64, last: 0x101 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x111 as i32 as i64, last: 0x111 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x113 as i32 as i64, last: 0x113 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x11b as i32 as i64, last: 0x11b as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x126 as i32 as i64, last: 0x127 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x12b as i32 as i64, last: 0x12b as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x131 as i32 as i64, last: 0x133 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x138 as i32 as i64, last: 0x138 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x13f as i32 as i64, last: 0x142 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x144 as i32 as i64, last: 0x144 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x148 as i32 as i64, last: 0x14b as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x14d as i32 as i64, last: 0x14d as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x152 as i32 as i64, last: 0x153 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x166 as i32 as i64, last: 0x167 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x16b as i32 as i64, last: 0x16b as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x1ce as i32 as i64, last: 0x1ce as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x1d0 as i32 as i64, last: 0x1d0 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x1d2 as i32 as i64, last: 0x1d2 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x1d4 as i32 as i64, last: 0x1d4 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x1d6 as i32 as i64, last: 0x1d6 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x1d8 as i32 as i64, last: 0x1d8 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x1da as i32 as i64, last: 0x1da as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x1dc as i32 as i64, last: 0x1dc as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x251 as i32 as i64, last: 0x251 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x261 as i32 as i64, last: 0x261 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x2c4 as i32 as i64, last: 0x2c4 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x2c7 as i32 as i64, last: 0x2c7 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x2c9 as i32 as i64, last: 0x2cb as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x2cd as i32 as i64, last: 0x2cd as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x2d0 as i32 as i64, last: 0x2d0 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x2d8 as i32 as i64, last: 0x2db as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x2dd as i32 as i64, last: 0x2dd as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x2df as i32 as i64, last: 0x2df as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x300 as i32 as i64, last: 0x36f as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x391 as i32 as i64, last: 0x3a1 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x3a3 as i32 as i64, last: 0x3a9 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x3b1 as i32 as i64, last: 0x3c1 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x3c3 as i32 as i64, last: 0x3c9 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x401 as i32 as i64, last: 0x401 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x410 as i32 as i64, last: 0x44f as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x451 as i32 as i64, last: 0x451 as i32 as i64 };
            init
        },
        {
            let mut init = interval {
                first: 0x2010 as i32 as i64,
                last: 0x2010 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2013 as i32 as i64,
                last: 0x2016 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2018 as i32 as i64,
                last: 0x2019 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x201c as i32 as i64,
                last: 0x201d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2020 as i32 as i64,
                last: 0x2022 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2024 as i32 as i64,
                last: 0x2027 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2030 as i32 as i64,
                last: 0x2030 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2032 as i32 as i64,
                last: 0x2033 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2035 as i32 as i64,
                last: 0x2035 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x203b as i32 as i64,
                last: 0x203b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x203e as i32 as i64,
                last: 0x203e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2074 as i32 as i64,
                last: 0x2074 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x207f as i32 as i64,
                last: 0x207f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2081 as i32 as i64,
                last: 0x2084 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x20ac as i32 as i64,
                last: 0x20ac as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2103 as i32 as i64,
                last: 0x2103 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2105 as i32 as i64,
                last: 0x2105 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2109 as i32 as i64,
                last: 0x2109 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2113 as i32 as i64,
                last: 0x2113 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2116 as i32 as i64,
                last: 0x2116 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2121 as i32 as i64,
                last: 0x2122 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2126 as i32 as i64,
                last: 0x2126 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x212b as i32 as i64,
                last: 0x212b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2153 as i32 as i64,
                last: 0x2154 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x215b as i32 as i64,
                last: 0x215e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2160 as i32 as i64,
                last: 0x216b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2170 as i32 as i64,
                last: 0x2179 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2189 as i32 as i64,
                last: 0x2189 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2190 as i32 as i64,
                last: 0x2199 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x21b8 as i32 as i64,
                last: 0x21b9 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x21d2 as i32 as i64,
                last: 0x21d2 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x21d4 as i32 as i64,
                last: 0x21d4 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x21e7 as i32 as i64,
                last: 0x21e7 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2200 as i32 as i64,
                last: 0x2200 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2202 as i32 as i64,
                last: 0x2203 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2207 as i32 as i64,
                last: 0x2208 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x220b as i32 as i64,
                last: 0x220b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x220f as i32 as i64,
                last: 0x220f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2211 as i32 as i64,
                last: 0x2211 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2215 as i32 as i64,
                last: 0x2215 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x221a as i32 as i64,
                last: 0x221a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x221d as i32 as i64,
                last: 0x2220 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2223 as i32 as i64,
                last: 0x2223 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2225 as i32 as i64,
                last: 0x2225 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2227 as i32 as i64,
                last: 0x222c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x222e as i32 as i64,
                last: 0x222e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2234 as i32 as i64,
                last: 0x2237 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x223c as i32 as i64,
                last: 0x223d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2248 as i32 as i64,
                last: 0x2248 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x224c as i32 as i64,
                last: 0x224c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2252 as i32 as i64,
                last: 0x2252 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2260 as i32 as i64,
                last: 0x2261 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2264 as i32 as i64,
                last: 0x2267 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x226a as i32 as i64,
                last: 0x226b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x226e as i32 as i64,
                last: 0x226f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2282 as i32 as i64,
                last: 0x2283 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2286 as i32 as i64,
                last: 0x2287 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2295 as i32 as i64,
                last: 0x2295 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2299 as i32 as i64,
                last: 0x2299 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x22a5 as i32 as i64,
                last: 0x22a5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x22bf as i32 as i64,
                last: 0x22bf as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2312 as i32 as i64,
                last: 0x2312 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2460 as i32 as i64,
                last: 0x24e9 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x24eb as i32 as i64,
                last: 0x254b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2550 as i32 as i64,
                last: 0x2573 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2580 as i32 as i64,
                last: 0x258f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2592 as i32 as i64,
                last: 0x2595 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25a0 as i32 as i64,
                last: 0x25a1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25a3 as i32 as i64,
                last: 0x25a9 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25b2 as i32 as i64,
                last: 0x25b3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25b6 as i32 as i64,
                last: 0x25b7 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25bc as i32 as i64,
                last: 0x25bd as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25c0 as i32 as i64,
                last: 0x25c1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25c6 as i32 as i64,
                last: 0x25c8 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25cb as i32 as i64,
                last: 0x25cb as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25ce as i32 as i64,
                last: 0x25d1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25e2 as i32 as i64,
                last: 0x25e5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25ef as i32 as i64,
                last: 0x25ef as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2605 as i32 as i64,
                last: 0x2606 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2609 as i32 as i64,
                last: 0x2609 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x260e as i32 as i64,
                last: 0x260f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x261c as i32 as i64,
                last: 0x261c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x261e as i32 as i64,
                last: 0x261e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2640 as i32 as i64,
                last: 0x2640 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2642 as i32 as i64,
                last: 0x2642 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2660 as i32 as i64,
                last: 0x2661 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2663 as i32 as i64,
                last: 0x2665 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2667 as i32 as i64,
                last: 0x266a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x266c as i32 as i64,
                last: 0x266d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x266f as i32 as i64,
                last: 0x266f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x269e as i32 as i64,
                last: 0x269f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26bf as i32 as i64,
                last: 0x26bf as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26c6 as i32 as i64,
                last: 0x26cd as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26cf as i32 as i64,
                last: 0x26d3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26d5 as i32 as i64,
                last: 0x26e1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26e3 as i32 as i64,
                last: 0x26e3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26e8 as i32 as i64,
                last: 0x26e9 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26eb as i32 as i64,
                last: 0x26f1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26f4 as i32 as i64,
                last: 0x26f4 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26f6 as i32 as i64,
                last: 0x26f9 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26fb as i32 as i64,
                last: 0x26fc as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26fe as i32 as i64,
                last: 0x26ff as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x273d as i32 as i64,
                last: 0x273d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2776 as i32 as i64,
                last: 0x277f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2b56 as i32 as i64,
                last: 0x2b59 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3248 as i32 as i64,
                last: 0x324f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xe000 as i32 as i64,
                last: 0xf8ff as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfe00 as i32 as i64,
                last: 0xfe0f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfffd as i32 as i64,
                last: 0xfffd as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f100 as i32 as i64,
                last: 0x1f10a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f110 as i32 as i64,
                last: 0x1f12d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f130 as i32 as i64,
                last: 0x1f169 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f170 as i32 as i64,
                last: 0x1f18d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f18f as i32 as i64,
                last: 0x1f190 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f19b as i32 as i64,
                last: 0x1f1ac as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xe0100 as i32 as i64,
                last: 0xe01ef as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf0000 as i32 as i64,
                last: 0xffffd as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x100000 as i32 as i64,
                last: 0x10fffd as i32 as i64,
            };
            init
        },
    ];
    pub static mut emoji_all: [interval; 151] = [
        {
            let mut init = interval { first: 0x23 as i32 as i64, last: 0x23 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x2a as i32 as i64, last: 0x2a as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0x30 as i32 as i64, last: 0x39 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xa9 as i32 as i64, last: 0xa9 as i32 as i64 };
            init
        },
        {
            let mut init = interval { first: 0xae as i32 as i64, last: 0xae as i32 as i64 };
            init
        },
        {
            let mut init = interval {
                first: 0x203c as i32 as i64,
                last: 0x203c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2049 as i32 as i64,
                last: 0x2049 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2122 as i32 as i64,
                last: 0x2122 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2139 as i32 as i64,
                last: 0x2139 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2194 as i32 as i64,
                last: 0x2199 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x21a9 as i32 as i64,
                last: 0x21aa as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x231a as i32 as i64,
                last: 0x231b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2328 as i32 as i64,
                last: 0x2328 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x23cf as i32 as i64,
                last: 0x23cf as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x23e9 as i32 as i64,
                last: 0x23f3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x23f8 as i32 as i64,
                last: 0x23fa as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x24c2 as i32 as i64,
                last: 0x24c2 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25aa as i32 as i64,
                last: 0x25ab as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25b6 as i32 as i64,
                last: 0x25b6 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25c0 as i32 as i64,
                last: 0x25c0 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25fb as i32 as i64,
                last: 0x25fe as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2600 as i32 as i64,
                last: 0x2604 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x260e as i32 as i64,
                last: 0x260e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2611 as i32 as i64,
                last: 0x2611 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2614 as i32 as i64,
                last: 0x2615 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2618 as i32 as i64,
                last: 0x2618 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x261d as i32 as i64,
                last: 0x261d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2620 as i32 as i64,
                last: 0x2620 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2622 as i32 as i64,
                last: 0x2623 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2626 as i32 as i64,
                last: 0x2626 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x262a as i32 as i64,
                last: 0x262a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x262e as i32 as i64,
                last: 0x262f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2638 as i32 as i64,
                last: 0x263a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2640 as i32 as i64,
                last: 0x2640 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2642 as i32 as i64,
                last: 0x2642 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2648 as i32 as i64,
                last: 0x2653 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x265f as i32 as i64,
                last: 0x2660 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2663 as i32 as i64,
                last: 0x2663 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2665 as i32 as i64,
                last: 0x2666 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2668 as i32 as i64,
                last: 0x2668 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x267b as i32 as i64,
                last: 0x267b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x267e as i32 as i64,
                last: 0x267f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2692 as i32 as i64,
                last: 0x2697 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2699 as i32 as i64,
                last: 0x2699 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x269b as i32 as i64,
                last: 0x269c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26a0 as i32 as i64,
                last: 0x26a1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26aa as i32 as i64,
                last: 0x26ab as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26b0 as i32 as i64,
                last: 0x26b1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26bd as i32 as i64,
                last: 0x26be as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26c4 as i32 as i64,
                last: 0x26c5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26c8 as i32 as i64,
                last: 0x26c8 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26ce as i32 as i64,
                last: 0x26cf as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26d1 as i32 as i64,
                last: 0x26d1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26d3 as i32 as i64,
                last: 0x26d4 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26e9 as i32 as i64,
                last: 0x26ea as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26f0 as i32 as i64,
                last: 0x26f5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26f7 as i32 as i64,
                last: 0x26fa as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26fd as i32 as i64,
                last: 0x26fd as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2702 as i32 as i64,
                last: 0x2702 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2705 as i32 as i64,
                last: 0x2705 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2708 as i32 as i64,
                last: 0x270d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x270f as i32 as i64,
                last: 0x270f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2712 as i32 as i64,
                last: 0x2712 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2714 as i32 as i64,
                last: 0x2714 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2716 as i32 as i64,
                last: 0x2716 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x271d as i32 as i64,
                last: 0x271d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2721 as i32 as i64,
                last: 0x2721 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2728 as i32 as i64,
                last: 0x2728 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2733 as i32 as i64,
                last: 0x2734 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2744 as i32 as i64,
                last: 0x2744 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2747 as i32 as i64,
                last: 0x2747 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x274c as i32 as i64,
                last: 0x274c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x274e as i32 as i64,
                last: 0x274e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2753 as i32 as i64,
                last: 0x2755 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2757 as i32 as i64,
                last: 0x2757 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2763 as i32 as i64,
                last: 0x2764 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2795 as i32 as i64,
                last: 0x2797 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x27a1 as i32 as i64,
                last: 0x27a1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x27b0 as i32 as i64,
                last: 0x27b0 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x27bf as i32 as i64,
                last: 0x27bf as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2934 as i32 as i64,
                last: 0x2935 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2b05 as i32 as i64,
                last: 0x2b07 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2b1b as i32 as i64,
                last: 0x2b1c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2b50 as i32 as i64,
                last: 0x2b50 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2b55 as i32 as i64,
                last: 0x2b55 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3030 as i32 as i64,
                last: 0x3030 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x303d as i32 as i64,
                last: 0x303d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3297 as i32 as i64,
                last: 0x3297 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3299 as i32 as i64,
                last: 0x3299 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f004 as i32 as i64,
                last: 0x1f004 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f0cf as i32 as i64,
                last: 0x1f0cf as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f170 as i32 as i64,
                last: 0x1f171 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f17e as i32 as i64,
                last: 0x1f17f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f18e as i32 as i64,
                last: 0x1f18e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f191 as i32 as i64,
                last: 0x1f19a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f1e6 as i32 as i64,
                last: 0x1f1ff as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f201 as i32 as i64,
                last: 0x1f202 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f21a as i32 as i64,
                last: 0x1f21a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f22f as i32 as i64,
                last: 0x1f22f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f232 as i32 as i64,
                last: 0x1f23a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f250 as i32 as i64,
                last: 0x1f251 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f300 as i32 as i64,
                last: 0x1f321 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f324 as i32 as i64,
                last: 0x1f393 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f396 as i32 as i64,
                last: 0x1f397 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f399 as i32 as i64,
                last: 0x1f39b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f39e as i32 as i64,
                last: 0x1f3f0 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3f3 as i32 as i64,
                last: 0x1f3f5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3f7 as i32 as i64,
                last: 0x1f4fd as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f4ff as i32 as i64,
                last: 0x1f53d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f549 as i32 as i64,
                last: 0x1f54e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f550 as i32 as i64,
                last: 0x1f567 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f56f as i32 as i64,
                last: 0x1f570 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f573 as i32 as i64,
                last: 0x1f57a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f587 as i32 as i64,
                last: 0x1f587 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f58a as i32 as i64,
                last: 0x1f58d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f590 as i32 as i64,
                last: 0x1f590 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f595 as i32 as i64,
                last: 0x1f596 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5a4 as i32 as i64,
                last: 0x1f5a5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5a8 as i32 as i64,
                last: 0x1f5a8 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5b1 as i32 as i64,
                last: 0x1f5b2 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5bc as i32 as i64,
                last: 0x1f5bc as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5c2 as i32 as i64,
                last: 0x1f5c4 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5d1 as i32 as i64,
                last: 0x1f5d3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5dc as i32 as i64,
                last: 0x1f5de as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5e1 as i32 as i64,
                last: 0x1f5e1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5e3 as i32 as i64,
                last: 0x1f5e3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5e8 as i32 as i64,
                last: 0x1f5e8 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5ef as i32 as i64,
                last: 0x1f5ef as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5f3 as i32 as i64,
                last: 0x1f5f3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5fa as i32 as i64,
                last: 0x1f64f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f680 as i32 as i64,
                last: 0x1f6c5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6cb as i32 as i64,
                last: 0x1f6d2 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6d5 as i32 as i64,
                last: 0x1f6d5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6e0 as i32 as i64,
                last: 0x1f6e5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6e9 as i32 as i64,
                last: 0x1f6e9 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6eb as i32 as i64,
                last: 0x1f6ec as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6f0 as i32 as i64,
                last: 0x1f6f0 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6f3 as i32 as i64,
                last: 0x1f6fa as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f7e0 as i32 as i64,
                last: 0x1f7eb as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f90d as i32 as i64,
                last: 0x1f93a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f93c as i32 as i64,
                last: 0x1f945 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f947 as i32 as i64,
                last: 0x1f971 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f973 as i32 as i64,
                last: 0x1f976 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f97a as i32 as i64,
                last: 0x1f9a2 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f9a5 as i32 as i64,
                last: 0x1f9aa as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f9ae as i32 as i64,
                last: 0x1f9ca as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f9cd as i32 as i64,
                last: 0x1f9ff as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1fa70 as i32 as i64,
                last: 0x1fa73 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1fa78 as i32 as i64,
                last: 0x1fa7a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1fa80 as i32 as i64,
                last: 0x1fa82 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1fa90 as i32 as i64,
                last: 0x1fa95 as i32 as i64,
            };
            init
        },
    ];
    pub static mut emoji_width: [interval; 39] = [
        {
            let mut init = interval {
                first: 0x1f1e6 as i32 as i64,
                last: 0x1f1ff as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f321 as i32 as i64,
                last: 0x1f321 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f324 as i32 as i64,
                last: 0x1f32c as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f336 as i32 as i64,
                last: 0x1f336 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f37d as i32 as i64,
                last: 0x1f37d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f396 as i32 as i64,
                last: 0x1f397 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f399 as i32 as i64,
                last: 0x1f39b as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f39e as i32 as i64,
                last: 0x1f39f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3cb as i32 as i64,
                last: 0x1f3ce as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3d4 as i32 as i64,
                last: 0x1f3df as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3f3 as i32 as i64,
                last: 0x1f3f5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3f7 as i32 as i64,
                last: 0x1f3f7 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f43f as i32 as i64,
                last: 0x1f43f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f441 as i32 as i64,
                last: 0x1f441 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f4fd as i32 as i64,
                last: 0x1f4fd as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f549 as i32 as i64,
                last: 0x1f54a as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f56f as i32 as i64,
                last: 0x1f570 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f573 as i32 as i64,
                last: 0x1f579 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f587 as i32 as i64,
                last: 0x1f587 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f58a as i32 as i64,
                last: 0x1f58d as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f590 as i32 as i64,
                last: 0x1f590 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5a5 as i32 as i64,
                last: 0x1f5a5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5a8 as i32 as i64,
                last: 0x1f5a8 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5b1 as i32 as i64,
                last: 0x1f5b2 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5bc as i32 as i64,
                last: 0x1f5bc as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5c2 as i32 as i64,
                last: 0x1f5c4 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5d1 as i32 as i64,
                last: 0x1f5d3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5dc as i32 as i64,
                last: 0x1f5de as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5e1 as i32 as i64,
                last: 0x1f5e1 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5e3 as i32 as i64,
                last: 0x1f5e3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5e8 as i32 as i64,
                last: 0x1f5e8 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5ef as i32 as i64,
                last: 0x1f5ef as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5f3 as i32 as i64,
                last: 0x1f5f3 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5fa as i32 as i64,
                last: 0x1f5fa as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6cb as i32 as i64,
                last: 0x1f6cf as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6e0 as i32 as i64,
                last: 0x1f6e5 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6e9 as i32 as i64,
                last: 0x1f6e9 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6f0 as i32 as i64,
                last: 0x1f6f0 as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6f3 as i32 as i64,
                last: 0x1f6f3 as i32 as i64,
            };
            init
        },
    ];
    use super::{convertStruct, interval};
}
use self::arabic_h_generated_h::{arabic_combine, arabic_maycombine};
use self::cursor_h_generated_h::get_cursor_pos_ptr;
use self::env_h_generated_h::os_getenv;
use self::libintl_h::gettext;
use self::log_h_generated_h::logmsg;
use self::mark_h_generated_h::mark_mb_adjustpos;
use self::memline_h_generated_h::ml_get_buf;
use self::misc1_h_generated_h::beep_flush;
use self::stdlib_h::{abort, calloc, free, malloc, realloc};
use self::string_h::{memcpy, memmove, memset, strchr, strcmp, strcpy, strlen, strncmp};
use self::strings_h::strncasecmp;
use self::wctype_wchar_h::{towlower, towupper};
/*
 * Canonical encoding names and their properties.
 * "iso-8859-n" is handled by enc_canonize() directly.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub name: *const i8,
    pub prop: i32,
    pub codepage: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct interval {
    pub first: i64,
    pub last: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clinterval {
    pub first: u32,
    pub last: u32,
    pub class: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct convertStruct {
    pub rangeStart: i32,
    pub rangeEnd: i32,
    pub step: i32,
    pub offset: i32,
}
/*
 * Aliases for encoding names.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub name: *const i8,
    pub canon: i32,
}
#[no_mangle]
pub static mut e_loadlib: [u8; 32] = unsafe { *::std::mem::transmute::<&[u8; 32], &mut [u8; 32]>(b"E370: Could not load library %s\x00") };
#[no_mangle]
pub static mut e_loadfunc: [u8; 41] = unsafe { *::std::mem::transmute::<&[u8; 41], &mut [u8; 41]>(b"E448: Could not load library function %s\x00") };
// To speed up BYTELEN(); keep a lookup table to quickly get the length in
// bytes of a UTF-8 character from the first byte of a UTF-8 string.  Bytes
// which are illegal when used as the first byte have a 1.  The NUL byte has
// length 1.
#[no_mangle]
pub static mut utf8len_tab: [u8; 256] = [
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    5,
    5,
    5,
    5,
    6,
    6,
    1,
    1,
];
// Like utf8len_tab above, but using a zero for illegal lead bytes.
#[no_mangle]
pub static mut utf8len_tab_zero: [u8; 256] = [
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    5,
    5,
    5,
    5,
    6,
    6,
    0,
    0,
];
static mut enc_canon_table: [C2RustUnnamed_2; 59] = [
    {
        let mut init = C2RustUnnamed_2 {
            name: b"latin1\x00" as *const u8 as *const i8,
            prop: ENC_8BIT + ENC_LATIN1,
            codepage: 1252,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-2\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-3\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-4\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-5\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-6\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-7\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-8\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-9\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-10\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-11\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-13\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-14\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"iso-8859-15\x00" as *const u8 as *const i8,
            prop: ENC_8BIT + ENC_LATIN9,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"koi8-r\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"koi8-u\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"utf-8\x00" as *const u8 as *const i8,
            prop: ENC_UNICODE,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"ucs-2\x00" as *const u8 as *const i8,
            prop: ENC_UNICODE + ENC_ENDIAN_B + ENC_2BYTE,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"ucs-2le\x00" as *const u8 as *const i8,
            prop: ENC_UNICODE + ENC_ENDIAN_L + ENC_2BYTE,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"utf-16\x00" as *const u8 as *const i8,
            prop: ENC_UNICODE + ENC_ENDIAN_B + ENC_2WORD,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"utf-16le\x00" as *const u8 as *const i8,
            prop: ENC_UNICODE + ENC_ENDIAN_L + ENC_2WORD,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"ucs-4\x00" as *const u8 as *const i8,
            prop: ENC_UNICODE + ENC_ENDIAN_B + ENC_4BYTE,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"ucs-4le\x00" as *const u8 as *const i8,
            prop: ENC_UNICODE + ENC_ENDIAN_L + ENC_4BYTE,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"debug\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_DEBUG,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"euc-jp\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_JPNU,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"sjis\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_JPN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"euc-kr\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_KORU,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"euc-cn\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_CHSU,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"euc-tw\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_CHTU,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"big5\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_CHT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp437\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 437,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp737\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 737,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp775\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 775,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp850\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 850,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp852\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 852,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp855\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 855,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp857\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 857,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp860\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 860,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp861\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 861,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp862\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 862,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp863\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 863,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp865\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 865,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp866\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 866,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp869\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 869,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp874\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 874,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp932\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_JPN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp936\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_CHS,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp949\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_KOR,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp950\x00" as *const u8 as *const i8,
            prop: ENC_DBCS,
            codepage: DBCS_CHT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1250\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1250,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1251\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1251,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1253\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1253,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1254\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1254,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1255\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1255,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1256\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1256,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1257\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1257,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"cp1258\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 1258,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"macroman\x00" as *const u8 as *const i8,
            prop: ENC_8BIT + ENC_MACROMAN,
            codepage: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            name: b"hp-roman8\x00" as *const u8 as *const i8,
            prop: ENC_8BIT,
            codepage: 0,
        };
        init
    },
];
pub const IDX_LATIN_1: i32 = 0;
pub const IDX_ISO_2: i32 = 1;
pub const IDX_ISO_3: i32 = 2;
pub const IDX_ISO_4: i32 = 3;
pub const IDX_ISO_5: i32 = 4;
pub const IDX_ISO_6: i32 = 5;
pub const IDX_ISO_7: i32 = 6;
pub const IDX_ISO_8: i32 = 7;
pub const IDX_ISO_9: i32 = 8;
pub const IDX_ISO_10: i32 = 9;
pub const IDX_ISO_11: i32 = 10;
pub const IDX_ISO_13: i32 = 11;
pub const IDX_ISO_14: i32 = 12;
pub const IDX_ISO_15: i32 = 13;
pub const IDX_UTF8: i32 = 16;
pub const IDX_UCS2: i32 = 17;
pub const IDX_UCS2LE: i32 = 18;
pub const IDX_UTF16: i32 = 19;
pub const IDX_UTF16LE: i32 = 20;
pub const IDX_UCS4: i32 = 21;
pub const IDX_UCS4LE: i32 = 22;
pub const IDX_EUC_JP: i32 = 24;
pub const IDX_SJIS: i32 = 25;
pub const IDX_EUC_KR: i32 = 26;
pub const IDX_EUC_CN: i32 = 27;
pub const IDX_EUC_TW: i32 = 28;
pub const IDX_BIG5: i32 = 29;
pub const IDX_CP932: i32 = 45;
pub const IDX_CP936: i32 = 46;
pub const IDX_CP949: i32 = 47;
pub const IDX_CP950: i32 = 48;
pub const IDX_MACROMAN: i32 = 57;
pub const IDX_COUNT: i32 = 59;
static mut enc_alias_table: [C2RustUnnamed_13; 64] = [
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ansi\x00" as *const u8 as *const i8,
            canon: IDX_LATIN_1,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"iso-8859-1\x00" as *const u8 as *const i8,
            canon: IDX_LATIN_1,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin2\x00" as *const u8 as *const i8,
            canon: IDX_ISO_2,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin3\x00" as *const u8 as *const i8,
            canon: IDX_ISO_3,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin4\x00" as *const u8 as *const i8,
            canon: IDX_ISO_4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"cyrillic\x00" as *const u8 as *const i8,
            canon: IDX_ISO_5,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"arabic\x00" as *const u8 as *const i8,
            canon: IDX_ISO_6,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"greek\x00" as *const u8 as *const i8,
            canon: IDX_ISO_7,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"hebrew\x00" as *const u8 as *const i8,
            canon: IDX_ISO_8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin5\x00" as *const u8 as *const i8,
            canon: IDX_ISO_9,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"turkish\x00" as *const u8 as *const i8,
            canon: IDX_ISO_9,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin6\x00" as *const u8 as *const i8,
            canon: IDX_ISO_10,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"nordic\x00" as *const u8 as *const i8,
            canon: IDX_ISO_10,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"thai\x00" as *const u8 as *const i8,
            canon: IDX_ISO_11,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin7\x00" as *const u8 as *const i8,
            canon: IDX_ISO_13,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin8\x00" as *const u8 as *const i8,
            canon: IDX_ISO_14,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"latin9\x00" as *const u8 as *const i8,
            canon: IDX_ISO_15,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf8\x00" as *const u8 as *const i8,
            canon: IDX_UTF8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"unicode\x00" as *const u8 as *const i8,
            canon: IDX_UCS2,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs2\x00" as *const u8 as *const i8,
            canon: IDX_UCS2,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs2be\x00" as *const u8 as *const i8,
            canon: IDX_UCS2,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs-2be\x00" as *const u8 as *const i8,
            canon: IDX_UCS2,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs2le\x00" as *const u8 as *const i8,
            canon: IDX_UCS2LE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf16\x00" as *const u8 as *const i8,
            canon: IDX_UTF16,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf16be\x00" as *const u8 as *const i8,
            canon: IDX_UTF16,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf-16be\x00" as *const u8 as *const i8,
            canon: IDX_UTF16,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf16le\x00" as *const u8 as *const i8,
            canon: IDX_UTF16LE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs4\x00" as *const u8 as *const i8,
            canon: IDX_UCS4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs4be\x00" as *const u8 as *const i8,
            canon: IDX_UCS4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs-4be\x00" as *const u8 as *const i8,
            canon: IDX_UCS4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ucs4le\x00" as *const u8 as *const i8,
            canon: IDX_UCS4LE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf32\x00" as *const u8 as *const i8,
            canon: IDX_UCS4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf-32\x00" as *const u8 as *const i8,
            canon: IDX_UCS4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf32be\x00" as *const u8 as *const i8,
            canon: IDX_UCS4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf-32be\x00" as *const u8 as *const i8,
            canon: IDX_UCS4,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf32le\x00" as *const u8 as *const i8,
            canon: IDX_UCS4LE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"utf-32le\x00" as *const u8 as *const i8,
            canon: IDX_UCS4LE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"932\x00" as *const u8 as *const i8,
            canon: IDX_CP932,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"949\x00" as *const u8 as *const i8,
            canon: IDX_CP949,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"936\x00" as *const u8 as *const i8,
            canon: IDX_CP936,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"gbk\x00" as *const u8 as *const i8,
            canon: IDX_CP936,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"950\x00" as *const u8 as *const i8,
            canon: IDX_CP950,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"eucjp\x00" as *const u8 as *const i8,
            canon: IDX_EUC_JP,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"unix-jis\x00" as *const u8 as *const i8,
            canon: IDX_EUC_JP,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"ujis\x00" as *const u8 as *const i8,
            canon: IDX_EUC_JP,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"shift-jis\x00" as *const u8 as *const i8,
            canon: IDX_SJIS,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"pck\x00" as *const u8 as *const i8,
            canon: IDX_SJIS,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"euckr\x00" as *const u8 as *const i8,
            canon: IDX_EUC_KR,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"5601\x00" as *const u8 as *const i8,
            canon: IDX_EUC_KR,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"euccn\x00" as *const u8 as *const i8,
            canon: IDX_EUC_CN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"gb2312\x00" as *const u8 as *const i8,
            canon: IDX_EUC_CN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"euctw\x00" as *const u8 as *const i8,
            canon: IDX_EUC_TW,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"japan\x00" as *const u8 as *const i8,
            canon: IDX_EUC_JP,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"korea\x00" as *const u8 as *const i8,
            canon: IDX_EUC_KR,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"prc\x00" as *const u8 as *const i8,
            canon: IDX_EUC_CN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"zh-cn\x00" as *const u8 as *const i8,
            canon: IDX_EUC_CN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"chinese\x00" as *const u8 as *const i8,
            canon: IDX_EUC_CN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"zh-tw\x00" as *const u8 as *const i8,
            canon: IDX_EUC_TW,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"taiwan\x00" as *const u8 as *const i8,
            canon: IDX_EUC_TW,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"cp950\x00" as *const u8 as *const i8,
            canon: IDX_BIG5,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"950\x00" as *const u8 as *const i8,
            canon: IDX_BIG5,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"mac\x00" as *const u8 as *const i8,
            canon: IDX_MACROMAN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 {
            name: b"mac-roman\x00" as *const u8 as *const i8,
            canon: IDX_MACROMAN,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_13 { name: NULL_0 as *const i8, canon: 0 };
        init
    },
];
/*
 * Find encoding "name" in the list of canonical encoding names.
 * Returns -1 if not found.
 */
unsafe extern "C" fn enc_canon_search(mut name: *const u8) -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while i < IDX_COUNT {
        if strcmp(name as *mut i8, enc_canon_table[i as usize].name as *mut i8) == 0 {
            return i;
        }
        i += 1
    }
    return -(1);
}
/*
 * Find canonical encoding "name" in the list and return its properties.
 * Returns 0 if not found.
 */
#[no_mangle]
pub unsafe extern "C" fn enc_canon_props(mut name: *const u8) -> i32 {
    let mut i: i32 = 0;
    i = enc_canon_search(name);
    if i >= 0 {
        return enc_canon_table[i as usize].prop;
    }
    if strncmp(name as *mut i8, b"2byte-\x00" as *const u8 as *const i8 as *mut i8, 6) == 0 {
        return ENC_DBCS;
    }
    if strncmp(name as *mut i8, b"8bit-\x00" as *const u8 as *const i8 as *mut i8, 5) == 0 || strncmp(name as *mut i8, b"iso-8859-\x00" as *const u8 as *const i8 as *mut i8, 9) == 0 {
        return ENC_8BIT;
    }
    return 0;
}
/*
 * Return the size of the BOM for the current buffer:
 * 0 - no BOM
 * 2 - UCS-2 or UTF-16 BOM
 * 4 - UCS-4 BOM
 * 3 - UTF-8 BOM
 */
#[no_mangle]
pub unsafe extern "C" fn bomb_size() -> i32 {
    let mut n = 0;
    if (*curbuf).b_p_bomb != 0 && (*curbuf).b_p_bin == 0 {
        if *(*curbuf).b_p_fenc as i32 == NUL || strcmp((*curbuf).b_p_fenc as *mut i8, b"utf-8\x00" as *const u8 as *const i8 as *mut i8) == 0 {
            n = 3
        } else if strncmp((*curbuf).b_p_fenc as *mut i8, b"ucs-2\x00" as *const u8 as *const i8 as *mut i8, 5) == 0 || strncmp((*curbuf).b_p_fenc as *mut i8, b"utf-16\x00" as *const u8 as *const i8 as *mut i8, 6) == 0 {
            n = 2
        } else if strncmp((*curbuf).b_p_fenc as *mut i8, b"ucs-4\x00" as *const u8 as *const i8 as *mut i8, 5) == 0 {
            n = 4
        }
    }
    return n;
}
/*
 * Remove all BOM from "s" by moving remaining text.
 */
#[no_mangle]
pub unsafe extern "C" fn remove_bom(mut s: *mut u8) {
    let mut p = s as *mut i8;
    loop {
        p = strchr(p, 0xef as i32);
        if p.is_null() {
            break;
        }
        if *p.offset(1) as u8 as i32 == 0xbb as i32 && *p.offset(2) as u8 as i32 == 0xbf as i32 {
            memmove(p as *mut libc::c_void, p.offset(3) as *const libc::c_void, strlen(p.offset(3)).wrapping_add(1));
        } else {
            p = p.offset(1)
        }
    }
}
/*
 * Get class of pointer:
 * 0 for blank or NUL
 * 1 for punctuation
 * 2 for an (ASCII) word character
 * >2 for other word characters
 */
#[no_mangle]
pub unsafe extern "C" fn mb_get_class(mut p: *const u8) -> i32 {
    return mb_get_class_tab(p, (*curbuf).b_chartab.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mb_get_class_tab(mut p: *const u8, chartab: *const u64) -> i32 {
    if utf8len_tab[*p.offset(0) as usize] as i32 == 1 {
        if *p.offset(0) as i32 == NUL || ascii_iswhite(*p.offset(0) as i32) as i32 != 0 {
            return 0;
        }
        if vim_iswordc_tab(*p.offset(0) as i32, chartab) {
            return 2;
        }
        return 1;
    }
    return utf_class_tab(utf_ptr2char(p), chartab);
}
/*
 * Return true if "c" is in "table".
 */
unsafe extern "C" fn intable(mut table: *const interval, mut n_items: size_t, mut c: i32) -> bool {
    let mut mid: i32 = 0;
    let mut bot: i32 = 0;
    let mut top: i32 = 0;
    /* first quick check for Latin1 etc. characters */
    if (c as i64) < (*table.offset(0)).first {
        return false;
    }
    /* binary search in table */
    bot = 0;
    top = n_items.wrapping_sub(1) as i32;
    while top >= bot {
        mid = (bot + top) / 2;
        if (*table.offset(mid as isize)).last < c as i64 {
            bot = mid + 1
        } else if (*table.offset(mid as isize)).first > c as i64 {
            top = mid - 1
        } else {
            return true;
        }
    }
    return false;
}
// / For UTF-8 character "c" return 2 for a double-width character, 1 for others.
// / Returns 4 or 6 for an unprintable character.
// / Is only correct for characters >= 0x80.
// / When p_ambw is "double", return 2 for a character with East Asian Width
// / class 'A'(mbiguous).
// /
// / @note Tables `doublewidth` and `ambiguous` are generated by
// /       gen_unicode_tables.lua, which must be manually invoked as needed.
#[no_mangle]
pub unsafe extern "C" fn utf_char2cells(mut c: i32) -> i32 {
    if c >= 0x100 as i32 {
        if !utf_printable(c) {
            return 6;
            // unprintable, displays <xxxx>
        }
        if intable(
            doublewidth.as_ptr(),
            (::std::mem::size_of::<[interval; 113]>() as u64)
                .wrapping_div(::std::mem::size_of::<interval>() as u64)
                .wrapping_div(((::std::mem::size_of::<[interval; 113]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as i32 as size_t),
            c,
        ) {
            return 2;
        }
        if p_emoji != 0
            && intable(
                emoji_width.as_ptr(),
                (::std::mem::size_of::<[interval; 39]>() as u64)
                    .wrapping_div(::std::mem::size_of::<interval>() as u64)
                    .wrapping_div(((::std::mem::size_of::<[interval; 39]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as i32 as size_t),
                c,
            ) as i32
                != 0
        {
            return 2;
        }
    } else if c >= 0x80 as i32 && !vim_isprintc(c) {
        // Characters below 0x100 are influenced by 'isprint' option.
        return 4;
        // unprintable, displays <xx>
    }
    if c >= 0x80 as i32
        && *p_ambw as i32 == 'd' as i32
        && intable(
            ambiguous.as_ptr(),
            (::std::mem::size_of::<[interval; 179]>() as u64)
                .wrapping_div(::std::mem::size_of::<interval>() as u64)
                .wrapping_div(((::std::mem::size_of::<[interval; 179]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as i32 as size_t),
            c,
        ) as i32
            != 0
    {
        return 2;
    }
    return 1;
}
// / Return the number of display cells character at "*p" occupies.
// / This doesn't take care of unprintable characters, use ptr2cells() for that.
#[no_mangle]
pub unsafe extern "C" fn utf_ptr2cells(mut p: *const u8) -> i32 {
    let mut c: i32 = 0;
    /* Need to convert to a wide character. */
    if *p as i32 >= 0x80 as i32 {
        c = utf_ptr2char(p);
        /* An illegal byte is displayed as <xx>. */
        if utf_ptr2len(p) == 1 || c == NUL {
            return 4;
        }
        /* If the char is ASCII it must be an overlong sequence. */
        if c < 0x80 as i32 {
            return char2cells(c);
        }
        return utf_char2cells(c);
    }
    return 1;
}
// / Like utf_ptr2cells(), but limit string length to "size".
// / For an empty string or truncated character returns 1.
#[no_mangle]
pub unsafe extern "C" fn utf_ptr2cells_len(mut p: *const u8, mut size: i32) -> i32 {
    let mut c: i32 = 0;
    /* Need to convert to a wide character. */
    if size > 0 && *p as i32 >= 0x80 as i32 {
        if utf_ptr2len_len(p, size) < utf8len_tab[*p as usize] as i32 {
            return 1;
        } /* truncated */
        c = utf_ptr2char(p);
        /* An illegal byte is displayed as <xx>. */
        if utf_ptr2len(p) == 1 || c == NUL {
            return 4;
        }
        /* If the char is ASCII it must be an overlong sequence. */
        if c < 0x80 as i32 {
            return char2cells(c);
        }
        return utf_char2cells(c);
    }
    return 1;
}
// / Calculate the number of cells occupied by string `str`.
// /
// / @param str The source string, may not be NULL, must be a NUL-terminated
// /            string.
// / @return The number of cells occupied by string `str`
#[no_mangle]
pub unsafe extern "C" fn mb_string2cells(mut str: *const u8) -> size_t {
    let mut clen = 0;
    let mut p = str;
    while *p as i32 != NUL {
        clen = (clen as u64).wrapping_add(utf_ptr2cells(p) as u64) as size_t as size_t;
        p = p.offset(Some(Some(mb_ptr2len).expect("non-null function pointer")).expect("non-null function pointer")(p) as isize)
    }
    return clen;
}
// / Get the number of cells occupied by string `str` with maximum length `size`
// /
// / @param str The source string, may not be NULL, must be a NUL-terminated
// /            string.
// / @param size maximum length of string. It will terminate on earlier NUL.
// / @return The number of cells occupied by string `str`
#[no_mangle]
pub unsafe extern "C" fn mb_string2cells_len(mut str: *const u8, mut size: size_t) -> size_t {
    let mut clen = 0;
    let mut p = str;
    while *p as i32 != NUL && p < str.offset(size as isize) {
        clen = (clen as u64).wrapping_add(utf_ptr2cells(p) as u64) as size_t as size_t;
        p = p.offset(utf_ptr2len_len(p, size.wrapping_add(p.offset_from(str) as i64 as u64) as i32) as isize)
    }
    return clen;
}
// / Convert a UTF-8 byte sequence to a wide character
// /
// / If the sequence is illegal or truncated by a NUL then the first byte is
// / returned.
// / For an overlong sequence this may return zero.
// / Does not include composing characters for obvious reasons.
// /
// / @param[in]  p  String to convert.
// /
// / @return Unicode codepoint or byte value.
#[no_mangle]
pub unsafe extern "C" fn utf_ptr2char(p: *const u8) -> i32 {
    if (*p.offset(0) as i32) < 0x80 as i32 {
        // Be quick for ASCII.
        return *p.offset(0) as i32;
    }
    let len = utf8len_tab_zero[*p.offset(0) as usize];
    if len as i32 > 1 && *p.offset(1) as i32 & 0xc0 as i32 == 0x80 as i32 {
        if len as i32 == 2 {
            return ((*p.offset(0) as i32 & 0x1f as i32) << 6) + (*p.offset(1) as i32 & 0x3f as i32);
        }
        if *p.offset(2) as i32 & 0xc0 as i32 == 0x80 as i32 {
            if len as i32 == 3 {
                return ((*p.offset(0) as i32 & 0xf as i32) << 12) + ((*p.offset(1) as i32 & 0x3f as i32) << 6) + (*p.offset(2) as i32 & 0x3f as i32);
            }
            if *p.offset(3) as i32 & 0xc0 as i32 == 0x80 as i32 {
                if len as i32 == 4 {
                    return ((*p.offset(0) as i32 & 0x7 as i32) << 18) + ((*p.offset(1) as i32 & 0x3f as i32) << 12) + ((*p.offset(2) as i32 & 0x3f as i32) << 6) + (*p.offset(3) as i32 & 0x3f as i32);
                }
                if *p.offset(4) as i32 & 0xc0 as i32 == 0x80 as i32 {
                    if len as i32 == 5 {
                        return ((*p.offset(0) as i32 & 0x3 as i32) << 24) + ((*p.offset(1) as i32 & 0x3f as i32) << 18) + ((*p.offset(2) as i32 & 0x3f as i32) << 12) + ((*p.offset(3) as i32 & 0x3f as i32) << 6) + (*p.offset(4) as i32 & 0x3f as i32);
                    }
                    if *p.offset(5) as i32 & 0xc0 as i32 == 0x80 as i32 && len as i32 == 6 {
                        return ((*p.offset(0) as i32 & 0x1 as i32) << 30) + ((*p.offset(1) as i32 & 0x3f as i32) << 24) + ((*p.offset(2) as i32 & 0x3f as i32) << 18) + ((*p.offset(3) as i32 & 0x3f as i32) << 12) + ((*p.offset(4) as i32 & 0x3f as i32) << 6) + (*p.offset(5) as i32 & 0x3f as i32);
                    }
                }
            }
        }
    }
    // Illegal value: just return the first byte.
    return *p.offset(0) as i32;
}
/*
 * Convert a UTF-8 byte sequence to a wide character.
 * String is assumed to be terminated by NUL or after "n" bytes, whichever
 * comes first.
 * The function is safe in the sense that it never accesses memory beyond the
 * first "n" bytes of "s".
 *
 * On success, returns decoded codepoint, advances "s" to the beginning of
 * next character and decreases "n" accordingly.
 *
 * If end of string was reached, returns 0 and, if "n" > 0, advances "s" past
 * NUL byte.
 *
 * If byte sequence is illegal or incomplete, returns -1 and does not advance
 * "s".
 */
unsafe extern "C" fn utf_safe_read_char_adv(mut s: *mut *const u8, mut n: *mut size_t) -> i32 {
    let mut c: i32 = 0;
    if *n == 0 {
        /* end of buffer */
        return 0;
    }
    let mut k = utf8len_tab_zero[**s as usize];
    if k as i32 == 1 {
        /* ASCII character or NUL */
        *n = (*n).wrapping_sub(1);
        let fresh3 = *s;
        *s = (*s).offset(1);
        return *fresh3 as i32;
    }
    if k as u64 <= *n {
        /* We have a multibyte sequence and it isn't truncated by buffer
         * limits so utf_ptr2char() is safe to use. Or the first byte is
         * illegal (k=0), and it's also safe to use utf_ptr2char(). */
        c = utf_ptr2char(*s);
        /* On failure, utf_ptr2char() returns the first byte, so here we
         * check equality with the first byte. The only non-ASCII character
         * which equals the first byte of its own UTF-8 representation is
         * U+00C3 (UTF-8: 0xC3 0x83), so need to check that special case too.
         * It's safe even if n=1, else we would have k=2 > n. */
        if c != **s as i32 || c == 0xc3 as i32 && *(*s).offset(1) as i32 == 0x83 as i32 {
            /* byte sequence was successfully decoded */
            *s = (*s).offset(k as i32 as isize);
            *n = (*n as u64).wrapping_sub(k as u64) as size_t as size_t;
            return c;
        }
    }
    /* byte sequence is incomplete or illegal */
    return -(1);
}
/*
 * Get character at **pp and advance *pp to the next character.
 * Note: composing characters are skipped!
 */
#[no_mangle]
pub unsafe extern "C" fn mb_ptr2char_adv(pp: *mut *const u8) -> i32 {
    let mut c: i32 = 0;
    c = utf_ptr2char(*pp);
    *pp = (*pp).offset(Some(Some(mb_ptr2len).expect("non-null function pointer")).expect("non-null function pointer")(*pp) as isize);
    return c;
}
/*
 * Get character at **pp and advance *pp to the next character.
 * Note: composing characters are returned as separate characters.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_cptr2char_adv(mut pp: *mut *const u8) -> i32 {
    let mut c: i32 = 0;
    c = utf_ptr2char(*pp);
    *pp = (*pp).offset(utf_ptr2len(*pp) as isize);
    return c;
}
/*
 * Check if the character pointed to by "p2" is a composing character when it
 * comes after "p1".  For Arabic sometimes "ab" is replaced with "c", which
 * behaves like a composing character.
 */
#[no_mangle]
pub unsafe extern "C" fn utf_composinglike(mut p1: *const u8, mut p2: *const u8) -> bool {
    let mut c2: i32 = 0;
    c2 = utf_ptr2char(p2);
    if utf_iscomposing(c2) {
        return true;
    }
    if !arabic_maycombine(c2) {
        return false;
    }
    return arabic_combine(utf_ptr2char(p1), c2);
}
// / Convert a UTF-8 string to a wide character
// /
// / Also gets up to #MAX_MCO composing characters.
// /
// / @param[out]  pcc  Location where to store composing characters. Must have
// /                   space at least for #MAX_MCO + 1 elements.
// /
// / @return leading character.
#[no_mangle]
pub unsafe extern "C" fn utfc_ptr2char(mut p: *const u8, mut pcc: *mut i32) -> i32 {
    let mut len: i32 = 0;
    let mut c: i32 = 0;
    let mut cc: i32 = 0;
    let mut i = 0;
    c = utf_ptr2char(p);
    len = utf_ptr2len(p);
    /* Only accept a composing char when the first char isn't illegal. */
    if (len > 1 || (*p as i32) < 0x80 as i32) && *p.offset(len as isize) as i32 >= 0x80 as i32 && utf_composinglike(p, p.offset(len as isize)) as i32 != 0 {
        cc = utf_ptr2char(p.offset(len as isize));
        loop {
            let fresh4 = i;
            i = i + 1;
            *pcc.offset(fresh4 as isize) = cc;
            if i == MAX_MCO {
                break;
            }
            len += utf_ptr2len(p.offset(len as isize));
            if (*p.offset(len as isize) as i32) < 0x80 as i32 || {
                cc = utf_ptr2char(p.offset(len as isize));
                !utf_iscomposing(cc)
            } {
                break;
            }
        }
    }
    if i < MAX_MCO {
        /* last composing char must be 0 */
        *pcc.offset(i as isize) = 0
    }
    return c;
}
/*
 * Convert a UTF-8 byte string to a wide character.  Also get up to MAX_MCO
 * composing characters.  Use no more than p[maxlen].
 *
 * @param [out] pcc: composing chars, last one is 0
 */
#[no_mangle]
pub unsafe extern "C" fn utfc_ptr2char_len(mut p: *const u8, mut pcc: *mut i32, mut maxlen: i32) -> i32 {
    if maxlen > 0 {
    } else {
        assert!(false, "maxlen > 0");
    }
    let mut i = 0;
    let mut len = utf_ptr2len_len(p, maxlen);
    // Is it safe to use utf_ptr2char()?
    let mut safe = len > 1 && len <= maxlen;
    let mut c = if safe as i32 != 0 { utf_ptr2char(p) } else { *p as i32 };
    // Only accept a composing char when the first char isn't illegal.
    if (safe as i32 != 0 || c < 0x80 as i32) && len < maxlen && *p.offset(len as isize) as i32 >= 0x80 as i32 {
        while i < MAX_MCO {
            let mut len_cc = utf_ptr2len_len(p.offset(len as isize), maxlen - len);
            safe = len_cc > 1 && len_cc <= maxlen - len;
            if !safe
                || {
                    let ref mut fresh5 = *pcc.offset(i as isize);
                    *fresh5 = utf_ptr2char(p.offset(len as isize));
                    (*fresh5) < 0x80 as i32
                }
                || (if i == 0 { utf_composinglike(p, p.offset(len as isize)) as i32 } else { utf_iscomposing(*pcc.offset(i as isize)) as i32 }) == 0
            {
                break;
            }
            len += len_cc;
            i += 1
        }
    }
    if i < MAX_MCO {
        // last composing char must be 0
        *pcc.offset(i as isize) = 0
    }
    return c;
}
// / Get the length of a UTF-8 byte sequence representing a single codepoint
// /
// / @param[in]  p  UTF-8 string.
// /
// / @return Sequence length, 0 for empty string and 1 for non-UTF-8 byte
// /         sequence.
#[no_mangle]
pub unsafe extern "C" fn utf_ptr2len(p: *const u8) -> i32 {
    if *p as i32 == NUL {
        return 0;
    }
    let len = utf8len_tab[*p as usize] as i32;
    let mut i = 1;
    while i < len {
        if *p.offset(i as isize) as i32 & 0xc0 as i32 != 0x80 as i32 {
            return 1;
        }
        i += 1
    }
    return len;
}
/*
 * Return length of UTF-8 character, obtained from the first byte.
 * "b" must be between 0 and 255!
 * Returns 1 for an invalid first byte value.
 */
#[no_mangle]
pub unsafe extern "C" fn utf_byte2len(mut b: i32) -> i32 {
    return utf8len_tab[b as usize] as i32;
}
/*
 * Get the length of UTF-8 byte sequence "p[size]".  Does not include any
 * following composing characters.
 * Returns 1 for "".
 * Returns 1 for an illegal byte sequence (also in incomplete byte seq.).
 * Returns number > "size" for an incomplete byte sequence.
 * Never returns zero.
 */
#[no_mangle]
pub unsafe extern "C" fn utf_ptr2len_len(mut p: *const u8, mut size: i32) -> i32 {
    let mut len: i32 = 0; /* NUL, ascii or illegal lead byte */
    let mut i: i32 = 0; /* incomplete byte sequence. */
    let mut m: i32 = 0;
    len = utf8len_tab[*p as usize] as i32;
    if len == 1 {
        return 1;
    }
    if len > size {
        m = size
    } else {
        m = len
    }
    i = 1;
    while i < m {
        if *p.offset(i as isize) as i32 & 0xc0 as i32 != 0x80 as i32 {
            return 1;
        }
        i += 1
    }
    return len;
}
// / Return the number of bytes occupied by a UTF-8 character in a string
// /
// / This includes following composing characters.
#[no_mangle]
pub unsafe extern "C" fn utfc_ptr2len(p: *const u8) -> i32 {
    let mut b0 = *p;
    if b0 as i32 == NUL {
        return 0;
    }
    if (b0 as i32) < 0x80 as i32 && (*p.offset(1) as i32) < 0x80 as i32 {
        // be quick for ASCII
        return 1;
    }
    // Skip over first UTF-8 char, stopping at a NUL byte.
    let mut len = utf_ptr2len(p);
    // Check for illegal byte.
    if len == 1 && b0 as i32 >= 0x80 as i32 {
        return 1;
    }
    // Check for composing characters.  We can handle only the first six, but
    // skip all of them (otherwise the cursor would get stuck).
    let mut prevlen = 0;
    loop {
        if (*p.offset(len as isize) as i32) < 0x80 as i32 || !utf_composinglike(p.offset(prevlen as isize), p.offset(len as isize)) {
            return len;
        }
        // Skip over composing char.
        prevlen = len;
        len += utf_ptr2len(p.offset(len as isize))
    }
}
/*
 * Return the number of bytes the UTF-8 encoding of the character at "p[size]"
 * takes.  This includes following composing characters.
 * Returns 0 for an empty string.
 * Returns 1 for an illegal char or an incomplete byte sequence.
 */
#[no_mangle]
pub unsafe extern "C" fn utfc_ptr2len_len(mut p: *const u8, mut size: i32) -> i32 {
    let mut len: i32 = 0;
    let mut prevlen: i32 = 0;
    if size < 1 || *p as i32 == NUL {
        return 0;
    }
    if (*p.offset(0) as i32) < 0x80 as i32 && (size == 1 || (*p.offset(1) as i32) < 0x80 as i32) {
        /* be quick for ASCII */
        return 1;
    }
    /* Skip over first UTF-8 char, stopping at a NUL byte. */
    len = utf_ptr2len_len(p, size);
    /* Check for illegal byte and incomplete byte sequence. */
    if len == 1 && *p.offset(0) as i32 >= 0x80 as i32 || len > size {
        return 1;
    }
    /*
     * Check for composing characters.  We can handle only the first six, but
     * skip all of them (otherwise the cursor would get stuck).
     */
    prevlen = 0;
    while len < size {
        let mut len_next_char: i32 = 0;
        if (*p.offset(len as isize) as i32) < 0x80 as i32 {
            break;
        }
        /*
         * Next character length should not go beyond size to ensure that
         * UTF_COMPOSINGLIKE(...) does not read beyond size.
         */
        len_next_char = utf_ptr2len_len(p.offset(len as isize), size - len);
        if len_next_char > size - len {
            break;
        }
        if !utf_composinglike(p.offset(prevlen as isize), p.offset(len as isize)) {
            break;
        }
        /* Skip over composing char */
        prevlen = len;
        len += len_next_char
    }
    return len;
}
// / Determine how many bytes certain unicode codepoint will occupy
#[no_mangle]
pub unsafe extern "C" fn utf_char2len(c: i32) -> i32 {
    if c < 0x80 as i32 {
        return 1;
    } else if c < 0x800 as i32 {
        return 2;
    } else if c < 0x10000 as i32 {
        return 3;
    } else if c < 0x200000 as i32 {
        return 4;
    } else if c < 0x4000000 as i32 {
        return 5;
    } else {
        return 6;
    };
}
// / Convert Unicode character to UTF-8 string
// /
// / @param c character to convert to \p buf
// / @param[out] buf UTF-8 string generated from \p c, does not add \0
// / @return Number of bytes (1-6).
#[no_mangle]
pub unsafe extern "C" fn utf_char2bytes(c: i32, buf: *mut u8) -> i32 {
    if c < 0x80 as i32 {
        // 7 bits
        *buf.offset(0) = c as u8;
        return 1;
    } else if c < 0x800 as i32 {
        // 11 bits
        *buf.offset(0) = (0xc0 as i32 as u32).wrapping_add(c as u32 >> 6) as u8;
        *buf.offset(1) = (0x80 as i32 + (c & 0x3f as i32)) as u8;
        return 2;
    } else if c < 0x10000 as i32 {
        // 16 bits
        *buf.offset(0) = (0xe0 as i32 as u32).wrapping_add(c as u32 >> 12) as u8;
        *buf.offset(1) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 6 & 0x3f as i32 as u32) as u8;
        *buf.offset(2) = (0x80 as i32 + (c & 0x3f as i32)) as u8;
        return 3;
    } else if c < 0x200000 as i32 {
        // 21 bits
        *buf.offset(0) = (0xf0 as i32 as u32).wrapping_add(c as u32 >> 18) as u8; // 31 bits
        *buf.offset(1) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 12 & 0x3f as i32 as u32) as u8;
        *buf.offset(2) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 6 & 0x3f as i32 as u32) as u8;
        *buf.offset(3) = (0x80 as i32 + (c & 0x3f as i32)) as u8;
        return 4;
    } else if c < 0x4000000 as i32 {
        // 26 bits
        *buf.offset(0) = (0xf8 as i32 as u32).wrapping_add(c as u32 >> 24) as u8;
        *buf.offset(1) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 18 & 0x3f as i32 as u32) as u8;
        *buf.offset(2) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 12 & 0x3f as i32 as u32) as u8;
        *buf.offset(3) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 6 & 0x3f as i32 as u32) as u8;
        *buf.offset(4) = (0x80 as i32 + (c & 0x3f as i32)) as u8;
        return 5;
    } else {
        *buf.offset(0) = (0xfc as i32 as u32).wrapping_add(c as u32 >> 30) as u8;
        *buf.offset(1) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 24 & 0x3f as i32 as u32) as u8;
        *buf.offset(2) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 18 & 0x3f as i32 as u32) as u8;
        *buf.offset(3) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 12 & 0x3f as i32 as u32) as u8;
        *buf.offset(4) = (0x80 as i32 as u32).wrapping_add(c as u32 >> 6 & 0x3f as i32 as u32) as u8;
        *buf.offset(5) = (0x80 as i32 + (c & 0x3f as i32)) as u8;
        return 6;
    };
}
/*
 * Return true if "c" is a composing UTF-8 character.  This means it will be
 * drawn on top of the preceding character.
 * Based on code from Markus Kuhn.
 */
#[no_mangle]
pub unsafe extern "C" fn utf_iscomposing(mut c: i32) -> bool {
    return intable(
        combining.as_ptr(),
        (::std::mem::size_of::<[interval; 280]>() as u64)
            .wrapping_div(::std::mem::size_of::<interval>() as u64)
            .wrapping_div(((::std::mem::size_of::<[interval; 280]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as i32 as size_t),
        c,
    );
}
/*
 * Return true for characters that can be displayed in a normal way.
 * Only for characters of 0x100 and above!
 */
#[no_mangle]
pub unsafe extern "C" fn utf_printable(mut c: i32) -> bool {
    /* Sorted list of non-overlapping intervals.
     * 0xd800-0xdfff is reserved for UTF-16, actually illegal. */
    static mut nonprint: [interval; 9] = [
        {
            let mut init = interval { first: 0x70f as i32 as i64, last: 0x70f as i32 as i64 };
            init
        },
        {
            let mut init = interval {
                first: 0x180b as i32 as i64,
                last: 0x180e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x200b as i32 as i64,
                last: 0x200f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x202a as i32 as i64,
                last: 0x202e as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x206a as i32 as i64,
                last: 0x206f as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xd800 as i32 as i64,
                last: 0xdfff as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfeff as i32 as i64,
                last: 0xfeff as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfff9 as i32 as i64,
                last: 0xfffb as i32 as i64,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfffe as i32 as i64,
                last: 0xffff as i32 as i64,
            };
            init
        },
    ];
    return !intable(
        nonprint.as_mut_ptr(),
        (::std::mem::size_of::<[interval; 9]>() as u64)
            .wrapping_div(::std::mem::size_of::<interval>() as u64)
            .wrapping_div(((::std::mem::size_of::<[interval; 9]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as i32 as size_t),
        c,
    );
}
/*
 * Get class of a Unicode character.
 * 0: white space
 * 1: punctuation
 * 2 or bigger: some class of word character.
 */
#[no_mangle]
pub unsafe extern "C" fn utf_class(c: i32) -> i32 {
    return utf_class_tab(c, (*curbuf).b_chartab.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn utf_class_tab(c: i32, chartab: *const u64) -> i32 {
    /* sorted list of non-overlapping intervals */
    static mut classes: [clinterval; 71] = [
        {
            let mut init = clinterval {
                first: 0x37e as i32 as u32,
                last: 0x37e as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x387 as i32 as u32,
                last: 0x387 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x55a as i32 as u32,
                last: 0x55f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x589 as i32 as u32,
                last: 0x589 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x5be as i32 as u32,
                last: 0x5be as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x5c0 as i32 as u32,
                last: 0x5c0 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x5c3 as i32 as u32,
                last: 0x5c3 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x5f3 as i32 as u32,
                last: 0x5f4 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x60c as i32 as u32,
                last: 0x60c as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x61b as i32 as u32,
                last: 0x61b as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x61f as i32 as u32,
                last: 0x61f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x66a as i32 as u32,
                last: 0x66d as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x6d4 as i32 as u32,
                last: 0x6d4 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x700 as i32 as u32,
                last: 0x70d as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x964 as i32 as u32,
                last: 0x965 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x970 as i32 as u32,
                last: 0x970 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xdf4 as i32 as u32,
                last: 0xdf4 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xe4f as i32 as u32,
                last: 0xe4f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xe5a as i32 as u32,
                last: 0xe5b as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xf04 as i32 as u32,
                last: 0xf12 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xf3a as i32 as u32,
                last: 0xf3d as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xf85 as i32 as u32,
                last: 0xf85 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x104a as i32 as u32,
                last: 0x104f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x10fb as i32 as u32,
                last: 0x10fb as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1361 as i32 as u32,
                last: 0x1368 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x166d as i32 as u32,
                last: 0x166e as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1680 as i32 as u32,
                last: 0x1680 as i32 as u32,
                class: 0,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x169b as i32 as u32,
                last: 0x169c as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x16eb as i32 as u32,
                last: 0x16ed as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1735 as i32 as u32,
                last: 0x1736 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x17d4 as i32 as u32,
                last: 0x17dc as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1800 as i32 as u32,
                last: 0x180a as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2000 as i32 as u32,
                last: 0x200b as i32 as u32,
                class: 0,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x200c as i32 as u32,
                last: 0x2027 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2028 as i32 as u32,
                last: 0x2029 as i32 as u32,
                class: 0,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x202a as i32 as u32,
                last: 0x202e as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x202f as i32 as u32,
                last: 0x202f as i32 as u32,
                class: 0,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2030 as i32 as u32,
                last: 0x205e as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x205f as i32 as u32,
                last: 0x205f as i32 as u32,
                class: 0,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2060 as i32 as u32,
                last: 0x27ff as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2070 as i32 as u32,
                last: 0x207f as i32 as u32,
                class: 0x2070 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2080 as i32 as u32,
                last: 0x2094 as i32 as u32,
                class: 0x2080 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x20a0 as i32 as u32,
                last: 0x27ff as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2800 as i32 as u32,
                last: 0x28ff as i32 as u32,
                class: 0x2800 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2900 as i32 as u32,
                last: 0x2998 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x29d8 as i32 as u32,
                last: 0x29db as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x29fc as i32 as u32,
                last: 0x29fd as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2e00 as i32 as u32,
                last: 0x2e7f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x3000 as i32 as u32,
                last: 0x3000 as i32 as u32,
                class: 0,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x3001 as i32 as u32,
                last: 0x3020 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x3030 as i32 as u32,
                last: 0x3030 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x303d as i32 as u32,
                last: 0x303d as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x3040 as i32 as u32,
                last: 0x309f as i32 as u32,
                class: 0x3040 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x30a0 as i32 as u32,
                last: 0x30ff as i32 as u32,
                class: 0x30a0 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x3300 as i32 as u32,
                last: 0x9fff as i32 as u32,
                class: 0x4e00 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xac00 as i32 as u32,
                last: 0xd7a3 as i32 as u32,
                class: 0xac00 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xf900 as i32 as u32,
                last: 0xfaff as i32 as u32,
                class: 0x4e00 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xfd3e as i32 as u32,
                last: 0xfd3f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xfe30 as i32 as u32,
                last: 0xfe6b as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xff00 as i32 as u32,
                last: 0xff0f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xff1a as i32 as u32,
                last: 0xff20 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xff3b as i32 as u32,
                last: 0xff40 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0xff5b as i32 as u32,
                last: 0xff65 as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1d000 as i32 as u32,
                last: 0x1d24f as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1d400 as i32 as u32,
                last: 0x1d7ff as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1f000 as i32 as u32,
                last: 0x1f2ff as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x1f300 as i32 as u32,
                last: 0x1f9ff as i32 as u32,
                class: 1,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x20000 as i32 as u32,
                last: 0x2a6df as i32 as u32,
                class: 0x4e00 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2a700 as i32 as u32,
                last: 0x2b73f as i32 as u32,
                class: 0x4e00 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2b740 as i32 as u32,
                last: 0x2b81f as i32 as u32,
                class: 0x4e00 as i32 as u32,
            };
            init
        },
        {
            let mut init = clinterval {
                first: 0x2f800 as i32 as u32,
                last: 0x2fa1f as i32 as u32,
                class: 0x4e00 as i32 as u32,
            };
            init
        },
    ];
    let mut bot = 0;
    let mut top = (::std::mem::size_of::<[clinterval; 71]>() as u64)
        .wrapping_div(::std::mem::size_of::<clinterval>() as u64)
        .wrapping_div(((::std::mem::size_of::<[clinterval; 71]>() as u64).wrapping_rem(::std::mem::size_of::<clinterval>() as u64) == 0) as i32 as size_t)
        .wrapping_sub(1) as i32;
    let mut mid: i32 = 0;
    /* First quick check for Latin1 characters, use 'iskeyword'. */
    if c < 0x100 as i32 {
        if c == ' ' as i32 || c == '\t' as i32 || c == NUL || c == 0xa0 as i32 {
            return 0;
            // blank
        }
        if vim_iswordc_tab(c, chartab) {
            return 2;
            // punctuation
            // word character
        }
        return 1;
    }
    /* binary search in table */
    while top >= bot {
        mid = (bot + top) / 2;
        if classes[mid as usize].last < c as u32 {
            bot = mid + 1
        } else if classes[mid as usize].first > c as u32 {
            top = mid - 1
        } else {
            return classes[mid as usize].class as i32;
        }
    }
    // emoji
    if intable(
        emoji_all.as_ptr(),
        (::std::mem::size_of::<[interval; 151]>() as u64)
            .wrapping_div(::std::mem::size_of::<interval>() as u64)
            .wrapping_div(((::std::mem::size_of::<[interval; 151]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as i32 as size_t),
        c,
    ) {
        return 3;
    }
    /* most other characters are "word" characters */
    return 2;
}
#[no_mangle]
pub unsafe extern "C" fn utf_ambiguous_width(mut c: i32) -> bool {
    return c >= 0x80 as i32
        && (intable(
            ambiguous.as_ptr(),
            (::std::mem::size_of::<[interval; 179]>() as u64)
                .wrapping_div(::std::mem::size_of::<interval>() as u64)
                .wrapping_div(((::std::mem::size_of::<[interval; 179]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as i32 as size_t),
            c,
        ) as i32
            != 0
            || intable(
                emoji_all.as_ptr(),
                (::std::mem::size_of::<[interval; 151]>() as u64)
                    .wrapping_div(::std::mem::size_of::<interval>() as u64)
                    .wrapping_div(((::std::mem::size_of::<[interval; 151]>() as u64).wrapping_rem(::std::mem::size_of::<interval>() as u64) == 0) as i32 as size_t),
                c,
            ) as i32
                != 0);
}
/*
 * Generic conversion function for case operations.
 * Return the converted equivalent of "a", which is a UCS-4 character.  Use
 * the given conversion "table".  Uses binary search on "table".
 */
unsafe extern "C" fn utf_convert(mut a: i32, table: *const convertStruct, mut n_items: size_t) -> i32 {
    let mut start: size_t = 0; /* indices into table */
    let mut mid: size_t = 0;
    let mut end: size_t = 0;
    start = 0;
    end = n_items;
    while start < end {
        /* need to search further */
        mid = end.wrapping_add(start).wrapping_div(2);
        if (*table.offset(mid as isize)).rangeEnd < a {
            start = mid.wrapping_add(1)
        } else {
            end = mid
        }
    }
    if start < n_items && (*table.offset(start as isize)).rangeStart <= a && a <= (*table.offset(start as isize)).rangeEnd && (a - (*table.offset(start as isize)).rangeStart) % (*table.offset(start as isize)).step == 0 {
        return a + (*table.offset(start as isize)).offset;
    } else {
        return a;
    };
}
/*
 * Return the folded-case equivalent of "a", which is a UCS-4 character.  Uses
 * simple case folding.
 */
#[no_mangle]
pub unsafe extern "C" fn utf_fold(mut a: i32) -> i32 {
    if a < 0x80 as i32 {
        // be fast for ASCII
        return if a >= 0x41 as i32 && a <= 0x5a as i32 { (a) + 32 } else { a };
    }
    return utf_convert(
        a,
        foldCase.as_ptr(),
        (::std::mem::size_of::<[convertStruct; 192]>() as u64)
            .wrapping_div(::std::mem::size_of::<convertStruct>() as u64)
            .wrapping_div(((::std::mem::size_of::<[convertStruct; 192]>() as u64).wrapping_rem(::std::mem::size_of::<convertStruct>() as u64) == 0) as i32 as size_t),
    );
}
// Vim's own character class functions.  These exist because many library
// islower()/toupper() etc. do not work properly: they crash when used with
// invalid values or can't handle latin1 when the locale is C.
// Speed is most important here.
// / Return the upper-case equivalent of "a", which is a UCS-4 character.  Use
// / simple case folding.
#[no_mangle]
pub unsafe extern "C" fn mb_toupper(mut a: i32) -> i32 {
    /* If 'casemap' contains "keepascii" use ASCII style toupper(). */
    if a < 128 && cmp_flags & CMP_KEEPASCII as u32 != 0 {
        return if a < 'a' as i32 || a > 'z' as i32 { a } else { (a) - ('a' as i32 - 'A' as i32) };
    }
    /* If towupper() is available and handles Unicode, use it. */
    if cmp_flags & CMP_INTERNAL as u32 == 0 {
        return towupper(a as wint_t) as i32;
    }
    /* For characters below 128 use locale sensitive toupper(). */
    if a < 128 {
        return toupper(a);
    }
    /* For any other characters use the above mapping table. */
    return utf_convert(
        a,
        toUpper.as_ptr(),
        (::std::mem::size_of::<[convertStruct; 187]>() as u64)
            .wrapping_div(::std::mem::size_of::<convertStruct>() as u64)
            .wrapping_div(((::std::mem::size_of::<[convertStruct; 187]>() as u64).wrapping_rem(::std::mem::size_of::<convertStruct>() as u64) == 0) as i32 as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn mb_islower(mut a: i32) -> bool {
    // German sharp s is lower case but has no upper case equivalent.
    return mb_toupper(a) != a || a == 0xdf as i32;
}
// / Return the lower-case equivalent of "a", which is a UCS-4 character.  Use
// / simple case folding.
#[no_mangle]
pub unsafe extern "C" fn mb_tolower(mut a: i32) -> i32 {
    /* If 'casemap' contains "keepascii" use ASCII style tolower(). */
    if a < 128 && cmp_flags & CMP_KEEPASCII as u32 != 0 {
        return if a < 'A' as i32 || a > 'Z' as i32 { a } else { (a) + ('a' as i32 - 'A' as i32) };
    }
    /* If towlower() is available and handles Unicode, use it. */
    if cmp_flags & CMP_INTERNAL as u32 == 0 {
        return towlower(a as wint_t) as i32;
    }
    /* For characters below 128 use locale sensitive tolower(). */
    if a < 128 {
        return tolower(a);
    }
    /* For any other characters use the above mapping table. */
    return utf_convert(
        a,
        toLower.as_ptr(),
        (::std::mem::size_of::<[convertStruct; 172]>() as u64)
            .wrapping_div(::std::mem::size_of::<convertStruct>() as u64)
            .wrapping_div(((::std::mem::size_of::<[convertStruct; 172]>() as u64).wrapping_rem(::std::mem::size_of::<convertStruct>() as u64) == 0) as i32 as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn mb_isupper(mut a: i32) -> bool {
    return mb_tolower(a) != a;
}
unsafe extern "C" fn utf_strnicmp(mut s1: *const u8, mut s2: *const u8, mut n1: size_t, mut n2: size_t) -> i32 {
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    let mut cdiff: i32 = 0;
    let mut buffer: [u8; 6] = [0; 6];
    loop {
        c1 = utf_safe_read_char_adv(&mut s1, &mut n1);
        c2 = utf_safe_read_char_adv(&mut s2, &mut n2);
        if c1 <= 0 || c2 <= 0 {
            break;
        }
        if c1 == c2 {
            continue;
        }
        cdiff = utf_fold(c1) - utf_fold(c2);
        if cdiff != 0 {
            return cdiff;
        }
    }
    /* some string ended or has an incomplete/illegal character sequence */
    if c1 == 0 || c2 == 0 {
        /* some string ended. shorter string is smaller */
        if c1 == 0 && c2 == 0 {
            return 0;
        }
        return if c1 == 0 { -(1) } else { 1 };
    }
    /* Continue with bytewise comparison to produce some result that
     * would make comparison operations involving this function transitive.
     *
     * If only one string had an error, comparison should be made with
     * folded version of the other string. In this case it is enough
     * to fold just one character to determine the result of comparison. */
    if c1 != -(1) && c2 == -(1) {
        n1 = utf_char2bytes(utf_fold(c1), buffer.as_mut_ptr()) as size_t;
        s1 = buffer.as_mut_ptr()
    } else if c2 != -(1) && c1 == -(1) {
        n2 = utf_char2bytes(utf_fold(c2), buffer.as_mut_ptr()) as size_t;
        s2 = buffer.as_mut_ptr()
    }
    while n1 > 0 && n2 > 0 && *s1 as i32 != NUL && *s2 as i32 != NUL {
        cdiff = *s1 as i32 - *s2 as i32;
        if cdiff != 0 {
            return cdiff;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
        n1 = n1.wrapping_sub(1);
        n2 = n2.wrapping_sub(1)
    }
    if n1 > 0 && *s1 as i32 == NUL {
        n1 = 0
    }
    if n2 > 0 && *s2 as i32 == NUL {
        n2 = 0
    }
    if n1 == 0 && n2 == 0 {
        return 0;
    }
    return if n1 == 0 { -(1) } else { 1 };
}
// / Measure the length of a string in corresponding UTF-32 and UTF-16 units.
// /
// / Invalid UTF-8 bytes, or embedded surrogates, count as one code point/unit
// / each.
// /
// / The out parameters are incremented. This is used to measure the size of
// / a buffer region consisting of multiple line segments.
// /
// / @param s the string
// / @param len maximum length (an earlier NUL terminates)
// / @param[out] codepoints incremented with UTF-32 code point size
// / @param[out] codeunits incremented with UTF-16 code unit size
#[no_mangle]
pub unsafe extern "C" fn mb_utflen(mut s: *const u8, mut len: size_t, mut codepoints: *mut size_t, mut codeunits: *mut size_t) {
    let mut count = 0;
    let mut extra = 0;
    let mut clen: size_t = 0;
    let mut i = 0;
    while i < len && *s.offset(i as isize) as i32 != NUL {
        clen = utf_ptr2len_len(s.offset(i as isize), len.wrapping_sub(i) as i32) as size_t;
        // NB: gets the byte value of invalid sequence bytes.
        // we only care whether the char fits in the BMP or not
        let mut c = if clen > 1 { utf_ptr2char(s.offset(i as isize)) } else { *s.offset(i as isize) as i32 };
        count = count.wrapping_add(1);
        if c > 0xffff as i32 {
            extra = extra.wrapping_add(1)
        }
        i = (i as u64).wrapping_add(clen) as size_t as size_t
    }
    *codepoints = (*codepoints as u64).wrapping_add(count) as size_t as size_t;
    *codeunits = (*codeunits as u64).wrapping_add(count.wrapping_add(extra)) as size_t as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mb_utf_index_to_bytes(mut s: *const u8, mut len: size_t, mut index: size_t, mut use_utf16_units: bool) -> ssize_t {
    let mut count = 0;
    let mut clen: size_t = 0;
    let mut i: size_t = 0;
    if index == 0 {
        return 0;
    }
    i = 0;
    while i < len && *s.offset(i as isize) as i32 != NUL {
        clen = utf_ptr2len_len(s.offset(i as isize), len.wrapping_sub(i) as i32) as size_t;
        // NB: gets the byte value of invalid sequence bytes.
        // we only care whether the char fits in the BMP or not
        let mut c = if clen > 1 { utf_ptr2char(s.offset(i as isize)) } else { *s.offset(i as isize) as i32 };
        count = count.wrapping_add(1);
        if use_utf16_units as i32 != 0 && c > 0xffff as i32 {
            count = count.wrapping_add(1)
        }
        if count >= index {
            return i.wrapping_add(clen) as ssize_t;
        }
        i = (i as u64).wrapping_add(clen) as size_t as size_t
    }
    return -(1) as ssize_t;
}
/*
 * Version of strnicmp() that handles multi-byte characters.
 * Needed for Big5, Shift-JIS and UTF-8 encoding.  Other DBCS encodings can
 * probably use strnicmp(), because there are no ASCII characters in the
 * second byte.
 * Returns zero if s1 and s2 are equal (ignoring case), the difference between
 * two characters otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_strnicmp(mut s1: *const u8, mut s2: *const u8, nn: size_t) -> i32 {
    return utf_strnicmp(s1, s2, nn, nn);
}
// / Compare strings case-insensitively
// /
// / @note We need to call mb_stricmp() even when we aren't dealing with
// /       a multi-byte encoding because mb_stricmp() takes care of all ASCII and
// /       non-ascii encodings, including characters with umlauts in latin1,
// /       etc., while STRICMP() only handles the system locale version, which
// /       often does not handle non-ascii properly.
// /
// / @param[in]  s1  First string to compare, not more then #MAXCOL characters.
// / @param[in]  s2  Second string to compare, not more then #MAXCOL characters.
// /
// / @return 0 if strings are equal, <0 if s1 < s2, >0 if s1 > s2.
#[no_mangle]
pub unsafe extern "C" fn mb_stricmp(mut s1: *const i8, mut s2: *const i8) -> i32 {
    return mb_strnicmp(s1 as *const u8, s2 as *const u8, MAXCOL as i32 as size_t);
}
/*
 * "g8": show bytes of the UTF-8 char under the cursor.  Doesn't matter what
 * 'encoding' has been set to.
 */
#[no_mangle]
pub unsafe extern "C" fn show_utf8() {
    let mut len: i32 = 0;
    let mut rlen = 0;
    let mut line = 0 as *mut u8;
    let mut clen: i32 = 0;
    let mut i: i32 = 0;
    /* Get the byte length of the char under the cursor, including composing
     * characters. */
    line = get_cursor_pos_ptr();
    len = utfc_ptr2len(line);
    if len == 0 {
        msg(b"NUL\x00" as *const u8 as *const i8 as *mut u8);
        return;
    }
    clen = 0;
    i = 0;
    while i < len {
        if clen == 0 {
            /* start of (composing) character, get its length */
            if i > 0 {
                strcpy(IObuff.as_mut_ptr().offset(rlen as isize) as *mut i8, b"+ \x00" as *const u8 as *const i8 as *mut i8); /* NUL is stored as NL */
                rlen += 2
            }
            clen = utf_ptr2len(line.offset(i as isize))
        }
        sprintf(
            (IObuff.as_mut_ptr() as *mut i8).offset(rlen as isize),
            b"%02x \x00" as *const u8 as *const i8,
            if *line.offset(i as isize) as i32 == NL { NUL } else { *line.offset(i as isize) as i32 },
        );
        clen -= 1;
        rlen += strlen(IObuff.as_mut_ptr().offset(rlen as isize) as *mut i8) as i32;
        if rlen > IOSIZE - 20 {
            break;
        }
        i += 1
    }
    msg(IObuff.as_mut_ptr());
}
// / Return offset from "p" to the first byte of the character it points into.
// / If "p" points to the NUL at the end of the string return 0.
// / Returns 0 when already at the first byte of a character.
#[no_mangle]
pub unsafe extern "C" fn utf_head_off(mut base: *const u8, mut p: *const u8) -> i32 {
    let mut c: i32 = 0;
    let mut len: i32 = 0;
    if (*p as i32) < 0x80 as i32 {
        /* be quick for ASCII */
        return 0;
    }
    /* Skip backwards over trailing bytes: 10xx.xxxx
     * Skip backwards again if on a composing char. */
    let mut q = 0 as *const u8;
    q = p;
    loop
    /* Move s to the last byte of this char. */
    {
        let mut s = 0 as *const u8;
        s = q;
        while *s.offset(1) as i32 & 0xc0 as i32 == 0x80 as i32 {
            s = s.offset(1)
        }
        /* Move q to the first byte of this char. */
        while q > base && *q as i32 & 0xc0 as i32 == 0x80 as i32 {
            q = q.offset(-1)
        }
        /* Check for illegal sequence. Do allow an illegal byte after where we
         * started. */
        len = utf8len_tab[*q as usize] as i32;
        if len != (s.offset_from(q) as i64 + 1) as i32 && len != (p.offset_from(q) as i64 + 1) as i32 {
            return 0;
        }
        if q <= base {
            break;
        }
        c = utf_ptr2char(q);
        if !utf_iscomposing(c) {
            if !arabic_maycombine(c) {
                break;
            }
            /* Advance to get a sneak-peak at the next char */
            let mut j = q;
            j = j.offset(-1);
            /* Move j to the first byte of this char. */
            while j > base && *j as i32 & 0xc0 as i32 == 0x80 as i32 {
                j = j.offset(-1)
            }
            if !arabic_combine(utf_ptr2char(j), c) {
                break;
            }
        }
        q = q.offset(-1)
    }
    return p.offset_from(q) as i64 as i32;
}
// / Copy a character, advancing the pointers
// /
// / @param[in,out]  fp  Source of the character to copy.
// / @param[in,out]  tp  Destination to copy to.
#[no_mangle]
pub unsafe extern "C" fn mb_copy_char(fp: *mut *const u8, tp: *mut *mut u8) {
    let l = utfc_ptr2len(*fp) as size_t;
    memmove(*tp as *mut libc::c_void, *fp as *const libc::c_void, l);
    *tp = (*tp).offset(l as isize);
    *fp = (*fp).offset(l as isize);
}
/*
 * Return the offset from "p" to the first byte of a character.  When "p" is
 * at the start of a character 0 is returned, otherwise the offset to the next
 * character.  Can start anywhere in a stream of bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_off_next(mut base: *mut u8, mut p: *mut u8) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if (*p as i32) < 0x80 as i32 {
        // be quick for ASCII
        return 0;
    }
    // Find the next character that isn't 10xx.xxxx
    i = 0;
    while *p.offset(i as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
        i += 1
    }
    if i > 0 {
        // Check for illegal sequence.
        j = 0;
        while p.offset(-(j as isize)) > base {
            if *p.offset(-j as isize) as i32 & 0xc0 as i32 != 0x80 as i32 {
                break;
            }
            j += 1
        }
        if utf8len_tab[*p.offset(-j as isize) as usize] as i32 != i + j {
            return 0;
        }
    }
    return i;
}
/*
 * Return the offset from "p" to the last byte of the character it points
 * into.  Can start anywhere in a stream of bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_tail_off(mut base: *mut u8, mut p: *mut u8) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if *p as i32 == NUL {
        return 0;
    }
    // Find the last character that is 10xx.xxxx
    i = 0;
    while *p.offset((i + 1) as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
        i += 1
    }
    // Check for illegal sequence.
    j = 0;
    while p.offset(-(j as isize)) > base {
        if *p.offset(-j as isize) as i32 & 0xc0 as i32 != 0x80 as i32 {
            break;
        }
        j += 1
    }
    if utf8len_tab[*p.offset(-j as isize) as usize] as i32 != i + j + 1 {
        return 0;
    }
    return i;
}
/*
 * Find the next illegal byte sequence.
 */
#[no_mangle]
pub unsafe extern "C" fn utf_find_illegal() {
    let mut current_block: u64;
    let mut pos = (*curwin).w_cursor;
    let mut p = 0 as *mut u8;
    let mut len: i32 = 0;
    let mut vimconv = vimconv_T {
        vc_type: 0,
        vc_factor: 0,
        vc_fd: 0 as *mut libc::c_void,
        vc_fail: false,
    };
    let mut tofree = NULL_0 as *mut u8;
    vimconv.vc_type = CONV_NONE as i32;
    if enc_canon_props((*curbuf).b_p_fenc) & ENC_8BIT != 0 {
        // 'encoding' is "utf-8" but we are editing a 8-bit encoded file,
        // possibly a utf-8 file with illegal bytes.  Setup for conversion
        // from utf-8 to 'fileencoding'.
        convert_setup(&mut vimconv, p_enc, (*curbuf).b_p_fenc);
    }
    (*curwin).w_cursor.coladd = 0;
    's_44: loop {
        p = get_cursor_pos_ptr();
        if vimconv.vc_type != CONV_NONE as i32 {
            xfree(tofree as *mut libc::c_void);
            tofree = string_convert(&mut vimconv, p, NULL_0 as *mut size_t);
            if tofree.is_null() {
                current_block = 3275366147856559585;
                break;
            }
            p = tofree
        }
        while *p as i32 != NUL {
            /* Illegal means that there are not enough trail bytes (checked by
             * utf_ptr2len()) or too many of them (overlong sequence). */
            len = utf_ptr2len(p);
            if *p as i32 >= 0x80 as i32 && (len == 1 || utf_char2len(utf_ptr2char(p)) != len) {
                if vimconv.vc_type == CONV_NONE as i32 {
                    (*curwin).w_cursor.col += p.offset_from(get_cursor_pos_ptr()) as i64 as colnr_T
                } else {
                    let mut l: i32 = 0;
                    len = p.offset_from(tofree) as i64 as i32;
                    p = get_cursor_pos_ptr();
                    while *p as i32 != NUL && {
                        let fresh6 = len;
                        len = len - 1;
                        (fresh6) > 0
                    } {
                        l = utf_ptr2len(p);
                        (*curwin).w_cursor.col += l;
                        p = p.offset(l as isize)
                    }
                }
                current_block = 11834598965984575227;
                break 's_44;
            } else {
                p = p.offset(len as isize)
            }
        }
        if (*curwin).w_cursor.lnum == (*curbuf).b_ml.ml_line_count {
            current_block = 3275366147856559585;
            break;
        }
        (*curwin).w_cursor.lnum += 1;
        (*curwin).w_cursor.col = 0
    }
    match current_block {
        3275366147856559585 => {
            /* didn't find it: don't move and beep */
            (*curwin).w_cursor = pos;
            beep_flush();
        }
        _ => {}
    }
    xfree(tofree as *mut libc::c_void);
    convert_setup(&mut vimconv, NULL_0 as *mut u8, NULL_0 as *mut u8);
}
/*
 * If the cursor moves on an trail byte, set the cursor on the lead byte.
 * Thus it moves left if necessary.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_adjust_cursor() {
    mark_mb_adjustpos(curbuf, &mut (*curwin).w_cursor);
}
// / Checks and adjusts cursor column. Not mode-dependent.
// / @see check_cursor_col_win
// /
// / @param  win_  Places cursor on a valid column for this window.
#[no_mangle]
pub unsafe extern "C" fn mb_check_adjust_col(mut win_: *mut libc::c_void) {
    let mut win = win_ as *mut win_T;
    let mut oldcol = (*win).w_cursor.col;
    // Column 0 is always valid.
    if oldcol != 0 {
        let mut p = ml_get_buf((*win).w_buffer, (*win).w_cursor.lnum, false);
        let mut len = strlen(p as *mut i8) as colnr_T;
        // Empty line or invalid column?
        if len == 0 || oldcol < 0 {
            (*win).w_cursor.col = 0
        } else {
            // Cursor column too big for line?
            if oldcol > len {
                (*win).w_cursor.col = len - 1
            }
            // Move the cursor to the head byte.
            (*win).w_cursor.col -= utf_head_off(p, p.offset((*win).w_cursor.col as isize))
        }
        // Reset `coladd` when the cursor would be on the right half of a
        // double-wide character.
        if (*win).w_cursor.coladd == 1 && *p.offset((*win).w_cursor.col as isize) as i32 != TAB && vim_isprintc(utf_ptr2char(p.offset((*win).w_cursor.col as isize))) as i32 != 0 && ptr2cells(p.offset((*win).w_cursor.col as isize)) > 1 {
            (*win).w_cursor.coladd = 0
        }
    };
}
/*
 * Return a pointer to the character before "*p", if there is one.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_prevptr(mut line: *mut u8, mut p: *mut u8) -> *mut u8 {
    if p > line {
        p = p.offset(-((utf_head_off(line, p.offset(-(1))) + 1) as isize))
    }
    return p;
}
/*
 * Return the character length of "str".  Each multi-byte character (with
 * following composing characters) counts as one.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_charlen(mut str: *mut u8) -> i32 {
    let mut p = str;
    let mut count: i32 = 0;
    if p.is_null() {
        return 0;
    }
    count = 0;
    while *p as i32 != NUL {
        p = p.offset(Some(Some(mb_ptr2len).expect("non-null function pointer")).expect("non-null function pointer")(p) as isize);
        count += 1
    }
    return count;
}
/*
 * Like mb_charlen() but for a string with specified length.
 */
#[no_mangle]
pub unsafe extern "C" fn mb_charlen_len(mut str: *mut u8, mut len: i32) -> i32 {
    let mut p = str;
    let mut count: i32 = 0;
    count = 0;
    while *p as i32 != NUL && p < str.offset(len as isize) {
        p = p.offset(Some(Some(mb_ptr2len).expect("non-null function pointer")).expect("non-null function pointer")(p) as isize);
        count += 1
    }
    return count;
}
// / Try to unescape a multibyte character
// /
// / Used for the rhs and lhs of the mappings.
// /
// / @param[in,out]  pp  String to unescape. Is advanced to just after the bytes
// /                     that form a multibyte character.
// /
// / @return Unescaped string if it is a multibyte character, NULL if no
// /         multibyte character was found. Returns a static buffer, always one
// /         and the same.
#[no_mangle]
pub unsafe extern "C" fn mb_unescape(pp: *mut *const i8) -> *const i8 {
    static mut buf: [i8; 6] = [0; 6];
    let mut buf_idx = 0;
    let mut str = *pp as *mut u8;
    // Must translate K_SPECIAL KS_SPECIAL KE_FILLER to K_SPECIAL and CSI
    // KS_EXTRA KE_CSI to CSI.
    // Maximum length of a utf-8 character is 4 bytes.
    let mut str_idx = 0;
    while *str.offset(str_idx as isize) as i32 != NUL && buf_idx < 4 {
        if *str.offset(str_idx as isize) as i32 == K_SPECIAL && *str.offset(str_idx.wrapping_add(1) as isize) as i32 == KS_SPECIAL && *str.offset(str_idx.wrapping_add(2) as isize) as i32 == KE_FILLER {
            let fresh7 = buf_idx;
            buf_idx = buf_idx.wrapping_add(1);
            buf[fresh7 as usize] = K_SPECIAL as i8;
            str_idx = (str_idx as u64).wrapping_add(2) as size_t as size_t
        } else if *str.offset(str_idx as isize) as i32 == K_SPECIAL && *str.offset(str_idx.wrapping_add(1) as isize) as i32 == KS_EXTRA && *str.offset(str_idx.wrapping_add(2) as isize) as i32 == KE_CSI as i32 {
            let fresh8 = buf_idx;
            buf_idx = buf_idx.wrapping_add(1);
            buf[fresh8 as usize] = CSI as i8;
            str_idx = (str_idx as u64).wrapping_add(2) as size_t as size_t
        } else {
            if *str.offset(str_idx as isize) as i32 == K_SPECIAL {
                break;
            }
            let fresh9 = buf_idx;
            buf_idx = buf_idx.wrapping_add(1);
            buf[fresh9 as usize] = *str.offset(str_idx as isize) as i8
        }
        buf[buf_idx as usize] = NUL as i8;
        // Return a multi-byte character if it's found.  An illegal sequence
        // will result in a 1 here.
        if utf_ptr2len(buf.as_mut_ptr() as *const u8) > 1 {
            *pp = (str as *const i8).offset(str_idx as isize).offset(1);
            return buf.as_mut_ptr();
        }
        // Bail out quickly for ASCII.
        if (buf[0 as i32 as usize] as u8 as i32) < 128 {
            break;
        }
        str_idx = str_idx.wrapping_add(1)
    }
    return NULL_0 as *const i8;
}
/*
 * Skip the Vim specific head of a 'encoding' name.
 */
#[no_mangle]
pub unsafe extern "C" fn enc_skip(mut p: *mut u8) -> *mut u8 {
    if strncmp(p as *mut i8, b"2byte-\x00" as *const u8 as *const i8 as *mut i8, 6) == 0 {
        return p.offset(6);
    }
    if strncmp(p as *mut i8, b"8bit-\x00" as *const u8 as *const i8 as *mut i8, 5) == 0 {
        return p.offset(5);
    }
    return p;
}
/*
 * Find the canonical name for encoding "enc".
 * When the name isn't recognized, returns "enc" itself, but with all lower
 * case characters and '_' replaced with '-'.
 * Returns an allocated string.
 */
#[no_mangle]
pub unsafe extern "C" fn enc_canonize(mut enc: *mut u8) -> *mut u8 {
    let mut p = 0 as *mut u8;
    let mut s = 0 as *mut u8;
    let mut i: i32 = 0;
    if strcmp(enc as *mut i8, b"default\x00" as *const u8 as *const i8 as *mut i8) == 0 {
        // Use the default encoding as found by set_init_1().
        return vim_strsave(fenc_default);
    }
    /* copy "enc" to allocated memory, with room for two '-' */
    let mut r = xmalloc(strlen(enc as *mut i8).wrapping_add(3)) as *mut u8;
    /* Make it all lower case and replace '_' with '-'. */
    p = r;
    s = enc;
    while *s as i32 != NUL {
        if *s as i32 == '_' as i32 {
            let fresh10 = p;
            p = p.offset(1);
            *fresh10 = '-' as i32 as u8
        } else {
            let fresh11 = p;
            p = p.offset(1);
            *fresh11 = if (*s as i32) < 'A' as i32 || *s as i32 > 'Z' as i32 { *s as i32 } else { (*s as i32) + ('a' as i32 - 'A' as i32) } as u8
        }
        s = s.offset(1)
    }
    *p = NUL as u8;
    /* Skip "2byte-" and "8bit-". */
    p = enc_skip(r);
    /* Change "microsoft-cp" to "cp".  Used in some spell files. */
    if strncmp(p as *mut i8, b"microsoft-cp\x00" as *const u8 as *const i8 as *mut i8, 12) == 0 {
        memmove(p as *mut libc::c_void, p.offset(10) as *const libc::c_void, strlen(p.offset(10) as *mut i8).wrapping_add(1));
    }
    /* "iso8859" -> "iso-8859" */
    if strncmp(p as *mut i8, b"iso8859\x00" as *const u8 as *const i8 as *mut i8, 7) == 0 {
        memmove(p.offset(4) as *mut libc::c_void, p.offset(3) as *const libc::c_void, strlen(p.offset(3) as *mut i8).wrapping_add(1));
        *p.offset(3) = '-' as i32 as u8
    }
    /* "iso-8859n" -> "iso-8859-n" */
    if strncmp(p as *mut i8, b"iso-8859\x00" as *const u8 as *const i8 as *mut i8, 8) == 0 && *p.offset(8) as i32 != '-' as i32 {
        memmove(p.offset(9) as *mut libc::c_void, p.offset(8) as *const libc::c_void, strlen(p.offset(8) as *mut i8).wrapping_add(1));
        *p.offset(8) = '-' as i32 as u8
    }
    /* "latin-N" -> "latinN" */
    if strncmp(p as *mut i8, b"latin-\x00" as *const u8 as *const i8 as *mut i8, 6) == 0 {
        memmove(p.offset(5) as *mut libc::c_void, p.offset(6) as *const libc::c_void, strlen(p.offset(6) as *mut i8).wrapping_add(1));
    }
    if enc_canon_search(p) >= 0 {
        /* canonical name can be used unmodified */
        if p != r {
            memmove(r as *mut libc::c_void, p as *const libc::c_void, strlen(p as *mut i8).wrapping_add(1));
        }
    } else {
        i = enc_alias_search(p);
        if i >= 0 {
            /* alias recognized, get canonical name */
            xfree(r as *mut libc::c_void);
            r = vim_strsave(enc_canon_table[i as usize].name as *mut u8)
        }
    }
    return r;
}
/*
 * Search for an encoding alias of "name".
 * Returns -1 when not found.
 */
unsafe extern "C" fn enc_alias_search(mut name: *mut u8) -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while !enc_alias_table[i as usize].name.is_null() {
        if strcmp(name as *mut i8, enc_alias_table[i as usize].name as *mut i8) == 0 {
            return enc_alias_table[i as usize].canon;
        }
        i += 1
    }
    return -(1);
}
/*
 * Get the canonicalized encoding of the current locale.
 * Returns an allocated string when successful, NULL when not.
 */
#[no_mangle]
pub unsafe extern "C" fn enc_locale() -> *mut u8 {
    let mut i: i32 = 0;
    let mut buf: [i8; 50] = [0; 50];
    let mut s = 0 as *const i8;
    s = nl_langinfo(CODESET_0);
    if s.is_null() || *s as i32 == NUL {
        s = setlocale(LC_CTYPE, NULL_0 as *const i8);
        if s.is_null() || *s as i32 == NUL {
            s = os_getenv(b"LC_ALL\x00" as *const u8 as *const i8);
            if !s.is_null() {
                s = os_getenv(b"LC_CTYPE\x00" as *const u8 as *const i8);
                if !s.is_null() {
                    s = os_getenv(b"LANG\x00" as *const u8 as *const i8)
                }
            }
        }
    }
    if s.is_null() {
        return NULL_0 as *mut u8;
    }
    // The most generic locale format is:
    // language[_territory][.codeset][@modifier][+special][,[sponsor][_revision]]
    // If there is a '.' remove the part before it.
    // if there is something after the codeset, remove it.
    // Make the name lowercase and replace '_' with '-'.
    // Exception: "ja_JP.EUC" == "euc-jp", "zh_CN.EUC" = "euc-cn",
    // "ko_KR.EUC" == "euc-kr"
    let mut p: *const i8 = vim_strchr(s as *mut u8, '.' as i32) as *mut i8;
    let mut current_block_24: u64;
    if !p.is_null() {
        if p > s.offset(2)
            && strncasecmp(p.offset(1) as *mut i8, b"EUC\x00" as *const u8 as *const i8 as *mut i8, 3) == 0
            && *(*__ctype_b_loc()).offset(*p.offset(4) as i32 as isize) as i32 & _ISalnum as i32 as u16 as i32 == 0
            && *p.offset(4) as i32 != '-' as i32
            && *p.offset(-(3) as isize) as i32 == '_' as i32
        {
            // Copy "XY.EUC" to "euc-XY" to buf[10].
            memmove(buf.as_mut_ptr() as *mut libc::c_void, b"euc-\x00" as *const u8 as *const i8 as *const libc::c_void, 4);
            buf[4 as i32 as usize] = if *p.offset(-(2) as isize) as u32 >= 'A' as i32 as u32 && *p.offset(-(2) as isize) as u32 <= 'Z' as i32 as u32
                || *p.offset(-(2) as isize) as u32 >= 'a' as i32 as u32 && *p.offset(-(2) as isize) as u32 <= 'z' as i32 as u32
                || ascii_isdigit(*p.offset(-(2) as isize) as i32) as i32 != 0
            {
                if (*p.offset(-(2) as isize) as i32) < 'A' as i32 || *p.offset(-(2) as isize) as i32 > 'Z' as i32 {
                    *p.offset(-(2) as isize) as i32
                } else {
                    (*p.offset(-(2) as isize) as i32) + ('a' as i32 - 'A' as i32)
                }
            } else {
                0
            } as i8;
            buf[5 as i32 as usize] = if *p.offset(-(1) as isize) as u32 >= 'A' as i32 as u32 && *p.offset(-(1) as isize) as u32 <= 'Z' as i32 as u32
                || *p.offset(-(1) as isize) as u32 >= 'a' as i32 as u32 && *p.offset(-(1) as isize) as u32 <= 'z' as i32 as u32
                || ascii_isdigit(*p.offset(-(1) as isize) as i32) as i32 != 0
            {
                if (*p.offset(-(1) as isize) as i32) < 'A' as i32 || *p.offset(-(1) as isize) as i32 > 'Z' as i32 {
                    *p.offset(-(1) as isize) as i32
                } else {
                    (*p.offset(-(1) as isize) as i32) + ('a' as i32 - 'A' as i32)
                }
            } else {
                0
            } as i8;
            buf[6 as i32 as usize] = NUL as i8;
            current_block_24 = 1538046216550696469;
        } else {
            s = p.offset(1);
            current_block_24 = 16296608149235199289;
        }
    } else {
        current_block_24 = 16296608149235199289;
    }
    match current_block_24 {
        16296608149235199289 => {
            i = 0;
            while i < ::std::mem::size_of::<[i8; 50]>() as u64 as i32 - 1 && *s.offset(i as isize) as i32 != NUL {
                if *s.offset(i as isize) as i32 == '_' as i32 || *s.offset(i as isize) as i32 == '-' as i32 {
                    buf[i as usize] = '-' as i32 as i8
                } else {
                    if !(*s.offset(i as isize) as u8 as u32 >= 'A' as i32 as u32 && *s.offset(i as isize) as u8 as u32 <= 'Z' as i32 as u32
                        || *s.offset(i as isize) as u8 as u32 >= 'a' as i32 as u32 && *s.offset(i as isize) as u8 as u32 <= 'z' as i32 as u32
                        || ascii_isdigit(*s.offset(i as isize) as u8 as i32) as i32 != 0)
                    {
                        break;
                    }
                    buf[i as usize] = if (*s.offset(i as isize) as i32) < 'A' as i32 || *s.offset(i as isize) as i32 > 'Z' as i32 {
                        *s.offset(i as isize) as i32
                    } else {
                        (*s.offset(i as isize) as i32) + ('a' as i32 - 'A' as i32)
                    } as i8
                }
                i += 1
            }
            buf[i as usize] = NUL as i8
        }
        _ => {}
    }
    return enc_canonize(buf.as_mut_ptr() as *mut u8);
}
/*
 * Call iconv_open() with a check if iconv() works properly (there are broken
 * versions).
 * Returns (void *)-1 if failed.
 * (should return iconv_t, but that causes problems with prototypes).
 */
#[no_mangle]
pub unsafe extern "C" fn my_iconv_open(mut to: *mut u8, mut from: *mut u8) -> *mut libc::c_void {
    let mut fd = 0 as *mut libc::c_void; /* detected a broken iconv() previously */
    let mut tobuf: [u8; 400] = [0; 400];
    let mut p = 0 as *mut i8;
    let mut tolen: size_t = 0;
    static mut iconv_working: WorkingStatus = kUnknown;
    if iconv_working as u32 == kBroken as i32 as u32 {
        return -(1) as *mut libc::c_void;
    }
    fd = iconv_open(enc_skip(to) as *mut i8, enc_skip(from) as *mut i8);
    if fd != -(1) as iconv_t && iconv_working as u32 == kUnknown as i32 as u32 {
        /*
         * Do a dummy iconv() call to check if it actually works.  There is a
         * version of iconv() on Linux that is broken.  We can't ignore it,
         * because it's wide-spread.  The symptoms are that after outputting
         * the initial shift state the "to" pointer is NULL and conversion
         * stops for no apparent reason after about 8160 characters.
         */
        p = tobuf.as_mut_ptr() as *mut i8;
        tolen = ICONV_TESTLEN as size_t;
        iconv(fd, NULL_0 as *mut *mut i8, NULL_0 as *mut size_t, &mut p, &mut tolen);
        if p.is_null() {
            iconv_working = kBroken;
            iconv_close(fd);
            fd = -(1) as iconv_t
        } else {
            iconv_working = kWorking
        }
    }
    return fd;
}
pub const ICONV_TESTLEN: i32 = 400;
/*
 * Convert the string "str[slen]" with iconv().
 * If "unconvlenp" is not NULL handle the string ending in an incomplete
 * sequence and set "*unconvlenp" to the length of it.
 * Returns the converted string in allocated memory.  NULL for an error.
 * If resultlenp is not NULL, sets it to the result length in bytes.
 */
unsafe extern "C" fn iconv_string(vcp: *const vimconv_T, mut str: *mut u8, mut slen: size_t, mut unconvlenp: *mut size_t, mut resultlenp: *mut size_t) -> *mut u8 {
    let mut from = 0 as *const i8;
    let mut fromlen: size_t = 0;
    let mut to = 0 as *mut i8;
    let mut tolen: size_t = 0;
    let mut len = 0;
    let mut done = 0;
    let mut result = NULL_0 as *mut u8;
    let mut p = 0 as *mut u8;
    let mut l: i32 = 0;
    from = str as *mut i8;
    fromlen = slen;
    loop {
        if len == 0 || ICONV_ERRNO == ICONV_E2BIG {
            /* Allocate enough room for most conversions.  When re-allocating
             * increase the buffer size. */
            len = len.wrapping_add(fromlen.wrapping_mul(2)).wrapping_add(40);
            p = xmalloc(len) as *mut u8;
            if done > 0 {
                memmove(p as *mut libc::c_void, result as *const libc::c_void, done);
            }
            xfree(result as *mut libc::c_void);
            result = p
        }
        to = (result as *mut i8).offset(done as isize);
        tolen = len.wrapping_sub(done).wrapping_sub(2);
        // Avoid a warning for systems with a wrong iconv() prototype by
        // casting the second argument to void *.
        if iconv((*vcp).vc_fd, &mut from as *mut *const i8 as *mut libc::c_void as *mut *mut i8, &mut fromlen, &mut to, &mut tolen) != SIZE_MAX {
            // Finished, append a NUL.
            *to = NUL as i8;
            break;
        } else if !(*vcp).vc_fail && !unconvlenp.is_null() && (ICONV_ERRNO == ICONV_EINVAL || ICONV_ERRNO == EINVAL) {
            // Check both ICONV_EINVAL and EINVAL, because the dynamically loaded
            // iconv library may use one of them.
            // Handle an incomplete sequence at the end.
            *to = NUL as i8;
            *unconvlenp = fromlen;
            break;
        } else {
            if !(*vcp).vc_fail && (ICONV_ERRNO == ICONV_EILSEQ || ICONV_ERRNO == EILSEQ || ICONV_ERRNO == ICONV_EINVAL || ICONV_ERRNO == EINVAL) {
                // Check both ICONV_EILSEQ and EILSEQ, because the dynamically loaded
                // iconv library may use one of them.
                // Can't convert: insert a '?' and skip a character.  This assumes
                // conversion from 'encoding' to something else.  In other
                // situations we don't know what to skip anyway.
                let fresh12 = to;
                to = to.offset(1);
                *fresh12 = '?' as i32 as i8;
                if utf_ptr2cells(from as *mut u8) > 1 {
                    let fresh13 = to;
                    to = to.offset(1);
                    *fresh13 = '?' as i32 as i8
                }
                l = utfc_ptr2len_len(from as *const u8, fromlen as i32);
                from = from.offset(l as isize);
                fromlen = (fromlen as u64).wrapping_sub(l as u64) as size_t as size_t
            } else if ICONV_ERRNO != ICONV_E2BIG {
                // conversion failed
                let mut ptr_ = &mut result as *mut *mut u8 as *mut *mut libc::c_void;
                xfree(*ptr_);
                *ptr_ = NULL_0 as *mut libc::c_void;
                break;
            }
            // Not enough room or skipping illegal sequence.
            done = to.offset_from(result as *mut i8) as i64 as size_t
        }
    }
    if !resultlenp.is_null() && !result.is_null() {
        *resultlenp = to.offset_from(result as *mut i8) as i64 as size_t
    }
    return result;
}
// HAVE_ICONV
/*
 * Setup "vcp" for conversion from "from" to "to".
 * The names must have been made canonical with enc_canonize().
 * vcp->vc_type must have been initialized to CONV_NONE.
 * Note: cannot be used for conversion from/to ucs-2 and ucs-4 (will use utf-8
 * instead).
 * Afterwards invoke with "from" and "to" equal to NULL to cleanup.
 * Return FAIL when conversion is not supported, OK otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn convert_setup(mut vcp: *mut vimconv_T, mut from: *mut u8, mut to: *mut u8) -> i32 {
    return convert_setup_ext(vcp, from, true, to, true);
}
/*
 * As convert_setup(), but only when from_unicode_is_utf8 is TRUE will all
 * "from" unicode charsets be considered utf-8.  Same for "to".
 */
#[no_mangle]
pub unsafe extern "C" fn convert_setup_ext(mut vcp: *mut vimconv_T, mut from: *mut u8, mut from_unicode_is_utf8: bool, mut to: *mut u8, mut to_unicode_is_utf8: bool) -> i32 {
    let mut from_prop: i32 = 0;
    let mut to_prop: i32 = 0;
    let mut from_is_utf8: i32 = 0;
    let mut to_is_utf8: i32 = 0;
    // Reset to no conversion.
    if (*vcp).vc_type == CONV_ICONV as i32 && (*vcp).vc_fd != -(1) as iconv_t {
        iconv_close((*vcp).vc_fd);
    }
    *vcp = {
        let mut init = vimconv_T {
            vc_type: CONV_NONE as i32,
            vc_factor: 1,
            vc_fd: 0 as *mut libc::c_void,
            vc_fail: false,
        };
        init
    };
    /* No conversion when one of the names is empty or they are equal. */
    if from.is_null() || *from as i32 == NUL || to.is_null() || *to as i32 == NUL || strcmp(from as *mut i8, to as *mut i8) == 0 {
        return OK;
    }
    from_prop = enc_canon_props(from);
    to_prop = enc_canon_props(to);
    if from_unicode_is_utf8 {
        from_is_utf8 = from_prop & ENC_UNICODE
    } else {
        from_is_utf8 = (from_prop == ENC_UNICODE) as i32
    }
    if to_unicode_is_utf8 {
        to_is_utf8 = to_prop & ENC_UNICODE
    } else {
        to_is_utf8 = (to_prop == ENC_UNICODE) as i32
    }
    if from_prop & ENC_LATIN1 != 0 && to_is_utf8 != 0 {
        /* Internal latin1 -> utf-8 conversion. */
        (*vcp).vc_type = CONV_TO_UTF8 as i32;
        (*vcp).vc_factor = 2
    /* up to twice as long */
    } else if from_prop & ENC_LATIN9 != 0 && to_is_utf8 != 0 {
        /* Internal latin9 -> utf-8 conversion. */
        (*vcp).vc_type = CONV_9_TO_UTF8 as i32;
        (*vcp).vc_factor = 3
    /* up to three as long (euro sign) */
    } else if from_is_utf8 != 0 && to_prop & ENC_LATIN1 != 0 {
        /* Internal utf-8 -> latin1 conversion. */
        (*vcp).vc_type = CONV_TO_LATIN1 as i32
    } else if from_is_utf8 != 0 && to_prop & ENC_LATIN9 != 0 {
        /* Internal utf-8 -> latin9 conversion. */
        (*vcp).vc_type = CONV_TO_LATIN9 as i32
    } else {
        // NOLINT(readability/braces)
        // Use iconv() for conversion.
        (*vcp).vc_fd = my_iconv_open(if to_is_utf8 != 0 { b"utf-8\x00" as *const u8 as *const i8 as *mut u8 } else { to }, if from_is_utf8 != 0 { b"utf-8\x00" as *const u8 as *const i8 as *mut u8 } else { from });
        if (*vcp).vc_fd != -(1) as iconv_t {
            (*vcp).vc_type = CONV_ICONV as i32;
            (*vcp).vc_factor = 4
            /* could be longer too... */
        }
    }
    if (*vcp).vc_type == CONV_NONE as i32 {
        return FAIL;
    }
    return OK;
}
/*
 * Convert text "ptr[*lenp]" according to "vcp".
 * Returns the result in allocated memory and sets "*lenp".
 * When "lenp" is NULL, use NUL terminated strings.
 * Illegal chars are often changed to "?", unless vcp->vc_fail is set.
 * When something goes wrong, NULL is returned and "*lenp" is unchanged.
 */
#[no_mangle]
pub unsafe extern "C" fn string_convert(vcp: *const vimconv_T, mut ptr: *mut u8, mut lenp: *mut size_t) -> *mut u8 {
    return string_convert_ext(vcp, ptr, lenp, NULL_0 as *mut size_t);
}
/*
 * Like string_convert(), but when "unconvlenp" is not NULL and there are is
 * an incomplete sequence at the end it is not converted and "*unconvlenp" is
 * set to the number of remaining bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn string_convert_ext(vcp: *const vimconv_T, mut ptr: *mut u8, mut lenp: *mut size_t, mut unconvlenp: *mut size_t) -> *mut u8 {
    let mut retval = NULL_0 as *mut u8;
    let mut d = 0 as *mut u8;
    let mut l: i32 = 0;
    let mut c: i32 = 0;
    let mut len: size_t = 0;
    if lenp.is_null() {
        len = strlen(ptr as *mut i8)
    } else {
        len = *lenp
    }
    if len == 0 {
        return vim_strsave(b"\x00" as *const u8 as *const i8 as *mut u8);
    }
    match (*vcp).vc_type {
        1 => {
            /* latin1 to utf-8 conversion */
            retval = xmalloc(len.wrapping_mul(2).wrapping_add(1)) as *mut u8;
            d = retval;
            let mut i = 0;
            while i < len {
                c = *ptr.offset(i as isize) as i32;
                if c < 0x80 as i32 {
                    let fresh14 = d;
                    d = d.offset(1);
                    *fresh14 = c as u8
                } else {
                    let fresh15 = d;
                    d = d.offset(1);
                    *fresh15 = (0xc0 as i32 as u32).wrapping_add(c as u32 >> 6) as u8;
                    let fresh16 = d;
                    d = d.offset(1);
                    *fresh16 = (0x80 as i32 + (c & 0x3f as i32)) as u8
                }
                i = i.wrapping_add(1)
            }
            *d = NUL as u8;
            if !lenp.is_null() {
                *lenp = d.offset_from(retval) as i64 as size_t
            }
        }
        2 => {
            /* latin9 to utf-8 conversion */
            retval = xmalloc(len.wrapping_mul(3).wrapping_add(1)) as *mut u8;
            d = retval;
            let mut i_0 = 0;
            while i_0 < len {
                c = *ptr.offset(i_0 as isize) as i32;
                match c {
                    164 => {
                        c = 0x20ac as i32
                        /* Y */
                    }
                    166 => c = 0x160 as i32,
                    168 => c = 0x161 as i32,
                    180 => c = 0x17d as i32,
                    184 => c = 0x17e as i32,
                    188 => c = 0x152 as i32,
                    189 => c = 0x153 as i32,
                    190 => c = 0x178 as i32,
                    _ => {}
                }
                d = d.offset(utf_char2bytes(c, d) as isize);
                i_0 = i_0.wrapping_add(1)
            }
            *d = NUL as u8;
            if !lenp.is_null() {
                *lenp = d.offset_from(retval) as i64 as size_t
            }
        }
        3 | 4 => {
            /* utf-8 to latin1 conversion */
            /* utf-8 to latin9 conversion */
            retval = xmalloc(len.wrapping_add(1)) as *mut u8;
            d = retval;
            let mut i_1 = 0;
            while i_1 < len {
                l = utf_ptr2len_len(ptr.offset(i_1 as isize), len.wrapping_sub(i_1) as i32);
                if l == 0 {
                    let fresh17 = d;
                    d = d.offset(1);
                    *fresh17 = NUL as u8
                } else if l == 1 {
                    let mut l_w = utf8len_tab_zero[*ptr.offset(i_1 as isize) as usize];
                    if l_w as i32 == 0 {
                        /* Illegal utf-8 byte cannot be converted */
                        xfree(retval as *mut libc::c_void);
                        return NULL_0 as *mut u8;
                    }
                    if !unconvlenp.is_null() && l_w as u64 > len.wrapping_sub(i_1) {
                        /* Incomplete sequence at the end. */
                        *unconvlenp = len.wrapping_sub(i_1);
                        break;
                    } else {
                        let fresh18 = d;
                        d = d.offset(1);
                        *fresh18 = *ptr.offset(i_1 as isize)
                    }
                } else {
                    c = utf_ptr2char(ptr.offset(i_1 as isize));
                    if (*vcp).vc_type == CONV_TO_LATIN9 as i32 {
                        match c {
                            8364 => {
                                c = 0xa4 as i32
                                /* not in latin9 */
                            }
                            352 => c = 0xa6 as i32,
                            353 => c = 0xa8 as i32,
                            381 => c = 0xb4 as i32,
                            382 => c = 0xb8 as i32,
                            338 => c = 0xbc as i32,
                            339 => c = 0xbd as i32,
                            376 => c = 0xbe as i32,
                            164 | 166 | 168 | 180 | 184 | 188 | 189 | 190 => c = 0x100 as i32,
                            _ => {}
                        }
                    }
                    if !utf_iscomposing(c) {
                        /* skip composing chars */
                        if c < 0x100 as i32 {
                            let fresh19 = d;
                            d = d.offset(1);
                            *fresh19 = c as u8
                        } else if (*vcp).vc_fail {
                            xfree(retval as *mut libc::c_void);
                            return NULL_0 as *mut u8;
                        } else {
                            let fresh20 = d;
                            d = d.offset(1);
                            *fresh20 = 0xbf as i32 as u8;
                            if utf_char2cells(c) > 1 {
                                let fresh21 = d;
                                d = d.offset(1);
                                *fresh21 = '?' as i32 as u8
                            }
                        }
                    }
                    i_1 = (i_1 as u64).wrapping_add((l - 1) as u64) as size_t as size_t
                }
                i_1 = i_1.wrapping_add(1)
            }
            *d = NUL as u8;
            if !lenp.is_null() {
                *lenp = d.offset_from(retval) as i64 as size_t
            }
        }
        5 => {
            // conversion with vcp->vc_fd
            retval = iconv_string(vcp, ptr, len, unconvlenp, lenp)
        }
        _ => {}
    }
    return retval;
}
