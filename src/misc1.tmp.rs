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
    #[c2rust::src_loc = "150:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "151:1"]
    pub type __off64_t = libc::c_long;
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
  "/usr/lib/llvm-6.0/lib/clang/6.0.1/include/stddef.h:11"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/types.h:14"]
pub mod nvim_types_h {
    #[c2rust::src_loc = "11:1"]
    pub type char_u = libc::c_uchar;
    // dummy to pass an ACL to a function
    // Shorthand for unsigned variables. Many systems, but not all, have u_char
// already defined, so we use char_u to avoid trouble.
    // Can hold one decoded UTF-8 character.
    // Opaque handle used by API clients to refer to various objects in vim
    #[c2rust::src_loc = "17:1"]
    pub type handle_T = libc::c_int;
    #[c2rust::src_loc = "22:1"]
    pub type LuaRef = libc::c_int;
    #[c2rust::src_loc = "24:1"]
    pub type expand_T = expand;
    use super::ex_cmds_defs_h::expand;
    // NVIM_TYPES_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/ex_cmds_defs.h:14"]
pub mod ex_cmds_defs_h {
    // for linenr_T
    // When adding an Ex command:
// 1. Add an entry to the table in src/nvim/ex_cmds.lua.  Keep it sorted on the
//    shortest version of the command name that works.  If it doesn't start with
//    a lower case letter, add it at the end.
//
//    Each table entry is a table with the following keys:
//
//      Key     | Description
//      ------- | -------------------------------------------------------------
//      command | Name of the command. Required.
//      enum    | Name of the enum entry. If not set defaults to CMD_{command}.
//      flags   | A set of the flags from below list joined by bitwise or.
//      func    | Name of the function containing the implementation.
//
//    Referenced function should be either non-static one or defined in
//    ex_docmd.c and be coercible to ex_func_T type from below.
//
//    All keys not described in the above table are reserved for future use.
//
// 2. Add a "case: CMD_xxx" in the big switch in ex_docmd.c.
// 3. Add an entry in the index for Ex commands at ":help ex-cmd-index".
// 4. Add documentation in ../doc/xxx.txt.  Add a tag for both the short and
//    long name of the command.
    // allow a linespecs
    // allow a ! after the command name
    // allow extra args after command name
    // expand wildcards in extra part
    // no spaces allowed in the extra part
    // default file range is 1,$
    // extend range to include whole fold also
                                // when less than two numbers given
    // argument required
    // check for trailing vertical bar
    // allow "x for register designation
    // allow count in argument, after command
    // no trailing comment allowed
    // zero line number allowed
    // do not remove CTRL-V from argument
    // number before command is not an address
    // allow "+command" argument
    // accepts buffer name
    // accepts unlisted buffer too
    // allow "++opt=val" argument
    // allowed in the sandbox
    // allowed in cmdline window; when missing
                                // disallows editing another buffer when
                                // curbuf_lock is set
    // forbidden in non-'modifiable' buffer
    // allow flags after count in argument
    // forbidden in restricted mode
    // multiple extra files allowed
    // one extra word allowed
    // 1 file allowed, defaults to current file
    // values for cmd_addr_type
    // Tab page that only relative
    /* behavior for bad character, "++bad=" argument */
    /* replace it with '?' (default) */
    /* leave it */
    /* erase it */
    // / Structure for command definition.
    // /< Name of the command.
    // /< Function with implementation of this command.
    // /< Relevant flags from the declared above.
    // /< Flag for address type
    // A list used for saving values of "emsg_silent".  Used by ex_try() to save the
// value of "emsg_silent" if it was non-zero.  When this is done, the CSF_SILENT
// flag below is set.
    // saved value of "emsg_silent"
    // next element on the list
    // For conditional commands a stack is kept of nested conditionals.
// When cs_idx < 0, there is no conditional command.
    // CSF_ flags
    // CSTP_: what's pending in ":finally"
    // return typeval for pending return
    // exception for pending throw
    // info used by ":for"
    // line nr of ":while"/":for" line
    // current entry, or -1 if none
    // nr of nested ":while"s and ":for"s
    // nr of nested ":try"s
    // saved values of "emsg_silent"
    // loop flags: CSL_ flags
    // Flags for the cs_lflags item in cstack_T.
    // just found ":while" or ":for"
    // just found ":endwhile" or ":endfor"
    // just found ":continue"
    // just found ":finally"
    // / Arguments used for Ex commands.
    // /< argument of the command
    // /< next command (NULL if none)
    // /< the name of the command (except for :make)
    // /< pointer to pointer of allocated cmdline
    // /< the index for the command
    // /< flags for the command
    // /< don't execute the command, only parse it
    // /< TRUE if ! present
    // /< the number of addresses given
    // /< the first line number
    // /< the second line number or count
    // /< type of the count/range
    // /< extra flags after count: EXFLAG_
    // /< +command arg to be used in edited file
    // /< the line number in an edited file
    // /< TRUE with ":w >>file" command
    // /< TRUE with ":w !command" and ":r!command"
    // /< number of '>' or '<' for shift command
    // /< register name (NUL if none)
    // /< 0, FORCE_BIN or FORCE_NOBIN
    // /< ++edit argument
    // /< ++ff= argument (first char of argument)
    // /< ++enc= argument (index in cmd[])
    // /< BAD_KEEP, BAD_DROP or replacement byte
    // /< user command index
    // /< returned error message
    // /< Function used to get the next line
    // /< argument for getline()
    // /< condition stack for ":if" etc.
    // ":edit ++bin file"
    // ":edit ++nobin file"
    // Values for "flags"
    // 'l': list
    // '#': number
    // 'p': print
    // used for completion on the command line
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "183:8"]
    pub struct expand {
        pub xp_context: libc::c_int,
        pub xp_pattern: *mut char_u,
        pub xp_pattern_len: size_t,
        pub xp_arg: *mut char_u,
        pub xp_script_ctx: sctx_T,
        pub xp_backslash: libc::c_int,
        pub xp_shell: libc::c_int,
        pub xp_numfiles: libc::c_int,
        pub xp_files: *mut *mut char_u,
        pub xp_line: *mut char_u,
        pub xp_col: libc::c_int,
    }
    use super::nvim_types_h::char_u;
    use super::stddef_h::size_t;
    use super::typval_h::sctx_T;
    // NVIM_EX_CMDS_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/eval/typval.h:14"]
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
        pub data: C2RustUnnamed_0,
        pub type_0: CallbackType,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:3"]
    pub union C2RustUnnamed_0 {
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
    pub type C2RustUnnamed_1 = libc::c_uint;
    #[c2rust::src_loc = "227:3"]
    pub const DI_FLAGS_ALLOC: C2RustUnnamed_1 = 16;
    #[c2rust::src_loc = "226:3"]
    pub const DI_FLAGS_LOCK: C2RustUnnamed_1 = 8;
    #[c2rust::src_loc = "225:3"]
    pub const DI_FLAGS_FIX: C2RustUnnamed_1 = 4;
    #[c2rust::src_loc = "224:3"]
    pub const DI_FLAGS_RO_SBX: C2RustUnnamed_1 = 2;
    #[c2rust::src_loc = "223:3"]
    pub const DI_FLAGS_RO: C2RustUnnamed_1 = 1;
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/pos.h:14"]
pub mod pos_h {
    #[c2rust::src_loc = "4:1"]
    pub type linenr_T = libc::c_long;
    #[c2rust::src_loc = "9:1"]
    pub type colnr_T = libc::c_int;
    #[c2rust::src_loc = "16:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "16:8"]
    pub const MAXCOL: C2RustUnnamed = 2147483647;
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
  "/usr/lib/llvm-6.0/lib/clang/6.0.1/include/stdarg.h:14"]
pub mod stdarg_h {
    #[c2rust::src_loc = "30:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h:14"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/FILE.h:14"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/stdio.h:14"]
pub mod stdio_h {
    #[c2rust::src_loc = "77:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
    use super::FILE_h::FILE;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "652:1"]
        pub fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
                     __stream: *mut FILE) -> size_t;
        #[no_mangle]
        #[c2rust::src_loc = "690:1"]
        pub fn fseek(__stream: *mut FILE, __off: libc::c_long,
                     __whence: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "695:1"]
        pub fn ftell(__stream: *mut FILE) -> libc::c_long;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/types.h:14"]
pub mod sys_types_h {
    #[c2rust::src_loc = "64:1"]
    pub type gid_t = __gid_t;
    #[c2rust::src_loc = "79:1"]
    pub type uid_t = __uid_t;
    use super::types_h::{__gid_t, __uid_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/time_t.h:14"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/pthreadtypes-arch.h:14"]
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
  "/usr/include/x86_64-linux-gnu/bits/thread-shared-types.h:14"]
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/pthreadtypes.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/hashtab.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/garray.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/lib/queue.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/profile.h:14"]
pub mod profile_h {
    #[c2rust::src_loc = "7:1"]
    pub type proftime_T = uint64_t;
    use super::stdint_uintn_h::uint64_t;
    // NVIM_PROFILE_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/grid_defs.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/keymap.h:14"]
pub mod keymap_h {
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
    #[c2rust::src_loc = "124:1"]
    pub type key_extra = libc::c_uint;
    // <Cmd> special key
    // event
    #[c2rust::src_loc = "248:5"]
    pub const KE_COMMAND: key_extra = 104;
    // no-op: does nothing
    // , KE_FOCUSGAINED = 98    // focus gained
  // , KE_FOCUSLOST = 99      // focus lost
  // , KE_MOUSEMOVE = 100     // mouse moved with no button down
  // , KE_CANCEL = 101        // return from vgetc
    #[c2rust::src_loc = "247:5"]
    pub const KE_EVENT: key_extra = 102;
    // DnD data is available
    // , KE_CURSORHOLD = 96     // CursorHold event
    #[c2rust::src_loc = "242:5"]
    pub const KE_NOP: key_extra = 97;
    #[c2rust::src_loc = "240:5"]
    pub const KE_DROP: key_extra = 95;
    #[c2rust::src_loc = "238:5"]
    pub const KE_X2RELEASE: key_extra = 94;
    #[c2rust::src_loc = "237:5"]
    pub const KE_X2DRAG: key_extra = 93;
    #[c2rust::src_loc = "236:5"]
    pub const KE_X2MOUSE: key_extra = 92;
    #[c2rust::src_loc = "235:5"]
    pub const KE_X1RELEASE: key_extra = 91;
    // X1/X2 mouse-buttons
    #[c2rust::src_loc = "234:5"]
    pub const KE_X1DRAG: key_extra = 90;
    // control-end
    #[c2rust::src_loc = "233:5"]
    pub const KE_X1MOUSE: key_extra = 89;
    // control-home
    #[c2rust::src_loc = "231:5"]
    pub const KE_C_END: key_extra = 88;
    // control-right
    #[c2rust::src_loc = "230:5"]
    pub const KE_C_HOME: key_extra = 87;
    // control-left
    #[c2rust::src_loc = "229:5"]
    pub const KE_C_RIGHT: key_extra = 86;
    // open command-line window from Command-line Mode
    #[c2rust::src_loc = "228:5"]
    pub const KE_C_LEFT: key_extra = 85;
    // <Plug>
    #[c2rust::src_loc = "226:5"]
    pub const KE_CMDWIN: key_extra = 84;
    // <SNR>
    #[c2rust::src_loc = "225:5"]
    pub const KE_PLUG: key_extra = 83;
    // CSI typed directly
    #[c2rust::src_loc = "224:5"]
    pub const KE_SNR: key_extra = 82;
    // keypad Delete key
    #[c2rust::src_loc = "223:5"]
    pub const KE_CSI: key_extra = 81;
    // keypad Insert key
    #[c2rust::src_loc = "221:5"]
    pub const KE_KDEL: key_extra = 80;
    // scroll wheel pseudo-button Right
    #[c2rust::src_loc = "220:5"]
    pub const KE_KINS: key_extra = 79;
    // scroll wheel pseudo-button Left
    #[c2rust::src_loc = "218:5"]
    pub const KE_MOUSERIGHT: key_extra = 78;
    // scroll wheel pseudo-button Up
    #[c2rust::src_loc = "217:5"]
    pub const KE_MOUSELEFT: key_extra = 77;
    // scroll wheel pseudo-button Down
    #[c2rust::src_loc = "216:5"]
    pub const KE_MOUSEUP: key_extra = 76;
    // NOTE: The scroll wheel events are inverted: i.e. UP is the same as
  // moving the actual scroll wheel down, LEFT is the same as moving the
  // scroll wheel right.
    #[c2rust::src_loc = "215:5"]
    pub const KE_MOUSEDOWN: key_extra = 75;
    #[c2rust::src_loc = "210:5"]
    pub const KE_S_XF4: key_extra = 74;
    #[c2rust::src_loc = "209:5"]
    pub const KE_S_XF3: key_extra = 73;
    // vt100 shifted function keys for xterm
    #[c2rust::src_loc = "208:5"]
    pub const KE_S_XF2: key_extra = 72;
    // non-mappable left mouse button release
    #[c2rust::src_loc = "207:5"]
    pub const KE_S_XF1: key_extra = 71;
    // non-mappable Left mouse button click
    #[c2rust::src_loc = "205:5"]
    pub const KE_LEFTRELEASE_NM: key_extra = 70;
    #[c2rust::src_loc = "204:5"]
    pub const KE_LEFTMOUSE_NM: key_extra = 69;
    #[c2rust::src_loc = "202:5"]
    pub const KE_XRIGHT: key_extra = 68;
    #[c2rust::src_loc = "201:5"]
    pub const KE_XLEFT: key_extra = 67;
    // extra vt100 cursor keys for xterm
    #[c2rust::src_loc = "200:5"]
    pub const KE_XDOWN: key_extra = 66;
    // extra (vt100) home key for xterm
    #[c2rust::src_loc = "199:5"]
    pub const KE_XUP: key_extra = 65;
    // extra (vt100) home key for xterm
    #[c2rust::src_loc = "198:5"]
    pub const KE_ZHOME: key_extra = 64;
    // extra (vt100) end key for xterm
    #[c2rust::src_loc = "197:5"]
    pub const KE_XHOME: key_extra = 63;
    // extra (vt100) end key for xterm
    #[c2rust::src_loc = "196:5"]
    pub const KE_ZEND: key_extra = 62;
    #[c2rust::src_loc = "195:5"]
    pub const KE_XEND: key_extra = 61;
    #[c2rust::src_loc = "194:5"]
    pub const KE_XF4: key_extra = 60;
    #[c2rust::src_loc = "193:5"]
    pub const KE_XF3: key_extra = 59;
    // extra vt100 function keys for xterm
    #[c2rust::src_loc = "192:5"]
    pub const KE_XF2: key_extra = 58;
    // shifted TAB key (no longer used)
    // , KE_SNIFF_UNUSED = 56   // obsolete
    #[c2rust::src_loc = "191:5"]
    pub const KE_XF1: key_extra = 57;
    // unshifted TAB key
    #[c2rust::src_loc = "188:5"]
    pub const KE_S_TAB_OLD: key_extra = 55;
    // Ignored mouse drag/release
    #[c2rust::src_loc = "187:5"]
    pub const KE_TAB: key_extra = 54;
    // Right mouse button release
    #[c2rust::src_loc = "185:5"]
    pub const KE_IGNORE: key_extra = 53;
    // Drag with right mouse button down
    #[c2rust::src_loc = "183:5"]
    pub const KE_RIGHTRELEASE: key_extra = 52;
    // Right mouse button click
    #[c2rust::src_loc = "182:5"]
    pub const KE_RIGHTDRAG: key_extra = 51;
    // Middle mouse button release
    #[c2rust::src_loc = "181:5"]
    pub const KE_RIGHTMOUSE: key_extra = 50;
    // Drag with middle mouse button down
    #[c2rust::src_loc = "180:5"]
    pub const KE_MIDDLERELEASE: key_extra = 49;
    // Middle mouse button click
    #[c2rust::src_loc = "179:5"]
    pub const KE_MIDDLEDRAG: key_extra = 48;
    // Left mouse button release
    #[c2rust::src_loc = "178:5"]
    pub const KE_MIDDLEMOUSE: key_extra = 47;
    // Drag with left mouse button down
    #[c2rust::src_loc = "177:5"]
    pub const KE_LEFTRELEASE: key_extra = 46;
    // Left mouse button click
    #[c2rust::src_loc = "176:5"]
    pub const KE_LEFTDRAG: key_extra = 45;
    // mouse event start
    // Symbols for pseudo keys which are translated from the real key symbols
  // above.
    #[c2rust::src_loc = "175:5"]
    pub const KE_LEFTMOUSE: key_extra = 44;
    #[c2rust::src_loc = "171:5"]
    pub const KE_MOUSE: key_extra = 43;
    #[c2rust::src_loc = "169:5"]
    pub const KE_S_F37: key_extra = 42;
    #[c2rust::src_loc = "168:5"]
    pub const KE_S_F36: key_extra = 41;
    #[c2rust::src_loc = "167:5"]
    pub const KE_S_F35: key_extra = 40;
    #[c2rust::src_loc = "166:5"]
    pub const KE_S_F34: key_extra = 39;
    #[c2rust::src_loc = "165:5"]
    pub const KE_S_F33: key_extra = 38;
    #[c2rust::src_loc = "164:5"]
    pub const KE_S_F32: key_extra = 37;
    #[c2rust::src_loc = "163:5"]
    pub const KE_S_F31: key_extra = 36;
    #[c2rust::src_loc = "161:5"]
    pub const KE_S_F30: key_extra = 35;
    #[c2rust::src_loc = "160:5"]
    pub const KE_S_F29: key_extra = 34;
    #[c2rust::src_loc = "159:5"]
    pub const KE_S_F28: key_extra = 33;
    #[c2rust::src_loc = "158:5"]
    pub const KE_S_F27: key_extra = 32;
    #[c2rust::src_loc = "157:5"]
    pub const KE_S_F26: key_extra = 31;
    #[c2rust::src_loc = "156:5"]
    pub const KE_S_F25: key_extra = 30;
    #[c2rust::src_loc = "155:5"]
    pub const KE_S_F24: key_extra = 29;
    #[c2rust::src_loc = "154:5"]
    pub const KE_S_F23: key_extra = 28;
    #[c2rust::src_loc = "153:5"]
    pub const KE_S_F22: key_extra = 27;
    #[c2rust::src_loc = "152:5"]
    pub const KE_S_F21: key_extra = 26;
    #[c2rust::src_loc = "150:5"]
    pub const KE_S_F20: key_extra = 25;
    #[c2rust::src_loc = "149:5"]
    pub const KE_S_F19: key_extra = 24;
    #[c2rust::src_loc = "148:5"]
    pub const KE_S_F18: key_extra = 23;
    #[c2rust::src_loc = "147:5"]
    pub const KE_S_F17: key_extra = 22;
    #[c2rust::src_loc = "146:5"]
    pub const KE_S_F16: key_extra = 21;
    #[c2rust::src_loc = "145:5"]
    pub const KE_S_F15: key_extra = 20;
    #[c2rust::src_loc = "144:5"]
    pub const KE_S_F14: key_extra = 19;
    #[c2rust::src_loc = "143:5"]
    pub const KE_S_F13: key_extra = 18;
    #[c2rust::src_loc = "142:5"]
    pub const KE_S_F12: key_extra = 17;
    #[c2rust::src_loc = "141:5"]
    pub const KE_S_F11: key_extra = 16;
    #[c2rust::src_loc = "139:5"]
    pub const KE_S_F10: key_extra = 15;
    #[c2rust::src_loc = "138:5"]
    pub const KE_S_F9: key_extra = 14;
    #[c2rust::src_loc = "137:5"]
    pub const KE_S_F8: key_extra = 13;
    #[c2rust::src_loc = "136:5"]
    pub const KE_S_F7: key_extra = 12;
    #[c2rust::src_loc = "135:5"]
    pub const KE_S_F6: key_extra = 11;
    #[c2rust::src_loc = "134:5"]
    pub const KE_S_F5: key_extra = 10;
    #[c2rust::src_loc = "133:5"]
    pub const KE_S_F4: key_extra = 9;
    #[c2rust::src_loc = "132:5"]
    pub const KE_S_F3: key_extra = 8;
    // shifted function keys
    #[c2rust::src_loc = "131:5"]
    pub const KE_S_F2: key_extra = 7;
    // shift-down
    #[c2rust::src_loc = "130:5"]
    pub const KE_S_F1: key_extra = 6;
    // shift-up
    #[c2rust::src_loc = "128:5"]
    pub const KE_S_DOWN: key_extra = 5;
    // name of this terminal entry
    #[c2rust::src_loc = "127:5"]
    pub const KE_S_UP: key_extra = 4;
    #[c2rust::src_loc = "125:3"]
    pub const KE_NAME: key_extra = 3;
    /*
 * the three byte codes are replaced with the following int when using vgetc()
 */
    // keypad up
    // keypad down
    // keypad left
    // keypad right
    /* extra set of function keys F1-F4, for vt100 compatible xterm */
    /* extra set of cursor keys for vt100 compatible xterm */
    // function keys
    /* extra set of shifted function keys F1-F4, for vt100 compatible xterm */
    // shifted func. keys
    /* K_S_F13 to K_S_F37  are currently not used */
    #[c2rust::src_loc = "358:9"]
    pub const K_DEL: libc::c_int =
        -('k' as i32 + (('D' as i32) << 8 as libc::c_int));
    #[c2rust::src_loc = "359:9"]
    pub const K_KDEL: libc::c_int =
        -(253 as libc::c_int +
              ((KE_KDEL as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "354:9"]
    pub const K_BS: libc::c_int =
        -('k' as i32 + (('b' as i32) << 8 as libc::c_int));
    #[c2rust::src_loc = "52:9"]
    pub const KS_SPECIAL: libc::c_int = 254 as libc::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const K_SPECIAL: libc::c_int = 0x80 as libc::c_int;
    #[c2rust::src_loc = "46:9"]
    pub const KS_ZERO: libc::c_int = 255 as libc::c_int;
    #[c2rust::src_loc = "254:9"]
    pub const K_ZERO: libc::c_int =
        -(255 as libc::c_int + (('X' as i32) << 8 as libc::c_int));
    // keypad home (upper left)
    // keypad end (lower left)
    // keypad pageup (upper R.)
    // keypad pagedown (lower R.)
    // keypad center
    // keypad plus
    // keypad minus
    // keypad /
    // keypad *
    // keypad Enter
    // keypad . or ,
    // keypad 0
    // keypad 1
    // keypad 2
    // keypad 3
    // keypad 4
    // keypad 5
    // keypad 6
    // keypad 7
    // keypad 8
    // keypad 9
    // keypad comma
    // keypad equal
    /*
 * Symbols for pseudo keys which are translated from the real key symbols
 * above.
 */
    #[c2rust::src_loc = "428:9"]
    pub const K_IGNORE: libc::c_int =
        -(253 as libc::c_int +
              ((KE_IGNORE as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "410:9"]
    pub const K_LEFTMOUSE_NM: libc::c_int =
        -(253 as libc::c_int +
              ((KE_LEFTMOUSE_NM as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "411:9"]
    pub const K_LEFTDRAG: libc::c_int =
        -(253 as libc::c_int +
              ((KE_LEFTDRAG as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "412:9"]
    pub const K_LEFTRELEASE: libc::c_int =
        -(253 as libc::c_int +
              ((KE_LEFTRELEASE as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "413:9"]
    pub const K_LEFTRELEASE_NM: libc::c_int =
        -(253 as libc::c_int +
              ((KE_LEFTRELEASE_NM as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "414:9"]
    pub const K_MIDDLEMOUSE: libc::c_int =
        -(253 as libc::c_int +
              ((KE_MIDDLEMOUSE as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "415:9"]
    pub const K_MIDDLEDRAG: libc::c_int =
        -(253 as libc::c_int +
              ((KE_MIDDLEDRAG as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "416:9"]
    pub const K_MIDDLERELEASE: libc::c_int =
        -(253 as libc::c_int +
              ((KE_MIDDLERELEASE as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "417:9"]
    pub const K_RIGHTMOUSE: libc::c_int =
        -(253 as libc::c_int +
              ((KE_RIGHTMOUSE as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "418:9"]
    pub const K_RIGHTDRAG: libc::c_int =
        -(253 as libc::c_int +
              ((KE_RIGHTDRAG as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "419:9"]
    pub const K_RIGHTRELEASE: libc::c_int =
        -(253 as libc::c_int +
              ((KE_RIGHTRELEASE as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "431:9"]
    pub const K_MOUSEDOWN: libc::c_int =
        -(253 as libc::c_int +
              ((KE_MOUSEDOWN as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "432:9"]
    pub const K_MOUSEUP: libc::c_int =
        -(253 as libc::c_int +
              ((KE_MOUSEUP as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "433:9"]
    pub const K_MOUSELEFT: libc::c_int =
        -(253 as libc::c_int +
              ((KE_MOUSELEFT as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "434:9"]
    pub const K_MOUSERIGHT: libc::c_int =
        -(253 as libc::c_int +
              ((KE_MOUSERIGHT as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "421:9"]
    pub const K_X1MOUSE: libc::c_int =
        -(253 as libc::c_int +
              ((KE_X1MOUSE as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "422:9"]
    pub const K_X1DRAG: libc::c_int =
        -(253 as libc::c_int +
              ((KE_X1DRAG as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "423:9"]
    pub const K_X1RELEASE: libc::c_int =
        -(253 as libc::c_int +
              ((KE_X1RELEASE as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "424:9"]
    pub const K_X2MOUSE: libc::c_int =
        -(253 as libc::c_int +
              ((KE_X2MOUSE as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "425:9"]
    pub const K_X2DRAG: libc::c_int =
        -(253 as libc::c_int +
              ((KE_X2DRAG as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "426:9"]
    pub const K_X2RELEASE: libc::c_int =
        -(253 as libc::c_int +
              ((KE_X2RELEASE as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "409:9"]
    pub const K_LEFTMOUSE: libc::c_int =
        -(253 as libc::c_int +
              ((KE_LEFTMOUSE as libc::c_int) << 8 as libc::c_int));
    #[c2rust::src_loc = "64:9"]
    pub const KS_MODIFIER: libc::c_int = 252 as libc::c_int;
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
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/buffer_defs.h:14"]
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
        pub update_channels: C2RustUnnamed_3,
        pub update_callbacks: C2RustUnnamed_2,
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
    pub struct C2RustUnnamed_2 {
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
    pub struct C2RustUnnamed_3 {
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
        pub w_p_lcs_chars: C2RustUnnamed_5,
        pub w_p_fcs_chars: C2RustUnnamed_4,
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
    pub struct C2RustUnnamed_4 {
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
    pub struct C2RustUnnamed_5 {
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/map.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/lib/khash.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/extmark_defs.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/marktree.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/map_defs.h:14"]
pub mod map_defs_h {
    #[c2rust::src_loc = "7:1"]
    pub type ptr_t = *mut libc::c_void;
    #[c2rust::src_loc = "6:1"]
    pub type cstr_t = *const libc::c_char;
    // NVIM_MAP_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/terminal.h:14"]
pub mod terminal_h {
    #[c2rust::src_loc = "8:1"]
    pub type Terminal = terminal;
    extern "C" {
        #[c2rust::src_loc = "8:16"]
        pub type terminal;
    }
    // NVIM_TERMINAL_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/sign_defs.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/regexp_defs.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/api/private/defs.h:14"]
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
        pub data: C2RustUnnamed_12,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:3"]
    pub union C2RustUnnamed_12 {
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/mark_defs.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/os/time.h:14"]
pub mod time_h {
    #[c2rust::src_loc = "8:1"]
    pub type Timestamp = uint64_t;
    use super::stdint_uintn_h::uint64_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "11:1"]
        pub fn os_hrtime() -> uint64_t;
    }
    // NVIM_OS_TIME_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/option_defs.h:14"]
pub mod option_defs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "862:9"]
    pub struct LastSet {
        pub script_ctx: sctx_T,
        pub channel_id: uint64_t,
    }
    #[c2rust::src_loc = "327:9"]
    pub const BO_ALL: libc::c_int = 0x1 as libc::c_int;
    #[c2rust::src_loc = "333:9"]
    pub const BO_ERROR: libc::c_int = 0x40 as libc::c_int;
    #[c2rust::src_loc = "210:9"]
    pub const COM_NEST: libc::c_int = 'n' as i32;
    #[c2rust::src_loc = "213:9"]
    pub const COM_MIDDLE: libc::c_int = 'm' as i32;
    #[c2rust::src_loc = "211:9"]
    pub const COM_BLANK: libc::c_int = 'b' as i32;
    #[c2rust::src_loc = "219:9"]
    pub const COM_NOBACK: libc::c_int = 'O' as i32;
    #[c2rust::src_loc = "214:9"]
    pub const COM_END: libc::c_int = 'e' as i32;
    #[c2rust::src_loc = "221:9"]
    pub const COM_MAX_LEN: libc::c_int = 50 as libc::c_int;
    use super::typval_h::sctx_T;
    use super::stdint_uintn_h::uint64_t;
    use super::nvim_types_h::char_u;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "316:13"]
        pub static mut breakat_flags: [libc::c_char; 256];
        #[no_mangle]
        #[c2rust::src_loc = "317:17"]
        pub static mut bo_flags: libc::c_uint;
        #[no_mangle]
        #[c2rust::src_loc = "387:18"]
        pub static mut p_debug: *mut char_u;
        #[no_mangle]
        #[c2rust::src_loc = "472:12"]
        pub static mut p_im: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "545:13"]
        pub static mut p_report: libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "588:18"]
        pub static mut p_sh: *mut char_u;
        #[no_mangle]
        #[c2rust::src_loc = "670:12"]
        pub static mut p_vb: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "680:13"]
        pub static mut p_verbose: libc::c_long;
    }
    // NVIM_OPTION_DEFS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/syntax_defs.h:14"]
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
        pub sst_union: C2RustUnnamed_6,
        pub sst_next_flags: libc::c_int,
        pub sst_stacksize: libc::c_int,
        pub sst_next_list: *mut int16_t,
        pub sst_tick: disptick_T,
        pub sst_change_lnum: linenr_T,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:3"]
    pub union C2RustUnnamed_6 {
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/undo_defs.h:14"]
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
        pub uh_next: C2RustUnnamed_10,
        pub uh_prev: C2RustUnnamed_9,
        pub uh_alt_next: C2RustUnnamed_8,
        pub uh_alt_prev: C2RustUnnamed_7,
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
    pub union C2RustUnnamed_7 {
        pub ptr: *mut u_header_T,
        pub seq: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:3"]
    pub union C2RustUnnamed_8 {
        pub ptr: *mut u_header_T,
        pub seq: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:3"]
    pub union C2RustUnnamed_9 {
        pub ptr: *mut u_header_T,
        pub seq: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:3"]
    pub union C2RustUnnamed_10 {
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/os/fs_defs.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/memline_defs.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/memfile_defs.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/highlight_defs.h:14"]
pub mod highlight_defs_h {
    // / Values for index in highlight_attr[].
// / When making changes, also update hlf_names below!
    #[c2rust::src_loc = "48:9"]
    pub type C2RustUnnamed_11 = libc::c_uint;
    // MUST be the last one
    // Message area
    #[c2rust::src_loc = "101:5"]
    pub const HLF_COUNT: C2RustUnnamed_11 = 50;
    // Floating window
    #[c2rust::src_loc = "100:5"]
    pub const HLF_MSG: C2RustUnnamed_11 = 49;
    // message separator line
    #[c2rust::src_loc = "99:5"]
    pub const HLF_NFLOAT: C2RustUnnamed_11 = 48;
    // NormalNC: Normal text in non-current windows
    #[c2rust::src_loc = "98:5"]
    pub const HLF_MSGSEP: C2RustUnnamed_11 = 47;
    // Whitespace
    #[c2rust::src_loc = "97:5"]
    pub const HLF_INACTIVE: C2RustUnnamed_11 = 46;
    // selected quickfix line
    #[c2rust::src_loc = "96:5"]
    pub const HLF_0: C2RustUnnamed_11 = 45;
    // 'colorcolumn'
    #[c2rust::src_loc = "95:5"]
    pub const HLF_QFL: C2RustUnnamed_11 = 44;
    // 'cursorline'
    #[c2rust::src_loc = "94:5"]
    pub const HLF_MC: C2RustUnnamed_11 = 43;
    // 'cursorcolumn'
    #[c2rust::src_loc = "93:5"]
    pub const HLF_CUL: C2RustUnnamed_11 = 42;
    // tabpage line filler
    #[c2rust::src_loc = "92:5"]
    pub const HLF_CUC: C2RustUnnamed_11 = 41;
    // tabpage line selected
    #[c2rust::src_loc = "91:5"]
    pub const HLF_TPF: C2RustUnnamed_11 = 40;
    // tabpage line
    #[c2rust::src_loc = "90:5"]
    pub const HLF_TPS: C2RustUnnamed_11 = 39;
    // popup menu scrollbar thumb
    #[c2rust::src_loc = "89:5"]
    pub const HLF_TP: C2RustUnnamed_11 = 38;
    // popup menu scrollbar
    #[c2rust::src_loc = "88:5"]
    pub const HLF_PST: C2RustUnnamed_11 = 37;
    // popup menu selected item
    #[c2rust::src_loc = "87:5"]
    pub const HLF_PSB: C2RustUnnamed_11 = 36;
    // popup menu normal item
    #[c2rust::src_loc = "86:5"]
    pub const HLF_PSI: C2RustUnnamed_11 = 35;
    // SpellLocal
    #[c2rust::src_loc = "85:5"]
    pub const HLF_PNI: C2RustUnnamed_11 = 34;
    // SpellRare
    #[c2rust::src_loc = "84:5"]
    pub const HLF_SPL: C2RustUnnamed_11 = 33;
    // SpellCap
    #[c2rust::src_loc = "83:5"]
    pub const HLF_SPR: C2RustUnnamed_11 = 32;
    // SpellBad
    #[c2rust::src_loc = "82:5"]
    pub const HLF_SPC: C2RustUnnamed_11 = 31;
    // Concealed text
    #[c2rust::src_loc = "81:5"]
    pub const HLF_SPB: C2RustUnnamed_11 = 30;
    // Sign column
    #[c2rust::src_loc = "80:5"]
    pub const HLF_CONCEAL: C2RustUnnamed_11 = 29;
    // Text Changed in diff line
    #[c2rust::src_loc = "79:5"]
    pub const HLF_SC: C2RustUnnamed_11 = 28;
    // Deleted diff line
    #[c2rust::src_loc = "78:5"]
    pub const HLF_TXD: C2RustUnnamed_11 = 27;
    // Changed diff line
    #[c2rust::src_loc = "77:5"]
    pub const HLF_DED: C2RustUnnamed_11 = 26;
    // Added diff line
    #[c2rust::src_loc = "76:5"]
    pub const HLF_CHD: C2RustUnnamed_11 = 25;
    // Fold column
    #[c2rust::src_loc = "75:5"]
    pub const HLF_ADD: C2RustUnnamed_11 = 24;
    // Folded line
    #[c2rust::src_loc = "74:5"]
    pub const HLF_FC: C2RustUnnamed_11 = 23;
    // Wildmenu highlight
    #[c2rust::src_loc = "73:5"]
    pub const HLF_FL: C2RustUnnamed_11 = 22;
    // warning messages
    #[c2rust::src_loc = "72:5"]
    pub const HLF_WM: C2RustUnnamed_11 = 21;
    // Visual mode, autoselecting and not clipboard owner
    #[c2rust::src_loc = "71:5"]
    pub const HLF_W: C2RustUnnamed_11 = 20;
    // Visual mode
    #[c2rust::src_loc = "70:5"]
    pub const HLF_VNC: C2RustUnnamed_11 = 19;
    // Titles for output from ":set all", ":autocmd" etc.
    #[c2rust::src_loc = "69:5"]
    pub const HLF_V: C2RustUnnamed_11 = 18;
    // column to separate vertically split windows
    #[c2rust::src_loc = "68:5"]
    pub const HLF_T: C2RustUnnamed_11 = 17;
    // status lines of not-current windows
    #[c2rust::src_loc = "67:5"]
    pub const HLF_C: C2RustUnnamed_11 = 16;
    // status lines
    #[c2rust::src_loc = "66:5"]
    pub const HLF_SNC: C2RustUnnamed_11 = 15;
    // return to continue message and yes/no questions
    #[c2rust::src_loc = "65:5"]
    pub const HLF_S: C2RustUnnamed_11 = 14;
    // current line number
    #[c2rust::src_loc = "64:5"]
    pub const HLF_R: C2RustUnnamed_11 = 13;
    // line number for ":number" and ":#" commands
    #[c2rust::src_loc = "63:5"]
    pub const HLF_CLN: C2RustUnnamed_11 = 12;
    // Mode (e.g., "-- INSERT --")
    #[c2rust::src_loc = "62:5"]
    pub const HLF_N: C2RustUnnamed_11 = 11;
    // "--More--" message
    #[c2rust::src_loc = "61:5"]
    pub const HLF_CM: C2RustUnnamed_11 = 10;
    // last search string
    #[c2rust::src_loc = "60:5"]
    pub const HLF_M: C2RustUnnamed_11 = 9;
    // incremental search
    #[c2rust::src_loc = "59:5"]
    pub const HLF_L: C2RustUnnamed_11 = 8;
    // error messages
    #[c2rust::src_loc = "58:5"]
    pub const HLF_I: C2RustUnnamed_11 = 7;
    // directories in CTRL-D listing
    #[c2rust::src_loc = "57:5"]
    pub const HLF_E: C2RustUnnamed_11 = 6;
    // @ characters at end of screen, characters that
                    // don't really exist in the text
    #[c2rust::src_loc = "56:5"]
    pub const HLF_D: C2RustUnnamed_11 = 5;
    // terminal cursor unfocused
    #[c2rust::src_loc = "54:5"]
    pub const HLF_AT: C2RustUnnamed_11 = 4;
    // terminal cursor focused
    #[c2rust::src_loc = "53:5"]
    pub const HLF_TERMNC: C2RustUnnamed_11 = 3;
    // after the last line in the buffer
    #[c2rust::src_loc = "52:5"]
    pub const HLF_TERM: C2RustUnnamed_11 = 2;
    // Meta & special keys listed with ":map", text that is
                    // displayed different from what it is
    #[c2rust::src_loc = "51:5"]
    pub const HLF_EOB: C2RustUnnamed_11 = 1;
    #[c2rust::src_loc = "49:3"]
    pub const HLF_8: C2RustUnnamed_11 = 0;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "158:12"]
        pub static mut highlight_attr: [libc::c_int; 50];
    }
    // NVIM_HIGHLIGHT_DEFS_H
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/types/struct_iovec.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/.deps/usr/include/uv.h:14"]
pub mod uv_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1693:8"]
    pub struct uv_loop_s {
        pub data: *mut libc::c_void,
        pub active_handles: libc::c_uint,
        pub handle_queue: [*mut libc::c_void; 2],
        pub active_reqs: C2RustUnnamed_18,
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
        pub timer_heap: C2RustUnnamed_16,
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
        pub u: C2RustUnnamed_15,
        pub next_closing: *mut uv_handle_t,
        pub flags: libc::c_uint,
        pub signal_cb: uv_signal_cb,
        pub signum: libc::c_int,
        pub tree_entry: C2RustUnnamed_13,
        pub caught_signals: libc::c_uint,
        pub dispatched_signals: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1512:3"]
    pub struct C2RustUnnamed_13 {
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
        pub u: C2RustUnnamed_14,
        pub next_closing: *mut uv_handle_t,
        pub flags: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "436:3"]
    pub union C2RustUnnamed_14 {
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
    pub union C2RustUnnamed_15 {
        pub fd: libc::c_int,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1705:3"]
    pub struct C2RustUnnamed_16 {
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
        pub u: C2RustUnnamed_17,
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
    pub union C2RustUnnamed_17 {
        pub fd: libc::c_int,
        pub reserved: [*mut libc::c_void; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1699:3"]
    pub union C2RustUnnamed_18 {
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
    pub union C2RustUnnamed_19 {
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
        pub u: C2RustUnnamed_20,
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
    pub union C2RustUnnamed_20 {
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
        pub ipc: libc::c_int,
        pub pipe_fname: *const libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "727:3"]
    pub union C2RustUnnamed_21 {
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
        pub u: C2RustUnnamed_22,
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
    pub union C2RustUnnamed_22 {
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
        pub u: C2RustUnnamed_23,
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
    pub union C2RustUnnamed_23 {
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
        pub u: C2RustUnnamed_24,
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
    pub union C2RustUnnamed_24 {
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
        pub data: C2RustUnnamed_25,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "903:3"]
    pub union C2RustUnnamed_25 {
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
#[c2rust::header_src = "/home/vole/neovim/.deps/usr/include/uv/unix.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/defs.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/multiqueue.h:14"]
pub mod multiqueue_h {
    #[c2rust::src_loc = "10:1"]
    pub type MultiQueue = multiqueue;
    extern "C" {
        #[c2rust::src_loc = "10:16"]
        pub type multiqueue;
    }
    // NVIM_EVENT_MULTIQUEUE_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/loop.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/os/shell.h:16"]
pub mod shell_h {
    #[c2rust::src_loc = "9:9"]
    pub type ShellOpts = libc::c_uint;
    #[c2rust::src_loc = "16:3"]
    pub const kShellOptHideMess: ShellOpts = 64;
    #[c2rust::src_loc = "15:3"]
    pub const kShellOptWrite: ShellOpts = 32;
    #[c2rust::src_loc = "14:3"]
    pub const kShellOptRead: ShellOpts = 16;
    #[c2rust::src_loc = "13:3"]
    pub const kShellOptSilent: ShellOpts = 8;
    #[c2rust::src_loc = "12:3"]
    pub const kShellOptDoOut: ShellOpts = 4;
    #[c2rust::src_loc = "11:3"]
    pub const kShellOptExpand: ShellOpts = 2;
    #[c2rust::src_loc = "10:3"]
    pub const kShellOptFilter: ShellOpts = 1;
    // NVIM_OS_SHELL_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/fold.h:27"]
pub mod fold_h {
    /*
 * Info used to pass info about a fold from the fold-detection code to the
 * code that displays the foldcolumn.
 */
    #[c2rust::src_loc = "15:1"]
    pub type foldinfo_T = foldinfo;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:16"]
    pub struct foldinfo {
        pub fi_lnum: linenr_T,
        pub fi_level: libc::c_int,
        pub fi_low_level: libc::c_int,
    }
    use super::pos_h::linenr_T;
    /* line number where fold starts */
    /* level of the fold; when this is zero the
                                   other fields are invalid */
    /* lowest fold level that starts in the same
                                   line */
    // NVIM_FOLD_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/getchar.h:28"]
pub mod getchar_h {
    #[c2rust::src_loc = "21:9"]
    pub type flush_buffers_T = libc::c_uint;
    #[c2rust::src_loc = "24:3"]
    pub const FLUSH_INPUT: flush_buffers_T = 2;
    #[c2rust::src_loc = "23:3"]
    pub const FLUSH_TYPEAHEAD: flush_buffers_T = 1;
    #[c2rust::src_loc = "22:3"]
    pub const FLUSH_MINIMAL: flush_buffers_T = 0;
    // NVIM_GETCHAR_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/eval.h:21"]
pub mod eval_h {
    #[c2rust::src_loc = "83:9"]
    pub type VimVarIndex = libc::c_uint;
    #[c2rust::src_loc = "168:5"]
    pub const VV_LUA: VimVarIndex = 84;
    #[c2rust::src_loc = "167:5"]
    pub const VV_EXITING: VimVarIndex = 83;
    #[c2rust::src_loc = "166:5"]
    pub const VV_ECHOSPACE: VimVarIndex = 82;
    #[c2rust::src_loc = "165:5"]
    pub const VV_TYPE_BOOL: VimVarIndex = 81;
    #[c2rust::src_loc = "164:5"]
    pub const VV_TYPE_FLOAT: VimVarIndex = 80;
    #[c2rust::src_loc = "163:5"]
    pub const VV_TYPE_DICT: VimVarIndex = 79;
    #[c2rust::src_loc = "162:5"]
    pub const VV_TYPE_LIST: VimVarIndex = 78;
    #[c2rust::src_loc = "161:5"]
    pub const VV_TYPE_FUNC: VimVarIndex = 77;
    #[c2rust::src_loc = "160:5"]
    pub const VV_TYPE_STRING: VimVarIndex = 76;
    #[c2rust::src_loc = "159:5"]
    pub const VV_TYPE_NUMBER: VimVarIndex = 75;
    #[c2rust::src_loc = "158:5"]
    pub const VV_TESTING: VimVarIndex = 74;
    #[c2rust::src_loc = "157:5"]
    pub const VV_VIM_DID_ENTER: VimVarIndex = 73;
    #[c2rust::src_loc = "156:5"]
    pub const VV__NULL_DICT: VimVarIndex = 72;
    #[c2rust::src_loc = "155:5"]
    pub const VV__NULL_LIST: VimVarIndex = 71;
    #[c2rust::src_loc = "154:5"]
    pub const VV_NULL: VimVarIndex = 70;
    #[c2rust::src_loc = "153:5"]
    pub const VV_TRUE: VimVarIndex = 69;
    #[c2rust::src_loc = "152:5"]
    pub const VV_FALSE: VimVarIndex = 68;
    #[c2rust::src_loc = "151:5"]
    pub const VV_EVENT: VimVarIndex = 67;
    #[c2rust::src_loc = "150:5"]
    pub const VV_MSGPACK_TYPES: VimVarIndex = 66;
    #[c2rust::src_loc = "149:5"]
    pub const VV_ERRORS: VimVarIndex = 65;
    #[c2rust::src_loc = "148:5"]
    pub const VV_OPTION_TYPE: VimVarIndex = 64;
    #[c2rust::src_loc = "147:5"]
    pub const VV_OPTION_OLD: VimVarIndex = 63;
    #[c2rust::src_loc = "146:5"]
    pub const VV_OPTION_NEW: VimVarIndex = 62;
    #[c2rust::src_loc = "145:5"]
    pub const VV_COMPLETED_ITEM: VimVarIndex = 61;
    #[c2rust::src_loc = "144:5"]
    pub const VV_PROGPATH: VimVarIndex = 60;
    #[c2rust::src_loc = "143:5"]
    pub const VV_WINDOWID: VimVarIndex = 59;
    #[c2rust::src_loc = "142:5"]
    pub const VV_OLDFILES: VimVarIndex = 58;
    #[c2rust::src_loc = "141:5"]
    pub const VV_HLSEARCH: VimVarIndex = 57;
    #[c2rust::src_loc = "140:5"]
    pub const VV_SEARCHFORWARD: VimVarIndex = 56;
    #[c2rust::src_loc = "139:5"]
    pub const VV_OP: VimVarIndex = 55;
    #[c2rust::src_loc = "138:5"]
    pub const VV_MOUSE_COL: VimVarIndex = 54;
    #[c2rust::src_loc = "137:5"]
    pub const VV_MOUSE_LNUM: VimVarIndex = 53;
    #[c2rust::src_loc = "136:5"]
    pub const VV_MOUSE_WINID: VimVarIndex = 52;
    #[c2rust::src_loc = "135:5"]
    pub const VV_MOUSE_WIN: VimVarIndex = 51;
    #[c2rust::src_loc = "134:5"]
    pub const VV_CHAR: VimVarIndex = 50;
    #[c2rust::src_loc = "133:5"]
    pub const VV_SWAPCOMMAND: VimVarIndex = 49;
    #[c2rust::src_loc = "132:5"]
    pub const VV_SWAPCHOICE: VimVarIndex = 48;
    #[c2rust::src_loc = "131:5"]
    pub const VV_SWAPNAME: VimVarIndex = 47;
    #[c2rust::src_loc = "130:5"]
    pub const VV_SCROLLSTART: VimVarIndex = 46;
    #[c2rust::src_loc = "129:5"]
    pub const VV_BEVAL_TEXT: VimVarIndex = 45;
    #[c2rust::src_loc = "128:5"]
    pub const VV_BEVAL_COL: VimVarIndex = 44;
    #[c2rust::src_loc = "127:5"]
    pub const VV_BEVAL_LNUM: VimVarIndex = 43;
    #[c2rust::src_loc = "126:5"]
    pub const VV_BEVAL_WINID: VimVarIndex = 42;
    #[c2rust::src_loc = "125:5"]
    pub const VV_BEVAL_WINNR: VimVarIndex = 41;
    #[c2rust::src_loc = "124:5"]
    pub const VV_BEVAL_BUFNR: VimVarIndex = 40;
    #[c2rust::src_loc = "123:5"]
    pub const VV_FCS_CHOICE: VimVarIndex = 39;
    #[c2rust::src_loc = "122:5"]
    pub const VV_FCS_REASON: VimVarIndex = 38;
    #[c2rust::src_loc = "121:5"]
    pub const VV_PROFILING: VimVarIndex = 37;
    #[c2rust::src_loc = "120:5"]
    pub const VV_KEY: VimVarIndex = 36;
    #[c2rust::src_loc = "119:5"]
    pub const VV_VAL: VimVarIndex = 35;
    #[c2rust::src_loc = "118:5"]
    pub const VV_INSERTMODE: VimVarIndex = 34;
    #[c2rust::src_loc = "117:5"]
    pub const VV_CMDBANG: VimVarIndex = 33;
    #[c2rust::src_loc = "116:5"]
    pub const VV_REG: VimVarIndex = 32;
    #[c2rust::src_loc = "115:5"]
    pub const VV_STDERR: VimVarIndex = 31;
    #[c2rust::src_loc = "114:5"]
    pub const VV_THROWPOINT: VimVarIndex = 30;
    #[c2rust::src_loc = "113:5"]
    pub const VV_EXCEPTION: VimVarIndex = 29;
    #[c2rust::src_loc = "112:5"]
    pub const VV_DYING: VimVarIndex = 28;
    #[c2rust::src_loc = "111:5"]
    pub const VV_SEND_SERVER: VimVarIndex = 27;
    #[c2rust::src_loc = "110:5"]
    pub const VV_PROGNAME: VimVarIndex = 26;
    #[c2rust::src_loc = "109:5"]
    pub const VV_FOLDLEVEL: VimVarIndex = 25;
    #[c2rust::src_loc = "108:5"]
    pub const VV_FOLDDASHES: VimVarIndex = 24;
    #[c2rust::src_loc = "107:5"]
    pub const VV_FOLDEND: VimVarIndex = 23;
    #[c2rust::src_loc = "106:5"]
    pub const VV_FOLDSTART: VimVarIndex = 22;
    #[c2rust::src_loc = "105:5"]
    pub const VV_CMDARG: VimVarIndex = 21;
    #[c2rust::src_loc = "104:5"]
    pub const VV_FNAME_DIFF: VimVarIndex = 20;
    #[c2rust::src_loc = "103:5"]
    pub const VV_FNAME_NEW: VimVarIndex = 19;
    #[c2rust::src_loc = "102:5"]
    pub const VV_FNAME_OUT: VimVarIndex = 18;
    #[c2rust::src_loc = "101:5"]
    pub const VV_FNAME_IN: VimVarIndex = 17;
    #[c2rust::src_loc = "100:5"]
    pub const VV_CC_TO: VimVarIndex = 16;
    #[c2rust::src_loc = "99:5"]
    pub const VV_CC_FROM: VimVarIndex = 15;
    #[c2rust::src_loc = "98:5"]
    pub const VV_CTYPE: VimVarIndex = 14;
    #[c2rust::src_loc = "97:5"]
    pub const VV_LC_TIME: VimVarIndex = 13;
    #[c2rust::src_loc = "96:5"]
    pub const VV_LANG: VimVarIndex = 12;
    #[c2rust::src_loc = "95:5"]
    pub const VV_FNAME: VimVarIndex = 11;
    #[c2rust::src_loc = "94:5"]
    pub const VV_TERMRESPONSE: VimVarIndex = 10;
    #[c2rust::src_loc = "93:5"]
    pub const VV_LNUM: VimVarIndex = 9;
    #[c2rust::src_loc = "92:5"]
    pub const VV_VERSION: VimVarIndex = 8;
    #[c2rust::src_loc = "91:5"]
    pub const VV_THIS_SESSION: VimVarIndex = 7;
    #[c2rust::src_loc = "90:5"]
    pub const VV_SHELL_ERROR: VimVarIndex = 6;
    #[c2rust::src_loc = "89:5"]
    pub const VV_STATUSMSG: VimVarIndex = 5;
    #[c2rust::src_loc = "88:5"]
    pub const VV_WARNINGMSG: VimVarIndex = 4;
    #[c2rust::src_loc = "87:5"]
    pub const VV_ERRMSG: VimVarIndex = 3;
    #[c2rust::src_loc = "86:5"]
    pub const VV_PREVCOUNT: VimVarIndex = 2;
    #[c2rust::src_loc = "85:5"]
    pub const VV_COUNT1: VimVarIndex = 1;
    #[c2rust::src_loc = "84:5"]
    pub const VV_COUNT: VimVarIndex = 0;
    // NVIM_EVAL_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/rbuffer.h:21"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/stream.h:21"]
pub mod stream_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:8"]
    pub struct stream {
        pub closed: bool,
        pub did_eof: bool,
        pub uv: C2RustUnnamed_26,
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
    pub union C2RustUnnamed_26 {
        pub pipe: uv_pipe_t,
        pub tcp: uv_tcp_t,
        pub idle: uv_idle_t,
    }
    use super::uv_h::{uv_stream_t, uv_pipe_t, uv_tcp_t, uv_idle_t};
    use super::unix_h::{uv_buf_t, uv_file};
    use super::rbuffer_h::RBuffer;
    use super::stddef_h::size_t;
    use super::multiqueue_h::MultiQueue;
    extern "C" {
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
        #[no_mangle]
        #[c2rust::src_loc = "60:1"]
        pub fn stream_set_blocking(fd: libc::c_int, blocking: bool)
         -> libc::c_int;
    }
    // NVIM_EVENT_STREAM_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/process.h:21"]
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
    #[c2rust::src_loc = "59:1"]
    pub unsafe extern "C" fn process_is_stopped(mut proc_0: *mut Process)
     -> bool {
        let mut exited = (*proc_0).status >= 0 as libc::c_int;
        return exited as libc::c_int != 0 ||
                   (*proc_0).stopped_time !=
                       0 as libc::c_int as libc::c_ulong;
    }
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
                                                      C2RustUnnamed_26{pipe:
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
                                                                                         C2RustUnnamed_21{fd:
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
                                                      C2RustUnnamed_26{pipe:
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
                                                                                         C2RustUnnamed_21{fd:
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
                                                      C2RustUnnamed_26{pipe:
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
                                                                                         C2RustUnnamed_21{fd:
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
    use super::loop_h::Loop;
    use super::stdint_uintn_h::{uint8_t, uint64_t};
    use super::stream_h::{Stream, C2RustUnnamed_26, stream_read_cb,
                          stream_write_cb, stream_close_cb};
    use super::multiqueue_h::MultiQueue;
    use super::stddef_h::{NULL_1, size_t};
    use super::uv_h::uv_stream_t;
    use super::unix_h::{uv_buf_t, uv_file};
    use super::rbuffer_h::RBuffer;
    use super::stdbool_h::false_0;
    // NVIM_EVENT_PROCESS_H
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/ioctl-types.h:21"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/os/pty_process_unix.h:21"]
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
                                                  C2RustUnnamed_26{pipe:
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
                                                                                     C2RustUnnamed_21{fd:
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
                                                  C2RustUnnamed_26{pipe:
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
                                                                                     C2RustUnnamed_21{fd:
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
                                                  C2RustUnnamed_26{pipe:
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
                                                                                     C2RustUnnamed_21{fd:
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
    use super::process_h::{Process, process_init, kProcessTypePty,
                           ProcessType};
    use super::stdint_uintn_h::uint16_t;
    use super::ioctl_types_h::winsize;
    use super::loop_h::Loop;
    use super::stddef_h::NULL_1;
    // NVIM_OS_PTY_PROCESS_UNIX_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/event/libuv_process.h:21"]
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
                                                       C2RustUnnamed_24{fd:
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
                                                                C2RustUnnamed_25{stream:
                                                                                     0
                                                                                         as
                                                                                         *mut uv_stream_t,},};
                                          3],};
                init
            };
        return rv;
    }
    use super::process_h::{Process, process_init, kProcessTypeUv, ProcessType,
                           process_exit_cb, internal_process_cb};
    use super::uv_h::{uv_process_t, uv_process_options_t,
                      uv_stdio_container_t, uv_loop_t, uv_handle_type,
                      uv_close_cb, C2RustUnnamed_24, uv_handle_t, uv_exit_cb,
                      UV_UNKNOWN_HANDLE, uv_stdio_flags, C2RustUnnamed_25,
                      UV_IGNORE, uv_stream_t, uv_pipe_t, C2RustUnnamed_21,
                      uv_alloc_cb, uv_read_cb, uv_connect_t, uv_shutdown_t,
                      uv_connection_cb};
    use super::loop_h::Loop;
    use super::unix_h::{uv_uid_t, uv_gid_t, uv_buf_t, uv_file, uv__io_t,
                        uv__io_cb};
    use super::stdint_uintn_h::{uint8_t, uint64_t};
    use super::stream_h::{Stream, C2RustUnnamed_26, stream_read_cb,
                          stream_write_cb, stream_close_cb};
    use super::multiqueue_h::MultiQueue;
    use super::rbuffer_h::RBuffer;
    use super::stddef_h::size_t;
    // NVIM_EVENT_LIBUV_PROCESS_H
}
#[c2rust::header_src =
  "/home/vole/neovim/.deps/usr/include/msgpack/zone.h:21"]
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
    #[c2rust::src_loc = "151:1"]
    pub unsafe extern "C" fn msgpack_zone_swap(mut a: *mut msgpack_zone,
                                               mut b: *mut msgpack_zone) {
        let mut tmp = *a;
        *a = *b;
        *b = tmp;
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
    #[c2rust::src_loc = "84:9"]
    pub const MSGPACK_ZONE_ALIGN: libc::c_ulong =
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong;
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
    use super::stddef_h::{size_t, NULL_1};
    use super::stdbool_h::true_0;
    extern "C" {
        #[c2rust::src_loc = "37:8"]
        pub type msgpack_zone_chunk;
        #[no_mangle]
        #[c2rust::src_loc = "130:1"]
        pub fn msgpack_zone_push_finalizer_expand(zone: *mut msgpack_zone,
                                                  func:
                                                      Option<unsafe extern "C" fn(_:
                                                                                      *mut libc::c_void)
                                                                 -> ()>,
                                                  data: *mut libc::c_void)
         -> bool;
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn msgpack_zone_malloc_expand(zone: *mut msgpack_zone,
                                          size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn msgpack_zone_free(zone: *mut msgpack_zone);
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/.deps/usr/include/msgpack/object.h:21"]
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
  "/home/vole/neovim/.deps/usr/include/msgpack/pack.h:21"]
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
  "/home/vole/neovim/.deps/usr/include/msgpack/pack_template.h:21"]
pub mod pack_template_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "654:5"]
    pub union C2RustUnnamed_27 {
        pub f: libc::c_float,
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "663:5"]
    pub union C2RustUnnamed_28 {
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
        let mut mem = C2RustUnnamed_27{f: 0.,};
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
        let mut mem = C2RustUnnamed_28{f: 0.,};
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
  "/home/vole/neovim/.deps/usr/include/msgpack/unpack.h:21"]
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
  "/home/vole/neovim/.deps/usr/include/msgpack/sbuffer.h:21"]
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
  "/home/vole/neovim/.deps/usr/include/msgpack/vrefbuffer.h:21"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/channel.h:21"]
pub mod channel_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:8"]
    pub struct Channel {
        pub id: uint64_t,
        pub refcount: size_t,
        pub events: *mut MultiQueue,
        pub streamtype: ChannelStreamType,
        pub stream: C2RustUnnamed_30,
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
    pub union C2RustUnnamed_30 {
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
    use super::typval_h::{Callback, dict_T, C2RustUnnamed_0, CallbackType,
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
  "/home/vole/neovim/src/nvim/msgpack_rpc/channel_defs.h:21"]
pub mod channel_defs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:9"]
    pub struct RpcState {
        pub subscribed_events: *mut Map_cstr_t_ptr_t,
        pub closed: bool,
        pub unpacker: *mut msgpack_unpacker,
        pub next_request_id: uint32_t,
        pub call_stack: C2RustUnnamed_29,
        pub info: Dictionary,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:3"]
    pub struct C2RustUnnamed_29 {
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/os/fileio.h:21"]
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
#[c2rust::header_src =
  "/usr/lib/llvm-6.0/lib/clang/6.0.1/include/stdbool.h:10"]
pub mod stdbool_h {
    #[c2rust::src_loc = "32:9"]
    pub const true_0: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "33:9"]
    pub const false_0: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/string.h:11"]
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
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:14"]
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
        #[no_mangle]
        #[c2rust::src_loc = "614:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/home/vole/neovim/build/include/log.h.generated.h:14"]
pub mod log_h_generated_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "7:1"]
        pub fn logmsg(log_level: libc::c_int, context: *const libc::c_char,
                      func_name: *const libc::c_char, line_num: libc::c_int,
                      eol: bool, fmt: *const libc::c_char, _: ...) -> bool;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/log.h:14"]
pub mod log_h {
    #[c2rust::src_loc = "11:9"]
    pub const WARN_LOG_LEVEL: libc::c_int = 2 as libc::c_int;
    // NVIM_LOG_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/mbyte.h:14"]
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
    use super::stdint_uintn_h::uint8_t;
    use super::mbyte_h_generated_h::{mb_stricmp, utfc_ptr2len};
    use super::string_h::strcmp;
    use super::nvim_types_h::char_u;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:22"]
        pub static utf8len_tab: [uint8_t; 256];
    }
    // NVIM_MBYTE_H
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/mbyte.h.generated.h:14"]
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
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:14"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/message.h.generated.h:14"]
pub mod message_h_generated_h {
    use super::nvim_types_h::char_u;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "7:1"]
        pub fn msg(s: *mut char_u) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "9:1"]
        pub fn msg_attr(s: *const libc::c_char, attr: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "14:1"]
        pub fn smsg(s: *mut libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "15:1"]
        pub fn smsg_attr(attr: libc::c_int, s: *mut libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "18:1"]
        pub fn msg_source(attr: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "20:1"]
        pub fn emsg(s: *const char_u) -> bool;
        #[no_mangle]
        #[c2rust::src_loc = "22:1"]
        pub fn emsgf(fmt: *const libc::c_char, _: ...) -> bool;
        #[no_mangle]
        #[c2rust::src_loc = "34:1"]
        pub fn set_keep_msg(s: *mut char_u, attr: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "38:1"]
        pub fn msg_putchar(c: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn msg_puts(s: *const libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn verbose_enter();
        #[no_mangle]
        #[c2rust::src_loc = "90:1"]
        pub fn verbose_leave();
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/eval/typval.h.generated.h:14"]
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
        #[c2rust::src_loc = "54:1"]
        pub fn tv_dict_find(d: *const dict_T, key: *const libc::c_char,
                            len: ptrdiff_t) -> *mut dictitem_T;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/strings.h:14"]
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
  "/home/vole/neovim/build/include/strings.h.generated.h:14"]
pub mod strings_h_generated_h {
    use super::nvim_types_h::char_u;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "15:1"]
        pub fn vim_strchr(string: *const char_u, c: libc::c_int)
         -> *mut char_u;
        #[no_mangle]
        #[c2rust::src_loc = "21:1"]
        pub fn vim_snprintf(str: *mut libc::c_char, str_m: size_t,
                            fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/memory.h.generated.h:14"]
pub mod memory_h_generated_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "7:1"]
        pub fn xmalloc(size: size_t) -> *mut libc::c_void;
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
        #[no_mangle]
        #[c2rust::src_loc = "22:1"]
        pub fn xstrlcat(dst: *mut libc::c_char, src: *const libc::c_char,
                        dsize: size_t) -> size_t;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:14"]
pub mod fcntl_h {
    #[c2rust::src_loc = "139:10"]
    pub const SEEK_END: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "137:10"]
    pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/macros.h:14"]
pub mod macros_h {
    // EXTERN is only defined in main.c. That's where global variables are
// actually defined and initialized.
    // / String with length
// /
// / For use in functions which accept (char *s, size_t len) pair in arguments.
// /
// / @param[in]  s  Static string.
// /
// / @return `s, sizeof(s) - 1`
    // / LINEEMPTY() - return TRUE if the line is empty
    // / BUFEMPTY() - return TRUE if the current buffer is empty
    /*
 * toupper() and tolower() that use the current locale.
 * Careful: Only call TOUPPER_LOC() and TOLOWER_LOC() with a character in the
 * range 0 - 255.  toupper()/tolower() on some systems can't handle others.
 * Note: It is often better to use mb_tolower() and mb_toupper(), because many
 * toupper() and tolower() implementations only work for ASCII.
 */
    /* toupper() and tolower() for ASCII only and ignore the current locale. */
    /* Like isalpha() but reject non-ASCII characters.  Can't be used with a
 * special key (negative value). */
    /* Returns empty string if it is NULL. */
    /*
 * Adjust chars in a language according to 'langmap' option.
 * NOTE that there is no noticeable overhead if 'langmap' is not set.
 * When set the overhead for characters < 256 is small.
 * Don't apply 'langmap' if the character comes from the Stuff buffer or from a
 * mapping and the langnoremap option was set.
 * The do-while is just to ignore a ';' after the macro.
 */
    /* no CR-LF translation */
    #[c2rust::src_loc = "87:9"]
    pub const READBIN: [libc::c_char; 3] =
        unsafe {
            *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"rb\x00")
        };
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
    // / Calculate the length of a C array
// /
// / This should be called with a real array. Calling this with a pointer is an
// / error. A mechanism to detect many (though not all) of those errors at
// / compile time is implemented. It works by the second division producing
// / a division by zero in those cases (-Wdiv-by-zero in GCC).
    // get length of multi-byte char, not including composing chars
    // Backup multi-byte pointer. Only use with "p" > "s" !
    // Advance multi-byte pointer, do not skip over composing chars.
    // Advance multi-byte pointer, skip over composing chars.
    // MB_PTR_ADV(): advance a pointer to the next character, taking care of
// multi-byte characters if needed.
// MB_PTR_BACK(): backup a pointer to the previous character, taking care of
// multi-byte characters if needed.
// MB_COPY_CHAR(f, t): copy one char from "f" to "t" and advance the pointers.
// PTR2CHAR(): get character from pointer.
    /* Whether to draw the vertical bar on the right side of the cell. */
    /* open in rw------- mode */
    /* mch_open_rw(): invoke os_open() with third argument for user R/W. */
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/globals.h:14"]
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
    #[c2rust::src_loc = "256:9"]
    pub const PROF_YES: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "19:10"]
    pub const MSG_BUF_LEN: libc::c_int = 480 as libc::c_int;
    use super::nvim_types_h::char_u;
    use super::buffer_defs_h::{win_T, tabpage_T, buf_T};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "125:12"]
        pub static mut mod_mask: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "137:12"]
        pub static mut cmdline_row: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "183:12"]
        pub static mut msg_col: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "184:12"]
        pub static mut msg_row: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "195:16"]
        pub static mut keep_msg: *mut char_u;
        #[no_mangle]
        #[c2rust::src_loc = "197:12"]
        pub static mut keep_msg_more: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "200:12"]
        pub static mut msg_didout: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "201:12"]
        pub static mut msg_didany: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "218:13"]
        pub static mut called_vim_beep: bool;
        #[no_mangle]
        #[c2rust::src_loc = "226:12"]
        pub static mut no_wait_return: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "227:12"]
        pub static mut need_wait_return: libc::c_int;
        // /< profiling busy
        // /< profiling paused
        #[no_mangle]
        #[c2rust::src_loc = "258:12"]
        pub static mut do_profiling: libc::c_int;
        // did check timestamps
                                                 // recently
        #[no_mangle]
        #[c2rust::src_loc = "366:12"]
        pub static mut no_check_timestamps: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "389:12"]
        pub static mut mouse_row: libc::c_int;
        // All windows are linked in a list. firstwin points to the first entry,
// lastwin to the last entry (can be the same as firstwin) and curwin to the
// currently active window.
        #[no_mangle]
        #[c2rust::src_loc = "409:18"]
        pub static mut firstwin: *mut win_T;
        // previous window
        // NOLINT
        // When using this macro "break" only breaks out of the inner loop. Use "goto"
// to break out of the tabpage loop.
        // -V:FOR_ALL_WINDOWS_IN_TAB:501
        #[no_mangle]
        #[c2rust::src_loc = "427:18"]
        pub static mut curwin: *mut win_T;
        #[no_mangle]
        #[c2rust::src_loc = "440:22"]
        pub static mut curtab: *mut tabpage_T;
        // need to redraw tabline
        // Iterates over all tabs in the tab list
        // All buffers are linked in a list. 'firstbuf' points to the first entry,
// 'lastbuf' to the last entry and 'curbuf' to the currently active buffer.
        #[no_mangle]
        #[c2rust::src_loc = "448:18"]
        pub static mut firstbuf: *mut buf_T;
        // last buffer
        #[no_mangle]
        #[c2rust::src_loc = "450:18"]
        pub static mut curbuf: *mut buf_T;
        // / "State" is the main state of Vim.
// / There are other variables that modify the state:
// /    Visual_mode:    When State is NORMAL or INSERT.
// /    finish_op  :    When State is NORMAL, after typing the operator and
// /                    before typing the motion command.
// /    motion_force:   Last motion_force from do_pending_operator()
// /    debug_mode:     Debug mode
        #[no_mangle]
        #[c2rust::src_loc = "614:12"]
        pub static mut State: libc::c_int;
        // motion force for pending operator
        // Ex Mode (Q) state
        #[no_mangle]
        #[c2rust::src_loc = "622:12"]
        pub static mut exmode_active: libc::c_int;
        // register being executed or zero
        #[no_mangle]
        #[c2rust::src_loc = "628:12"]
        pub static mut no_mapping: libc::c_int;
        // true when no abbreviations loaded
        #[no_mangle]
        #[c2rust::src_loc = "650:12"]
        pub static mut mapped_ctrl_c: libc::c_int;
        // Ex command modifiers
        #[no_mangle]
        #[c2rust::src_loc = "654:12"]
        pub static mut msg_silent: libc::c_int;
        // don't print messages
        #[no_mangle]
        #[c2rust::src_loc = "655:12"]
        pub static mut emsg_silent: libc::c_int;
        // Selected "quit" at the dialog.
        #[no_mangle]
        #[c2rust::src_loc = "671:15"]
        pub static mut IObuff: [char_u; 1025];
        // /< Buffer for expanding file names
        #[no_mangle]
        #[c2rust::src_loc = "673:15"]
        pub static mut msg_buf: [char_u; 480];
        // for ":stopinsert" and 'insertmode'
        #[no_mangle]
        #[c2rust::src_loc = "695:13"]
        pub static mut KeyTyped: bool;
        // tick for each non-mapped char
        #[no_mangle]
        #[c2rust::src_loc = "699:12"]
        pub static mut must_redraw: libc::c_int;
        // type of redraw necessary
        #[no_mangle]
        #[c2rust::src_loc = "700:13"]
        pub static mut skip_redraw: bool;
        // skip redraw once
        #[no_mangle]
        #[c2rust::src_loc = "701:13"]
        pub static mut do_redraw: bool;
        // /< Stream to write script to.
        // volatile because it is used in a signal handler.
        #[no_mangle]
        #[c2rust::src_loc = "710:21"]
        pub static mut got_int: libc::c_int;
        // set after swap write error msg
        #[no_mangle]
        #[c2rust::src_loc = "726:12"]
        pub static mut global_busy: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "919:15"]
        pub static mut e_notmp: [char_u; 0];
        #[no_mangle]
        #[c2rust::src_loc = "920:15"]
        pub static mut e_notopen: [char_u; 0];
        #[no_mangle]
        #[c2rust::src_loc = "922:15"]
        pub static mut e_notread: [char_u; 0];
        #[no_mangle]
        #[c2rust::src_loc = "953:15"]
        pub static mut e_shellempty: [char_u; 0];
    }
    // NVIM_GLOBALS_H
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/vim.h:14"]
pub mod vim_h {
    #[c2rust::src_loc = "66:9"]
    pub const ASKMORE: libc::c_int = 0x300 as libc::c_int;
    #[c2rust::src_loc = "71:9"]
    pub const CONFIRM: libc::c_int = 0x800 as libc::c_int;
    #[c2rust::src_loc = "180:10"]
    pub const TRUE: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "179:10"]
    pub const FALSE: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "49:9"]
    pub const NORMAL: libc::c_int = 0x1 as libc::c_int;
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/lib/kvec.h:14"]
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
#[c2rust::header_src = "/usr/include/netinet/in.h:14"]
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
  "/usr/include/x86_64-linux-gnu/bits/uintn-identity.h:14"]
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/byteswap.h:14"]
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
#[c2rust::header_src = "/home/vole/neovim/src/nvim/ascii.h:15"]
pub mod ascii_h {
    // Definitions of various common control characters.
    #[c2rust::src_loc = "19:9"]
    pub const NUL: libc::c_int = '\u{0}' as i32;
    #[c2rust::src_loc = "22:9"]
    pub const TAB: libc::c_int = '\t' as i32;
    #[c2rust::src_loc = "23:9"]
    pub const NL: libc::c_int = '\n' as i32;
    #[c2rust::src_loc = "26:9"]
    pub const CAR: libc::c_int = '\r' as i32;
    /* CR is used by Mac OS X */
    #[c2rust::src_loc = "27:9"]
    pub const ESC: libc::c_int = '\u{1b}' as i32;
    // Control Sequence Introducer
    /* Device Control String */
    /* String Terminator */
    /* '?' -> DEL, '@' -> ^@, etc. */
    /* @ */
    #[c2rust::src_loc = "48:9"]
    pub const Ctrl_C: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "53:9"]
    pub const Ctrl_H: libc::c_int = 8 as libc::c_int;
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
#[c2rust::header_src =
  "/home/vole/neovim/build/include/os/shell.h.generated.h:16"]
pub mod shell_h_generated_h {
    use super::nvim_types_h::char_u;
    use super::shell_h::ShellOpts;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "8:1"]
        pub fn os_call_shell(cmd: *mut char_u, opts: ShellOpts,
                             extra_args: *mut char_u) -> libc::c_int;
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
    use super::buffer_defs_h::win_T;
    use super::nvim_types_h::char_u;
    use super::pos_h::colnr_T;
    use super::option_defs_h::breakat_flags;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn win_linetabsize(wp: *mut win_T, line: *mut char_u,
                               len: colnr_T) -> libc::c_uint;
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn win_lbr_chartabsize(wp: *mut win_T, line: *mut char_u,
                                   s: *mut char_u, col: colnr_T,
                                   headp: *mut libc::c_int) -> libc::c_int;
    }
    // NVIM_CHARSET_H
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/diff.h.generated.h:19"]
pub mod diff_h_generated_h {
    use super::buffer_defs_h::win_T;
    use super::pos_h::linenr_T;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "19:1"]
        pub fn diff_check_fill(wp: *mut win_T, lnum: linenr_T) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/eval.h.generated.h:21"]
pub mod eval_h_generated_h {
    use super::profile_h::proftime_T;
    use super::eval_h::{VimVarIndex, VV_COUNT};
    use super::typval_h::varnumber_T;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn prof_child_enter(tm: *mut proftime_T);
        #[no_mangle]
        #[c2rust::src_loc = "38:1"]
        pub fn prof_child_exit(tm: *mut proftime_T);
        #[no_mangle]
        #[c2rust::src_loc = "124:1"]
        pub fn set_vim_var_nr(idx: VimVarIndex, val: varnumber_T);
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/main.h.generated.h:21"]
pub mod main_h_generated_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "9:1"]
        pub fn getout(exitval: libc::c_int) -> !;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/ex_cmds.h.generated.h:22"]
pub mod ex_cmds_h_generated_h {
    use super::nvim_types_h::char_u;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "12:1"]
        pub fn make_filter_cmd(cmd: *mut char_u, itmp: *mut char_u,
                               otmp: *mut char_u) -> *mut char_u;
        #[no_mangle]
        #[c2rust::src_loc = "30:1"]
        pub fn check_restricted() -> bool;
        #[no_mangle]
        #[c2rust::src_loc = "31:1"]
        pub fn check_secure() -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/fileio.h.generated.h:25"]
pub mod fileio_h_generated_h {
    use super::nvim_types_h::char_u;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "31:1"]
        pub fn vim_tempname() -> *mut char_u;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/os/fs.h.generated.h:25"]
pub mod fs_h_generated_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn os_remove(path: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "14:1"]
        pub fn os_fopen(path: *const libc::c_char, flags: *const libc::c_char)
         -> *mut FILE;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/os/users.h.generated.h:25"]
pub mod users_h_generated_h {
    use super::garray_h::garray_T;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "4:1"]
        pub fn os_get_usernames(users: *mut garray_T) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/fold.h.generated.h:27"]
pub mod fold_h_generated_h {
    use super::buffer_defs_h::win_T;
    use super::pos_h::linenr_T;
    use super::fold_h::foldinfo_T;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "7:1"]
        pub fn hasFoldingWin(win: *mut win_T, lnum: linenr_T,
                             firstp: *mut linenr_T, lastp: *mut linenr_T,
                             cache: bool, infop: *mut foldinfo_T) -> bool;
        #[no_mangle]
        #[c2rust::src_loc = "9:1"]
        pub fn lineFolded(win: *mut win_T, lnum: linenr_T) -> bool;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/getchar.h.generated.h:28"]
pub mod getchar_h_generated_h {
    use super::getchar_h::{flush_buffers_T, FLUSH_MINIMAL};
    use super::nvim_types_h::char_u;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "7:1"]
        pub fn stuff_empty() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "10:1"]
        pub fn flush_buffers(flush_typeahead: flush_buffers_T);
        #[no_mangle]
        #[c2rust::src_loc = "23:1"]
        pub fn stuffcharReadbuff(c: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "31:1"]
        pub fn typebuf_typed() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn safe_vgetc() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "45:1"]
        pub fn plain_vgetc() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "51:1"]
        pub fn fix_input_buffer(buf: *mut char_u, len: libc::c_int)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/memline.h.generated.h:34"]
pub mod memline_h_generated_h {
    use super::pos_h::{pos_T, linenr_T};
    use super::nvim_types_h::char_u;
    use super::buffer_defs_h::buf_T;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "10:1"]
        pub fn ml_close_all(del_file: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "11:1"]
        pub fn ml_close_notmod();
        #[no_mangle]
        #[c2rust::src_loc = "17:1"]
        pub fn ml_sync_all(check_file: libc::c_int, check_char: libc::c_int,
                           do_fsync: bool);
        #[no_mangle]
        #[c2rust::src_loc = "20:1"]
        pub fn ml_get_pos(pos: *const pos_T) -> *mut char_u;
        #[no_mangle]
        #[c2rust::src_loc = "21:1"]
        pub fn ml_get_buf(buf: *mut buf_T, lnum: linenr_T, will_change: bool)
         -> *mut char_u;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/move.h.generated.h:38"]
pub mod move_h_generated_h {
    use super::buffer_defs_h::win_T;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "27:1"]
        pub fn win_col_off(wp: *mut win_T) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "29:1"]
        pub fn win_col_off2(wp: *mut win_T) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/mouse.h.generated.h:39"]
pub mod mouse_h_generated_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "7:1"]
        pub fn setmouse();
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/option.h.generated.h:40"]
pub mod option_h_generated_h {
    use super::nvim_types_h::char_u;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn copy_option_part(option: *mut *mut char_u, buf: *mut char_u,
                                maxlen: size_t, sep_chars: *mut libc::c_char)
         -> size_t;
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/screen.h.generated.h:44"]
pub mod screen_h_generated_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "62:1"]
        pub fn messaging() -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/screen.h:44"]
pub mod screen_h {
    /*
 * flags for update_screen()
 * The higher the value, the higher the priority
 */
    #[c2rust::src_loc = "15:9"]
    pub const VALID: libc::c_int = 10 as libc::c_int;
    // NVIM_SCREEN_H
}
#[c2rust::header_src = "/home/vole/neovim/build/include/tag.h.generated.h:48"]
pub mod tag_h_generated_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "5:1"]
        pub fn tag_freematch();
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/ui_events_call.h.generated.h:49"]
pub mod ui_events_call_h_generated_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "11:1"]
        pub fn ui_call_bell();
        #[no_mangle]
        #[c2rust::src_loc = "12:1"]
        pub fn ui_call_visual_bell();
    }
}
#[c2rust::header_src = "/home/vole/neovim/build/include/ui.h.generated.h:49"]
pub mod ui_h_generated_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "26:1"]
        pub fn ui_flush();
        #[no_mangle]
        #[c2rust::src_loc = "20:1"]
        pub fn ui_cursor_goto(new_row: libc::c_int, new_col: libc::c_int);
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/os/signal.h.generated.h:54"]
pub mod signal_h_generated_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "7:1"]
        pub fn signal_reject_deadly();
    }
}
#[c2rust::header_src =
  "/home/vole/neovim/build/include/os/input.h.generated.h:55"]
pub mod input_h_generated_h {
    use super::stdint_uintn_h::uint8_t;
    use super::multiqueue_h::MultiQueue;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "6:1"]
        pub fn input_global_fd() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "9:1"]
        pub fn os_inchar(buf: *mut uint8_t, maxlen: libc::c_int,
                         ms: libc::c_int, tb_change_cnt: libc::c_int,
                         events: *mut MultiQueue) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "11:1"]
        pub fn os_breakcheck();
    }
}
#[c2rust::header_src = "/home/vole/neovim/src/nvim/buffer.h:58"]
pub mod buffer_h {
    // for linenr_T
    // for exarg_T
    // for StlClickRecord
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
        // For some reason formatc does not like the below.
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
    // / Get b:changedtick value
// /
// / Faster then querying b:.
// /
// / @param[in]  buf  Buffer to get b:changedtick from.
    #[inline(always)]
    #[c2rust::src_loc = "110:1"]
    pub unsafe extern "C" fn buf_get_changedtick(buf: *const buf_T)
     -> varnumber_T {
        return (*buf).changedtick_di.di_tv.vval.v_number;
    }
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
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::types_h::{__int8_t, __uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __int64_t, __uint64_t, __uid_t, __gid_t,
                        __off_t, __off64_t, __time_t, __ssize_t};
pub use self::stdint_intn_h::{int8_t, int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
pub use self::stddef_h::{size_t, ptrdiff_t, NULL_1, NULL, NULL_0};
pub use self::nvim_types_h::{char_u, handle_T, LuaRef, expand_T};
pub use self::ex_cmds_defs_h::expand;
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
                         kCallbackNone, Callback, C2RustUnnamed_0,
                         dict_watcher, DictWatcher, ScopeDictDictItem,
                         dictitem_T, C2RustUnnamed_1, DI_FLAGS_ALLOC,
                         DI_FLAGS_LOCK, DI_FLAGS_FIX, DI_FLAGS_RO_SBX,
                         DI_FLAGS_RO, tv_list_ref, tv_list_set_ret,
                         tv_list_locked, tv_list_set_lock, tv_list_set_copyid,
                         tv_list_len, tv_list_copyid, tv_list_latest_copy,
                         tv_list_uidx, tv_list_has_watchers, tv_list_first,
                         tv_list_last, tv_dict_set_ret, tv_dict_len,
                         tv_dict_is_watched, tv_init, tv_get_float_chk,
                         tv_dict_watcher_node_data, tv_is_func, funccall_S};
pub use self::pos_h::{linenr_T, colnr_T, C2RustUnnamed, MAXCOL, pos_T,
                      lpos_T};
pub use self::stdarg_h::va_list;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdio_h::{ssize_t, stderr, fclose, fprintf, fread, fseek,
                        ftell};
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
pub use self::keymap_h::{key_extra, KE_COMMAND, KE_EVENT, KE_NOP, KE_DROP,
                         KE_X2RELEASE, KE_X2DRAG, KE_X2MOUSE, KE_X1RELEASE,
                         KE_X1DRAG, KE_X1MOUSE, KE_C_END, KE_C_HOME,
                         KE_C_RIGHT, KE_C_LEFT, KE_CMDWIN, KE_PLUG, KE_SNR,
                         KE_CSI, KE_KDEL, KE_KINS, KE_MOUSERIGHT,
                         KE_MOUSELEFT, KE_MOUSEUP, KE_MOUSEDOWN, KE_S_XF4,
                         KE_S_XF3, KE_S_XF2, KE_S_XF1, KE_LEFTRELEASE_NM,
                         KE_LEFTMOUSE_NM, KE_XRIGHT, KE_XLEFT, KE_XDOWN,
                         KE_XUP, KE_ZHOME, KE_XHOME, KE_ZEND, KE_XEND, KE_XF4,
                         KE_XF3, KE_XF2, KE_XF1, KE_S_TAB_OLD, KE_TAB,
                         KE_IGNORE, KE_RIGHTRELEASE, KE_RIGHTDRAG,
                         KE_RIGHTMOUSE, KE_MIDDLERELEASE, KE_MIDDLEDRAG,
                         KE_MIDDLEMOUSE, KE_LEFTRELEASE, KE_LEFTDRAG,
                         KE_LEFTMOUSE, KE_MOUSE, KE_S_F37, KE_S_F36, KE_S_F35,
                         KE_S_F34, KE_S_F33, KE_S_F32, KE_S_F31, KE_S_F30,
                         KE_S_F29, KE_S_F28, KE_S_F27, KE_S_F26, KE_S_F25,
                         KE_S_F24, KE_S_F23, KE_S_F22, KE_S_F21, KE_S_F20,
                         KE_S_F19, KE_S_F18, KE_S_F17, KE_S_F16, KE_S_F15,
                         KE_S_F14, KE_S_F13, KE_S_F12, KE_S_F11, KE_S_F10,
                         KE_S_F9, KE_S_F8, KE_S_F7, KE_S_F6, KE_S_F5, KE_S_F4,
                         KE_S_F3, KE_S_F2, KE_S_F1, KE_S_DOWN, KE_S_UP,
                         KE_NAME, K_DEL, K_KDEL, K_BS, KS_SPECIAL, K_SPECIAL,
                         KS_ZERO, K_ZERO, K_IGNORE, K_LEFTMOUSE_NM,
                         K_LEFTDRAG, K_LEFTRELEASE, K_LEFTRELEASE_NM,
                         K_MIDDLEMOUSE, K_MIDDLEDRAG, K_MIDDLERELEASE,
                         K_RIGHTMOUSE, K_RIGHTDRAG, K_RIGHTRELEASE,
                         K_MOUSEDOWN, K_MOUSEUP, K_MOUSELEFT, K_MOUSERIGHT,
                         K_X1MOUSE, K_X1DRAG, K_X1RELEASE, K_X2MOUSE,
                         K_X2DRAG, K_X2RELEASE, K_LEFTMOUSE, KS_MODIFIER};
pub use self::buffer_defs_h::{file_buffer, C2RustUnnamed_2,
                              BufUpdateCallbacks, C2RustUnnamed_3, synblock_T,
                              buf_T, win_T, window_S, qf_info_T, FloatConfig,
                              WinStyle, kWinStyleMinimal, kWinStyleUnused,
                              FloatRelative, kFloatRelativeCursor,
                              kFloatRelativeWindow, kFloatRelativeEditor,
                              FloatAnchor, taggy_T, taggy, matchitem_T,
                              matchitem, match_T, posmatch_T, posmatch,
                              llpos_T, winopt_T, alist_T, arglist, wline_T,
                              w_line, pos_save_T, C2RustUnnamed_4,
                              C2RustUnnamed_5, frame_T, frame_S, disptick_T,
                              syn_time_T, mapblock_T, mapblock, wininfo_T,
                              wininfo_S, ChangedtickDictItem, diffblock_S,
                              diff_T, tabpage_S, tabpage_T, win_hl_attr,
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
                       C2RustUnnamed_12, Dictionary, KeyValuePair,
                       key_value_pair, Object, Array, ObjectType,
                       kObjectTypeTabpage, kObjectTypeWindow,
                       kObjectTypeBuffer, kObjectTypeLuaRef,
                       kObjectTypeDictionary, kObjectTypeArray,
                       kObjectTypeString, kObjectTypeFloat,
                       kObjectTypeInteger, kObjectTypeBoolean, kObjectTypeNil,
                       is_internal_call};
pub use self::mark_defs_h::{fmark_T, filemark, xfmark_T, xfilemark};
pub use self::time_h::{Timestamp, os_hrtime};
pub use self::option_defs_h::{LastSet, BO_ALL, BO_ERROR, COM_NEST, COM_MIDDLE,
                              COM_BLANK, COM_NOBACK, COM_END, COM_MAX_LEN,
                              breakat_flags, bo_flags, p_debug, p_im,
                              p_report, p_sh, p_vb, p_verbose};
pub use self::syntax_defs_h::{synstate_T, syn_state, C2RustUnnamed_6,
                              bufstate_T, buf_state};
pub use self::undo_defs_h::{u_header_T, u_header, visualinfo_T, u_entry_T,
                            u_entry, C2RustUnnamed_7, C2RustUnnamed_8,
                            C2RustUnnamed_9, C2RustUnnamed_10};
pub use self::fs_defs_h::FileID;
pub use self::memline_defs_h::{memline_T, memline, chunksize_T, ml_chunksize,
                               infoptr_T, info_pointer};
pub use self::memfile_defs_h::{bhdr_T, bhdr, mf_hashitem_T, mf_hashitem,
                               blocknr_T, memfile_T, memfile, mf_hashtab_T,
                               mf_hashtab};
pub use self::highlight_defs_h::{C2RustUnnamed_11, HLF_COUNT, HLF_MSG,
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
pub use self::uv_h::{uv_loop_s, uv_signal_t, uv_signal_s, C2RustUnnamed_13,
                     uv_signal_cb, uv_handle_t, uv_handle_s, C2RustUnnamed_14,
                     uv_close_cb, uv_handle_type, UV_HANDLE_TYPE_MAX, UV_FILE,
                     UV_SIGNAL, UV_UDP, UV_TTY, UV_TIMER, UV_TCP, UV_STREAM,
                     UV_PROCESS, UV_PREPARE, UV_POLL, UV_NAMED_PIPE, UV_IDLE,
                     UV_HANDLE, UV_FS_POLL, UV_FS_EVENT, UV_CHECK, UV_ASYNC,
                     UV_UNKNOWN_HANDLE, uv_loop_t, C2RustUnnamed_15,
                     C2RustUnnamed_16, uv_async_t, uv_async_s, uv_async_cb,
                     C2RustUnnamed_17, C2RustUnnamed_18, uv_req_type,
                     UV_REQ_TYPE_MAX, UV_GETNAMEINFO, UV_GETADDRINFO, UV_WORK,
                     UV_FS, UV_UDP_SEND, UV_SHUTDOWN, UV_WRITE, UV_CONNECT,
                     UV_REQ, UV_UNKNOWN_REQ, uv_stream_s, uv_connection_cb,
                     uv_stream_t, uv_shutdown_t, uv_shutdown_s,
                     uv_shutdown_cb, uv_connect_t, uv_connect_s,
                     uv_connect_cb, uv_read_cb, uv_alloc_cb, C2RustUnnamed_19,
                     uv_tcp_s, C2RustUnnamed_20, uv_tcp_t, uv_pipe_s,
                     C2RustUnnamed_21, uv_pipe_t, uv_timer_s, uv_timer_cb,
                     uv_timer_t, C2RustUnnamed_22, uv_idle_s, uv_idle_cb,
                     uv_idle_t, C2RustUnnamed_23, uv_process_s, uv_exit_cb,
                     uv_process_t, C2RustUnnamed_24, uv_stdio_flags,
                     UV_OVERLAPPED_PIPE, UV_WRITABLE_PIPE, UV_READABLE_PIPE,
                     UV_INHERIT_STREAM, UV_INHERIT_FD, UV_CREATE_PIPE,
                     UV_IGNORE, uv_stdio_container_s, C2RustUnnamed_25,
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
pub use self::shell_h::{ShellOpts, kShellOptHideMess, kShellOptWrite,
                        kShellOptRead, kShellOptSilent, kShellOptDoOut,
                        kShellOptExpand, kShellOptFilter};
pub use self::fold_h::{foldinfo_T, foldinfo};
pub use self::getchar_h::{flush_buffers_T, FLUSH_INPUT, FLUSH_TYPEAHEAD,
                          FLUSH_MINIMAL};
pub use self::eval_h::{VimVarIndex, VV_LUA, VV_EXITING, VV_ECHOSPACE,
                       VV_TYPE_BOOL, VV_TYPE_FLOAT, VV_TYPE_DICT,
                       VV_TYPE_LIST, VV_TYPE_FUNC, VV_TYPE_STRING,
                       VV_TYPE_NUMBER, VV_TESTING, VV_VIM_DID_ENTER,
                       VV__NULL_DICT, VV__NULL_LIST, VV_NULL, VV_TRUE,
                       VV_FALSE, VV_EVENT, VV_MSGPACK_TYPES, VV_ERRORS,
                       VV_OPTION_TYPE, VV_OPTION_OLD, VV_OPTION_NEW,
                       VV_COMPLETED_ITEM, VV_PROGPATH, VV_WINDOWID,
                       VV_OLDFILES, VV_HLSEARCH, VV_SEARCHFORWARD, VV_OP,
                       VV_MOUSE_COL, VV_MOUSE_LNUM, VV_MOUSE_WINID,
                       VV_MOUSE_WIN, VV_CHAR, VV_SWAPCOMMAND, VV_SWAPCHOICE,
                       VV_SWAPNAME, VV_SCROLLSTART, VV_BEVAL_TEXT,
                       VV_BEVAL_COL, VV_BEVAL_LNUM, VV_BEVAL_WINID,
                       VV_BEVAL_WINNR, VV_BEVAL_BUFNR, VV_FCS_CHOICE,
                       VV_FCS_REASON, VV_PROFILING, VV_KEY, VV_VAL,
                       VV_INSERTMODE, VV_CMDBANG, VV_REG, VV_STDERR,
                       VV_THROWPOINT, VV_EXCEPTION, VV_DYING, VV_SEND_SERVER,
                       VV_PROGNAME, VV_FOLDLEVEL, VV_FOLDDASHES, VV_FOLDEND,
                       VV_FOLDSTART, VV_CMDARG, VV_FNAME_DIFF, VV_FNAME_NEW,
                       VV_FNAME_OUT, VV_FNAME_IN, VV_CC_TO, VV_CC_FROM,
                       VV_CTYPE, VV_LC_TIME, VV_LANG, VV_FNAME,
                       VV_TERMRESPONSE, VV_LNUM, VV_VERSION, VV_THIS_SESSION,
                       VV_SHELL_ERROR, VV_STATUSMSG, VV_WARNINGMSG, VV_ERRMSG,
                       VV_PREVCOUNT, VV_COUNT1, VV_COUNT};
pub use self::rbuffer_h::{rbuffer, rbuffer_callback, RBuffer, rbuffer_size};
pub use self::stream_h::{stream, stream_close_cb, Stream, stream_write_cb,
                         stream_read_cb, C2RustUnnamed_26,
                         stream_set_blocking};
pub use self::process_h::{ProcessType, kProcessTypePty, kProcessTypeUv,
                          process, internal_process_cb, Process,
                          process_exit_cb, process_is_stopped, process_init};
pub use self::ioctl_types_h::winsize;
pub use self::pty_process_unix_h::{pty_process, PtyProcess, pty_process_init};
pub use self::libuv_process_h::{libuv_process, LibuvProcess,
                                libuv_process_init};
pub use self::zone_h::{msgpack_zone_finalizer, msgpack_zone_finalizer_array,
                       msgpack_zone_chunk_list, msgpack_zone,
                       msgpack_zone_swap, msgpack_zone_push_finalizer,
                       msgpack_zone_malloc_no_align, MSGPACK_ZONE_ALIGN,
                       msgpack_zone_malloc, msgpack_zone_chunk,
                       msgpack_zone_push_finalizer_expand,
                       msgpack_zone_malloc_expand, msgpack_zone_free};
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
pub use self::pack_template_h::{C2RustUnnamed_27, C2RustUnnamed_28,
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
pub use self::channel_h::{Channel, CallbackReader, C2RustUnnamed_30,
                          StderrState, StdioPair, ChannelStreamType,
                          kChannelStreamInternal, kChannelStreamStderr,
                          kChannelStreamStdio, kChannelStreamSocket,
                          kChannelStreamProc, callback_reader_set,
                          find_channel, channel_instream, channel_outstream,
                          channels};
pub use self::channel_defs_h::{RpcState, C2RustUnnamed_29, ChannelCallFrame};
pub use self::fileio_h::{FileDescriptor, file_eof, file_fd};
pub use self::assert_h::{__ASSERT_FUNCTION, __assert_fail};
pub use self::stdbool_h::{true_0, false_0};
use self::string_h::{memcpy, memmove, memset, strcmp, strncmp, strlen};
use self::stdlib_h::{malloc, calloc, realloc, free, abort, exit};
use self::log_h_generated_h::logmsg;
pub use self::log_h::WARN_LOG_LEVEL;
pub use self::mbyte_h::{mb_strcmp_ic, mb_ptr2len, utf8len_tab};
use self::mbyte_h_generated_h::{utf_ptr2char, utfc_ptr2len, mb_stricmp};
use self::libintl_h::gettext;
use self::message_h_generated_h::{msg, msg_attr, smsg, smsg_attr, msg_source,
                                  emsg, emsgf, set_keep_msg, msg_putchar,
                                  msg_puts, verbose_enter, verbose_leave};
use self::typval_h_generated_h::{tv_dict_watcher_notify, tv_dict_find};
pub use self::strings_h::strappend;
use self::strings_h_generated_h::{vim_strchr, vim_snprintf};
use self::memory_h_generated_h::{xmalloc, xfree, xcalloc, xrealloc, xstrlcpy,
                                 xstrlcat};
pub use self::fcntl_h::{SEEK_END, SEEK_SET};
pub use self::macros_h::READBIN;
pub use self::globals_h::{PROF_YES, MSG_BUF_LEN, mod_mask, cmdline_row,
                          msg_col, msg_row, keep_msg, keep_msg_more,
                          msg_didout, msg_didany, called_vim_beep,
                          no_wait_return, need_wait_return, do_profiling,
                          no_check_timestamps, mouse_row, firstwin, curwin,
                          curtab, firstbuf, curbuf, State, exmode_active,
                          no_mapping, mapped_ctrl_c, msg_silent, emsg_silent,
                          IObuff, msg_buf, KeyTyped, must_redraw, skip_redraw,
                          do_redraw, got_int, global_busy, e_notmp, e_notopen,
                          e_notread, e_shellempty};
pub use self::vim_h::{ASKMORE, CONFIRM, TRUE, FALSE, NORMAL};
pub use self::kvec_h::_memcpy_free;
use self::in_h::{ntohl, ntohs};
pub use self::uintn_identity_h::{__uint64_identity, __uint32_identity,
                                 __uint16_identity};
pub use self::byteswap_h::{__bswap_64, __bswap_32, __bswap_16};
pub use self::ascii_h::{NUL, TAB, NL, CAR, ESC, Ctrl_C, Ctrl_H, ascii_iswhite,
                        ascii_isdigit, ascii_isxdigit, ascii_isident,
                        ascii_isbdigit, ascii_isspace};
use self::shell_h_generated_h::os_call_shell;
pub use self::charset_h::{vim_isbreak, win_linetabsize, win_lbr_chartabsize};
use self::diff_h_generated_h::diff_check_fill;
use self::eval_h_generated_h::{prof_child_enter, prof_child_exit,
                               set_vim_var_nr};
use self::main_h_generated_h::getout;
use self::ex_cmds_h_generated_h::{make_filter_cmd, check_restricted,
                                  check_secure};
use self::fileio_h_generated_h::vim_tempname;
use self::fs_h_generated_h::{os_remove, os_fopen};
use self::users_h_generated_h::os_get_usernames;
use self::fold_h_generated_h::{hasFoldingWin, lineFolded};
use self::getchar_h_generated_h::{stuff_empty, flush_buffers,
                                  stuffcharReadbuff, typebuf_typed,
                                  safe_vgetc, plain_vgetc, fix_input_buffer};
use self::memline_h_generated_h::{ml_close_all, ml_close_notmod, ml_sync_all,
                                  ml_get_pos, ml_get_buf};
use self::move_h_generated_h::{win_col_off, win_col_off2};
use self::mouse_h_generated_h::setmouse;
use self::option_h_generated_h::copy_option_part;
use self::screen_h_generated_h::messaging;
pub use self::screen_h::VALID;
use self::tag_h_generated_h::tag_freematch;
use self::ui_events_call_h_generated_h::{ui_call_bell, ui_call_visual_bell};
use self::ui_h_generated_h::{ui_flush, ui_cursor_goto};
use self::signal_h_generated_h::signal_reject_deadly;
use self::input_h_generated_h::{input_global_fd, os_inchar, os_breakcheck};
pub use self::buffer_h::{buf_set_changedtick, buf_get_changedtick,
                         buf_inc_changedtick};
// This is an open source non-commercial project. Dear PVS-Studio, please check
// it. PVS-Studio Static Code Analyzer for C, C++ and C#: http://www.viva64.com
/*
 * misc1.c: functions that didn't seem to fit elsewhere
 */
#[no_mangle]
#[c2rust::src_loc = "60:1"]
pub unsafe extern "C" fn exit_free() -> bool { return false_0 != 0; }
// All user names (for ~user completion as done by shell).
#[c2rust::src_loc = "72:17"]
static mut ga_users: garray_T =
    {
        let mut init =
            growarray{ga_len: 0 as libc::c_int,
                      ga_maxlen: 0 as libc::c_int,
                      ga_itemsize: 0 as libc::c_int,
                      ga_growsize: 1 as libc::c_int,
                      ga_data: NULL_1 as *mut libc::c_void,};
        init
    };
/*
 * get_leader_len() returns the length in bytes of the prefix of the given
 * string which introduces a comment.  If this string is not a comment then
 * 0 is returned.
 * When "flags" is not NULL, it is set to point to the flags of the recognized
 * comment leader.
 * "backward" must be true for the "O" command.
 * If "include_space" is set, include trailing whitespace while calculating the
 * length.
 */
#[no_mangle]
#[c2rust::src_loc = "84:1"]
pub unsafe extern "C" fn get_leader_len(mut line: *mut char_u,
                                        mut flags: *mut *mut char_u,
                                        mut backward: libc::c_int,
                                        mut include_space: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0; /* buffer for one option part */
    let mut j: libc::c_int = 0; /* pointer to comment string */
    let mut result: libc::c_int = 0;
    let mut got_com = FALSE;
    let mut found_one: libc::c_int = 0;
    let mut part_buf: [char_u; 50] = [0; 50];
    let mut string = 0 as *mut char_u;
    let mut list = 0 as *mut char_u;
    let mut middle_match_len = 0 as libc::c_int;
    let mut prev_list = 0 as *mut char_u;
    let mut saved_flags = NULL_1 as *mut char_u;
    i = 0 as libc::c_int;
    result = i;
    while ascii_iswhite(*line.offset(i as isize) as libc::c_int) {
        /* leading white space is ignored */
        i += 1
    }
    /*
   * Repeat to match several nested comment strings.
   */
    while *line.offset(i as isize) as libc::c_int != NUL {
        /*
     * scan through the 'comments' option for a match
     */
        found_one = FALSE;
        list = (*curbuf).b_p_com;
        /* missing ':', ignore this part */
        while *list != 0 {
            /* Get one option part into part_buf[].  Advance "list" to next
       * one.  Put "string" at start of string.  */
            if got_com == 0 && !flags.is_null() {
                *flags = list
            } /* remember where flags started */
            prev_list = list; /* isolate flags from string */
            copy_option_part(&mut list, part_buf.as_mut_ptr(),
                             COM_MAX_LEN as size_t,
                             b",\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char);
            string = vim_strchr(part_buf.as_mut_ptr(), ':' as i32);
            if string.is_null() { continue ; }
            let fresh3 = string;
            string = string.offset(1);
            *fresh3 = NUL as char_u;
            /* If we found a middle match previously, use that match when this
       * is not a middle or end. */
            if middle_match_len != 0 as libc::c_int &&
                   vim_strchr(part_buf.as_mut_ptr(), COM_MIDDLE).is_null() &&
                   vim_strchr(part_buf.as_mut_ptr(), COM_END).is_null() {
                break ;
            }
            /* When we already found a nested comment, only accept further
       * nested comments. */
            if got_com != 0 &&
                   vim_strchr(part_buf.as_mut_ptr(), COM_NEST).is_null() {
                continue ;
            }
            /* When 'O' flag present and using "O" command skip this one. */
            if backward != 0 &&
                   !vim_strchr(part_buf.as_mut_ptr(), COM_NOBACK).is_null() {
                continue ;
            }
            /* Line contents and string must match.
       * When string starts with white space, must have some white space
       * (but the amount does not need to match, there might be a mix of
       * TABs and spaces). */
            if ascii_iswhite(*string.offset(0 as libc::c_int as isize) as
                                 libc::c_int) {
                if i == 0 as libc::c_int ||
                       !ascii_iswhite(*line.offset((i - 1 as libc::c_int) as
                                                       isize) as libc::c_int)
                   {
                    continue ; /* missing white space */
                } /* string doesn't match */
                while ascii_iswhite(*string.offset(0 as libc::c_int as isize)
                                        as libc::c_int) {
                    string = string.offset(1)
                }
            }
            j = 0 as libc::c_int;
            while *string.offset(j as isize) as libc::c_int != NUL &&
                      *string.offset(j as isize) as libc::c_int ==
                          *line.offset((i + j) as isize) as libc::c_int {
                j += 1
            }
            if *string.offset(j as isize) as libc::c_int != NUL { continue ; }
            /* When 'b' flag used, there must be white space or an
       * end-of-line after the string in the line. */
            if !vim_strchr(part_buf.as_mut_ptr(), COM_BLANK).is_null() &&
                   !ascii_iswhite(*line.offset((i + j) as isize) as
                                      libc::c_int) &&
                   *line.offset((i + j) as isize) as libc::c_int != NUL {
                continue ;
            }
            /* We have found a match, stop searching unless this is a middle
       * comment. The middle comment can be a substring of the end
       * comment in which case it's better to return the length of the
       * end comment and its flags.  Thus we keep searching with middle
       * and end matches and use an end match if it matches better. */
            if !vim_strchr(part_buf.as_mut_ptr(), COM_MIDDLE).is_null() {
                if middle_match_len == 0 as libc::c_int {
                    middle_match_len = j;
                    saved_flags = prev_list
                }
            } else {
                if middle_match_len != 0 as libc::c_int &&
                       j > middle_match_len {
                    /* Use this match instead of the middle match, since it's a
         * longer thus better match. */
                    middle_match_len = 0 as libc::c_int
                }
                if middle_match_len == 0 as libc::c_int { i += j }
                found_one = TRUE;
                break ;
            }
        }
        if middle_match_len != 0 as libc::c_int {
            /* Use the previously found middle match after failing to find a
       * match with an end. */
            if got_com == 0 && !flags.is_null() { *flags = saved_flags }
            i += middle_match_len;
            found_one = TRUE
        }
        /* No match found, stop scanning. */
        if found_one == 0 { break ; }
        result = i;
        /* Include any trailing white space. */
        while ascii_iswhite(*line.offset(i as isize) as libc::c_int) {
            i += 1
        }
        if include_space != 0 { result = i }
        /* If this comment doesn't nest, stop here. */
        got_com = TRUE;
        if vim_strchr(part_buf.as_mut_ptr(), COM_NEST).is_null() { break ; }
    }
    return result;
}
/*
 * Return the offset at which the last comment in line starts. If there is no
 * comment in the whole line, -1 is returned.
 *
 * When "flags" is not null, it is set to point to the flags describing the
 * recognized comment leader.
 */
#[no_mangle]
#[c2rust::src_loc = "218:1"]
pub unsafe extern "C" fn get_last_leader_offset(mut line: *mut char_u,
                                                mut flags: *mut *mut char_u)
 -> libc::c_int {
    let mut result = -(1 as libc::c_int); /* buffer for one option part */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lower_check_bound = 0 as libc::c_int;
    let mut string = 0 as *mut char_u;
    let mut com_leader = 0 as *mut char_u;
    let mut com_flags = 0 as *mut char_u;
    let mut list = 0 as *mut char_u;
    let mut found_one: libc::c_int = 0;
    let mut part_buf: [char_u; 50] = [0; 50];
    /*
   * Repeat to match several nested comment strings.
   */
    i = strlen(line as *mut libc::c_char) as libc::c_int;
    loop  {
        i -= 1;
        if !(i >= lower_check_bound) { break ; }
        /*
     * scan through the 'comments' option for a match
     */
        found_one = FALSE;
        list = (*curbuf).b_p_com;
        /* If everything is fine, this cannot actually
                                 * happen. */
        while *list != 0 {
            let mut flags_save = list;
            /*
       * Get one option part into part_buf[].  Advance list to next one.
       * put string at start of string.
       */
            copy_option_part(&mut list, part_buf.as_mut_ptr(),
                             COM_MAX_LEN as size_t,
                             b",\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char); /* Isolate flags from string. */
            string = vim_strchr(part_buf.as_mut_ptr(), ':' as i32);
            if string.is_null() { continue ; }
            let fresh4 = string;
            string = string.offset(1);
            *fresh4 = NUL as char_u;
            com_leader = string;
            /*
       * Line contents and string must match.
       * When string starts with white space, must have some white space
       * (but the amount does not need to match, there might be a mix of
       * TABs and spaces).
       */
            if ascii_iswhite(*string.offset(0 as libc::c_int as isize) as
                                 libc::c_int) {
                if i == 0 as libc::c_int ||
                       !ascii_iswhite(*line.offset((i - 1 as libc::c_int) as
                                                       isize) as libc::c_int)
                   {
                    continue ;
                }
                while ascii_iswhite(*string as libc::c_int) {
                    string = string.offset(1)
                }
            }
            j = 0 as libc::c_int;
            while *string.offset(j as isize) as libc::c_int != NUL &&
                      *string.offset(j as isize) as libc::c_int ==
                          *line.offset((i + j) as isize) as libc::c_int {
                /* do nothing */
                j += 1
            }
            if *string.offset(j as isize) as libc::c_int != NUL { continue ; }
            /*
       * When 'b' flag used, there must be white space or an
       * end-of-line after the string in the line.
       */
            if !vim_strchr(part_buf.as_mut_ptr(), COM_BLANK).is_null() &&
                   !ascii_iswhite(*line.offset((i + j) as isize) as
                                      libc::c_int) &&
                   *line.offset((i + j) as isize) as libc::c_int != NUL {
                continue ;
            }
            if !vim_strchr(part_buf.as_mut_ptr(), COM_MIDDLE).is_null() {
                // For a middlepart comment, only consider it to match if
        // everything before the current position in the line is
        // whitespace.  Otherwise we would think we are inside a
        // comment if the middle part appears somewhere in the middle
        // of the line.  E.g. for C the "*" appears often.
                j = 0 as libc::c_int;
                while j <= i &&
                          ascii_iswhite(*line.offset(j as isize) as
                                            libc::c_int) as libc::c_int != 0 {
                    j += 1
                }
                if j < i { continue ; }
            }
            /*
       * We have found a match, stop searching.
       */
            found_one = TRUE; /* buffer for one option part */
            if !flags.is_null() { *flags = flags_save }
            com_flags = flags_save;
            break ;
        }
        if !(found_one != 0) { continue ; }
        let mut part_buf2: [char_u; 50] = [0; 50];
        let mut len1: libc::c_int = 0;
        let mut len2: libc::c_int = 0;
        let mut off: libc::c_int = 0;
        result = i;
        /*
       * If this comment nests, continue searching.
       */
        if !vim_strchr(part_buf.as_mut_ptr(), COM_NEST).is_null() {
            continue ;
        }
        lower_check_bound = i;
        /* Let's verify whether the comment leader found is a substring
       * of other comment leaders. If it is, let's adjust the
       * lower_check_bound so that we make sure that we have determined
       * the comment leader correctly.
       */
        while ascii_iswhite(*com_leader as libc::c_int) {
            com_leader = com_leader.offset(1)
        }
        len1 = strlen(com_leader as *mut libc::c_char) as libc::c_int;
        list = (*curbuf).b_p_com;
        while *list != 0 {
            let mut flags_save_0 = list;
            copy_option_part(&mut list, part_buf2.as_mut_ptr(),
                             COM_MAX_LEN as size_t,
                             b",\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char);
            if flags_save_0 == com_flags { continue ; }
            string = vim_strchr(part_buf2.as_mut_ptr(), ':' as i32);
            string = string.offset(1);
            while ascii_iswhite(*string as libc::c_int) {
                string = string.offset(1)
            }
            len2 = strlen(string as *mut libc::c_char) as libc::c_int;
            if len2 == 0 as libc::c_int { continue ; }
            /* Now we have to verify whether string ends with a substring
         * beginning the com_leader. */
            off = if len2 > i { i } else { len2 };
            while off > 0 as libc::c_int && off + len1 > len2 {
                off -= 1;
                if strncmp(string.offset(off as isize) as *mut libc::c_char,
                           com_leader as *mut libc::c_char,
                           (len2 - off) as size_t) == 0 {
                    if i - off < lower_check_bound {
                        lower_check_bound = i - off
                    }
                }
            }
        }
    }
    return result;
}
/*
 * Return the number of window lines occupied by buffer line "lnum".
 */
#[no_mangle]
#[c2rust::src_loc = "362:1"]
pub unsafe extern "C" fn plines(lnum: linenr_T) -> libc::c_int {
    return plines_win(curwin, lnum, true_0 != 0);
}
#[no_mangle]
#[c2rust::src_loc = "367:1"]
pub unsafe extern "C" fn plines_win(wp: *mut win_T, lnum: linenr_T,
                                    winheight: bool) -> libc::c_int 
 // when true limit to window height
 {
    /* Check for filler lines above this buffer line.  When folded the result
   * is one line anyway. */
    return plines_win_nofill(wp, lnum, winheight) + diff_check_fill(wp, lnum);
}
#[no_mangle]
#[c2rust::src_loc = "378:1"]
pub unsafe extern "C" fn plines_nofill(lnum: linenr_T) -> libc::c_int {
    return plines_win_nofill(curwin, lnum, true_0 != 0);
}
#[no_mangle]
#[c2rust::src_loc = "383:1"]
pub unsafe extern "C" fn plines_win_nofill(wp: *mut win_T, lnum: linenr_T,
                                           winheight: bool) -> libc::c_int 
 // when true limit to window height
 {
    if (*wp).w_onebuf_opt.wo_wrap == 0 { return 1 as libc::c_int }
    if (*wp).w_width_inner == 0 as libc::c_int { return 1 as libc::c_int }
    // A folded lines is handled just like an empty line.
    if lineFolded(wp, lnum) { return 1 as libc::c_int }
    let lines = plines_win_nofold(wp, lnum);
    if winheight as libc::c_int != 0 && lines > (*wp).w_height_inner {
        return (*wp).w_height_inner
    }
    return lines;
}
/*
 * Return number of window lines physical line "lnum" will occupy in window
 * "wp".  Does not care about folding, 'wrap' or 'diff'.
 */
#[no_mangle]
#[c2rust::src_loc = "413:1"]
pub unsafe extern "C" fn plines_win_nofold(mut wp: *mut win_T,
                                           mut lnum: linenr_T)
 -> libc::c_int {
    let mut s = 0 as *mut char_u;
    let mut col: libc::c_uint = 0;
    let mut width: libc::c_int = 0;
    s = ml_get_buf((*wp).w_buffer, lnum, FALSE != 0);
    if *s as libc::c_int == NUL {
        /* empty line */
        return 1 as libc::c_int
    }
    col = win_linetabsize(wp, s, MAXCOL as libc::c_int);
    // If list mode is on, then the '$' at the end of the line may take up one
  // extra column.
    if (*wp).w_onebuf_opt.wo_list != 0 && (*wp).w_p_lcs_chars.eol != NUL {
        col = col.wrapping_add(1 as libc::c_int as libc::c_uint)
    }
    /*
   * Add column offset for 'number', 'relativenumber' and 'foldcolumn'.
   */
    width = (*wp).w_width_inner - win_col_off(wp);
    if width <= 0 as libc::c_int || col > 32000 as libc::c_int as libc::c_uint
       {
        return 32000 as libc::c_int
        // bigger than the number of screen columns
    }
    if col <= width as libc::c_uint { return 1 as libc::c_int }
    col = col.wrapping_sub(width as libc::c_uint);
    width += win_col_off2(wp);
    if col <= 2147483647 as libc::c_int as libc::c_uint &&
           (col as libc::c_int) <
               2147483647 as libc::c_int - (width - 1 as libc::c_int) {
    } else {
        __assert_fail(b"col <= INT_MAX && (int)col < INT_MAX - (width -1)\x00"
                          as *const u8 as *const libc::c_char,
                      b"/home/vole/neovim/src/nvim/misc1.c\x00" as *const u8
                          as *const libc::c_char,
                      442 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"int plines_win_nofold(win_T *, linenr_T)\x00")).as_ptr());
    }
    return (col as libc::c_int + (width - 1 as libc::c_int)) / width +
               1 as libc::c_int;
}
/*
 * Like plines_win(), but only reports the number of physical screen lines
 * used from the start of the line to the given column number.
 */
#[no_mangle]
#[c2rust::src_loc = "450:1"]
pub unsafe extern "C" fn plines_win_col(mut wp: *mut win_T,
                                        mut lnum: linenr_T,
                                        mut column: libc::c_long)
 -> libc::c_int {
    // Check for filler lines above this buffer line.  When folded the result
  // is one line anyway.
    let mut lines = diff_check_fill(wp, lnum);
    if (*wp).w_onebuf_opt.wo_wrap == 0 { return lines + 1 as libc::c_int }
    if (*wp).w_width_inner == 0 as libc::c_int {
        return lines + 1 as libc::c_int
    }
    let mut line = ml_get_buf((*wp).w_buffer, lnum, false_0 != 0);
    let mut s = line;
    let mut col = 0 as libc::c_int;
    while *s as libc::c_int != NUL &&
              { column -= 1; (column) >= 0 as libc::c_int as libc::c_long } {
        col +=
            win_lbr_chartabsize(wp, line, s, col, NULL_1 as *mut libc::c_int);
        s = s.offset(utfc_ptr2len(s) as isize)
    }
    // If *s is a TAB, and the TAB is not displayed as ^I, and we're not in
  // INSERT mode, then col must be adjusted so that it represents the last
  // screen position of the TAB.  This only fixes an error when the TAB wraps
  // from one screen line to the next (when 'columns' is not a multiple of
  // 'ts') -- webb.
    if *s as libc::c_int == TAB && State & NORMAL != 0 &&
           ((*wp).w_onebuf_opt.wo_list == 0 || (*wp).w_p_lcs_chars.tab1 != 0)
       {
        col +=
            win_lbr_chartabsize(wp, line, s, col, NULL_1 as *mut libc::c_int)
                - 1 as libc::c_int
    }
    // Add column offset for 'number', 'relativenumber', 'foldcolumn', etc.
    let mut width = (*wp).w_width_inner - win_col_off(wp);
    if width <= 0 as libc::c_int { return 9999 as libc::c_int }
    lines += 1 as libc::c_int;
    if col > width {
        lines += (col - width) / (width + win_col_off2(wp)) + 1 as libc::c_int
    }
    return lines;
}
// / Get the number of screen lines lnum takes up. This takes care of
// / both folds and topfill, and limits to the current window height.
// /
// / @param[in]  wp       window line is in
// / @param[in]  lnum     line number
// / @param[out] nextp    if not NULL, the line after a fold
// / @param[out] foldedp  if not NULL, whether lnum is on a fold
// / @param[in]  cache    whether to use the window's cache for folds
// /
// / @return the total number of screen lines
#[no_mangle]
#[c2rust::src_loc = "504:1"]
pub unsafe extern "C" fn plines_win_full(mut wp: *mut win_T,
                                         mut lnum: linenr_T,
                                         nextp: *mut linenr_T,
                                         foldedp: *mut bool, cache: bool)
 -> libc::c_int {
    let mut folded =
        hasFoldingWin(wp, lnum, NULL_1 as *mut linenr_T, nextp, cache,
                      NULL_1 as *mut foldinfo_T);
    if !foldedp.is_null() { *foldedp = folded }
    if folded {
        return 1 as libc::c_int
    } else {
        if lnum == (*wp).w_topline {
            return plines_win_nofill(wp, lnum, true_0 != 0) + (*wp).w_topfill
        }
    }
    return plines_win(wp, lnum, true_0 != 0);
}
#[no_mangle]
#[c2rust::src_loc = "519:1"]
pub unsafe extern "C" fn plines_m_win(mut wp: *mut win_T, mut first: linenr_T,
                                      mut last: linenr_T) -> libc::c_int {
    let mut count = 0 as libc::c_int;
    while first <= last {
        let mut next = first;
        count +=
            plines_win_full(wp, first, &mut next, NULL_1 as *mut bool,
                            false_0 != 0);
        first = next + 1 as libc::c_int as libc::c_long
    }
    return count;
}
#[no_mangle]
#[c2rust::src_loc = "531:1"]
pub unsafe extern "C" fn gchar_pos(mut pos: *mut pos_T) -> libc::c_int {
    // When searching columns is sometimes put at the end of a line.
    if (*pos).col == MAXCOL as libc::c_int { return NUL }
    return utf_ptr2char(ml_get_pos(pos));
}
/*
 * check_status: called when the status bars for the buffer 'buf'
 *		 need to be updated
 */
#[no_mangle]
#[c2rust::src_loc = "545:1"]
pub unsafe extern "C" fn check_status(mut buf: *mut buf_T) {
    let mut wp =
        if curtab == curtab { firstwin } else { (*curtab).tp_firstwin };
    while !wp.is_null() {
        if (*wp).w_buffer == buf && (*wp).w_status_height != 0 {
            (*wp).w_redr_status = TRUE;
            if must_redraw < VALID { must_redraw = VALID }
        }
        wp = (*wp).w_next
    };
}
// / Ask for a reply from the user, 'y' or 'n'
// /
// / No other characters are accepted, the message is repeated until a valid
// / reply is entered or <C-c> is hit.
// /
// / @param[in]  str  Prompt: question to ask user. Is always followed by
// /                  " (y/n)?".
// / @param[in]  direct  Determines what function to use to get user input. If
// /                     true then ui_inchar() will be used, otherwise vgetc().
// /                     I.e. when direct is true then characters are obtained
// /                     directly from the user without buffers involved.
// /
// / @return 'y' or 'n'. Last is also what will be returned in case of interrupt.
#[no_mangle]
#[c2rust::src_loc = "570:1"]
pub unsafe extern "C" fn ask_yesno(str: *const libc::c_char, direct: bool)
 -> libc::c_int {
    let save_State = State; // Mouse behaves like with :confirm.
    no_wait_return += 1; // Disable mouse in xterm.
    State = CONFIRM;
    setmouse();
    no_mapping += 1;
    let mut r = ' ' as i32;
    while r != 'y' as i32 && r != 'n' as i32 {
        // Same highlighting as for wait_return.
        smsg_attr(highlight_attr[HLF_R as libc::c_int as usize],
                  b"%s (y/n)?\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, str); // Show what you typed.
        if direct {
            r = get_keystroke(NULL_1 as *mut MultiQueue)
        } else { r = plain_vgetc() }
        if r == Ctrl_C || r == ESC { r = 'n' as i32 }
        msg_putchar(r);
        ui_flush();
    }
    no_wait_return -= 1;
    State = save_State;
    setmouse();
    no_mapping -= 1;
    return r;
}
/*
 * Return TRUE if "c" is a mouse key.
 */
#[no_mangle]
#[c2rust::src_loc = "605:1"]
pub unsafe extern "C" fn is_mouse_key(mut c: libc::c_int) -> libc::c_int {
    return (c == K_LEFTMOUSE || c == K_LEFTMOUSE_NM || c == K_LEFTDRAG ||
                c == K_LEFTRELEASE || c == K_LEFTRELEASE_NM ||
                c == K_MIDDLEMOUSE || c == K_MIDDLEDRAG ||
                c == K_MIDDLERELEASE || c == K_RIGHTMOUSE || c == K_RIGHTDRAG
                || c == K_RIGHTRELEASE || c == K_MOUSEDOWN || c == K_MOUSEUP
                || c == K_MOUSELEFT || c == K_MOUSERIGHT || c == K_X1MOUSE ||
                c == K_X1DRAG || c == K_X1RELEASE || c == K_X2MOUSE ||
                c == K_X2DRAG || c == K_X2RELEASE) as libc::c_int;
}
/*
 * Get a key stroke directly from the user.
 * Ignores mouse clicks and scrollbar events, except a click for the left
 * button (used at the more prompt).
 * Doesn't use vgetc(), because it syncs undo and eats mapped characters.
 * Disadvantage: typeahead is ignored.
 * Translates the interrupt character for unix to ESC.
 */
#[no_mangle]
#[c2rust::src_loc = "638:1"]
pub unsafe extern "C" fn get_keystroke(mut events: *mut MultiQueue)
 -> libc::c_int {
    let mut buf = NULL_1 as *mut char_u; // mappings are not used here
    let mut buflen = 150 as libc::c_int;
    let mut maxlen: libc::c_int = 0;
    let mut len = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut save_mapped_ctrl_c = mapped_ctrl_c;
    let mut waited = 0 as libc::c_int;
    mapped_ctrl_c = 0 as libc::c_int;
    loop 
         // more bytes to get.
         // flush output before waiting
         {
        ui_flush();
        /* Leave some room for check_termcode() to insert a key code into (max
     * 5 chars plus NUL).  And fix_input_buffer() can triple the number of
     * bytes. */
        maxlen = (buflen - 6 as libc::c_int - len) / 3 as libc::c_int;
        if buf.is_null() {
            buf = xmalloc(buflen as size_t) as *mut char_u
        } else if maxlen < 10 as libc::c_int {
            // Need some more space. This might happen when receiving a long
      // escape sequence.
            buflen += 100 as libc::c_int;
            buf =
                xrealloc(buf as *mut libc::c_void, buflen as size_t) as
                    *mut char_u;
            maxlen = (buflen - 6 as libc::c_int - len) / 3 as libc::c_int
        }
        /* First time: blocking wait.  Second time: wait up to 100ms for a
     * terminal code to complete. */
        n =
            os_inchar(buf.offset(len as isize), maxlen,
                      if len == 0 as libc::c_int {
                          -(1 as libc::c_long)
                      } else { 100 as libc::c_long } as libc::c_int,
                      0 as libc::c_int,
                      events); /* keep track of the waiting time */
        if n > 0 as libc::c_int {
            // Replace zero and CSI by a special key code.
            n = fix_input_buffer(buf.offset(len as isize), n);
            len += n;
            waited = 0 as libc::c_int
        } else if len > 0 as libc::c_int { waited += 1 }
        if n > 0 as libc::c_int {
            // found a termcode: adjust length
            len = n
        }
        if !(len == 0 as libc::c_int) {
            /* Handle modifier and/or special key code. */
            n = *buf.offset(0 as libc::c_int as isize) as libc::c_int;
            if n == K_SPECIAL {
                n =
                    if *buf.offset(1 as libc::c_int as isize) as libc::c_int
                           == KS_SPECIAL {
                        K_SPECIAL
                    } else if *buf.offset(1 as libc::c_int as isize) as
                                  libc::c_int == KS_ZERO {
                        K_ZERO
                    } else {
                        -(*buf.offset(1 as libc::c_int as isize) as
                              libc::c_int +
                              ((*buf.offset(2 as libc::c_int as isize) as
                                    libc::c_int) << 8 as libc::c_int))
                    };
                if !(*buf.offset(1 as libc::c_int as isize) as libc::c_int ==
                         KS_MODIFIER || n == K_IGNORE ||
                         is_mouse_key(n) != 0 && n != K_LEFTMOUSE) {
                    break ;
                }
                if *buf.offset(1 as libc::c_int as isize) as libc::c_int ==
                       KS_MODIFIER {
                    mod_mask =
                        *buf.offset(2 as libc::c_int as isize) as libc::c_int
                }
                len -= 3 as libc::c_int;
                if len > 0 as libc::c_int {
                    memmove(buf as *mut libc::c_void,
                            buf.offset(3 as libc::c_int as isize) as
                                *const libc::c_void, len as size_t);
                }
            } else {
                if utf8len_tab[n as usize] as libc::c_int > len { continue ; }
                *buf.offset(if len >= buflen {
                                (buflen) - 1 as libc::c_int
                            } else { len } as isize) = NUL as char_u;
                n = utf_ptr2char(buf);
                break ;
            }
        }
    }
    xfree(buf as *mut libc::c_void);
    mapped_ctrl_c = save_mapped_ctrl_c;
    return n;
}
/*
 * Get a number from the user.
 * When "mouse_used" is not NULL allow using the mouse.
 */
#[no_mangle]
#[c2rust::src_loc = "719:1"]
pub unsafe extern "C" fn get_number(mut colon: libc::c_int,
                                    mut mouse_used: *mut libc::c_int)
 -> libc::c_int {
    let mut n = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut typed = 0 as libc::c_int;
    if !mouse_used.is_null() { *mouse_used = FALSE }
    /* When not printing messages, the user won't know what to type, return a
   * zero (as if CR was hit). */
    if msg_silent != 0 as libc::c_int {
        return 0 as libc::c_int
    } /* skip redraw once */
    no_mapping += 1;
    loop  {
        ui_cursor_goto(msg_row, msg_col);
        c = safe_vgetc();
        if ascii_isdigit(c) {
            n = n * 10 as libc::c_int + c - '0' as i32;
            msg_putchar(c);
            typed += 1
        } else if c == K_DEL || c == K_KDEL || c == K_BS || c == Ctrl_H {
            if typed > 0 as libc::c_int {
                msg_puts(b"\x08 \x08\x00" as *const u8 as
                             *const libc::c_char);
                typed -= 1
            }
            n /= 10 as libc::c_int
        } else if !mouse_used.is_null() && c == K_LEFTMOUSE {
            *mouse_used = TRUE;
            n = mouse_row + 1 as libc::c_int;
            break ;
        } else if n == 0 as libc::c_int && c == ':' as i32 && colon != 0 {
            stuffcharReadbuff(':' as i32);
            if exmode_active == 0 { cmdline_row = msg_row }
            skip_redraw = TRUE != 0;
            do_redraw = FALSE != 0;
            break ;
        } else if c == CAR || c == NL || c == Ctrl_C || c == ESC { break ; }
    }
    no_mapping -= 1;
    return n;
}
/*
 * Ask the user to enter a number.
 * When "mouse_used" is not NULL allow using the mouse and in that case return
 * the line number.
 */
#[no_mangle]
#[c2rust::src_loc = "774:1"]
pub unsafe extern "C" fn prompt_for_number(mut mouse_used: *mut libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut save_cmdline_row: libc::c_int = 0;
    let mut save_State: libc::c_int = 0;
    /* When using ":silent" assume that <CR> was entered. */
    if !mouse_used.is_null() {
        msg_puts(gettext(b"Type number and <Enter> or click with mouse (empty cancels): \x00"
                             as *const u8 as *const libc::c_char as
                             *mut libc::c_char) as *const libc::c_char);
    } else {
        msg_puts(gettext(b"Type number and <Enter> (empty cancels): \x00" as
                             *const u8 as *const libc::c_char as
                             *mut libc::c_char) as *const libc::c_char);
    }
    /* Set the state such that text can be selected/copied/pasted and we still
   * get mouse events. */
    save_cmdline_row =
        cmdline_row; // prevents a screen update when using a timer
    cmdline_row = 0 as libc::c_int;
    save_State = State;
    State = ASKMORE;
    // May show different mouse shape.
    setmouse();
    i = get_number(TRUE, mouse_used);
    if KeyTyped {
        // don't call wait_return() now
        if msg_row > 0 as libc::c_int {
            cmdline_row = msg_row - 1 as libc::c_int
        }
        need_wait_return = false_0;
        msg_didany = false_0;
        msg_didout = false_0
    } else { cmdline_row = save_cmdline_row }
    State = save_State;
    // May need to restore mouse shape.
    setmouse();
    return i;
}
#[no_mangle]
#[c2rust::src_loc = "814:1"]
pub unsafe extern "C" fn msgmore(mut n: libc::c_long) {
    let mut pn: libc::c_long = 0;
    if global_busy != 0 || messaging() == 0 {
        /* 'lazyredraw' set, don't do messages now */
        return
    }
    /* We don't want to overwrite another important message, but do overwrite
   * a previous "more lines" or "fewer lines" message, so that "5dd" and
   * then "put" reports the last action. */
    if !keep_msg.is_null() && keep_msg_more == 0 { return }
    if n > 0 as libc::c_int as libc::c_long { pn = n } else { pn = -n }
    if pn > p_report {
        if pn == 1 as libc::c_int as libc::c_long {
            if n > 0 as libc::c_int as libc::c_long {
                xstrlcpy(msg_buf.as_mut_ptr() as *mut libc::c_char,
                         gettext(b"1 more line\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char),
                         480 as libc::c_int as size_t);
            } else {
                xstrlcpy(msg_buf.as_mut_ptr() as *mut libc::c_char,
                         gettext(b"1 line less\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char),
                         480 as libc::c_int as size_t);
            }
        } else if n > 0 as libc::c_int as libc::c_long {
            vim_snprintf(msg_buf.as_mut_ptr() as *mut libc::c_char,
                         MSG_BUF_LEN as size_t,
                         gettext(b"%ld more lines\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char), pn);
        } else {
            vim_snprintf(msg_buf.as_mut_ptr() as *mut libc::c_char,
                         MSG_BUF_LEN as size_t,
                         gettext(b"%ld fewer lines\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char), pn);
        }
        if got_int != 0 {
            xstrlcat(msg_buf.as_mut_ptr() as *mut libc::c_char,
                     gettext(b" (Interrupted)\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char),
                     MSG_BUF_LEN as size_t);
        }
        if msg(msg_buf.as_mut_ptr()) != 0 {
            set_keep_msg(msg_buf.as_mut_ptr(), 0 as libc::c_int);
            keep_msg_more = TRUE
        }
    };
}
/*
 * flush map and typeahead buffers and give a warning for an error
 */
#[no_mangle]
#[c2rust::src_loc = "860:1"]
pub unsafe extern "C" fn beep_flush() {
    if emsg_silent == 0 as libc::c_int {
        flush_buffers(FLUSH_MINIMAL);
        vim_beep(BO_ERROR as libc::c_uint);
    };
}
// Give a warning for an error
// val is one of the BO_ values, e.g., BO_OPER
#[no_mangle]
#[c2rust::src_loc = "870:1"]
pub unsafe extern "C" fn vim_beep(mut val: libc::c_uint) {
    called_vim_beep = true_0 != 0;
    if emsg_silent == 0 as libc::c_int {
        if !(bo_flags & val != 0 || bo_flags & BO_ALL as libc::c_uint != 0) {
            static mut beeps: libc::c_int = 0 as libc::c_int;
            static mut start_time: uint64_t = 0 as libc::c_int as uint64_t;
            // Only beep up to three times per half a second,
      // otherwise a sequence of beeps would freeze Vim.
            if start_time == 0 as libc::c_int as libc::c_ulong ||
                   os_hrtime().wrapping_sub(start_time) >
                       500000000 as libc::c_uint as libc::c_ulong {
                beeps = 0 as libc::c_int;
                start_time = os_hrtime()
            }
            beeps += 1;
            if beeps <= 3 as libc::c_int {
                if p_vb != 0 {
                    ui_call_visual_bell();
                } else { ui_call_bell(); }
            }
        }
        // When 'debug' contains "beep" produce a message.  If we are sourcing
    // a script or executing a function give the user a hint where the beep
    // comes from.
        if !vim_strchr(p_debug, 'e' as i32).is_null() {
            msg_source(highlight_attr[HLF_W as libc::c_int as usize]);
            msg_attr(gettext(b"Beep!\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char),
                     highlight_attr[HLF_W as libc::c_int as usize]);
        }
    };
}
/*
 * Find all user names for user completion.
 * Done only once and then cached.
 */
#[c2rust::src_loc = "918:1"]
unsafe extern "C" fn init_users() {
    static mut lazy_init_done: libc::c_int = FALSE;
    if lazy_init_done != 0 { return }
    lazy_init_done = TRUE;
    os_get_usernames(&mut ga_users);
}
/*
 * Function given to ExpandGeneric() to obtain an user names.
 */
#[no_mangle]
#[c2rust::src_loc = "934:1"]
pub unsafe extern "C" fn get_users(mut xp: *mut expand_T,
                                   mut idx: libc::c_int) -> *mut char_u {
    init_users();
    if idx < ga_users.ga_len {
        return *(ga_users.ga_data as *mut *mut char_u).offset(idx as isize)
    }
    return NULL_1 as *mut char_u;
}
/*
 * Check whether name matches a user name. Return:
 * 0 if name does not match any user name.
 * 1 if name partially matches the beginning of a user name.
 * 2 is name fully matches a user name.
 */
#[no_mangle]
#[c2rust::src_loc = "948:1"]
pub unsafe extern "C" fn match_user(mut name: *mut char_u) -> libc::c_int {
    let mut n =
        strlen(name as *mut libc::c_char) as libc::c_int; /* full match */
    let mut result = 0 as libc::c_int;
    init_users();
    let mut i = 0 as libc::c_int;
    while i < ga_users.ga_len {
        if strcmp(*(ga_users.ga_data as *mut *mut char_u).offset(i as isize)
                      as *mut libc::c_char, name as *mut libc::c_char) ==
               0 as libc::c_int {
            return 2 as libc::c_int
        }
        if strncmp(*(ga_users.ga_data as *mut *mut char_u).offset(i as isize)
                       as *mut libc::c_char, name as *mut libc::c_char,
                   n as size_t) == 0 as libc::c_int {
            result = 1 as libc::c_int
        }
        i += 1
        /* partial match */
    }
    return result;
}
// / Preserve files and exit.
// / @note IObuff must contain a message.
// / @note This may be called from deadly_signal() in a signal handler, avoid
// /       unsafe functions, such as allocating memory.
#[no_mangle]
#[c2rust::src_loc = "967:1"]
pub unsafe extern "C" fn preserve_exit() -> ! {
    // 'true' when we are sure to exit, e.g., after a deadly signal
    static mut really_exiting: bool = false_0 != 0;
    // Prevent repeated calls into this method.
    if really_exiting {
        if input_global_fd() >= 0 as libc::c_int {
            // normalize stream (#2598)
            stream_set_blocking(input_global_fd(), true_0 != 0);
        }
        exit(2 as libc::c_int);
    }
    really_exiting = true_0 != 0;
    // Ignore SIGHUP while we are already exiting. #9274
    signal_reject_deadly(); // close all not-modified buffers
    fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
            IObuff.as_mut_ptr()); // preserve all swap files
    fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
            b"\n\x00" as *const u8 as
                *const libc::c_char); // close all memfiles, without deleting
    ui_flush();
    ml_close_notmod();
    let mut buf = firstbuf;
    while !buf.is_null() {
        if !(*buf).b_ml.ml_mfp.is_null() &&
               !(*(*buf).b_ml.ml_mfp).mf_fname.is_null() {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    b"Vim: preserving files...\n\x00" as *const u8 as
                        *const libc::c_char as *mut uint8_t);
            ui_flush();
            ml_sync_all(false_0, false_0, true_0 != 0);
            break ;
        } else { buf = (*buf).b_next }
    }
    ml_close_all(false_0);
    fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
            b"Vim: Finished.\n\x00" as *const u8 as *const libc::c_char);
    getout(1 as libc::c_int);
}
/*
 * Check for CTRL-C pressed, but only once in a while.
 * Should be used instead of os_breakcheck() for functions that check for
 * each line in the file.  Calling os_breakcheck() each time takes too much
 * time, because it can be a system call.
 */
#[c2rust::src_loc = "1015:11"]
pub const BREAKCHECK_SKIP: libc::c_int = 1000 as libc::c_int;
#[c2rust::src_loc = "1018:12"]
static mut breakcheck_count: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "1020:1"]
pub unsafe extern "C" fn line_breakcheck() {
    breakcheck_count += 1;
    if breakcheck_count >= BREAKCHECK_SKIP {
        breakcheck_count = 0 as libc::c_int;
        os_breakcheck();
    };
}
/*
 * Like line_breakcheck() but check 10 times less often.
 */
#[no_mangle]
#[c2rust::src_loc = "1031:1"]
pub unsafe extern "C" fn fast_breakcheck() {
    breakcheck_count += 1;
    if breakcheck_count >= BREAKCHECK_SKIP * 10 as libc::c_int {
        breakcheck_count = 0 as libc::c_int;
        os_breakcheck();
    };
}
// / os_call_shell() wrapper. Handles 'verbose', :profile, and v:shell_error.
// / Invalidates cached tags.
// /
// / @return shell command exit code
#[no_mangle]
#[c2rust::src_loc = "1043:1"]
pub unsafe extern "C" fn call_shell(mut cmd: *mut char_u, mut opts: ShellOpts,
                                    mut extra_shell_arg: *mut char_u)
 -> libc::c_int {
    let mut retval: libc::c_int = 0;
    let mut wait_time: proftime_T = 0;
    if p_verbose > 3 as libc::c_int as libc::c_long {
        verbose_enter();
        smsg(gettext(b"Executing command: \"%s\"\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char),
             if cmd.is_null() { p_sh } else { cmd });
        msg_putchar('\n' as i32);
        verbose_leave();
    }
    if do_profiling == PROF_YES { prof_child_enter(&mut wait_time); }
    if *p_sh as libc::c_int == NUL {
        emsg(gettext(e_shellempty.as_mut_ptr() as *mut libc::c_char) as
                 *mut char_u);
        retval = -(1 as libc::c_int)
    } else {
        // The external command may update a tags file, clear cached tags.
        tag_freematch();
        retval = os_call_shell(cmd, opts, extra_shell_arg)
    }
    set_vim_var_nr(VV_SHELL_ERROR, retval as varnumber_T);
    if do_profiling == PROF_YES { prof_child_exit(&mut wait_time); }
    return retval;
}
// / Get the stdout of an external command.
// / If "ret_len" is NULL replace NUL characters with NL. When "ret_len" is not
// / NULL store the length there.
// /
// / @param  cmd      command to execute
// / @param  infile   optional input file name
// / @param  flags    can be kShellOptSilent or 0
// / @param  ret_len  length of the stdout
// /
// / @return an allocated string, or NULL for error.
#[no_mangle]
#[c2rust::src_loc = "1087:1"]
pub unsafe extern "C" fn get_cmd_output(mut cmd: *mut char_u,
                                        mut infile: *mut char_u,
                                        mut flags: ShellOpts,
                                        mut ret_len: *mut size_t)
 -> *mut char_u {
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    let mut buffer = NULL_1 as *mut char_u;
    if check_restricted() as libc::c_int != 0 || check_secure() != 0 {
        return NULL_1 as *mut char_u
    }
    // get a name for the temp file
    let mut tempname = vim_tempname();
    if tempname.is_null() {
        emsg(gettext(e_notmp.as_mut_ptr() as *mut libc::c_char) as
                 *mut char_u);
        return NULL_1 as *mut char_u
    }
    // Add the redirection stuff
    let mut command = make_filter_cmd(cmd, infile, tempname);
    /*
   * Call the shell to execute the command (errors are ignored).
   * Don't check timestamps here.
   */
    no_check_timestamps += 1;
    call_shell(command,
               ((kShellOptDoOut as libc::c_int |
                     kShellOptExpand as libc::c_int) as libc::c_uint |
                    flags as libc::c_uint) as ShellOpts,
               NULL_1 as *mut char_u);
    no_check_timestamps -= 1;
    xfree(command as *mut libc::c_void);
    // read the names from the file into memory
    let mut fd =
        os_fopen(tempname as *mut libc::c_char,
                 READBIN.as_ptr()); // get size of temp file
    if fd.is_null() {
        emsgf(gettext(e_notopen.as_mut_ptr() as *mut libc::c_char) as
                  *const libc::c_char, tempname);
    } else {
        fseek(fd, 0 as libc::c_long, SEEK_END);
        len = ftell(fd) as size_t;
        fseek(fd, 0 as libc::c_long, SEEK_SET);
        buffer =
            xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as
                *mut char_u;
        i =
            fread(buffer as *mut libc::c_char as *mut libc::c_void,
                  1 as libc::c_int as size_t, len, fd);
        fclose(fd);
        os_remove(tempname as *mut libc::c_char);
        if i != len {
            emsgf(gettext(e_notread.as_mut_ptr() as *mut libc::c_char) as
                      *const libc::c_char, tempname);
            let mut ptr_ =
                &mut buffer as *mut *mut char_u as *mut *mut libc::c_void;
            xfree(*ptr_);
            *ptr_ = NULL_1 as *mut libc::c_void
        } else if ret_len.is_null() {
            /* Change NUL into SOH, otherwise the string is truncated. */
            i = 0 as libc::c_int as size_t;
            while i < len {
                if *buffer.offset(i as isize) as libc::c_int == NUL {
                    *buffer.offset(i as isize) = 1 as libc::c_int as char_u
                }
                i = i.wrapping_add(1)
            }
            *buffer.offset(len as isize) = NUL as char_u
            /* make sure the buffer is terminated */
        } else { *ret_len = len }
    }
    xfree(tempname as *mut libc::c_void);
    return buffer;
}
/*
 * Free the list of files returned by expand_wildcards() or other expansion
 * functions.
 */
#[no_mangle]
#[c2rust::src_loc = "1154:1"]
pub unsafe extern "C" fn FreeWild(mut count: libc::c_int,
                                  mut files: *mut *mut char_u) {
    if count <= 0 as libc::c_int || files.is_null() { return }
    loop  {
        let fresh5 = count;
        count = count - 1;
        if !(fresh5 != 0) { break ; }
        xfree(*files.offset(count as isize) as *mut libc::c_void);
    }
    xfree(files as *mut libc::c_void);
}
/*
 * Return TRUE when need to go to Insert mode because of 'insertmode'.
 * Don't do this when still processing a command or a mapping.
 * Don't do this when inside a ":normal" command.
 */
#[no_mangle]
#[c2rust::src_loc = "1168:1"]
pub unsafe extern "C" fn goto_im() -> libc::c_int {
    return (p_im != 0 && stuff_empty() != 0 && typebuf_typed() != 0) as
               libc::c_int;
}
