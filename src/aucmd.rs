use crate::auevents::*;
use crate::eval::typval::*;
use crate::eval::*;
use crate::event::r#loop::*;
use crate::event::*;
use crate::fileio::*;
use crate::globals::*;
use crate::main_loop;
use crate::memory::*;
use std::ptr;

#[no_mangle]
pub unsafe extern "C" fn do_autocmd_uienter(chanid: u64, attached: bool) {
    #[allow(non_upper_case_globals)]
    static mut recursive: bool = false;
    if recursive {
        return; // disallow recursion
    }
    recursive = true;

    let dict: *mut dict_T = get_vim_var_dict(VimVar::VV_EVENT);
    c_assert!(chanid < VARNUMBER_MAX as u64);
    let chan_str = std::ffi::CString::new("chan").unwrap();
    tv_dict_add_nr(
        dict,
        chan_str.as_ptr(),
        chan_str.to_bytes().len(),
        chanid as varnumber_T,
    );
    tv_dict_set_keys_readonly(dict);
    apply_autocmds(
        if attached as i32 != 0 {
            event_T::EVENT_UIENTER
        } else {
            event_T::EVENT_UILEAVE
        },
        ptr::null(),
        ptr::null(),
        false,
        curbuf,
    );
    tv_dict_clear(dict);
    recursive = false;
}
unsafe extern "C" fn focusgained_event(argv: *mut *mut libc::c_void) {
    let gainedp: *mut bool = *argv.offset(0) as *mut bool;
    do_autocmd_focusgained(*gainedp);
    xfree(gainedp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn aucmd_schedule_focusgained(gained: bool) {
    let gainedp: *mut bool = xmalloc(::std::mem::size_of::<bool>()) as *mut bool;
    *gainedp = gained;
    loop_schedule_deferred(
        &mut main_loop,
        defs::event_create(
            Some(focusgained_event as unsafe extern "C" fn(_: *mut *mut libc::c_void) -> ()),
            1,
            gainedp,
        ),
    );
}
unsafe extern "C" fn do_autocmd_focusgained(gained: bool) {
    #[allow(non_upper_case_globals)]
    static mut recursive: bool = false;
    if recursive {
        return; // disallow recursion
    }
    recursive = true;

    apply_autocmds(
        if gained as i32 != 0 {
            event_T::EVENT_FOCUSGAINED
        } else {
            event_T::EVENT_FOCUSLOST
        },
        ptr::null(),
        ptr::null(),
        false,
        curbuf,
    );
    recursive = false;
}
