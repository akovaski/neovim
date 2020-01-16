// dummy to pass an ACL to a function
#[allow(dead_code)]
pub type vim_acl_T = *mut libc::c_void;

// Opaque handle used by API clients to refer to various objects in vim
pub type handle_T = libc::c_int;

// Opaque handle to a lua value. Must be free with `executor_free_luaref` when
// not needed anymore! LUA_NOREF represents missing reference, i e to indicate
// absent callback etc.
pub type LuaRef = libc::c_int;
