//
// buffer.c: functions for dealing with the buffer structure
//

//
// The buffer list is a double linked list of all buffers.
// Each buffer can be in one of these states:
// never loaded: BF_NEVERLOADED is set, only the file name is valid
//   not loaded: b_ml.ml_mfp == NULL, no memfile allocated
//       hidden: b_nwindows == 0, loaded but not displayed in a window
//       normal: loaded and displayed in a window
//
// Instead of storing file names all over the place, each file name is
// stored in the buffer list. It can be referenced by a number.
//
// The current implementation remembers all file names ever used.
//

use crate::*;
use bitflags::bitflags;

// Values for buflist_getfile()
bitflags! {
    pub struct getf_values: i32 {
        const SETMARK = 0x01; // set pcmark before jumping
        const ALT = 0x02; // jumping to alternate file (not buf num)
        const SWITCH = 0x04; // respect 'switchbuf' settings when jumping
    }
}
pub const GETF_SETMARK: i32 = getf_values::SETMARK.bits;
pub const GETF_ALT: i32 = getf_values::ALT.bits;
pub const GETF_SWITCH: i32 = getf_values::SWITCH.bits;

// Return values of getfile()
bitflags! {
    pub struct getf_retvalues: i32 {
        const ERROR       = 1;    // normal error
        const NOT_WRITTEN = 2;    // "not written" error
        const SAME_FILE   = 0;    // success, same file
        const OPEN_OTHER  = -1;   // success, opened another file
        const UNUSED      = 8;
    }
}
pub const GETFILE_ERROR: i32 = getf_retvalues::ERROR.bits;
pub const GETFILE_NOT_WRITTEN: i32 = getf_retvalues::NOT_WRITTEN.bits;
pub const GETFILE_SAME_FILE: i32 = getf_retvalues::SAME_FILE.bits;
pub const GETFILE_OPEN_OTHER: i32 = getf_retvalues::OPEN_OTHER.bits;
pub const GETFILE_UNUSED: i32 = getf_retvalues::UNUSED.bits;

// Values for buflist_new() flags
bitflags! {
    pub struct bln_values: i32 {
        const CURBUF = 1;   // May re-use curbuf for new buffer
        const LISTED = 2;   // Put new buffer in buffer list
        const DUMMY  = 4;   // Allocating dummy buffer
        const NEW    = 8;   // create a new buffer
        const NOOPT  = 16;  // Don't copy options to existing buffer
    }
}
pub const BLN_CURBUF: i32 = bln_values::CURBUF.bits;
pub const BLN_LISTED: i32 = bln_values::LISTED.bits;
pub const BLN_DUMMY: i32 = bln_values::DUMMY.bits;
pub const BLN_NEW: i32 = bln_values::NEW.bits;
pub const BLN_NOOPT: i32 = bln_values::NOOPT.bits;

// Values for action argument for do_buffer()
bitflags! {
    pub struct dobuf_action_values: i32 {
        const GOTO   = 0; // go to specified buffer
        const SPLIT  = 1; // split window and go to specified buffer
        const UNLOAD = 2; // unload specified buffer(s)
        const DEL    = 3; // delete specified buffer(s) from buflist
        const WIPE   = 4; // delete specified buffer(s) really
    }
}
pub const DOBUF_GOTO: i32 = dobuf_action_values::GOTO.bits;
pub const DOBUF_SPLIT: i32 = dobuf_action_values::SPLIT.bits;
pub const DOBUF_UNLOAD: i32 = dobuf_action_values::UNLOAD.bits;
pub const DOBUF_DEL: i32 = dobuf_action_values::DEL.bits;
pub const DOBUF_WIPE: i32 = dobuf_action_values::WIPE.bits;

// Values for start argument for do_buffer()
bitflags! {
    pub struct dobuf_start_values: i32 {
        const CURRENT = 0; // "count" buffer from current buffer
        const FIRST   = 1; // "count" buffer from first buffer
        const LAST    = 2; // "count" buffer from last buffer
        const MOD     = 3; // "count" mod. buffer from current buffer
    }
}
pub const DOBUF_CURRENT: i32 = dobuf_start_values::CURRENT.bits;
pub const DOBUF_FIRST: i32 = dobuf_start_values::FIRST.bits;
pub const DOBUF_LAST: i32 = dobuf_start_values::LAST.bits;
pub const DOBUF_MOD: i32 = dobuf_start_values::MOD.bits;

// flags for buf_freeall()
bitflags! {
    pub struct bfa_values: i32 {
        const DEL       = 1; // buffer is going to be deleted
        const WIPE      = 2; // buffer is going to be wiped out
        const KEEP_UNDO = 4; // do not free undo information
    }
}
pub const BFA_DEL: i32 = bfa_values::DEL.bits;
pub const BFA_WIPE: i32 = bfa_values::WIPE.bits;
pub const BFA_KEEP_UNDO: i32 = bfa_values::KEEP_UNDO.bits;

/// Set b:changedtick, also checking b: for consistency in debug build
///
/// @param[out]  buf  Buffer to set changedtick in.
/// @param[in]  changedtick  New value.
#[inline(always)]
pub unsafe extern "C" fn buf_set_changedtick(buf: *mut buf_T, changedtick: varnumber_T) {
    let buf = buf.as_mut().unwrap();
    let mut old_val = buf.changedtick_di.di_tv;

    if cfg!(debug_assertions) {
        let changedtick_di = tv_dict_find(buf.b_vars, S!("changedtick"), 11)
            .as_ref()
            .unwrap();
        assert!(changedtick_di.di_tv.v_type == VAR_NUMBER);
        assert!(changedtick_di.di_tv.v_lock == VAR_FIXED);
        assert!(changedtick_di.di_flags == DI_FLAGS_RO | DI_FLAGS_FIX);
        assert!(
            changedtick_di as *const dictitem_T
                == &buf.changedtick_di as *const _ as *const dictitem_T
        );
    }

    buf.changedtick_di.di_tv.vval.v_number = changedtick;

    if tv_dict_is_watched(buf.b_vars) {
        tv_dict_watcher_notify(
            buf.b_vars,
            buf.changedtick_di.di_key.as_mut_ptr() as *mut i8,
            &mut buf.changedtick_di.di_tv,
            &mut old_val,
        );
    };
}

/// Get b:changedtick value
///
/// Faster then querying b:.
///
/// @param[in]  buf  Buffer to get b:changedtick from.
#[inline(always)]
pub unsafe extern "C" fn buf_get_changedtick(buf: *const buf_T) -> varnumber_T {
    let buf = buf.as_ref().unwrap();
    buf.changedtick_di.di_tv.vval.v_number
}

/// Increment b:changedtick value
///
/// Also checks b: for consistency in case of debug build.
///
/// @param[in,out]  buf  Buffer to increment value in.
#[inline(always)]
pub unsafe extern "C" fn buf_inc_changedtick(buf: *mut buf_T) {
    buf_set_changedtick(buf, buf_get_changedtick(buf) + 1);
}

const msg_loclist: *const i8 = S!("[Location List]");
const msg_qflist: *const i8 = S!("[Quickfix List]");
const e_auabort: &str = "E855: Autocommands caused command to abort";

// Number of times free_buffer() was called.
static mut buf_free_count: i32 = 0;

bitflags! {
    pub struct BufFreeFlags: i32 {
        const ClearWinInfo = 1;
        const InitChangedtick = 2;
    }
}
pub const kBffClearWinInfo: i32 = BufFreeFlags::ClearWinInfo.bits;
pub const kBffInitChangedtick: i32 = BufFreeFlags::InitChangedtick.bits;

// Read data from buffer for retrying.
#[no_mangle]
unsafe fn read_buffer(
    read_stdin: i32,   // read file from stdin, otherwise fifo
    eap: *mut exarg_T, // for forced 'ff' and 'fenc' or NULL
    flags: i32,        // extra flags for readfile()
) -> i32 {
    //
    // Read from the buffer which the text is already filled in and append at
    // the end.  This makes it possible to retry when 'fileformat' or
    // 'fileencoding' was guessed wrong.
    //
    let mut line_count = (*curbuf).b_ml.ml_line_count;
    let mut retval = readfile(
        if read_stdin != 0 {
            ptr::null_mut()
        } else {
            (*curbuf).b_ffname
        },
        if read_stdin != 0 {
            ptr::null_mut()
        } else {
            (*curbuf).b_fname
        },
        line_count,
        0,
        MAXLNUM as linenr_T,
        eap,
        flags | READ_BUFFER,
    );
    if retval == OK {
        // Delete the binary lines.
        while {
            line_count -= 1;
            line_count >= 0
        } {
            ml_delete(1, false);
        }
    } else {
        // Delete the converted lines.
        while (*curbuf).b_ml.ml_line_count > line_count {
            ml_delete(line_count, false);
        }
    }
    // Put the cursor on the first line.
    (*curwin).w_cursor.lnum = 1;
    (*curwin).w_cursor.col = 0;

    if read_stdin != 0 {
        // Set or reset 'modified' before executing autocommands, so that
        // it can be changed there.
        if readonlymode == 0 && !BUFEMPTY() {
            changed();
        } else if retval != FAIL {
            unchanged(curbuf, false_0, true);
        }

        apply_autocmds_retval(
            EVENT_STDINREADPOST,
            ptr::null_mut(),
            ptr::null_mut(),
            false,
            curbuf,
            &mut retval,
        );
    }
    return retval;
}

// Open current buffer, that is: open the memfile and read the file into
// memory.
// Return FAIL for failure, OK otherwise.
#[no_mangle]
pub unsafe extern "C" fn open_buffer(
    read_stdin: i32,   // read file from stdin
    eap: *mut exarg_T, // for forced 'ff' and 'fenc' or NULL
    flags: i32,        // extra flags for readfile()
) -> i32 {
    let mut retval = OK;
    let mut old_curbuf = bufref_T::default();
    let old_tw = (*curbuf).b_p_tw;
    let mut read_fifo = false;

    /*
     * The 'readonly' flag is only set when BF_NEVERLOADED is being reset.
     * When re-entering the same buffer, it should not change, because the
     * user may have reset the flag by hand.
     */
    if readonlymode != 0 && !(*curbuf).b_ffname.is_null() && (*curbuf).b_flags & BF_NEVERLOADED != 0
    {
        (*curbuf).b_p_ro = true_0
    }
    if ml_open(curbuf) == FAIL {
        /*
         * There MUST be a memfile, otherwise we can't do anything
         * If we can't create one for the current buffer, take another buffer
         */
        close_buffer(ptr::null_mut(), curbuf, 0, false);

        curbuf = ptr::null_mut();
        FOR_ALL_BUFFERS!(buf, {
            if !(*buf).b_ml.ml_mfp.is_null() {
                curbuf = buf;
                break;
            }
        });

        /*
         * if there is no memfile at all, exit
         * This is OK, since there are no changes to lose.
         */
        if curbuf.is_null() {
            emsg(gettext("E82: Cannot allocate any buffer, exiting..."));
            getout(2);
        }
        emsg(gettext("E83: Cannot allocate buffer, using other one..."));
        enter_buffer(curbuf);
        if old_tw != (*curbuf).b_p_tw {
            check_colorcolumn(curwin);
        }
        return FAIL;
    }

    // The autocommands in readfile() may change the buffer, but only AFTER
    // reading the file.
    set_bufref(&mut old_curbuf, curbuf);
    modified_was_set = false_0;

    // mark cursor position as being invalid
    (*curwin).w_valid = 0;

    if !(*curbuf).b_ffname.is_null() {
        let old_msg_silent = msg_silent;
        let mut save_bin = None;
        if cfg!(unix) {
            save_bin = Some((*curbuf).b_p_bin);
            let perm: i32;
            perm = os_getperm((*curbuf).b_ffname as *const i8);
            // TODO: OPEN_CHR_FILES
            if perm >= 0 && (false || S_ISFIFO(perm) || S_ISSOCK(perm)) {
                read_fifo = true
            }
            if read_fifo {
                (*curbuf).b_p_bin = true_0
            }
        }
        if shortmess(SHM_FILEINFO) {
            msg_silent = 1
        }

        retval = readfile(
            (*curbuf).b_ffname,
            (*curbuf).b_fname,
            0,
            0,
            MAXLNUM,
            eap,
            flags | READ_NEW | if read_fifo { READ_FIFO } else { 0 },
        );

        if cfg!(unix) {
            if read_fifo {
                (*curbuf).b_p_bin = save_bin.unwrap();
                if retval == OK {
                    retval = read_buffer(false_0, eap, flags)
                }
            }
        }
        msg_silent = old_msg_silent;

        // Help buffer is filtered.
        if bt_help(curbuf) {
            fix_help_buffer();
        }
    } else if read_stdin != 0 {
        let save_bin = (*curbuf).b_p_bin;

        /*
         * First read the text in binary mode into the buffer.
         * Then read from that same buffer and append at the end.  This makes
         * it possible to retry when 'fileformat' or 'fileencoding' was
         * guessed wrong.
         */
        (*curbuf).b_p_bin = true_0;
        retval = readfile(
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            0,
            MAXLNUM,
            ptr::null_mut(),
            flags | READ_NEW + READ_STDIN,
        );
        (*curbuf).b_p_bin = save_bin;
        if retval == OK {
            retval = read_buffer(true_0, eap, flags)
        }
    }

    // if first time loading this buffer, init b_chartab[]
    if (*curbuf).b_flags & BF_NEVERLOADED != 0 {
        buf_init_chartab(curbuf, false_0);
        parse_cino(curbuf);
    }

    // Set/reset the Changed flag first, autocmds may change the buffer.
    // Apply the automatic commands, before processing the modelines.
    // So the modelines have priority over auto commands.

    // When reading stdin, the buffer contents always needs writing, so set
    // the changed flag.  Unless in readonly mode: "ls | nvim -R -".
    // When interrupted and 'cpoptions' contains 'i' set changed flag.
    if got_int != 0 && !vim_strchr(p_cpo, CPO_INTMOD).is_null()
        || modified_was_set != 0          // ":set modified" used in autocmd
        || aborting() != 0 && !vim_strchr(p_cpo, CPO_INTMOD).is_null()
    {
        changed(); // keep this fileformat
    } else if retval != FAIL && read_stdin == 0 && !read_fifo {
        unchanged(curbuf, false_0, true);
    }
    save_file_ff(curbuf); // keep this fileformat

    // Set last_changedtick to avoid triggering a TextChanged autocommand right
    // after it was added.
    (*curbuf).b_last_changedtick = buf_get_changedtick(curbuf);
    (*curbuf).b_last_changedtick_pum = buf_get_changedtick(curbuf);

    // require "!" to overwrite the file, because it wasn't read completely
    if aborting() != 0 {
        (*curbuf).b_flags |= BF_READERR
    }

    /* Need to update automatic folding.  Do this before the autocommands,
     * they may use the fold info. */
    foldUpdateAll(curwin);

    // need to set w_topline, unless some autocommand already did that.
    if (*curwin).w_valid & VALID_TOPLINE == 0 {
        (*curwin).w_topline = 1;
        (*curwin).w_topfill = 0
    }
    apply_autocmds_retval(
        EVENT_BUFENTER,
        ptr::null_mut(),
        ptr::null_mut(),
        false,
        curbuf,
        &mut retval,
    );

    if retval == FAIL {
        return FAIL;
    }

    /*
     * The autocommands may have changed the current buffer.  Apply the
     * modelines to the correct buffer, if it still exists and is loaded.
     */
    if bufref_valid(&mut old_curbuf) && !(*old_curbuf.br_buf).b_ml.ml_mfp.is_null() {
        let mut aco = aco_save_T::default();

        // Go to the buffer that was opened.
        aucmd_prepbuf(&mut aco, old_curbuf.br_buf);
        do_modelines(0);
        (*curbuf).b_flags &= !(BF_CHECK_RO | BF_NEVERLOADED);

        apply_autocmds_retval(
            EVENT_BUFWINENTER,
            ptr::null_mut(),
            ptr::null_mut(),
            false,
            curbuf,
            &mut retval,
        );

        // restore curwin/curbuf and a few other things
        aucmd_restbuf(&mut aco);
    }

    return retval;
}

/// Store "buf" in "bufref" and set the free count.
///
/// @param bufref Reference to be used for the buffer.
/// @param buf    The buffer to reference.
#[no_mangle]
pub unsafe extern "C" fn set_bufref(bufref: *mut bufref_T, buf: *mut buf_T) {
    let bufref = bufref.as_mut().unwrap();
    bufref.br_buf = buf;
    bufref.br_fnum = if let Some(buf) = buf.as_ref() {
        buf.handle
    } else {
        0
    };
    bufref.br_buf_free_count = buf_free_count;
}

/// Return true if "bufref->br_buf" points to the same buffer as when
/// set_bufref() was called and it is a valid buffer.
/// Only goes through the buffer list if buf_free_count changed.
/// Also checks if b_fnum is still the same, a :bwipe followed by :new might get
/// the same allocated memory, but it's a different buffer.
///
/// @param bufref Buffer reference to check for.
#[no_mangle]
pub unsafe extern "C" fn bufref_valid(bufref: *const bufref_T) -> bool {
    let bufref = bufref.as_ref().unwrap();
    bufref.br_buf_free_count == buf_free_count
        || buf_valid(bufref.br_buf) && bufref.br_fnum == (*bufref.br_buf).handle
}

/// Check that "buf" points to a valid buffer in the buffer list.
///
/// Can be slow if there are many buffers, prefer using bufref_valid().
///
/// @param buf The buffer to check for.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn buf_valid(buf: *mut buf_T) -> bool {
    if buf.is_null() {
        return false;
    }
    // Assume that we more often have a recent buffer,
    // start with the last one.
    let mut bp = lastbuf;
    while !bp.is_null() {
        if bp == buf {
            return true;
        }
        bp = (*bp).b_prev
    }
    return false;
}

/// Close the link to a buffer.
///
/// @param win    If not NULL, set b_last_cursor.
/// @param buf
/// @param action Used when there is no longer a window for the buffer.
///               Possible values:
///                 0            buffer becomes hidden
///                 DOBUF_UNLOAD buffer is unloaded
///                 DOBUF_DELETE buffer is unloaded and removed from buffer list
///                 DOBUF_WIPE   buffer is unloaded and really deleted
///               When doing all but the first one on the current buffer, the
///               caller should get a new buffer very soon!
///               The 'bufhidden' option can force freeing and deleting.
/// @param abort_if_last
///               If true, do not close the buffer if autocommands cause
///               there to be only one window with this buffer. e.g. when
///               ":quit" is supposed to close the window but autocommands
///               close all other windows.
#[no_mangle]
pub unsafe extern "C" fn close_buffer(
    mut win: *mut win_T,
    buf: *mut buf_T,
    action: i32,
    abort_if_last: bool,
) {
    let buf = buf.as_mut().unwrap();
    let mut unload_buf = action != 0;
    let mut del_buf = action == DOBUF_DEL || action == DOBUF_WIPE;
    let mut wipe_buf = action == DOBUF_WIPE;

    let is_curwin = !curwin.is_null() && (*curwin).w_buffer == buf;
    let the_curwin = curwin;
    let the_curtab = curtab;

    // Force unloading or deleting when 'bufhidden' says so, but not for terminal
    // buffers.
    // The caller must take care of NOT deleting/freeing when 'bufhidden' is
    // "hide" (otherwise we could never free or delete a buffer).
    if buf.terminal.is_null() {
        if *buf.b_p_bh.offset(0) == 'd' as u8 {
            // 'bufhidden' == "delete"
            del_buf = true;
            unload_buf = true
        } else if *buf.b_p_bh.offset(0) == 'w' as u8 {
            // 'bufhidden' == "wipe"
            del_buf = true;
            unload_buf = true;
            wipe_buf = true
        } else if *buf.b_p_bh.offset(0) == 'u' as u8 {
            // 'bufhidden' == "unload"
            unload_buf = true
        }
    }

    if !buf.terminal.is_null() && (unload_buf || del_buf || wipe_buf) {
        // terminal buffers can only be wiped
        unload_buf = true;
        del_buf = true;
        wipe_buf = true
    }

    // Disallow deleting the buffer when it is locked (already being closed or
    // halfway a command that relies on it). Unloading is allowed.
    if buf.b_locked > 0 && (del_buf || wipe_buf) {
        emsg(gettext("E937: Attempt to delete a buffer that is in use"));
        return;
    }
    if !win.is_null() && win_valid_any_tab(win) {
        // Set b_last_cursor when closing the last window for the buffer.
        // Remember the last cursor position and window options of the buffer.
        // This used to be only for the current window, but then options like
        // 'foldmethod' may be lost with a ":only" command.
        let win = win.as_mut().unwrap();
        if buf.b_nwindows == 1 {
            set_last_cursor(win);
        }
        buflist_setfpos(
            buf,
            win,
            if win.w_cursor.lnum == 1 {
                0
            } else {
                win.w_cursor.lnum
            },
            win.w_cursor.col,
            true,
        );
    }

    let mut bufref = bufref_T::default();
    set_bufref(&mut bufref, buf);

    // When the buffer is no longer in a window, trigger BufWinLeave
    if buf.b_nwindows == 1 {
        buf.b_locked += 1;
        if apply_autocmds(EVENT_BUFWINLEAVE, buf.b_fname, buf.b_fname, false, buf)
            && !bufref_valid(&mut bufref)
        {
            // Autocommands deleted the buffer.
            emsg(gettext(e_auabort));
            return;
        }
        buf.b_locked -= 1;
        if abort_if_last && last_nonfloat(win) {
            // Autocommands made this the only window.
            emsg(gettext(e_auabort));
            return;
        }

        // When the buffer becomes hidden, but is not unloaded, trigger
        // BufHidden
        if !unload_buf {
            buf.b_locked += 1;
            if apply_autocmds(EVENT_BUFHIDDEN, buf.b_fname, buf.b_fname, false, buf)
                && !bufref_valid(&mut bufref)
            {
                // Autocommands deleted the buffer.
                emsg(gettext(e_auabort));
                return;
            }
            buf.b_locked -= 1;
            if abort_if_last && last_nonfloat(win) {
                // Autocommands made this the only window.
                emsg(gettext(e_auabort));
                return;
            }
        }
        if aborting() != 0 {
            // autocmds may abort script processing
            return;
        }
    }

    // If the buffer was in curwin and the window has changed, go back to that
    // window, if it still exists.  This avoids that ":edit x" triggering a
    // "tabnext" BufUnload autocmd leaves a window behind without a buffer.
    if is_curwin && curwin != the_curwin && win_valid_any_tab(the_curwin) {
        block_autocmds();
        goto_tabpage_win(the_curtab, the_curwin);
        unblock_autocmds();
    }

    let nwindows = buf.b_nwindows;

    // decrease the link count from windows (unless not in any window)
    if buf.b_nwindows > 0 {
        buf.b_nwindows -= 1
    }

    if diffopt_hiddenoff() && !unload_buf && buf.b_nwindows == 0 {
        diff_buf_delete(buf); // Clear 'diff' for hidden buffer.
    }

    /* Return when a window is displaying the buffer or when it's not
     * unloaded. */
    if buf.b_nwindows > 0 || !unload_buf {
        return;
    }

    if !buf.terminal.is_null() {
        terminal_close(buf.terminal, ptr::null_mut());
    }

    // Always remove the buffer when there is no file name.
    if buf.b_ffname.is_null() {
        del_buf = true
    }

    /*
     * Free all things allocated for this buffer.
     * Also calls the "BufDelete" autocommands when del_buf is TRUE.
     */
    /* Remember if we are closing the current buffer.  Restore the number of
     * windows, so that autocommands in buf_freeall() don't get confused. */
    let is_curbuf = buf as *mut _ == curbuf;

    // When closing the current buffer stop Visual mode before freeing
    // anything.
    if is_curbuf && VIsual_active != 0 {
        end_visual_mode();
    }

    buf.b_nwindows = nwindows;

    buf_freeall(
        buf,
        if del_buf { BFA_DEL } else { 0 } + if wipe_buf { BFA_WIPE } else { 0 },
    );

    if !bufref_valid(&mut bufref) {
        // Autocommands may have deleted the buffer.
        return;
    }
    if aborting() != 0 {
        // Autocmds may abort script processing.
        return;
    }

    /*
     * It's possible that autocommands change curbuf to the one being deleted.
     * This might cause the previous curbuf to be deleted unexpectedly.  But
     * in some cases it's OK to delete the curbuf, because a new one is
     * obtained anyway.  Therefore only return if curbuf changed to the
     * deleted buffer.
     */
    if buf as *mut _ == curbuf && !is_curbuf {
        return;
    }

    if !win.is_null() && win_valid_any_tab(win) && (*win).w_buffer == buf {
        (*win).w_buffer = ptr::null_mut(); // make sure we don't use the buffer now
    }

    // Autocommands may have opened or closed windows for this buffer.
    // Decrement the count for the close we do here.
    if buf.b_nwindows > 0 {
        buf.b_nwindows -= 1;
    }

    // Change directories when the 'acd' option is set.
    do_autochdir();

    // Disable buffer-updates for the current buffer.
    // No need to check `unload_buf`: in that case the function returned above.
    buf_updates_unregister_all(buf);

    /*
     * Remove the buffer from the list.
     */
    if wipe_buf {
        xfree(buf.b_ffname);
        xfree(buf.b_sfname);
        if buf.b_prev.is_null() {
            firstbuf = buf.b_next;
        } else {
            (*buf.b_prev).b_next = buf.b_next;
        }
        if buf.b_next.is_null() {
            lastbuf = buf.b_prev;
        } else {
            (*buf.b_next).b_prev = buf.b_prev;
        }
        free_buffer(buf);
    } else {
        if del_buf {
            // Free all internal variables and reset option values, to make
            // ":bdel" compatible with Vim 5.7.
            free_buffer_stuff(buf, kBffClearWinInfo | kBffInitChangedtick);

            // Make it look like a new buffer.
            buf.b_flags = BF_CHECK_RO | BF_NEVERLOADED;

            // Init the options when loaded again.
            buf.b_p_initialized = false;
        }
        buf_clear_file(buf);
        if del_buf {
            buf.b_p_bl = false_0;
        }
    }
}

/// Make buffer not contain a file.
#[no_mangle]
pub unsafe extern "C" fn buf_clear_file(buf: *mut buf_T) {
    let buf = buf.as_mut().unwrap();
    buf.b_ml.ml_line_count = 1;
    unchanged(buf, true_0, true);
    buf.b_p_eol = true_0;
    buf.b_start_eol = true_0;
    buf.b_p_bomb = false_0;
    buf.b_start_bomb = false_0;
    buf.b_ml.ml_mfp = ptr::null_mut();
    buf.b_ml.ml_flags = ML_EMPTY; // empty buffer
}

/// Clears the current buffer contents.
#[no_mangle]
pub unsafe extern "C" fn buf_clear() {
    let line_count = (*curbuf).b_ml.ml_line_count;
    while (*curbuf).b_ml.ml_flags & ML_EMPTY == 0 {
        ml_delete(1, false);
    }
    deleted_lines_mark(1, line_count); // prepare for display
    ml_close(curbuf, true_0); // free memline_T
    buf_clear_file(curbuf);
}

/// buf_freeall() - free all things allocated for a buffer that are related to
/// the file.  Careful: get here with "curwin" NULL when exiting.
///
/// @param flags BFA_DEL buffer is going to be deleted
///              BFA_WIPE buffer is going to be wiped out
///              BFA_KEEP_UNDO  do not free undo information
#[no_mangle]
pub unsafe extern "C" fn buf_freeall(buf: *mut buf_T, flags: i32) {
    let buf = buf.as_mut().unwrap();
    let is_curbuf = buf as *mut _ == curbuf;
    let is_curwin = !curwin.is_null() && (*curwin).w_buffer == buf;
    let the_curwin = curwin;
    let the_curtab = curtab;

    // Make sure the buffer isn't closed by autocommands.
    buf.b_locked += 1;

    let mut bufref = bufref_T::default();
    set_bufref(&mut bufref, buf);

    if !buf.b_ml.ml_mfp.is_null()
        && apply_autocmds(EVENT_BUFUNLOAD, buf.b_fname, buf.b_fname, false, buf)
        && !bufref_valid(&mut bufref)
    {
        // Autocommands deleted the buffer.
        return;
    }
    if flags & BFA_DEL != 0
        && buf.b_p_bl != 0
        && apply_autocmds(EVENT_BUFDELETE, buf.b_fname, buf.b_fname, false, buf)
        && !bufref_valid(&mut bufref)
    {
        // Autocommands may delete the buffer.
        return;
    }
    if flags & BFA_WIPE != 0
        && apply_autocmds(EVENT_BUFWIPEOUT, buf.b_fname, buf.b_fname, false, buf)
        && !bufref_valid(&mut bufref)
    {
        // Autocommands may delete the buffer.
        return;
    }
    buf.b_locked -= 1;

    // If the buffer was in curwin and the window has changed, go back to that
    // window, if it still exists.  This avoids that ":edit x" triggering a
    // "tabnext" BufUnload autocmd leaves a window behind without a buffer.
    if is_curwin && curwin != the_curwin && win_valid_any_tab(the_curwin) {
        block_autocmds();
        goto_tabpage_win(the_curtab, the_curwin);
        unblock_autocmds();
    }
    if aborting() != 0 {
        // autocmds may abort script processing
        return;
    }

    /*
     * It's possible that autocommands change curbuf to the one being deleted.
     * This might cause curbuf to be deleted unexpectedly.  But in some cases
     * it's OK to delete the curbuf, because a new one is obtained anyway.
     * Therefore only return if curbuf changed to the deleted buffer.
     */
    if buf as *mut _ == curbuf && !is_curbuf {
        return;
    }
    diff_buf_delete(buf); // Can't use 'diff' for unloaded buffer.

    // Remove any ownsyntax, unless exiting.
    if !curwin.is_null() && (*curwin).w_buffer == buf {
        reset_synblock(curwin);
    }

    // No folds in an empty buffer.
    FOR_ALL_TAB_WINDOWS!(tp, win, {
        if (*win).w_buffer == buf {
            clearFolding(win);
        }
    });

    ml_close(buf, true_0); // close and delete the memline/memfile
    buf.b_ml.ml_line_count = 0; // no lines in buffer
    if flags & BFA_KEEP_UNDO == 0 {
        u_blockfree(buf); // free the memory allocated for undo
        u_clearall(buf); // reset all undo information
    }
    syntax_clear(&mut buf.b_s); // reset syntax info
    buf.b_flags &= !BF_READERR; // a read error is no longer relevant
}

/*
 * Free a buffer structure and the things it contains related to the buffer
 * itself (not the file, that must have been done already).
 */
#[no_mangle]
unsafe extern "C" fn free_buffer(buf: *mut buf_T) {
    let buf = buf.as_mut().unwrap();
    handle_unregister_buffer(buf);
    buf_free_count += 1;
    // b:changedtick uses an item in buf_T.
    free_buffer_stuff(buf, kBffClearWinInfo);
    if (*buf.b_vars).dv_refcount > DO_NOT_FREE_CNT {
        tv_dict_add(
            buf.b_vars,
            tv_dict_item_copy(&mut buf.changedtick_di as *mut _ as *mut dictitem_T),
        );
    }
    unref_var_dict(buf.b_vars);
    aubuflocal_remove(buf);
    tv_dict_unref(buf.additional_data);
    xfree(buf.b_prompt_text);
    callback_free(&mut buf.b_prompt_callback);
    callback_free(&mut buf.b_prompt_interrupt);
    clear_fmark(&mut buf.b_last_cursor);
    clear_fmark(&mut buf.b_last_insert);
    clear_fmark(&mut buf.b_last_change);
    for i in 0..NMARKS {
        free_fmark(buf.b_namedm[i]);
    }
    for i in 0..buf.b_changelistlen as usize {
        free_fmark(buf.b_changelist[i]);
    }
    if autocmd_busy != 0 {
        // Do not free the buffer structure while autocommands are executing,
        // it's still needed. Free it when autocmd_busy is reset.
        memset(buf.b_namedm.as_mut_ptr(), 0, var_size(buf.b_namedm));
        memset(buf.b_changelist.as_mut_ptr(), 0, var_size(buf.b_changelist));
        buf.b_next = au_pending_free_buf;
        au_pending_free_buf = buf;
    } else {
        xfree(buf);
    }
}

/// Free stuff in the buffer for ":bdel" and when wiping out the buffer.
///
/// @param buf  Buffer pointer
/// @param free_flags  BufFreeFlags
unsafe fn free_buffer_stuff(buf: *mut buf_T, free_flags: i32) {
    let buf = buf.as_mut().unwrap();
    if free_flags & kBffClearWinInfo != 0 {
        clear_wininfo(buf); // including window-local options
        free_buf_options(buf, true_0);
        ga_clear(&mut buf.b_s.b_langp);
    }

    // Avoid losing b:changedtick when deleting buffer: clearing variables
    // implies using clear_tv() on b:changedtick and that sets changedtick to
    // zero.
    let changedtick_hi = hash_find(&mut (*buf.b_vars).dv_hashtab, b"changedtick\x00".as_ptr());
    assert!(!changedtick_hi.is_null());
    hash_remove(&mut (*buf.b_vars).dv_hashtab, changedtick_hi);

    vars_clear(&mut (*buf.b_vars).dv_hashtab); // free all internal variables
    hash_init(&mut (*buf.b_vars).dv_hashtab);
    if free_flags & kBffInitChangedtick != 0 {
        buf_init_changedtick(buf);
    }
    uc_clear(&mut buf.b_ucmds); // clear local user commands
    buf_delete_signs(buf, b"*\x00".as_ptr()); // delete any signs
    extmark_free_all(buf); // delete any extmarks
    map_clear_int(buf, MAP_ALL_MODES, true_0, false_0); // clear local mappings
    map_clear_int(buf, MAP_ALL_MODES, true_0, true_0); // clear local abbrevs
    XFREE_CLEAR(&mut buf.b_start_fenc);

    buf_updates_unregister_all(buf);
}

/*
 * Free the b_wininfo list for buffer "buf".
 */
unsafe fn clear_wininfo(buf: *mut buf_T) {
    let buf = buf.as_mut().unwrap();

    while !buf.b_wininfo.is_null() {
        let wip = buf.b_wininfo;
        buf.b_wininfo = (*wip).wi_next;
        if (*wip).wi_optset {
            clear_winopt(&mut (*wip).wi_opt);
            deleteFoldRecurse(&mut (*wip).wi_folds);
        }
        xfree(wip);
    }
}

/*
 * Go to another buffer.  Handles the result of the ATTENTION dialog.
 */
#[no_mangle]
pub unsafe extern "C" fn goto_buffer(eap: *const exarg_T, start: i32, dir: i32, count: i32) {
    let eap = eap.as_ref().unwrap();
    let mut old_curbuf = bufref_T::default();
    set_bufref(&mut old_curbuf, curbuf);
    swap_exists_action = SEA_DIALOG;

    do_buffer(
        if *eap.cmd == 's' as u8 {
            DOBUF_SPLIT
        } else {
            DOBUF_GOTO
        },
        start,
        dir,
        count,
        eap.forceit,
    );

    if swap_exists_action == SEA_QUIT && *eap.cmd == 's' as u8 {
        let mut cs = cleanup_T::default();

        // Reset the error/interrupt/exception state here so that
        // aborting() returns false when closing a window.
        enter_cleanup(&mut cs);

        // Quitting means closing the split window, nothing else.
        win_close(curwin, true);
        swap_exists_action = SEA_NONE;
        swap_exists_did_quit = true_0;

        /* Restore the error/interrupt/exception state if not discarded by a
         * new aborting error, interrupt, or uncaught exception. */
        leave_cleanup(&mut cs);
    } else {
        handle_swap_exists(&mut old_curbuf);
    }
}

/// Handle the situation of swap_exists_action being set.
///
/// It is allowed for "old_curbuf" to be NULL or invalid.
///
/// @param old_curbuf The buffer to check for.
#[no_mangle]
pub unsafe extern "C" fn handle_swap_exists(old_curbuf: *mut bufref_T) {
    let mut cs = cleanup_T::default();
    let old_tw = (*curbuf).b_p_tw;
    let buf: *mut buf_T;

    if swap_exists_action == SEA_QUIT {
        // Reset the error/interrupt/exception state here so that
        // aborting() returns false when closing a buffer.
        enter_cleanup(&mut cs);

        // User selected Quit at ATTENTION prompt.  Go back to previous
        // buffer.  If that buffer is gone or the same as the current one,
        // open a new, empty buffer.
        swap_exists_action = SEA_NONE; // don't want it again
        swap_exists_did_quit = true_0;
        close_buffer(curwin, curbuf, DOBUF_UNLOAD, false);
        if old_curbuf.is_null() || !bufref_valid(old_curbuf) || (*old_curbuf).br_buf == curbuf {
            buf = buflist_new(ptr::null_mut(), ptr::null_mut(), 1, BLN_CURBUF | BLN_LISTED);
        } else {
            buf = (*old_curbuf).br_buf;
        }
        if !buf.is_null() {
            let old_msg_silent = msg_silent;

            if shortmess(SHM_FILEINFO) {
                msg_silent = 1; // prevent fileinfo message
            }
            enter_buffer(buf);
            // restore msg_silent, so that the command line will be shown
            msg_silent = old_msg_silent;

            if old_tw != (*curbuf).b_p_tw {
                check_colorcolumn(curwin);
            }
        }
        // If "old_curbuf" is NULL we are in big trouble here...

        /* Restore the error/interrupt/exception state if not discarded by a
         * new aborting error, interrupt, or uncaught exception. */
        leave_cleanup(&mut cs);
    } else if swap_exists_action == SEA_RECOVER {
        // Reset the error/interrupt/exception state here so that
        // aborting() returns false when closing a buffer.
        enter_cleanup(&mut cs);

        // User selected Recover at ATTENTION prompt.
        msg_scroll = true_0;
        ml_recover(false);
        msg_puts(b"\n\x00" as *const u8 as *const i8); // don't overwrite the last message
        cmdline_row = msg_row;
        do_modelines(0);

        /* Restore the error/interrupt/exception state if not discarded by a
         * new aborting error, interrupt, or uncaught exception. */
        leave_cleanup(&mut cs);
    }
    swap_exists_action = SEA_NONE; // -V519
}

/*
 * do_bufdel() - delete or unload buffer(s)
 *
 * addr_count == 0: ":bdel" - delete current buffer
 * addr_count == 1: ":N bdel" or ":bdel N [N ..]" - first delete
 *		    buffer "end_bnr", then any other arguments.
 * addr_count == 2: ":N,N bdel" - delete buffers in range
 *
 * command can be DOBUF_UNLOAD (":bunload"), DOBUF_WIPE (":bwipeout") or
 * DOBUF_DEL (":bdel")
 *
 * Returns error message or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn do_bufdel(
    command: i32,
    mut arg: *const u8, // pointer to extra arguments
    addr_count: i32,
    start_bnr: i32, // first buffer number in a range
    end_bnr: i32,   // buffer nr or last buffer nr in a range
    forceit: i32,
) -> *mut u8 {
    let mut do_current = 0; // delete current buffer?
    let mut deleted = 0; // number of buffers deleted
    let mut errormsg = ptr::null_mut(); // return value
    let mut bnr: i32; // buffer number
    let mut p: *mut u8;

    if addr_count == 0 {
        do_buffer(command, DOBUF_CURRENT, FORWARD, 0, forceit);
    } else {
        if addr_count == 2 {
            if *arg != 0 {
                // both range and argument is not allowed
                return c_gettext(e_trailing.as_mut_ptr() as *mut i8) as *mut u8;
            }
            bnr = start_bnr
        } else {
            // addr_count == 1
            bnr = end_bnr
        }

        while got_int == 0 {
            /*
             * delete the current buffer last, otherwise when the
             * current buffer is deleted, the next buffer becomes
             * the current one and will be loaded, which may then
             * also be deleted, etc.
             */
            if bnr == (*curbuf).handle {
                do_current = bnr
            } else if do_buffer(command, DOBUF_FIRST, FORWARD, bnr, forceit) == OK {
                deleted += 1
            }

            /*
             * find next buffer number to delete/unload
             */
            if addr_count == 2 {
                bnr += 1;
                if bnr > end_bnr {
                    break;
                }
            } else {
                // addr_count == 1
                arg = skipwhite(arg);
                if *arg == 0 {
                    break;
                }
                if !ascii_isdigit(*arg) {
                    p = skiptowhite_esc(arg);
                    bnr = buflist_findpat(arg, p, (command == DOBUF_WIPE) as i32, false_0, false_0);
                    if bnr < 0 {
                        // failed
                        break;
                    }
                    arg = p;
                } else {
                    bnr = getdigits_int(&mut arg, false, 0);
                }
            }
            os_breakcheck();
        }
        if got_int == 0
            && do_current != 0
            && do_buffer(command, DOBUF_FIRST, FORWARD, do_current, forceit) == OK
        {
            deleted += 1;
        }

        if deleted == 0 {
            strcpy(
                IObuff.as_mut_ptr() as *mut i8,
                gettext(match command {
                    DOBUF_UNLOAD => "E515: No buffers were unloaded",
                    DOBUF_DEL => "E515: No buffers were unloaded",
                    _ => "E515: No buffers were unloaded",
                }) as *const i8,
            );
            errormsg = IObuff.as_mut_ptr()
        } else if deleted >= p_report {
            if deleted == 1 {
                msg(gettext(match command {
                    DOBUF_UNLOAD => "1 buffer unloaded",
                    DOBUF_DEL => "1 buffer deleted",
                    _ => "1 buffer wiped out",
                }));
            } else {
                smsg(
                    gettext(match command {
                        DOBUF_UNLOAD => "%d buffers unloaded",
                        DOBUF_DEL => "%d buffers deleted",
                        _ => "%d buffers wiped out",
                    }) as *const i8,
                    deleted,
                );
            }
        }
    }
    return errormsg;
}

/*
 * Make the current buffer empty.
 * Used when it is wiped out and it's the last buffer.
 */
unsafe fn empty_curbuf(close_others: i32, forceit: i32, action: i32) -> i32 {
    let retval: i32;
    let buf = curbuf;

    if action == DOBUF_UNLOAD {
        emsg(gettext("E90: Cannot unload last buffer"));
        return FAIL;
    }

    let mut bufref = bufref_T::default();
    set_bufref(&mut bufref, buf);

    if close_others != 0 {
        // Close any other windows on this buffer, then make it empty.
        close_windows(buf, true_0);
    }

    setpcmark();
    retval = do_ecmd(
        0,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        ECMD_ONE,
        if forceit != 0 { ECMD_FORCEIT } else { 0 },
        curwin,
    );

    // do_ecmd() may create a new buffer, then we have to delete
    // the old one.  But do_ecmd() may have done that already, check
    // if the buffer still exists.
    if buf != curbuf && bufref_valid(&mut bufref) && (*buf).b_nwindows == 0 {
        close_buffer(ptr::null_mut(), buf, action, false);
    }

    if close_others == 0 {
        need_fileinfo = false_0
    }

    return retval;
}
/*
 * Implementation of the commands for the buffer list.
 *
 * action == DOBUF_GOTO	    go to specified buffer
 * action == DOBUF_SPLIT    split window and go to specified buffer
 * action == DOBUF_UNLOAD   unload specified buffer(s)
 * action == DOBUF_DEL	    delete specified buffer(s) from buffer list
 * action == DOBUF_WIPE	    delete specified buffer(s) really
 *
 * start == DOBUF_CURRENT   go to "count" buffer from current buffer
 * start == DOBUF_FIRST	    go to "count" buffer from first buffer
 * start == DOBUF_LAST	    go to "count" buffer from last buffer
 * start == DOBUF_MOD	    go to "count" modified buffer from current buffer
 *
 * Return FAIL or OK.
 */
#[no_mangle]
pub unsafe extern "C" fn do_buffer(
    action: i32,
    start: i32,
    dir: i32,       // FORWARD or BACKWARD
    mut count: i32, // buffer number or number of buffers
    forceit: i32,   // true for :...!
) -> i32 {
    let mut buf: *mut buf_T;
    let mut bp: *mut buf_T;
    let unload = action == DOBUF_UNLOAD || action == DOBUF_DEL || action == DOBUF_WIPE;

    match start {
        DOBUF_FIRST => buf = firstbuf,
        DOBUF_LAST => buf = lastbuf,
        _ => buf = curbuf,
    }
    if start == DOBUF_MOD {
        // find next modified buffer
        loop {
            let while_test = count > 0;
            count = count - 1;
            if !while_test {
                break;
            }
            loop {
                buf = (*buf).b_next;
                if buf.is_null() {
                    buf = firstbuf
                }
                if !(buf != curbuf && !bufIsChanged(buf)) {
                    break;
                }
            }
        }
        if !bufIsChanged(buf) {
            emsg(gettext("E84: No modified buffer found"));
            return FAIL;
        }
    } else if start == DOBUF_FIRST && count != 0 {
        // find specified buffer number
        while !buf.is_null() && (*buf).handle != count {
            buf = (*buf).b_next
        }
    } else {
        bp = ptr::null_mut();
        while count > 0 || !unload && (*buf).b_p_bl == 0 && bp != buf {
            /* remember the buffer where we start, we come back there when all
             * buffers are unlisted. */
            if bp.is_null() {
                bp = buf
            }
            if dir == FORWARD {
                buf = (*buf).b_next;
                if buf.is_null() {
                    buf = firstbuf
                }
            } else {
                buf = (*buf).b_prev;
                if buf.is_null() {
                    buf = lastbuf
                }
            }
            // don't count unlisted buffers
            if unload || (*buf).b_p_bl != 0 {
                count -= 1;
                bp = ptr::null_mut(); // use this buffer as new starting point
            }
            if bp == buf {
                // back where we started, didn't find anything.
                emsg(gettext("E85: There is no listed buffer"));
                return FAIL;
            }
        }
    }

    if buf.is_null() {
        // could not find it
        if start == DOBUF_FIRST {
            // don't warn when deleting
            if !unload {
                emsgf(c_gettext(e_nobufnr.as_ptr() as *mut i8), count);
            }
        } else if dir == FORWARD {
            emsg(gettext("E87: Cannot go beyond last buffer"));
        } else {
            emsg(gettext("E88: Cannot go before first buffer"));
        }
        return FAIL;
    }

    /*
     * delete buffer buf from memory and/or the list
     */
    if unload {
        let mut forward: i32;
        let mut bufref = bufref_T::default();
        set_bufref(&mut bufref, buf);

        /* When unloading or deleting a buffer that's already unloaded and
         * unlisted: fail silently. */
        if action != DOBUF_WIPE && (*buf).b_ml.ml_mfp.is_null() && (*buf).b_p_bl == 0 {
            return FAIL;
        }

        if forceit == 0 && (!(*buf).terminal.is_null() || bufIsChanged(buf)) {
            if (p_confirm != 0 || cmdmod.confirm) && p_write != 0 && (*buf).terminal.is_null() {
                dialog_changed(buf, false);
                if !bufref_valid(&mut bufref) {
                    // Autocommand deleted buffer, oops! It's not changed now.
                    return FAIL;
                }
                // If it's still changed fail silently, the dialog already
                // mentioned why it fails.
                if bufIsChanged(buf) {
                    return FAIL;
                }
            } else if !(*buf).terminal.is_null() {
                if p_confirm != 0 || cmdmod.confirm {
                    if !dialog_close_terminal(buf) {
                        return FAIL;
                    }
                } else {
                    emsgf(
                        gettext("E89: %s will be killed (add ! to override)"),
                        (*buf).b_fname as *mut i8,
                    );
                    return FAIL;
                }
            } else {
                emsgf(
                    gettext("E89: No write since last change for buffer %ld (add ! to override)")
                        as *const i8,
                    (*buf).handle,
                );
                return FAIL;
            }
        }

        // When closing the current buffer stop Visual mode.
        if buf == curbuf && VIsual_active != 0 {
            end_visual_mode();
        }

        /*
         * If deleting the last (listed) buffer, make it empty.
         * The last (listed) buffer cannot be unloaded.
         */
        bp = ptr::null_mut();
        FOR_ALL_BUFFERS!(bp2, {
            if (*bp2).b_p_bl != 0 && bp2 != buf {
                bp = bp2;
                break;
            }
        });
        if bp.is_null() && buf == curbuf {
            return empty_curbuf(true_0, forceit, action);
        }

        /*
         * If the deleted buffer is the current one, close the current window
         * (unless it's the only window).  Repeat this so long as we end up in
         * a window with this buffer.
         */
        while buf == curbuf
            && !((*curwin).w_closing || (*(*curwin).w_buffer).b_locked > 0)
            && (!ONE_WINDOW!() || !(*first_tabpage).tp_next.is_null())
        {
            if win_close(curwin, false) == FAIL {
                break;
            }
        }

        /*
         * If the buffer to be deleted is not the current one, delete it here.
         */
        if buf != curbuf {
            close_windows(buf, false_0);
            if buf != curbuf && bufref_valid(&mut bufref) && (*buf).b_nwindows <= 0 {
                close_buffer(ptr::null_mut(), buf, action, false);
            }
            return OK;
        }

        // Deleting the current buffer: Need to find another buffer to go to.
        // There should be another, otherwise it would have been handled
        // above.  However, autocommands may have deleted all buffers.
        // First use au_new_curbuf.br_buf, if it is valid.
        // Then prefer the buffer we most recently visited.
        // Else try to find one that is loaded, after the current buffer,
        // then before the current buffer.
        // Finally use any buffer.
        buf = ptr::null_mut(); // Selected buffer.
        bp = ptr::null_mut(); // Used when no loaded buffer found.
        if !au_new_curbuf.br_buf.is_null() && bufref_valid(&mut au_new_curbuf) {
            buf = au_new_curbuf.br_buf
        } else if (*curwin).w_jumplistlen > 0 {
            let mut jumpidx: i32;

            jumpidx = (*curwin).w_jumplistidx - 1;
            if jumpidx < 0 {
                jumpidx = (*curwin).w_jumplistlen - 1
            }

            forward = jumpidx;
            while jumpidx != (*curwin).w_jumplistidx {
                buf = buflist_findnr((*curwin).w_jumplist[jumpidx as usize].fmark.fnum);
                if !buf.is_null() {
                    if buf == curbuf || (*buf).b_p_bl == 0 {
                        buf = ptr::null_mut() // skip current and unlisted bufs
                    } else if (*buf).b_ml.ml_mfp.is_null() {
                        // skip unloaded buf, but may keep it for later
                        if bp.is_null() {
                            bp = buf
                        }
                        buf = ptr::null_mut()
                    }
                }
                if !buf.is_null() {
                    // found a valid buffer: stop searching
                    break;
                }
                // advance to older entry in jump list
                if jumpidx == 0 && (*curwin).w_jumplistidx == (*curwin).w_jumplistlen {
                    break;
                }
                jumpidx -= 1;
                if jumpidx < 0 {
                    jumpidx = (*curwin).w_jumplistlen - 1;
                }
                if jumpidx == forward {
                    // List exhausted for sure
                    break;
                }
            }
        }

        if buf.is_null() {
            // No previous buffer, Try 2'nd approach
            forward = true_0;
            buf = (*curbuf).b_next;
            loop {
                if buf.is_null() {
                    if forward == 0 {
                        // tried both directions
                        break;
                    }
                    buf = (*curbuf).b_prev;
                    forward = false_0;
                    continue;
                }
                // in non-help buffer, try to skip help buffers, and vv
                if (*buf).b_help == (*curbuf).b_help && (*buf).b_p_bl != 0 {
                    if !(*buf).b_ml.ml_mfp.is_null() {
                        // found loaded buffer
                        break;
                    }
                    if bp.is_null() {
                        // remember unloaded buf for later
                        bp = buf;
                    }
                }
                if forward != 0 {
                    buf = (*buf).b_next
                } else {
                    buf = (*buf).b_prev
                }
            }
        }
        if buf.is_null() {
            // No loaded buffer, use unloaded one
            buf = bp
        }
        if buf.is_null() {
            // No loaded buffer, find listed one
            FOR_ALL_BUFFERS!(buf2, {
                if (*buf2).b_p_bl != 0 && buf2 != curbuf {
                    buf = buf2;
                    break;
                }
            });
        }
        if buf.is_null() {
            // Still no buffer, just take one
            if !(*curbuf).b_next.is_null() {
                buf = (*curbuf).b_next
            } else {
                buf = (*curbuf).b_prev
            }
        }
    }

    if buf.is_null() {
        /* Autocommands must have wiped out all other buffers.  Only option
         * now is to make the current buffer empty. */
        return empty_curbuf(false_0, forceit, action);
    }

    /*
     * make buf current buffer
     */
    if action == DOBUF_SPLIT {
        // split window first
        // If 'switchbuf' contains "useopen": jump to first window containing
        // "buf" if one exists
        if swb_flags & SWB_USEOPEN != 0 && !buf_jump_open_win(buf).is_null() {
            return OK;
        }
        // If 'switchbuf' contains "usetab": jump to first window in any tab
        // page containing "buf" if one exists
        if swb_flags & SWB_USETAB != 0 && !buf_jump_open_tab(buf).is_null() {
            return OK;
        }
        if win_split(0, 0) == FAIL {
            return FAIL;
        }
    }
    // go to current buffer - nothing to do
    if buf == curbuf {
        return OK;
    }

    /*
     * Check if the current buffer may be abandoned.
     */
    if action == DOBUF_GOTO && !can_abandon(curbuf, forceit) {
        if (p_confirm != 0 || cmdmod.confirm) && p_write != 0 {
            let mut bufref = bufref_T::default();
            set_bufref(&mut bufref, buf);
            dialog_changed(curbuf, false);
            if !bufref_valid(&mut bufref) {
                // Autocommand deleted buffer, oops!
                return FAIL;
            }
        }
        if bufIsChanged(curbuf) {
            no_write_message();
            return FAIL;
        }
    }

    // Go to the other buffer.
    set_curbuf(buf, action);

    if action == DOBUF_SPLIT {
        RESET_BINDING(curwin); // reset 'scrollbind' and 'cursorbind'
    }
    if aborting() != 0 {
        // autocmds may abort script processing
        return FAIL;
    }

    return OK;
}

/*
 * Set current buffer to "buf".  Executes autocommands and closes current
 * buffer.  "action" tells how to close the current buffer:
 * DOBUF_GOTO	    free or hide it
 * DOBUF_SPLIT	    nothing
 * DOBUF_UNLOAD	    unload it
 * DOBUF_DEL	    delete it
 * DOBUF_WIPE	    wipe it out
 */
#[no_mangle]
pub unsafe extern "C" fn set_curbuf(buf: *mut buf_T, action: i32) {
    let prevbuf: *mut buf_T;
    let unload = action == DOBUF_UNLOAD || action == DOBUF_DEL || action == DOBUF_WIPE;
    let old_tw = (*curbuf).b_p_tw;

    setpcmark();
    if !cmdmod.keepalt {
        (*curwin).w_alt_fnum = (*curbuf).handle; // remember alternate file
    }
    buflist_altfpos(curwin); // remember curpos

    // Don't restart Select mode after switching to another buffer.
    VIsual_reselect = false_0;

    // close_windows() or apply_autocmds() may change curbuf and wipe out "buf"
    prevbuf = curbuf;
    let mut newbufref = bufref_T::default();
    let mut prevbufref = bufref_T::default();
    set_bufref(&mut prevbufref, prevbuf);
    set_bufref(&mut newbufref, buf);

    // Autocommands may delete the curren buffer and/or the buffer we wan to go
    // to.  In those cases don't close the buffer.
    if !apply_autocmds(
        EVENT_BUFLEAVE,
        ptr::null_mut(),
        ptr::null_mut(),
        false,
        curbuf,
    ) || bufref_valid(&mut prevbufref) && bufref_valid(&mut newbufref) && aborting() == 0
    {
        if prevbuf == (*curwin).w_buffer {
            reset_synblock(curwin);
        }
        if unload {
            close_windows(prevbuf, false_0);
        }
        if bufref_valid(&mut prevbufref) && aborting() == 0 {
            let previouswin = curwin;
            if prevbuf == curbuf {
                u_sync(false_0);
            }
            close_buffer(
                if prevbuf == (*curwin).w_buffer {
                    curwin
                } else {
                    ptr::null_mut()
                },
                prevbuf,
                if unload {
                    action
                } else if action == DOBUF_GOTO && !buf_hide(prevbuf) && !bufIsChanged(prevbuf) {
                    DOBUF_UNLOAD
                } else {
                    0
                },
                false,
            );
            if curwin != previouswin && win_valid(previouswin) {
                // autocommands changed curwin, Grr!
                curwin = previouswin;
            }
        }
    }
    /* An autocommand may have deleted "buf", already entered it (e.g., when
     * it did ":bunload") or aborted the script processing!
     * If curwin->w_buffer is null, enter_buffer() will make it valid again */
    if buf_valid(buf) && buf != curbuf && aborting() == 0 || (*curwin).w_buffer.is_null() {
        enter_buffer(buf);
        if old_tw != (*curbuf).b_p_tw {
            check_colorcolumn(curwin);
        }
    }

    if bufref_valid(&mut prevbufref) && !(*prevbuf).terminal.is_null() {
        terminal_check_size((*prevbuf).terminal);
    }
}

/*
 * Enter a new current buffer.
 * Old curbuf must have been abandoned already!  This also means "curbuf" may
 * be pointing to freed memory.
 */
#[no_mangle]
pub unsafe extern "C" fn enter_buffer(buf: *mut buf_T) {
    // Copy buffer and window local option values.  Not for a help buffer.
    buf_copy_options(buf, BCO_ENTER | BCO_NOHELP);
    if !(*buf).b_help {
        get_winopts(buf);
    } else {
        // Remove all folds in the window.
        clearFolding(curwin);
    }
    foldUpdateAll(curwin); // update folds (later).

    // Get the buffer in the current window.
    (*curwin).w_buffer = buf;
    curbuf = buf;
    (*curbuf).b_nwindows += 1;

    if (*curwin).w_onebuf_opt.wo_diff != 0 {
        diff_buf_add(curbuf);
    }

    (*curwin).w_s = &mut (*curbuf).b_s;

    // Cursor on first line by default.
    (*curwin).w_cursor.lnum = 1;
    (*curwin).w_cursor.col = 0;
    (*curwin).w_cursor.coladd = 0;
    (*curwin).w_set_curswant = true_0;
    (*curwin).w_topline_was_set = false_0 as i8;

    // mark cursor position as being invalid
    (*curwin).w_valid = 0;

    // Make sure the buffer is loaded.
    if (*curbuf).b_ml.ml_mfp.is_null() {
        // need to load the file
        // If there is no filetype, allow for detecting one.  Esp. useful for
        // ":ball" used in an autocommand.  If there already is a filetype we
        // might prefer to keep it.
        if *(*curbuf).b_p_ft == 0 {
            did_filetype = false_0;
        }

        open_buffer(false_0, ptr::null_mut(), 0);
    } else {
        if msg_silent == 0 && !shortmess(SHM_FILEINFO) {
            need_fileinfo = true_0; // display file info after redraw
        }
        // check if file changed
        buf_check_timestamp(curbuf, false_0);

        (*curwin).w_topline = 1;
        (*curwin).w_topfill = 0;
        apply_autocmds(
            EVENT_BUFENTER,
            ptr::null_mut(),
            ptr::null_mut(),
            false,
            curbuf,
        );
        apply_autocmds(
            EVENT_BUFWINENTER,
            ptr::null_mut(),
            ptr::null_mut(),
            false,
            curbuf,
        );
    }

    /* If autocommands did not change the cursor position, restore cursor lnum
     * and possibly cursor col. */
    if (*curwin).w_cursor.lnum == 1 && inindent(0) != 0 {
        buflist_getfpos();
    }

    check_arg_idx(curwin); // check for valid arg_idx
    maketitle();
    // when autocmds didn't change it
    if (*curwin).w_topline == 1 && (*curwin).w_topline_was_set == 0 {
        scroll_cursor_halfway(false_0); // redisplay at correct position
    }

    // Change directories when the 'acd' option is set.
    do_autochdir();

    if (*curbuf).b_kmap_state & KEYMAP_INIT != 0 {
        keymap_init();
    }
    // May need to set the spell language.  Can only do this after the buffer
    // has been properly setup.
    if !(*curbuf).b_help && (*curwin).w_onebuf_opt.wo_spell != 0 && *(*(*curwin).w_s).b_p_spl != 0 {
        did_set_spelllang(curwin);
    }

    redraw_later(NOT_VALID);
}

// Change to the directory of the current buffer.
// Don't do this while still starting up.
#[no_mangle]
pub unsafe extern "C" fn do_autochdir() {
    if p_acd != 0 {
        if starting == 0 && !(*curbuf).b_ffname.is_null() && vim_chdirfile((*curbuf).b_ffname) == OK
        {
            post_chdir(kCdScopeGlobal, false);
            shorten_fnames(true_0);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn no_write_message() {
    emsg(gettext(
        "E37: No write since last change (add ! to override)",
    ));
}

#[no_mangle]
pub unsafe extern "C" fn no_write_message_nobang() {
    emsg(gettext("E37: No write since last change"));
}

//
// functions for dealing with the buffer list
//

///highest file number
static mut top_file_num: i32 = 1;

/// Initialize b:changedtick and changedtick_val attribute
///
/// @param[out]  buf  Buffer to intialize for.
#[inline(always)]
unsafe fn buf_init_changedtick(buf: *mut buf_T) {
    let buf = buf.as_mut().unwrap();
    buf.changedtick_di = ChangedtickDictItem {
        di_flags: DI_FLAGS_RO | DI_FLAGS_FIX, // Must not include DI_FLAGS_ALLOC.
        di_tv: typval_T {
            v_type: VAR_NUMBER,
            v_lock: VAR_FIXED,
            vval: typval_vval_union {
                v_number: buf_get_changedtick(buf),
            },
        },
        di_key: *b"changedtick\x00",
    };
    tv_dict_add(
        buf.b_vars,
        &mut buf.changedtick_di as *mut _ as *mut dictitem_T,
    );
}

/// Add a file name to the buffer list.
/// If the same file name already exists return a pointer to that buffer.
/// If it does not exist, or if fname == NULL, a new entry is created.
/// If (flags & BLN_CURBUF) is true, may use current buffer.
/// If (flags & BLN_LISTED) is true, add new buffer to buffer list.
/// If (flags & BLN_DUMMY) is true, don't count it as a real buffer.
/// If (flags & BLN_NEW) is true, don't use an existing buffer.
/// If (flags & BLN_NOOPT) is true, don't copy options from the current buffer
///                                 if the buffer already exists.
/// This is the ONLY way to create a new buffer.
///
/// @param ffname full path of fname or relative
/// @param sfname short fname or NULL
/// @param lnum   preferred cursor line
/// @param flags  BLN_ defines
/// @param bufnr
///
/// @return pointer to the buffer
#[no_mangle]
pub unsafe extern "C" fn buflist_new(
    mut ffname: *mut u8,
    mut sfname: *mut u8,
    lnum: linenr_T,
    flags: i32,
) -> *mut buf_T {
    let mut buf: *mut buf_T = ptr::null_mut();

    fname_expand(curbuf, &mut ffname, &mut sfname); // will allocate ffname

    /*
     * If file name already exists in the list, update the entry.
     */
    /* We can use inode numbers when the file exists.  Works better
     * for hard links. */
    let mut file_id = FileID::default();
    let file_id_valid = !sfname.is_null() && os_fileid(sfname as *mut i8, &mut file_id);
    if !ffname.is_null() && flags & (BLN_DUMMY | BLN_NEW) == 0 && {
        buf = buflist_findname_file_id(ffname, &mut file_id, file_id_valid);
        !buf.is_null()
    } {
        xfree(ffname);
        if lnum != 0 {
            buflist_setfpos(buf, curwin, lnum, 0, false);
        }
        if flags & BLN_NOOPT == 0 {
            // Copy the options now, if 'cpo' doesn't have 's' and not done already.
            buf_copy_options(buf, 0);
        }
        if flags & BLN_LISTED != 0 && (*buf).b_p_bl == 0 {
            (*buf).b_p_bl = true_0;
            let mut bufref = bufref_T::default();
            set_bufref(&mut bufref, buf);
            if flags & BLN_DUMMY == 0 {
                if apply_autocmds(EVENT_BUFADD, ptr::null_mut(), ptr::null_mut(), false, buf)
                    && !bufref_valid(&mut bufref)
                {
                    return ptr::null_mut();
                }
            }
        }
        return buf;
    }

    /*
     * If the current buffer has no name and no contents, use the current
     * buffer.	Otherwise: Need to allocate a new buffer structure.
     *
     * This is the ONLY place where a new buffer structure is allocated!
     * (A spell file buffer is allocated in spell.c, but that's not a normal
     * buffer.)
     */
    buf = ptr::null_mut();
    if flags & BLN_CURBUF != 0 && curbuf_reusable() {
        assert!(!curbuf.is_null());
        buf = curbuf;
        /* It's like this buffer is deleted.  Watch out for autocommands that
         * change curbuf!  If that happens, allocate a new buffer anyway. */
        if (*curbuf).b_p_bl != 0 {
            apply_autocmds(
                EVENT_BUFDELETE,
                ptr::null_mut(),
                ptr::null_mut(),
                false,
                curbuf,
            );
        }
        if buf == curbuf {
            apply_autocmds(
                EVENT_BUFWIPEOUT,
                ptr::null_mut(),
                ptr::null_mut(),
                false,
                curbuf,
            );
        }
        if aborting() != 0 {
            // autocmds may abort script processing
            return ptr::null_mut();
        }
        if buf == curbuf {
            // Make sure 'bufhidden' and 'buftype' are empty
            clear_string_option(&mut (*buf).b_p_bh);
            clear_string_option(&mut (*buf).b_p_bt);
        }
    }
    if buf != curbuf || curbuf.is_null() {
        buf = xcalloc(1, std::mem::size_of::<buf_T>());
        // init b: variables
        (*buf).b_vars = tv_dict_alloc();
        (*buf).b_signcols_max = -1;
        init_var_dict((*buf).b_vars, &mut (*buf).b_bufvar, VAR_SCOPE as i32);
        buf_init_changedtick(buf);
    }

    if !ffname.is_null() {
        (*buf).b_ffname = ffname;
        (*buf).b_sfname = vim_strsave(sfname)
    }

    clear_wininfo(buf);
    (*buf).b_wininfo = xcalloc(1, std::mem::size_of::<wininfo_T>());

    if !ffname.is_null() && ((*buf).b_ffname.is_null() || (*buf).b_sfname.is_null()) {
        XFREE_CLEAR(&mut (*buf).b_ffname);
        XFREE_CLEAR(&mut (*buf).b_sfname);
        if buf != curbuf {
            free_buffer(buf);
        }
        return ptr::null_mut();
    }

    if buf == curbuf {
        // free all things allocated for this buffer
        buf_freeall(buf, 0);
        if buf != curbuf {
            // autocommands deleted the buffer!
            return ptr::null_mut();
        }
        if aborting() != 0 {
            // autocmds may abort script processing
            return ptr::null_mut();
        }
        free_buffer_stuff(buf, kBffInitChangedtick); // delete local vars et al.

        // Init the options.
        (*buf).b_p_initialized = false;
        buf_copy_options(buf, BCO_ENTER);

        // need to reload lmaps and set b:keymap_name
        (*curbuf).b_kmap_state |= KEYMAP_INIT;
    } else {
        /*
         * put new buffer at the end of the buffer list
         */
        (*buf).b_next = ptr::null_mut();
        if firstbuf.is_null() {
            // buffer list is empty
            (*buf).b_prev = ptr::null_mut();
            firstbuf = buf
        } else {
            // append new buffer at end of list
            (*lastbuf).b_next = buf;
            (*buf).b_prev = lastbuf
        }
        lastbuf = buf;

        (*buf).handle = top_file_num;
        top_file_num += 1;
        handle_register_buffer(buf);
        if top_file_num < 0 {
            // wrap around (may cause duplicates)
            emsg(gettext("W14: Warning: List of file names overflow"));
            if emsg_silent == 0 {
                ui_flush();
                os_delay(3000, true); // make sure it is noticed
            }
            top_file_num = 1
        }

        /*
         * Always copy the options from the current buffer.
         */
        buf_copy_options(buf, BCO_ALWAYS);
    }
    (*(*buf).b_wininfo).wi_fpos.lnum = lnum;
    (*(*buf).b_wininfo).wi_win = curwin;

    hash_init(&mut (*buf).b_s.b_keywtab);
    hash_init(&mut (*buf).b_s.b_keywtab_ic);

    (*buf).b_fname = (*buf).b_sfname;
    if !file_id_valid {
        (*buf).file_id_valid = false
    } else {
        (*buf).file_id_valid = true;
        (*buf).file_id = file_id
    }
    (*buf).b_u_synced = true;
    (*buf).b_flags = BF_CHECK_RO | BF_NEVERLOADED;
    if flags & BLN_DUMMY != 0 {
        (*buf).b_flags |= BF_DUMMY
    }
    buf_clear_file(buf);
    clrallmarks(buf); // clear marks
    fmarks_check_names(buf); // check file marks for this file

    // init 'buflisted'
    (*buf).b_p_bl = if flags & BLN_LISTED != 0 {
        true_0
    } else {
        false_0
    };
    (*buf).update_channels.destroy();
    (*buf).update_channels.init();
    (*buf).update_callbacks.destroy();
    (*buf).update_callbacks.init();
    if flags & BLN_DUMMY as i32 == 0 {
        // Tricky: these autocommands may change the buffer list.  They could also
        // split the window with re-using the one empty buffer. This may result in
        // unexpectedly losing the empty buffer.
        let mut bufref = bufref_T::default();
        set_bufref(&mut bufref, buf);
        if apply_autocmds(EVENT_BUFNEW, ptr::null_mut(), ptr::null_mut(), false, buf)
            && !bufref_valid(&mut bufref)
        {
            return ptr::null_mut();
        }
        if flags & BLN_LISTED != 0
            && apply_autocmds(EVENT_BUFADD, ptr::null_mut(), ptr::null_mut(), false, buf)
            && !bufref_valid(&mut bufref)
        {
            return ptr::null_mut();
        }
        if aborting() != 0 {
            // Autocmds may abort script processing.
            return ptr::null_mut();
        }
    }

    (*buf).b_prompt_callback.type_0 = kCallbackNone;
    (*buf).b_prompt_interrupt.type_0 = kCallbackNone;
    (*buf).b_prompt_text = ptr::null_mut();

    return buf;
}

/// Return true if the current buffer is empty, unnamed, unmodified and used in
/// only one window. That means it can be reused.
#[no_mangle]
pub unsafe extern "C" fn curbuf_reusable() -> bool {
    return !curbuf.is_null()
        && (*curbuf).b_ffname.is_null()
        && (*curbuf).b_nwindows <= 1
        && ((*curbuf).b_ml.ml_mfp.is_null() || BUFEMPTY())
        && !bt_quickfix(curbuf)
        && !curbufIsChanged();
}

/*
 * Free the memory for the options of a buffer.
 * If "free_p_ff" is true also free 'fileformat', 'buftype' and
 * 'fileencoding'.
 */
#[no_mangle]
pub unsafe extern "C" fn free_buf_options(buf: *mut buf_T, free_p_ff: i32) {
    let buf = buf.as_mut().unwrap();
    if free_p_ff != 0 {
        clear_string_option(&mut buf.b_p_fenc);
        clear_string_option(&mut buf.b_p_ff);
        clear_string_option(&mut buf.b_p_bh);
        clear_string_option(&mut buf.b_p_bt);
    }
    clear_string_option(&mut buf.b_p_def);
    clear_string_option(&mut buf.b_p_inc);
    clear_string_option(&mut buf.b_p_inex);
    clear_string_option(&mut buf.b_p_inde);
    clear_string_option(&mut buf.b_p_indk);
    clear_string_option(&mut buf.b_p_fp);
    clear_string_option(&mut buf.b_p_fex);
    clear_string_option(&mut buf.b_p_kp);
    clear_string_option(&mut buf.b_p_mps);
    clear_string_option(&mut buf.b_p_fo);
    clear_string_option(&mut buf.b_p_flp);
    clear_string_option(&mut buf.b_p_isk);
    clear_string_option(&mut buf.b_p_keymap);
    keymap_ga_clear(&mut buf.b_kmap_ga);
    ga_clear(&mut buf.b_kmap_ga);
    clear_string_option(&mut buf.b_p_com);
    clear_string_option(&mut buf.b_p_cms);
    clear_string_option(&mut buf.b_p_nf);
    clear_string_option(&mut buf.b_p_syn);
    clear_string_option(&mut buf.b_s.b_syn_isk);
    clear_string_option(&mut buf.b_s.b_p_spc);
    clear_string_option(&mut buf.b_s.b_p_spf);
    vim_regfree(buf.b_s.b_cap_prog);
    buf.b_s.b_cap_prog = ptr::null_mut();
    clear_string_option(&mut buf.b_s.b_p_spl);
    clear_string_option(&mut buf.b_p_sua);
    clear_string_option(&mut buf.b_p_ft);
    clear_string_option(&mut buf.b_p_cink);
    clear_string_option(&mut buf.b_p_cino);
    clear_string_option(&mut buf.b_p_cinw);
    clear_string_option(&mut buf.b_p_cpt);
    clear_string_option(&mut buf.b_p_cfu);
    clear_string_option(&mut buf.b_p_ofu);
    clear_string_option(&mut buf.b_p_gp);
    clear_string_option(&mut buf.b_p_mp);
    clear_string_option(&mut buf.b_p_efm);
    clear_string_option(&mut buf.b_p_ep);
    clear_string_option(&mut buf.b_p_path);
    clear_string_option(&mut buf.b_p_tags);
    clear_string_option(&mut buf.b_p_tc);
    clear_string_option(&mut buf.b_p_tfu);
    clear_string_option(&mut buf.b_p_dict);
    clear_string_option(&mut buf.b_p_tsr);
    clear_string_option(&mut buf.b_p_qe);
    buf.b_p_ar = -1;
    buf.b_p_ul = NO_LOCAL_UNDOLEVEL;
    clear_string_option(&mut buf.b_p_lw);
    clear_string_option(&mut buf.b_p_bkc);
    clear_string_option(&mut buf.b_p_menc);
}

/// Get alternate file "n".
/// Set linenr to "lnum" or altfpos.lnum if "lnum" == 0.
/// Also set cursor column to altfpos.col if 'startofline' is not set.
/// if (options & GETF_SETMARK) call setpcmark()
/// if (options & GETF_ALT) we are jumping to an alternate file.
/// if (options & GETF_SWITCH) respect 'switchbuf' settings when jumping
///
/// Return FAIL for failure, OK for success.
#[no_mangle]
pub unsafe extern "C" fn buflist_getfile(
    n: i32,
    mut lnum: linenr_T,
    options: i32,
    forceit: i32,
) -> i32 {
    let buf: *mut buf_T;
    let mut wp = ptr::null_mut();
    let col: colnr_T;

    buf = buflist_findnr(n);
    if buf.is_null() {
        if options & GETF_ALT != 0 && n == 0 {
            emsg(c_gettext(e_noalt.as_ptr() as *const i8) as *mut u8);
        } else {
            emsgf(gettext("E92: Buffer %ld not found"), n as i64);
        }
        return FAIL;
    }

    // if alternate file is the current buffer, nothing to do
    if buf == curbuf {
        return OK;
    }

    if text_locked() != 0 {
        text_locked_msg();
        return FAIL;
    }
    if curbuf_locked() != 0 {
        return FAIL;
    }

    // altfpos may be changed by getfile(), get it now
    if lnum == 0 {
        let fpos: &pos_T = buflist_findfpos(buf).as_ref().unwrap();
        lnum = fpos.lnum;
        col = fpos.col;
    } else {
        col = 0;
    }

    if options & GETF_SWITCH != 0 {
        // If 'switchbuf' contains "useopen": jump to first window containing
        // "buf" if one exists
        if swb_flags & SWB_USEOPEN != 0 {
            wp = buf_jump_open_win(buf)
        }

        // If 'switchbuf' contains "usetab": jump to first window in any tab
        // page containing "buf" if one exists
        if wp.is_null() && swb_flags & SWB_USETAB != 0 {
            wp = buf_jump_open_tab(buf)
        }

        // If 'switchbuf' contains "split", "vsplit" or "newtab" and the
        // current buffer isn't empty: open new tab or window
        if wp.is_null() && swb_flags & (SWB_VSPLIT | SWB_SPLIT | SWB_NEWTAB) != 0 && !BUFEMPTY() {
            if swb_flags & SWB_NEWTAB != 0 {
                tabpage_new();
            } else if win_split(
                0,
                if swb_flags & SWB_VSPLIT != 0 {
                    WSP_VERT
                } else {
                    0
                },
            ) == FAIL
            {
                return FAIL;
            }
            RESET_BINDING(curwin);
        }
    }

    RedrawingDisabled += 1;
    if GETFILE_SUCCESS(getfile(
        (*buf).handle,
        0 as *mut u8,
        0 as *mut u8,
        options & GETF_SETMARK,
        lnum,
        forceit,
    )) {
        RedrawingDisabled -= 1;

        // cursor is at to BOL and w_cursor.lnum is checked due to getfile()
        if p_sol == 0 && col != 0 {
            (*curwin).w_cursor.col = col;
            check_cursor_col();
            (*curwin).w_cursor.coladd = 0;
            (*curwin).w_set_curswant = true_0
        }
        return OK;
    }
    RedrawingDisabled -= 1;
    return FAIL;
}

// Go to the last known line number for the current buffer.
#[no_mangle]
pub unsafe extern "C" fn buflist_getfpos() {
    let fpos: *mut pos_T;

    fpos = buflist_findfpos(curbuf);

    (*curwin).w_cursor.lnum = (*fpos).lnum;
    check_cursor_lnum();

    if p_sol != 0 {
        (*curwin).w_cursor.col = 0
    } else {
        (*curwin).w_cursor.col = (*fpos).col;
        check_cursor_col();
        (*curwin).w_cursor.coladd = 0;
        (*curwin).w_set_curswant = true_0;
    }
}

/*
 * Find file in buffer list by name (it has to be for the current window).
 * Returns NULL if not found.
 */
#[no_mangle]
pub unsafe extern "C" fn buflist_findname_exp(fname: *mut u8) -> *mut buf_T {
    let ffname: *mut u8;
    let mut buf = ptr::null_mut();

    // First make the name into a full path name
    ffname = FullName_save(
        fname as *mut i8,
        cfg!(unix), // force expansion, get rid of symbolic links
    ) as *mut u8;
    if !ffname.is_null() {
        buf = buflist_findname(ffname);
        xfree(ffname);
    }
    return buf;
}

/*
 * Find file in buffer list by name (it has to be for the current window).
 * "ffname" must have a full path.
 * Skips dummy buffers.
 * Returns NULL if not found.
 */
#[no_mangle]
pub unsafe extern "C" fn buflist_findname(ffname: *mut u8) -> *mut buf_T {
    let mut file_id = FileID::default();
    let file_id_valid = os_fileid(ffname as *mut i8, &mut file_id);
    return buflist_findname_file_id(ffname, &mut file_id, file_id_valid);
}

/*
 * Same as buflist_findname(), but pass the FileID structure to avoid
 * getting it twice for the same file.
 * Returns NULL if not found.
 */
unsafe extern "C" fn buflist_findname_file_id(
    ffname: *mut u8,
    file_id: *mut FileID,
    file_id_valid: bool,
) -> *mut buf_T {
    // Start at the last buffer, expect to find a match sooner.
    FOR_ALL_BUFFERS_BACKWARDS!(buf, {
        if (*buf).b_flags & BF_DUMMY == 0 && !otherfile_buf(buf, ffname, file_id, file_id_valid) {
            return buf;
        }
    });
    return ptr::null_mut();
}

/// Find file in buffer list by a regexp pattern.
/// Return fnum of the found buffer.
/// Return < 0 for error.
#[no_mangle]
pub unsafe extern "C" fn buflist_findpat(
    pattern: *const u8,
    pattern_end: *const u8, // find buffers in current tab only
    unlisted: i32,          // find unlisted buffers
    diffmode: i32,          // find diff-mode buffers only
    curtab_only: i32,       // find buffers in current tab only
) -> i32 {
    let mut match_0 = -1;
    let mut find_listed: i32;
    let pat: *mut u8;
    let patend: *mut u8;
    let mut p: *mut u8;

    if pattern_end == pattern.offset(1) && (*pattern == '%' as u8 || *pattern == '#' as u8) {
        if *pattern == '%' as u8 {
            match_0 = (*curbuf).handle
        } else {
            match_0 = (*curwin).w_alt_fnum
        }
        let found_buf = buflist_findnr(match_0);
        if diffmode != 0 && !(!found_buf.is_null() && diff_mode_buf(found_buf)) {
            match_0 = -1;
        }
    } else {
        //
        // Try four ways of matching a listed buffer:
        // attempt == 0: without '^' or '$' (at any position)
        // attempt == 1: with '^' at start (only at position 0)
        // attempt == 2: with '$' at end (only match at end)
        // attempt == 3: with '^' at start and '$' at end (only full match)
        // Repeat this for finding an unlisted buffer if there was no matching
        // listed buffer.
        //

        pat = file_pat_to_reg_pat(pattern, pattern_end, ptr::null_mut(), false_0);
        if pat.is_null() {
            return -1;
        }
        patend = pat.offset(strlen(pat as *mut i8) as isize - 1);
        let toggledollar = patend > pat && *patend == '$' as u8;

        // First try finding a listed buffer.  If not found and "unlisted"
        // is true, try finding an unlisted buffer.
        find_listed = true_0;
        loop {
            for attempt in 0..=3 {
                // may add '^' and '$'
                if toggledollar {
                    *patend = if attempt < 2 { 0 } else { '$' as u8 }; // add/remove '$'
                }
                p = pat;
                if *p == '^' as u8 && attempt & 1 == 0 {
                    // add/remove '^'
                    p = p.offset(1)
                }

                let mut regmatch = regmatch_T::default();
                regmatch.regprog = vim_regcomp(p, if p_magic != 0 { RE_MAGIC } else { 0 });
                if regmatch.regprog.is_null() {
                    xfree(pat);
                    return -1;
                }

                FOR_ALL_BUFFERS_BACKWARDS!(buf, {
                    if (*buf).b_p_bl == find_listed
                        && (diffmode == 0 || diff_mode_buf(buf))
                        && !buflist_match(&mut regmatch, buf, false).is_null()
                    {
                        if curtab_only != 0 {
                            /* Ignore the match if the buffer is not open in
                             * the current tab. */
                            let mut found_window = false;
                            FOR_ALL_WINDOWS_IN_TAB!(wp, curtab, {
                                if (*wp).w_buffer == buf {
                                    found_window = true;
                                    break;
                                }
                            });
                            if !found_window {
                                continue;
                            }
                        }
                        if match_0 >= 0 {
                            // already found a match
                            match_0 = -2;
                            break;
                        }
                        match_0 = (*buf).handle; // remember first match
                    }
                });

                vim_regfree(regmatch.regprog);
                if match_0 >= 0 {
                    break;
                }
            }

            /* Only search for unlisted buffers if there was no match with
             * a listed buffer. */
            if unlisted == 0 || find_listed == 0 || match_0 != -1 {
                break;
            }
            find_listed = false_0;
        }

        xfree(pat);
    }

    if match_0 == -2 {
        emsgf(gettext("E93: More than one match for %s"), pattern);
    } else if match_0 < 0 {
        emsgf(gettext("E94: No matching buffer for %s"), pattern);
    }
    return match_0;
}

/*
 * Find all buffer names that match.
 * For command line expansion of ":buf" and ":sbuf".
 * Return OK if matches found, FAIL otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn ExpandBufnames(
    pat: *mut u8,
    num_file: *mut i32,
    file: *mut *mut *mut u8,
    options: i32,
) -> i32 {
    let mut count = 0;
    let mut p: *mut u8;
    let patc: *mut u8;

    *num_file = 0; // return values in case of FAIL
    *file = ptr::null_mut();

    // Make a copy of "pat" and change "^" to "\(^\|[\/]\)".
    if *pat as i32 == '^' as i32 {
        patc = xmalloc(strlen(pat as *mut i8) + 11);
        strcpy(
            patc as *mut i8,
            b"\\(^\\|[\\/]\\)\x00" as *const u8 as *const i8,
        );
        strcpy(patc.offset(11) as *mut i8, pat.offset(1) as *mut i8);
    } else {
        patc = pat;
    }

    /*
     * attempt == 0: try match with    '\<', match at start of word
     * attempt == 1: try match without '\<', match anywhere
     */
    for attempt in 0..=1 {
        if attempt > 0 && patc == pat {
            break; // there was no anchor, no need to try again
        }

        let mut regmatch = regmatch_T::default();
        regmatch.regprog = vim_regcomp(patc.offset(attempt * 11), RE_MAGIC);
        if regmatch.regprog.is_null() {
            if patc != pat {
                xfree(patc);
            }
            return FAIL;
        }

        /*
         * round == 1: Count the matches.
         * round == 2: Build the array to keep the matches.
         */
        for round in 1..=2 {
            count = 0;
            FOR_ALL_BUFFERS!(buf, {
                if (*buf).b_p_bl == 0 {
                    // skip unlisted buffers
                    continue;
                }
                p = buflist_match(&mut regmatch, buf, p_wic != 0);
                if !p.is_null() {
                    if round == 1 {
                        count += 1;
                    } else {
                        if options & WILD_HOME_REPLACE != 0 {
                            p = home_replace_save(buf, p);
                        } else {
                            p = vim_strsave(p);
                        }
                        *(*file).offset(count) = p;
                        count += 1;
                    }
                }
            });
            if count == 0 {
                // no match found, break here
                break;
            }
            if round == 1 {
                *file = xmalloc(count * std::mem::size_of::<*mut u8>() as isize);
            }
        }
        vim_regfree(regmatch.regprog);
        if count != 0 {
            break;
        }
    }

    if patc != pat {
        xfree(patc);
    }

    *num_file = count as i32;
    return if count == 0 { FAIL } else { OK };
}

/// Check for a match on the file name for buffer "buf" with regprog "prog".
///
/// @param ignore_case When true, ignore case. Use 'fic' otherwise.
unsafe fn buflist_match(rmp: *mut regmatch_T, buf: *const buf_T, ignore_case: bool) -> *mut u8 {
    let buf = buf.as_ref().unwrap();
    // First try the short file name, then the long file name.
    let mut match_0 = fname_match(rmp, buf.b_sfname, ignore_case);
    if match_0.is_null() {
        match_0 = fname_match(rmp, buf.b_ffname, ignore_case)
    }
    return match_0;
}

/// Try matching the regexp in "prog" with file name "name".
///
/// @param ignore_case When true, ignore case. Use 'fileignorecase' otherwise.
/// @return "name" when there is a match, NULL when not.
unsafe fn fname_match(rmp: *mut regmatch_T, name: *mut u8, ignore_case: bool) -> *mut u8 {
    let rmp = rmp.as_mut().unwrap();
    let mut match_0 = ptr::null_mut();
    let p: *mut u8;

    if !name.is_null() {
        // Ignore case when 'fileignorecase' or the argument is set.
        rmp.rm_ic = p_fic != 0 || ignore_case;
        if vim_regexec(rmp, name, 0) != 0 {
            match_0 = name;
        } else {
            // Replace $(HOME) with '~' and try matching again.
            p = home_replace_save(ptr::null_mut(), name);
            if vim_regexec(rmp, p, 0) != 0 {
                match_0 = name;
            }
            xfree(p);
        }
    }

    return match_0;
}

/// Find a file in the buffer list by buffer number.
#[no_mangle]
pub unsafe extern "C" fn buflist_findnr(mut nr: i32) -> *mut buf_T {
    if nr == 0 {
        nr = (*curwin).w_alt_fnum
    }

    return handle_get_buffer(nr);
}

/*
 * Get name of file 'n' in the buffer list.
 * When the file has no name an empty string is returned.
 * home_replace() is used to shorten the file name (used for marks).
 * Returns a pointer to allocated memory, of NULL when failed.
 */
#[no_mangle]
pub unsafe extern "C" fn buflist_nr2name(
    n: i32,
    fullname: i32,
    helptail: i32, // for help buffers return tail only
) -> *mut u8 {
    let buf: *mut buf_T;

    buf = buflist_findnr(n);
    if buf.is_null() {
        return ptr::null_mut();
    }
    let buf = buf.as_mut().unwrap();
    return home_replace_save(
        if helptail != 0 { buf } else { ptr::null_mut() },
        if fullname != 0 {
            buf.b_ffname
        } else {
            buf.b_fname
        },
    );
}

/// Set the line and column numbers for the given buffer and window
///
/// @param[in,out]  buf           Buffer for which line and column are set.
/// @param[in,out]  win           Window for which line and column are set.
/// @param[in]      lnum          Line number to be set. If it is zero then only
///                               options are touched.
/// @param[in]      col           Column number to be set.
/// @param[in]      copy_options  If true save the local window option values.
#[no_mangle]
pub unsafe extern "C" fn buflist_setfpos(
    buf: *mut buf_T,
    win: *mut win_T,
    mut lnum: linenr_T,
    col: colnr_T,
    copy_options: bool,
) {
    let buf = buf.as_mut().unwrap();
    let win = win.as_mut().unwrap();
    let mut wip: *mut wininfo_T;

    wip = buf.b_wininfo;
    while !wip.is_null() {
        if (*wip).wi_win == win {
            break;
        }
        wip = (*wip).wi_next;
    }
    if wip.is_null() {
        // allocate a new entry
        wip = xcalloc(1, std::mem::size_of::<wininfo_T>());
        (*wip).wi_win = win;
        if lnum == 0 {
            // set lnum even when it's 0
            lnum = 1;
        }
    } else {
        let wip = wip.as_mut().unwrap();
        // remove the entry from the list
        if let Some(wi_prev) = wip.wi_prev.as_mut() {
            wi_prev.wi_next = wip.wi_next;
        } else {
            buf.b_wininfo = wip.wi_next;
        }
        if let Some(wi_next) = wip.wi_next.as_mut() {
            wi_next.wi_prev = wip.wi_prev;
        }
        if copy_options && wip.wi_optset {
            clear_winopt(&mut wip.wi_opt);
            deleteFoldRecurse(&mut wip.wi_folds);
        }
    }
    let wip = wip.as_mut().unwrap();
    if lnum != 0 {
        wip.wi_fpos.lnum = lnum;
        wip.wi_fpos.col = col;
    }
    if copy_options {
        // Save the window-specific option values.
        copy_winopt(&mut win.w_onebuf_opt, &mut wip.wi_opt);
        wip.wi_fold_manual = win.w_fold_manual;
        cloneFoldGrowArray(&mut win.w_folds, &mut wip.wi_folds);
        wip.wi_optset = true;
    }

    // insert the entry in front of the list
    wip.wi_next = buf.b_wininfo;
    buf.b_wininfo = wip;
    wip.wi_prev = ptr::null_mut();
    if let Some(wi_next) = wip.wi_next.as_mut() {
        wi_next.wi_prev = wip;
    }
}

/// Check that "wip" has 'diff' set and the diff is only for another tab page.
/// That's because a diff is local to a tab page.
#[must_use]
unsafe fn wininfo_other_tab_diff(wip: *mut wininfo_T) -> bool {
    let wip = wip.as_ref().unwrap();
    if wip.wi_opt.wo_diff != 0 {
        FOR_ALL_WINDOWS_IN_TAB!(wp, curtab, {
            // return false when it's a window in the current tab page, thus
            // the buffer was in diff mode here
            if wip.wi_win == wp {
                return false;
            }
        });
        return true;
    }
    return false;
}

/*
 * Find info for the current window in buffer "buf".
 * If not found, return the info for the most recently used window.
 * When "skip_diff_buffer" is true avoid windows with 'diff' set that is in
 * another tab page.
 * Returns NULL when there isn't any info.
 */
unsafe fn find_wininfo(buf: *const buf_T, skip_diff_buffer: i32) -> *mut wininfo_T {
    let buf = buf.as_ref().unwrap();
    let mut wip: *mut wininfo_T;

    wip = buf.b_wininfo;
    while !wip.is_null() {
        if (*wip).wi_win == curwin && (skip_diff_buffer == 0 || !wininfo_other_tab_diff(wip)) {
            break;
        }
        wip = (*wip).wi_next;
    }

    /* If no wininfo for curwin, use the first in the list (that doesn't have
     * 'diff' set and is in another tab page). */
    if wip.is_null() {
        if skip_diff_buffer != 0 {
            wip = buf.b_wininfo;
            while !wip.is_null() {
                if !wininfo_other_tab_diff(wip) {
                    break;
                }
                wip = (*wip).wi_next;
            }
        } else {
            wip = buf.b_wininfo;
        }
    }
    return wip;
}
/*
 * Reset the local window options to the values last used in this window.
 * If the buffer wasn't used in this window before, use the values from
 * the most recently used window.  If the values were never set, use the
 * global values for the window.
 */
#[no_mangle]
pub unsafe extern "C" fn get_winopts(buf: *mut buf_T) {
    let wip: *mut wininfo_T;

    clear_winopt(&mut (*curwin).w_onebuf_opt);
    clearFolding(curwin);

    wip = find_wininfo(buf, true_0);
    if !wip.is_null()
        && (*wip).wi_win != curwin
        && !(*wip).wi_win.is_null()
        && (*(*wip).wi_win).w_buffer == buf
    {
        let wip = wip.as_mut().unwrap();
        let wp = wip.wi_win.as_mut().unwrap();
        copy_winopt(&mut wp.w_onebuf_opt, &mut (*curwin).w_onebuf_opt);
        (*curwin).w_fold_manual = wp.w_fold_manual;
        (*curwin).w_foldinvalid = true;
        cloneFoldGrowArray(&mut wp.w_folds, &mut (*curwin).w_folds);
    } else if !wip.is_null() && (*wip).wi_optset {
        let wip = wip.as_mut().unwrap();
        copy_winopt(&mut wip.wi_opt, &mut (*curwin).w_onebuf_opt);
        (*curwin).w_fold_manual = wip.wi_fold_manual;
        (*curwin).w_foldinvalid = true;
        cloneFoldGrowArray(&mut wip.wi_folds, &mut (*curwin).w_folds);
    } else {
        copy_winopt(&mut (*curwin).w_allbuf_opt, &mut (*curwin).w_onebuf_opt);
    }

    if (*curwin).w_float_config.style == kWinStyleMinimal {
        didset_window_options(curwin);
        win_set_minimal_style(curwin);
    }

    // Set 'foldlevel' to 'foldlevelstart' if it's not negative.
    if p_fdls >= 0 {
        (*curwin).w_onebuf_opt.wo_fdl = p_fdls
    }
    didset_window_options(curwin);
}

/*
 * Find the position (lnum and col) for the buffer 'buf' for the current
 * window.
 * Returns a pointer to no_position if no position is found.
 */
#[no_mangle]
pub unsafe extern "C" fn buflist_findfpos(buf: *const buf_T) -> *mut pos_T {
    static mut no_position: pos_T = pos_T {
        lnum: 1,
        col: 0,
        coladd: 0,
    };
    let wip = find_wininfo(buf, false_0);
    return if wip.is_null() {
        &mut no_position
    } else {
        &mut (*wip).wi_fpos
    };
}

/*
 * Find the lnum for the buffer 'buf' for the current window.
 */
#[no_mangle]
pub unsafe extern "C" fn buflist_findlnum(buf: *const buf_T) -> linenr_T {
    return (*buflist_findfpos(buf)).lnum;
}

// List all known file names (for :files and :buffers command).
#[no_mangle]
pub unsafe extern "C" fn buflist_list(eap: *mut exarg_T) {
    let eap = eap.as_mut().unwrap();
    let mut len: i32;
    let mut i: i32;

    for buf in it!(buf = &firstbuf; !buf.is_null() && got_int == 0; &(**buf).b_next) {
        let buf = buf.as_mut().unwrap();
        // skip unspecified buffers
        if buf.b_p_bl == 0 && eap.forceit == 0 && strchr(eap.arg as *mut i8, 'u' as i32).is_null()
            || !strchr(eap.arg as *mut i8, 'u' as i32).is_null() && buf.b_p_bl != 0
            || !strchr(eap.arg as *mut i8, '+' as i32).is_null()
                && (buf.b_flags & BF_READERR != 0 || !bufIsChanged(buf))
            || !strchr(eap.arg as *mut i8, 'a' as i32).is_null()
                && (buf.b_ml.ml_mfp.is_null() || buf.b_nwindows == 0)
            || !strchr(eap.arg as *mut i8, 'h' as i32).is_null()
                && (buf.b_ml.ml_mfp.is_null() || buf.b_nwindows != 0)
            || !strchr(eap.arg as *mut i8, '-' as i32).is_null() && buf.b_p_ma != 0
            || !strchr(eap.arg as *mut i8, '=' as i32).is_null() && buf.b_p_ro == 0
            || !strchr(eap.arg as *mut i8, 'x' as i32).is_null() && buf.b_flags & BF_READERR == 0
            || !strchr(eap.arg as *mut i8, '%' as i32).is_null() && buf as *mut _ != curbuf
            || !strchr(eap.arg as *mut i8, '#' as i32).is_null()
                && (buf as *mut _ == curbuf || (*curwin).w_alt_fnum != buf.handle)
        {
            continue;
        }
        if !buf_spname(buf).is_null() {
            xstrlcpy(
                NameBuff.as_mut_ptr() as *mut i8,
                buf_spname(buf) as *mut i8,
                MAXPATHL,
            );
        } else {
            home_replace(buf, buf.b_fname, NameBuff.as_mut_ptr(), MAXPATHL, true);
        }
        if message_filtered(NameBuff.as_mut_ptr()) {
            continue;
        }

        let changed_char = if buf.b_flags & BF_READERR != 0 {
            'x'
        } else if bufIsChanged(buf) {
            '+'
        } else {
            ' '
        } as i32;
        let mut ro_char = if !MODIFIABLE(buf) {
            '-' as i32
        } else if buf.b_p_ro != 0 {
            '=' as i32
        } else {
            ' ' as i32
        };
        if !buf.terminal.is_null() {
            ro_char = if channel_job_running(buf.b_p_channel as u64) {
                'R'
            } else {
                'F'
            } as i32;
        }

        msg_putchar('\n' as i32);
        len = vim_snprintf(
            IObuff.as_mut_ptr() as *mut i8,
            IOSIZE as usize - 20,
            b"%3d%c%c%c%c%c \"%s\"\x00" as *const u8 as *const i8,
            buf.handle,
            if buf.b_p_bl != 0 { ' ' } else { 'u' } as i32,
            if buf as *mut _ == curbuf {
                '%'
            } else if (*curwin).w_alt_fnum == buf.handle {
                '#'
            } else {
                ' '
            } as i32,
            if buf.b_ml.ml_mfp.is_null() {
                ' '
            } else if buf.b_nwindows == 0 {
                'h'
            } else {
                'a'
            } as i32,
            ro_char,
            changed_char,
            NameBuff.as_mut_ptr(),
        );

        if len > IOSIZE - 20 {
            len = IOSIZE - 20
        }

        // put "line 999" in column 40 or after the file name
        i = 40 - vim_strsize(IObuff.as_mut_ptr());
        loop {
            IObuff[len as usize] = ' ' as u8;
            len += 1;
            i -= 1;
            if !(i > 0 && len < IOSIZE - 18) {
                break;
            }
        }
        vim_snprintf(
            (IObuff.as_mut_ptr() as *mut i8).offset(len as isize),
            (IOSIZE - len) as size_t,
            gettext("line %ld"),
            if buf as *mut _ == curbuf {
                (*curwin).w_cursor.lnum
            } else {
                buflist_findlnum(buf)
            },
        );
        msg_outtrans(IObuff.as_mut_ptr());
        line_breakcheck();
    }
}

/*
 * Get file name and line number for file 'fnum'.
 * Used by DoOneCmd() for translating '%' and '#'.
 * Used by insert_reg() and cmdline_paste() for '#' register.
 * Return FAIL if not found, OK for success.
 */
#[no_mangle]
pub unsafe extern "C" fn buflist_name_nr(
    fnum: i32,
    fname: *mut *mut u8,
    lnum: *mut linenr_T,
) -> i32 {
    let buf: *mut buf_T;

    buf = buflist_findnr(fnum);
    if let Some(buf) = buf.as_ref() {
        if let Some(b_fname) = buf.b_fname.as_mut() {
            *fname = b_fname;
            *lnum = buflist_findlnum(buf);

            return OK;
        }
    }

    return FAIL;
}

/*
 * Set the file name for "buf"' to 'ffname', short file name to 'sfname'.
 * The file name with the full path is also remembered, for when :cd is used.
 * Returns FAIL for failure (file name already in use by other buffer)
 *	OK otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn setfname(
    buf: *mut buf_T,
    mut ffname: *mut u8,
    mut sfname: *mut u8,
    message: bool, // give message when buffer already exists
) -> i32 {
    let buf = buf.as_mut().unwrap();
    let mut obuf = ptr::null_mut();
    let mut file_id = FileID::default();
    let mut file_id_valid = false;

    if ffname.is_null() || *ffname == 0 {
        // Removing the name.
        XFREE_CLEAR(&mut buf.b_ffname);
        XFREE_CLEAR(&mut buf.b_sfname);
    } else {
        fname_expand(buf, &mut ffname, &mut sfname); // will allocate ffname
        if ffname.is_null() {
            // out of memory
            return FAIL;
        }

        /*
         * if the file name is already used in another buffer:
         * - if the buffer is loaded, fail
         * - if the buffer is not loaded, delete it from the list
         */
        file_id_valid = os_fileid(ffname as *mut i8, &mut file_id);
        if buf.b_flags & BF_DUMMY == 0 {
            obuf = buflist_findname_file_id(ffname, &mut file_id, file_id_valid)
        }
        if !obuf.is_null() && obuf != buf {
            if !(*obuf).b_ml.ml_mfp.is_null() {
                // it's loaded, fail
                if message {
                    emsg(gettext("E95: Buffer with this name already exists"));
                }
                xfree(ffname);
                return FAIL;
            }
            // delete from the list
            close_buffer(ptr::null_mut(), obuf, DOBUF_WIPE, false);
        }
        sfname = vim_strsave(sfname);
        //TODO: USE_FNAME_CASE
        xfree(buf.b_ffname);
        xfree(buf.b_sfname);
        buf.b_ffname = ffname;
        buf.b_sfname = sfname
    }
    buf.b_fname = buf.b_sfname;
    if !file_id_valid {
        buf.file_id_valid = false
    } else {
        buf.file_id_valid = true;
        buf.file_id = file_id
    }

    buf_name_changed(buf);
    return OK;
}

/*
 * Crude way of changing the name of a buffer.  Use with care!
 * The name should be relative to the current directory.
 */
#[no_mangle]
pub unsafe extern "C" fn buf_set_name(fnum: i32, name: *mut u8) {
    let buf = buflist_findnr(fnum);
    if let Some(buf) = buf.as_mut() {
        xfree(buf.b_sfname);
        xfree(buf.b_ffname);
        buf.b_ffname = vim_strsave(name);
        buf.b_sfname = ptr::null_mut();
        /* Allocate ffname and expand into full path.  Also resolves .lnk
         * files on Win32. */
        fname_expand(buf, &mut buf.b_ffname, &mut buf.b_sfname);
        buf.b_fname = buf.b_sfname;
    }
}

/*
 * Take care of what needs to be done when the name of buffer "buf" has
 * changed.
 */
#[no_mangle]
pub unsafe extern "C" fn buf_name_changed(buf: *mut buf_T) {
    /*
     * If the file name changed, also change the name of the swapfile
     */
    if !(*buf).b_ml.ml_mfp.is_null() {
        ml_setname(buf);
    }

    if (*curwin).w_buffer == buf {
        check_arg_idx(curwin); // check file name for arg list
    }
    maketitle(); // set window title
    status_redraw_all(); // status lines need to be redrawn
    fmarks_check_names(buf); // check named file marks
    ml_timestamp(buf); // reset timestamp
}

/*
 * set alternate file name for current window
 *
 * Used by do_one_cmd(), do_write() and do_ecmd().
 * Return the buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn setaltfname(
    ffname: *mut u8,
    sfname: *mut u8,
    lnum: linenr_T,
) -> *mut buf_T {
    let buf: *mut buf_T;

    // Create a buffer.  'buflisted' is not set if it's a new buffer
    buf = buflist_new(ffname, sfname, lnum, 0);
    if !buf.is_null() && !cmdmod.keepalt {
        (*curwin).w_alt_fnum = (*buf).handle
    }
    return buf;
}

/*
 * Get alternate file name for current window.
 * Return NULL if there isn't any, and give error message if requested.
 */
#[no_mangle]
pub unsafe extern "C" fn getaltfname(errmsg: bool, // give error message
) -> *mut u8 {
    let mut fname: *mut u8 = ptr::null_mut();
    let mut dummy: linenr_T = 0;

    if buflist_name_nr(0, &mut fname, &mut dummy) == FAIL {
        if errmsg {
            emsg(c_gettext(e_noalt.as_mut_ptr() as *mut i8) as *mut u8);
        }
        return ptr::null_mut();
    }
    return fname;
}

/*
 * Add a file name to the buflist and return its number.
 * Uses same flags as buflist_new(), except BLN_DUMMY.
 *
 * used by qf_init(), main() and doarglist()
 */
#[no_mangle]
pub unsafe extern "C" fn buflist_add(fname: *mut u8, flags: i32) -> i32 {
    let buf: *mut buf_T;

    buf = buflist_new(fname, ptr::null_mut(), 0, flags);
    if let Some(buf) = buf.as_ref() {
        return buf.handle;
    }
    return 0;
}

//TODO: BACKSLASH_IN_FILENAME

/*
 * Set alternate cursor position for the current buffer and window "win".
 * Also save the local window option values.
 */
#[no_mangle]
pub unsafe extern "C" fn buflist_altfpos(win: *mut win_T) {
    let win = win.as_mut().unwrap();
    buflist_setfpos(curbuf, win, win.w_cursor.lnum, win.w_cursor.col, true);
}

/// Check that "ffname" is not the same file as current file.
/// Fname must have a full path (expanded by path_to_absolute()).
///
/// @param  ffname  full path name to check
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn otherfile(ffname: *mut u8) -> bool {
    return otherfile_buf(curbuf, ffname, ptr::null_mut(), false);
}

/// Check that "ffname" is not the same file as the file loaded in "buf".
/// Fname must have a full path (expanded by path_to_absolute()).
///
/// @param  buf            buffer to check
/// @param  ffname         full path name to check
/// @param  file_id_p      information about the file at "ffname".
/// @param  file_id_valid  whether a valid "file_id_p" was passed in.
#[must_use]
unsafe fn otherfile_buf(
    buf: *mut buf_T,
    ffname: *mut u8,
    mut file_id_p: *mut FileID,
    mut file_id_valid: bool,
) -> bool {
    let buf = buf.as_mut().unwrap();
    // no name is different
    if ffname.is_null() || *ffname as i32 == NUL || buf.b_ffname.is_null() {
        return true;
    }
    if path_fnamecmp(ffname as *const i8, buf.b_ffname as *const i8) == 0 {
        return false;
    }

    let mut file_id = FileID::default();
    // If no struct stat given, get it now
    if file_id_p.is_null() {
        file_id_p = &mut file_id;
        file_id_valid = os_fileid(ffname as *mut i8, file_id_p)
    }
    if !file_id_valid {
        // file_id not valid, assume files are different.
        return true;
    }
    // Use dev/ino to check if the files are the same, even when the names
    // are different (possible with links).  Still need to compare the
    // name above, for when the file doesn't exist yet.
    // Problem: The dev/ino changes when a file is deleted (and created
    // again) and remains the same when renamed/moved.  We don't want to
    // stat() each buffer each time, that would be too slow.  Get the
    // dev/ino again when they appear to match, but not when they appear
    // to be different: Could skip a buffer when it's actually the same
    // file.
    if buf_same_file_id(buf, file_id_p) {
        buf_set_file_id(buf);
        if buf_same_file_id(buf, file_id_p) {
            return false;
        }
    }
    return true;
}

// Set file_id for a buffer.
// Must always be called when b_fname is changed!
#[no_mangle]
pub unsafe extern "C" fn buf_set_file_id(buf: *mut buf_T) {
    let buf = buf.as_mut().unwrap();
    let mut file_id = FileID::default();
    if !buf.b_fname.is_null() && os_fileid(buf.b_fname as *mut i8, &mut file_id) {
        buf.file_id_valid = true;
        buf.file_id = file_id;
    } else {
        buf.file_id_valid = false;
    }
}

/// Check that file_id in buffer "buf" matches with "file_id".
///
/// @param  buf      buffer
/// @param  file_id  file id
#[must_use]
unsafe fn buf_same_file_id(buf: *mut buf_T, file_id: *mut FileID) -> bool {
    let buf = buf.as_mut().unwrap();
    return buf.file_id_valid && os_fileid_equal(&mut buf.file_id, file_id);
}

/*
 * Print info about the current buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn fileinfo(
    fullname: i32, // when non-zero print full path
    shorthelp: i32,
    dont_truncate: i32,
) {
    let name: *mut u8;
    let mut n: i32;
    let mut p: *mut u8;
    let buffer: *mut u8;
    let len: size_t;

    buffer = xmalloc(IOSIZE as size_t);

    if fullname > 1 {
        // 2 CTRL-G: include buffer number
        vim_snprintf(
            buffer as *mut i8,
            IOSIZE as size_t,
            b"buf %d: \x00" as *const u8 as *const i8,
            (*curbuf).handle,
        );
        p = buffer.offset(strlen(buffer as *mut i8) as isize);
    } else {
        p = buffer;
    }

    *p = '\"' as u8;
    p = p.offset(1);
    if !buf_spname(curbuf).is_null() {
        xstrlcpy(
            p as *mut i8,
            buf_spname(curbuf) as *mut i8,
            IOSIZE as usize - p.offset_from(buffer) as usize,
        );
    } else {
        if fullname == 0 && !(*curbuf).b_fname.is_null() {
            name = (*curbuf).b_fname;
        } else {
            name = (*curbuf).b_ffname;
        }
        home_replace(
            if shorthelp != 0 {
                curbuf
            } else {
                ptr::null_mut()
            },
            name,
            p,
            IOSIZE as usize - p.offset_from(buffer) as usize,
            true,
        );
    }

    vim_snprintf_add(
        buffer as *mut i8,
        IOSIZE as size_t,
        b"\"%s%s%s%s%s%s\x00" as *const u8 as *const i8,
        if curbufIsChanged() {
            if shortmess(SHM_MOD) {
                b" [+]\x00" as *const u8
            } else {
                gettext(" [Modified]")
            }
        } else {
            b" \x00" as *const u8
        } as *const i8,
        if (*curbuf).b_flags & BF_NOTEDITED != 0 && !bt_dontwrite(curbuf) {
            gettext("[Not edited]")
        } else {
            b"\x00" as *const u8
        } as *const i8,
        if (*curbuf).b_flags & BF_NEW != 0 && !bt_dontwrite(curbuf) {
            gettext("[New file]")
        } else {
            b"\x00" as *const u8
        } as *const i8,
        if (*curbuf).b_flags & BF_READERR != 0 {
            gettext("[Read errors]")
        } else {
            b"\x00" as *const u8
        } as *const i8,
        if (*curbuf).b_p_ro != 0 {
            if shortmess(SHM_RO) {
                gettext("[RO]")
            } else {
                gettext("[readonly]")
            }
        } else {
            b"\x00" as *const u8
        } as *const i8,
        if curbufIsChanged() || (*curbuf).b_flags & BF_WRITE_MASK != 0 || (*curbuf).b_p_ro != 0 {
            b" \x00" as *const u8
        } else {
            b"\x00" as *const u8
        } as *const i8,
    );
    /* With 32 bit longs and more than 21,474,836 lines multiplying by 100
     * causes an overflow, thus for large numbers divide instead. */
    if (*curwin).w_cursor.lnum > 1000000 {
        n = ((*curwin).w_cursor.lnum / ((*curbuf).b_ml.ml_line_count / 100)) as i32
    } else {
        n = ((*curwin).w_cursor.lnum * 100 / (*curbuf).b_ml.ml_line_count) as i32
    }
    if (*curbuf).b_ml.ml_flags & ML_EMPTY != 0 {
        vim_snprintf_add(
            buffer as *mut i8,
            IOSIZE as size_t,
            b"%s\x00" as *const u8 as *const i8,
            c_gettext(no_lines_msg.as_mut_ptr() as *const i8),
        );
    } else if p_ru != 0 {
        // Current line and column are already on the screen -- webb
        if (*curbuf).b_ml.ml_line_count == 1 {
            vim_snprintf_add(
                buffer as *mut i8,
                IOSIZE as size_t,
                gettext("1 line --%d%%--"),
                n,
            );
        } else {
            vim_snprintf_add(
                buffer as *mut i8,
                IOSIZE as size_t,
                gettext("%ld lines --%d%%--"),
                (*curbuf).b_ml.ml_line_count,
                n,
            );
        }
    } else {
        vim_snprintf_add(
            buffer as *mut i8,
            IOSIZE as size_t,
            gettext("line %ld of %ld --%d%%-- col "),
            (*curwin).w_cursor.lnum,
            (*curbuf).b_ml.ml_line_count,
            n,
        );
        validate_virtcol();
        len = strlen(buffer as *mut i8) as _;
        col_print(
            buffer.offset(len as isize),
            IOSIZE as usize - len as usize,
            (*curwin).w_cursor.col + 1,
            (*curwin).w_virtcol + 1,
        );
    }

    append_arg_number(curwin, buffer, IOSIZE, !shortmess(SHM_FILE));

    if dont_truncate != 0 {
        /* Temporarily set msg_scroll to avoid the message being truncated.
         * First call msg_start() to get the message in the right place. */
        msg_start();
        n = msg_scroll;
        msg_scroll = true_0;
        msg(buffer);
        msg_scroll = n
    } else {
        p = msg_trunc_attr(buffer, false_0, 0);
        if restart_edit != 0 || msg_scrolled != 0 && need_wait_return == 0 {
            // Need to repeat the message after redrawing when:
            // - When restart_edit is set (otherwise there will be a delay
            //   before redrawing).
            // - When the screen was scrolled but there is no wait-return
            //   prompt.
            set_keep_msg(p, 0);
        }
    }

    xfree(buffer);
}

#[no_mangle]
pub unsafe extern "C" fn col_print(buf: *mut u8, buflen: size_t, col: i32, vcol: i32) {
    if col == vcol {
        vim_snprintf(
            buf as *mut i8,
            buflen,
            b"%d\x00" as *const u8 as *const i8,
            col,
        );
    } else {
        vim_snprintf(
            buf as *mut i8,
            buflen,
            b"%d-%d\x00" as *const u8 as *const i8,
            col,
            vcol,
        );
    };
}

static mut lasttitle: *mut u8 = ptr::null_mut();
static mut lasticon: *mut u8 = ptr::null_mut();

// Put the title name in the title bar and icon of the window.
#[no_mangle]
pub unsafe extern "C" fn maketitle() {
    let mut title_str = ptr::null_mut();
    let mut icon_str = ptr::null_mut();
    let mut maxlen = 0;
    let mut len: i32;
    let mut mustset: bool;
    let mut buf: [i8; IOSIZE as usize] = [0; IOSIZE as usize];

    if redrawing() == 0 {
        // Postpone updating the title when 'lazyredraw' is set.
        need_maketitle = true_0;
        return;
    }

    need_maketitle = false_0;
    if p_title == 0 && p_icon == 0 && lasttitle.is_null() && lasticon.is_null() {
        return; // nothing to do
    }

    if p_title != 0 {
        if p_titlelen > 0 {
            maxlen = (p_titlelen * Columns as i64 / 100) as i32;
            if maxlen < 10 {
                maxlen = 10;
            }
        }

        if *p_titlestring != 0 {
            if stl_syntax & STL_IN_TITLE != 0 {
                let save_called_emsg = called_emsg;

                let use_sandbox = was_set_insecurely(b"titlestring\x00" as *const u8, 0);
                called_emsg = false_0;
                build_stl_str_hl(
                    curwin,
                    buf.as_mut_ptr() as *mut u8,
                    var_size(buf),
                    p_titlestring,
                    use_sandbox,
                    0,
                    maxlen,
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                title_str = buf.as_mut_ptr() as *mut u8;
                if called_emsg != 0 {
                    set_string_option_direct(
                        b"titlestring\x00" as *const u8,
                        -1,
                        b"\x00" as *const u8,
                        OPT_FREE,
                        SID_ERROR,
                    );
                }
                called_emsg |= save_called_emsg
            } else {
                title_str = p_titlestring
            }
        } else {
            // Format: "fname + (path) (1 of 2) - VIM".

            let SPACE_FOR_FNAME = var_size(buf) - 100;
            let SPACE_FOR_DIR = var_size(buf) - 20;
            let SPACE_FOR_ARGNR = var_size(buf) - 10; // At least room for " - NVIM".

            let mut buf_p = buf.as_mut_ptr();
            if (*curbuf).b_fname.is_null() {
                let size = xstrlcpy(buf_p, gettext("[No Name]"), SPACE_FOR_FNAME + 1);
                buf_p = buf_p.offset(min(size, SPACE_FOR_FNAME) as isize);
            } else {
                buf_p = buf_p.offset(transstr_buf(
                    path_tail((*curbuf).b_fname) as *const i8,
                    buf_p,
                    SPACE_FOR_FNAME + 1,
                ) as isize);
            }

            match (
                bufIsChanged(curbuf),
                (*curbuf).b_p_ro != 0,
                !MODIFIABLE(curbuf),
            ) {
                (false, false, false) => {}
                (true, false, false) => {
                    buf_p = strappend(buf_p, b" +\x00" as *const u8 as *const i8)
                }
                (false, true, false) => {
                    buf_p = strappend(buf_p, b" =\x00" as *const u8 as *const i8)
                }
                (true, true, false) => {
                    buf_p = strappend(buf_p, b" =+\x00" as *const u8 as *const i8)
                }
                (false, _, true) => buf_p = strappend(buf_p, b" -\x00" as *const u8 as *const i8),
                (true, _, true) => buf_p = strappend(buf_p, b" -+\x00" as *const u8 as *const i8),
            }

            if !(*curbuf).b_fname.is_null() {
                // Get path of file, replace home dir with ~.
                *buf_p = ' ' as i8;
                buf_p = buf_p.offset(1);
                *buf_p = '(' as i8;
                buf_p = buf_p.offset(1);
                home_replace(
                    curbuf,
                    (*curbuf).b_ffname,
                    buf_p as *mut u8,
                    SPACE_FOR_DIR - (buf_p.offset_from(buf.as_mut_ptr()) as usize),
                    true,
                );
                //TODO: BACKSLASH_IN_FILENAME
                // Remove the file name.
                let p = path_tail_with_sep(buf_p as *mut u8) as *mut i8;
                if p == buf_p {
                    // Must be a help buffer.
                    xstrlcpy(
                        buf_p,
                        gettext("help"),
                        SPACE_FOR_DIR - (buf_p.offset_from(buf.as_mut_ptr()) as usize),
                    );
                } else {
                    *p = 0;
                }

                // Translate unprintable chars and concatenate.  Keep some
                // room for the server name.  When there is no room (very long
                // file name) use (...).
                if (buf_p.offset_from(buf.as_mut_ptr()) as usize) < SPACE_FOR_DIR {
                    let tbuf = transstr(buf_p);
                    let free_space =
                        SPACE_FOR_DIR - (buf_p.offset_from(buf.as_mut_ptr()) as usize) + 1;
                    let dir_len = xstrlcpy(buf_p, tbuf, free_space);
                    buf_p = buf_p.offset(min(dir_len, free_space - 1) as isize);
                    xfree(tbuf);
                } else {
                    let free_space =
                        SPACE_FOR_ARGNR - (buf_p.offset_from(buf.as_mut_ptr()) as usize) + 1;
                    let dots_len =
                        xstrlcpy(buf_p, b"...\x00" as *const u8 as *const i8, free_space);
                    buf_p = buf_p.offset(min(dots_len, free_space - 1) as isize);
                }
                *buf_p = ')' as i8;
                buf_p = buf_p.offset(1);
                *buf_p = 0;
            } else {
                *buf_p = 0;
            }

            append_arg_number(
                curwin,
                buf_p as *mut u8,
                (SPACE_FOR_ARGNR - (buf_p.offset_from(buf.as_mut_ptr()) as usize)) as i32,
                false,
            );

            xstrlcat(
                buf_p,
                b" - NVIM\x00" as *const u8 as *const i8,
                var_size(buf) - (buf_p.offset_from(buf.as_mut_ptr()) as usize),
            );

            if maxlen > 0 {
                // Make it shorter by removing a bit in the middle.
                if vim_strsize(buf.as_mut_ptr() as *mut u8) > maxlen {
                    trunc_string(
                        buf.as_mut_ptr() as *mut u8,
                        buf.as_mut_ptr() as *mut u8,
                        maxlen,
                        var_size(buf) as i32,
                    );
                }
            }
            title_str = buf.as_mut_ptr() as *mut u8
        }
    }
    mustset = value_change(title_str, &mut lasttitle);

    if p_icon != 0 {
        icon_str = buf.as_mut_ptr() as *mut u8;
        if *p_iconstring != 0 {
            if stl_syntax & STL_IN_ICON != 0 {
                let use_sandbox;
                let save_called_emsg = called_emsg;

                use_sandbox = was_set_insecurely(b"iconstring\x00" as *const u8, 0);
                called_emsg = false_0;
                build_stl_str_hl(
                    curwin,
                    icon_str,
                    var_size(buf),
                    p_iconstring,
                    use_sandbox,
                    0,
                    0,
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                if called_emsg != 0 {
                    set_string_option_direct(
                        b"iconstring\x00" as *const u8,
                        -1,
                        b"\x00" as *const u8,
                        OPT_FREE,
                        SID_ERROR,
                    );
                }
                called_emsg |= save_called_emsg;
            } else {
                icon_str = p_iconstring;
            }
        } else {
            let mut buf_p: *const u8;
            if let Some(spname) = buf_spname(curbuf).as_ref() {
                buf_p = spname;
            } else {
                buf_p = path_tail((*curbuf).b_ffname);
            }
            *icon_str = 0;
            // Truncate name at 100 bytes.
            len = strlen(buf_p as *const i8) as i32;
            if len > 100 {
                len -= 100;
                if has_mbyte {
                    len += mb_tail_off(buf_p, buf_p.offset(len as isize)) + 1;
                }
                buf_p = buf_p.offset(len as isize)
            }
            strcpy(icon_str as *mut i8, buf_p as *const i8);
            trans_characters(icon_str, IOSIZE);
        }
    }

    mustset |= value_change(icon_str, &mut lasticon);

    if mustset {
        resettitle();
    }
}

/// Used for title and icon: Check if "str" differs from "*last".  Set "*last"
/// from "str" if it does by freeing the old value of "*last" and duplicating
/// "str".
///
/// @param          str   desired title string
/// @param[in,out]  last  current title string
///
/// @return true if resettitle() is to be called.
#[must_use]
unsafe extern "C" fn value_change(str: *const u8, last: *mut *mut u8) -> bool {
    let last = last.as_mut().unwrap();
    if (str.is_null() != last.is_null())
        || !str.is_null() && !last.is_null() && strcmp(str as *const i8, *last as *mut i8) != 0
    {
        xfree(*last);
        if str.is_null() {
            *last = ptr::null_mut();
            resettitle();
        } else {
            *last = vim_strsave(str);
            return true;
        }
    }
    return false;
}

/// Set current window title
#[no_mangle]
pub unsafe extern "C" fn resettitle() {
    ui_call_set_icon(cstr_as_string(lasticon as *mut i8));
    ui_call_set_title(cstr_as_string(lasttitle as *mut i8));
    ui_flush();
}

//TODO: EXITFREE

/// Enumeration specifying the valid numeric bases that can
/// be used when printing numbers in the status line.
pub type NumberBase = u32;
pub const kNumBaseHexadecimal: NumberBase = 16;
pub const kNumBaseDecimal: NumberBase = 10;

/// Build a string from the status line items in "fmt".
/// Return length of string in screen cells.
///
/// Normally works for window "wp", except when working for 'tabline' then it
/// is "curwin".
///
/// Items are drawn interspersed with the text that surrounds it
/// Specials: %-<wid>(xxx%) => group, %= => separation marker, %< => truncation
/// Item: %-<minwid>.<maxwid><itemch> All but <itemch> are optional
///
/// If maxwidth is not zero, the string will be filled at any middle marker
/// or truncated if too long, fillchar is used for all whitespace.
///
/// @param wp The window to build a statusline for
/// @param out The output buffer to write the statusline to
///            Note: This should not be NameBuff
/// @param outlen The length of the output buffer
/// @param fmt The statusline format string
/// @param use_sandbox Use a sandboxed environment when evaluating fmt
/// @param fillchar Character to use when filling empty space in the statusline
/// @param maxwidth The maximum width to make the statusline
/// @param hltab HL attributes (can be NULL)
/// @param tabtab Tab clicks definition (can be NULL).
///
/// @return The final width of the statusline
#[no_mangle]
pub unsafe extern "C" fn build_stl_str_hl(
    wp: *mut win_T,
    out: *mut u8,
    outlen: size_t,
    fmt: *mut u8,
    use_sandbox: i32,
    mut fillchar: u8,
    maxwidth: i32,
    hltab: *mut stl_hlrec,
    tabtab: *mut StlClickRecord,
) -> i32 {
    let wp = wp.as_mut().unwrap();
    let mut groupitems: [usize; 80] = [0; 80];

    #[derive(Copy, Clone, PartialEq, Eq)]
    enum stl_item_type {
        Normal,
        Empty,
        Group,
        Separate,
        Highlight,
        TabPage,
        ClickFunc,
        Trunc,
    }
    use stl_item_type::*;
    #[derive(Copy, Clone)]
    struct stl_item {
        // Where the item starts in the status line output buffer
        start: *mut u8,
        // Function to run for ClickFunc items.
        cmd: *mut i8,
        // The minimum width of the item
        minwid: i32,
        // The maximum width of the item
        maxwid: i32,
        type_0: stl_item_type,
    }

    let mut items: [stl_item; STL_MAX_ITEM as usize] = [stl_item {
        start: 0 as *mut u8,
        cmd: 0 as *mut i8,
        minwid: 0,
        maxwid: 0,
        type_0: Normal,
    }; STL_MAX_ITEM as usize];

    const TMPLEN: usize = 70;
    let mut tmp: [u8; TMPLEN] = [0; TMPLEN];
    let mut usefmt = fmt;
    let save_must_redraw = must_redraw;
    let save_redr_type = (*curwin).w_redr_type;

    // When the format starts with "%!" then evaluate it as an expression and
    // use the result as the actual format string.
    if *fmt.offset(0) == '%' as u8 && *fmt.offset(1) == '!' as u8 {
        usefmt = eval_to_string_safe(fmt.offset(2), ptr::null_mut(), use_sandbox);
        if usefmt.is_null() {
            usefmt = fmt;
        }
    }

    if fillchar == 0 {
        fillchar = ' ' as u8;
    } else if utf_char2len(fillchar as i32) > 1 {
        // Can't handle a multi-byte fill character yet.
        fillchar = '-' as u8;
    }

    // The cursor in windows other than the current one isn't always
    // up-to-date, esp. because of autocommands and timers.
    let mut lnum = wp.w_cursor.lnum;
    if lnum > (*wp.w_buffer).b_ml.ml_line_count {
        lnum = (*wp.w_buffer).b_ml.ml_line_count;
        wp.w_cursor.lnum = lnum;
    }

    // Get line & check if empty (cursorpos will show "0-1").
    let line_ptr: *const u8 = ml_get_buf(wp.w_buffer, lnum, false);
    let empty_line = *line_ptr == 0;

    // Get the byte value now, in case we need it below. This is more
    // efficient than making a copy of the line.
    let byteval: i32;
    let len = strlen(line_ptr as *mut i8);
    if wp.w_cursor.col > len as colnr_T {
        // Line may have changed since checking the cursor column, or the lnum
        // was adjusted above.
        wp.w_cursor.col = len as colnr_T;
        wp.w_cursor.coladd = 0;
        byteval = 0
    } else {
        byteval = utf_ptr2char(line_ptr.offset(wp.w_cursor.col as isize))
    }

    let mut groupdepth: usize = 0;

    let mut curitem: usize = 0;
    let mut prevchar_isflag = true;
    let mut prevchar_isitem = false;

    // out_p is the current position in the output buffer
    let mut out_p = out;

    // out_end_p is the last valid character in the output buffer
    // Note: The null termination character must occur here or earlier,
    //       so any user-visible characters must occur before here.
    let out_end_p = out.offset(outlen as isize).offset(-(1));

    // Proceed character by character through the statusline format string
    // fmt_p is the current positon in the input buffer
    let mut fmt_p = usefmt;
    while *fmt_p != 0 {
        if curitem == STL_MAX_ITEM {
            // There are too many items.  Add the error code to the statusline
            // to give the user a hint about what went wrong.
            if out_p.offset(5) < out_end_p {
                memmove(out_p, b" E541\x00" as *const u8, 5);
                out_p = out_p.offset(5)
            }
            break;
        }

        if *fmt_p != 0 && *fmt_p != '%' as u8 {
            prevchar_isitem = false;
            prevchar_isflag = false;
        }

        // Copy the formatting verbatim until we reach the end of the string
        // or find a formatting item (denoted by `%`)
        // or run out of room in our output buffer.
        while *fmt_p != 0 && *fmt_p != '%' as u8 && out_p < out_end_p {
            *out_p = *fmt_p;
            fmt_p = fmt_p.offset(1);
            out_p = out_p.offset(1);
        }

        // If we have processed the entire format string or run out of
        // room in our output buffer, exit the loop.
        if *fmt_p == 0 || out_p >= out_end_p {
            break;
        }

        // The rest of this loop will handle a single `%` item.
        // Note: We increment here to skip over the `%` character we are currently
        //       on so we can process the item's contents.
        fmt_p = fmt_p.offset(1);

        // Ignore `%` at the end of the format string
        if *fmt_p == 0 {
            break;
        }

        // Two `%` in a row is the escape sequence to print a
        // single `%` in the output buffer.
        if *fmt_p == '%' as u8 {
            *out_p = *fmt_p;
            fmt_p = fmt_p.offset(1);
            out_p = out_p.offset(1);
            prevchar_isitem = false;
            prevchar_isflag = false;
            continue;
        }

        // STL_SEPARATE: Separation place between left and right aligned items.
        if *fmt_p == STL_SEPARATE {
            fmt_p = fmt_p.offset(1);
            // Ignored when we are inside of a grouping
            if groupdepth > 0 {
                continue;
            }
            items[curitem].type_0 = Separate;
            items[curitem].start = out_p;
            curitem = curitem + 1;
            continue;
        }

        // STL_TRUNCMARK: Where to begin truncating if the statusline is too long.
        if *fmt_p == STL_TRUNCMARK {
            fmt_p = fmt_p.offset(1);
            items[curitem].type_0 = Trunc;
            items[curitem as usize].start = out_p;
            curitem = curitem + 1;
            continue;
        }

        // The end of a grouping
        if *fmt_p == ')' as u8 {
            fmt_p = fmt_p.offset(1);
            // Ignore if we are not actually inside a group currently
            if groupdepth < 1 {
                continue;
            }
            groupdepth -= 1;

            // Determine how long the group is.
            // Note: We set the current output position to null
            //       so `vim_strsize` will work.
            let mut t = items[groupitems[groupdepth]].start;
            *out_p = 0;
            let mut group_len = vim_strsize(t) as i64;

            // If the group contained internal items
            // and the group did not have a minimum width,
            // and if there were no normal items in the group,
            // move the output pointer back to where the group started.
            // Note: This erases any non-item characters that were in the group.
            //       Otherwise there would be no reason to do this step.
            if curitem > groupitems[groupdepth] + 1 && items[groupitems[groupdepth]].minwid == 0 {
                // remove group if all items are empty and highlight group
                // doesn't change
                let mut group_start_userhl = 0;
                let mut group_end_userhl = 0;
                for n in (0..groupitems[groupdepth]).rev() {
                    if items[n].type_0 == Highlight {
                        group_end_userhl = items[n].minwid;
                        group_start_userhl = group_end_userhl;
                        break;
                    }
                }
                let mut break_early = false;
                for n in groupitems[groupdepth] + 1..curitem {
                    if items[n].type_0 == Normal {
                        break_early = true;
                        break;
                    }
                    if items[n].type_0 == Highlight {
                        group_end_userhl = items[n].minwid;
                    }
                }
                if !break_early && group_start_userhl == group_end_userhl {
                    out_p = t;
                    group_len = 0;
                    // do not use the highlighting from the removed group
                    for n in groupitems[groupdepth] + 1..curitem {
                        if items[n].type_0 == Highlight {
                            items[n].type_0 = Empty;
                        }
                    }
                }
            }

            // If the group is longer than it is allowed to be
            // truncate by removing bytes from the start of the group text.
            if group_len > items[groupitems[groupdepth]].maxwid as i64 {
                // { Determine the number of bytes to remove
                let mut n: isize;
                if has_mbyte {
                    // Find the first character that should be included.
                    n = 0;
                    while group_len >= items[groupitems[groupdepth]].maxwid as i64 {
                        group_len -= ptr2cells(t.offset(n)) as i64;
                        n += mb_ptr2len(t.offset(n)) as isize;
                    }
                } else {
                    n = out_p as isize - t as isize - items[groupitems[groupdepth]].maxwid as isize
                        + 1;
                }
                // }

                // Prepend the `<` to indicate that the output was truncated.
                *t = '<' as u8;

                // { Move the truncated output
                memmove(
                    t.offset(1),
                    t.offset(n),
                    out_p as isize - (t as isize + n as isize),
                );
                out_p = out_p.offset(-(n as isize)).offset(1);
                // Fill up space left over by half a double-wide char.
                while {
                    group_len += 1;
                    group_len < items[groupitems[groupdepth]].minwid as i64
                } {
                    *out_p = fillchar;
                    out_p = out_p.offset(1);
                }
                // }

                // correct the start of the items for the truncation
                for idx in groupitems[groupdepth] + 1..curitem {
                    // Shift everything back by the number of removed bytes
                    items[idx].start = items[idx].start.offset(-n);

                    // If the item was partially or completely truncated, set its
                    // start to the start of the group
                    if items[idx].start < t {
                        items[idx].start = t;
                    }
                }
            } else if abs(items[groupitems[groupdepth]].minwid) as i64 > group_len {
                // If the group is shorter than the minimum width, add padding characters.
                let mut min_group_width = items[groupitems[groupdepth]].minwid as i64;
                if min_group_width < 0 {
                    // If the group is left-aligned, add characters to the right.
                    min_group_width = 0 - min_group_width;
                    loop {
                        let fresh16 = group_len;
                        group_len = group_len + 1;
                        if !(fresh16 < min_group_width && out_p < out_end_p) {
                            break;
                        }
                        *out_p = fillchar;
                        out_p = out_p.offset(1);
                    }
                } else {
                    // If the group is right-aligned, shift everything to the right and
                    // prepend with filler characters.
                    // { Move the group to the right
                    memmove(
                        t.offset(min_group_width as isize)
                            .offset(-(group_len as isize)),
                        t,
                        out_p.offset_from(t) as size_t,
                    );
                    group_len = min_group_width - group_len;
                    if out_p.offset(group_len as isize) >= out_end_p.offset(1) {
                        group_len = out_end_p.offset_from(out_p) as i64;
                    }
                    out_p = out_p.offset(group_len as isize);
                    // }

                    // Adjust item start positions
                    for n in groupitems[groupdepth] + 1..curitem {
                        items[n].start = items[n].start.offset(group_len as isize);
                    }

                    // Prepend the fill characters
                    while group_len > 0 {
                        *t = fillchar;
                        t = t.offset(1);
                        group_len -= 1;
                    }
                }
            }
            continue;
        }
        let mut minwid = 0;
        let mut maxwid = 9999;
        let mut left_align = false;

        // Denotes that numbers should be left-padded with zeros
        let zeropad = *fmt_p == '0' as u8;
        if zeropad {
            fmt_p = fmt_p.offset(1)
        }

        // Denotes that the item should be left-aligned.
        // This is tracked by using a negative length.
        if *fmt_p == '-' as u8 {
            fmt_p = fmt_p.offset(1);
            left_align = true;
        }

        // The first digit group is the item's min width
        if ascii_isdigit(*fmt_p) {
            minwid = getdigits_int(&mut fmt_p as *mut *mut _ as *mut *const _, false, 0)
        }

        // User highlight groups override the min width field
        // to denote the styling to use.
        if *fmt_p == STL_USER_HL {
            items[curitem].type_0 = Highlight;
            items[curitem].start = out_p;
            items[curitem].minwid = if minwid > 9 { 1 } else { minwid };
            fmt_p = fmt_p.offset(1);
            curitem += 1;
            continue;
        }

        // TABPAGE pairs are used to denote a region that when clicked will
        // either switch to or close a tab.
        //
        // Ex: tabline=%0Ttab\ zero%X
        //   This tabline has a TABPAGENR item with minwid `0`,
        //   which is then closed with a TABCLOSENR item.
        //   Clicking on this region with mouse enabled will switch to tab 0.
        //   Setting the minwid to a different value will switch
        //   to that tab, if it exists
        //
        // Ex: tabline=%1Xtab\ one%X
        //   This tabline has a TABCLOSENR item with minwid `1`,
        //   which is then closed with a TABCLOSENR item.
        //   Clicking on this region with mouse enabled will close tab 0.
        //   This is determined by the following formula:
        //      tab to close = (1 - minwid)
        //   This is because for TABPAGENR we use `minwid` = `tab number`.
        //   For TABCLOSENR we store the tab number as a negative value.
        //   Because 0 is a valid TABPAGENR value, we have to
        //   start our numbering at `-1`.
        //   So, `-1` corresponds to us wanting to close tab `0`
        //
        // Note: These options are only valid when creating a tabline.
        if *fmt_p == STL_TABPAGENR || *fmt_p == STL_TABCLOSENR {
            if *fmt_p == STL_TABCLOSENR {
                if minwid == 0 {
                    // %X ends the close label, go back to the previous tab label nr.
                    for n in (0..curitem).rev() {
                        if items[n].type_0 == TabPage && items[n].minwid >= 0 {
                            minwid = items[n].minwid;
                            break;
                        }
                    }
                } else {
                    // close nrs are stored as negative values
                    minwid = -minwid;
                }
            }
            items[curitem].type_0 = TabPage;
            items[curitem].start = out_p;
            items[curitem].minwid = minwid;
            fmt_p = fmt_p.offset(1);
            curitem += 1;
            continue;
        }

        if *fmt_p == STL_CLICK_FUNC {
            fmt_p = fmt_p.offset(1);
            let t = fmt_p as *mut i8;
            while *fmt_p != STL_CLICK_FUNC && *fmt_p != 0 {
                fmt_p = fmt_p.offset(1);
            }
            if *fmt_p != STL_CLICK_FUNC {
                break;
            }
            items[curitem].type_0 = ClickFunc;
            items[curitem].start = out_p;
            items[curitem].cmd = xmemdupz(t, (fmt_p as *mut i8).offset_from(t) as usize);
            items[curitem].minwid = minwid;
            fmt_p = fmt_p.offset(1);
            curitem += 1;
            continue;
        }

        // Denotes the end of the minwid
        // the maxwid may follow immediately after
        if *fmt_p == '.' as u8 {
            fmt_p = fmt_p.offset(1);
            if ascii_isdigit(*fmt_p) {
                maxwid = getdigits_int(&mut fmt_p as *mut *mut _ as *mut *const _, false, 50)
            }
        }

        // Bound the minimum width at 50.
        // Make the number negative to denote left alignment of the item
        minwid = min(minwid, 50) * if left_align { -1 } else { 1 };

        // Denotes the start of a new group
        if *fmt_p == '(' as u8 {
            groupitems[groupdepth] = curitem;
            groupdepth = groupdepth + 1;
            items[curitem].type_0 = Group;
            items[curitem].start = out_p;
            items[curitem].minwid = minwid;
            items[curitem].maxwid = maxwid;
            fmt_p = fmt_p.offset(1);
            curitem += 1;
            continue;
        }

        // An invalid item was specified.
        // Continue processing on the next character of the format string.
        if vim_strchr(STL_ALL.as_ptr(), *fmt_p as i32).is_null() {
            fmt_p = fmt_p.offset(1);
            continue;
        }

        // The status line item type
        let opt = *fmt_p;
        fmt_p = fmt_p.offset(1);

        // OK - now for the real work
        let mut base = kNumBaseDecimal;
        let mut itemisflag = false;
        let mut fillable = true;
        let mut num: i64 = -1;
        let mut str = ptr::null();
        match opt {
            STL_FILEPATH | STL_FULLPATH | STL_FILENAME => {
                // Set fillable to false so that ' ' in the filename will not
                // get replaced with the fillchar
                fillable = false;
                if !buf_spname(wp.w_buffer).is_null() {
                    xstrlcpy(
                        NameBuff.as_mut_ptr() as *mut i8,
                        buf_spname(wp.w_buffer) as *const i8,
                        MAXPATHL,
                    );
                } else {
                    let t = if opt == STL_FULLPATH {
                        (*wp.w_buffer).b_ffname
                    } else {
                        (*wp.w_buffer).b_fname
                    };
                    home_replace(wp.w_buffer, t, NameBuff.as_mut_ptr(), MAXPATHL, true);
                }
                trans_characters(NameBuff.as_mut_ptr(), MAXPATHL as i32);
                if opt != STL_FILENAME {
                    str = NameBuff.as_ptr();
                } else {
                    str = path_tail(NameBuff.as_mut_ptr());
                }
            }
            STL_VIM_EXPR => {
                // '{'
                itemisflag = true;

                // Attempt to copy the expression to evaluate into
                // the output buffer as a null-terminated string.
                let mut t = out_p;
                while *fmt_p != '}' as u8 && *fmt_p != 0 && out_p < out_end_p {
                    *out_p = *fmt_p;
                    fmt_p = fmt_p.offset(1);
                    out_p = out_p.offset(1);
                }
                if *fmt_p as i32 != '}' as i32 {
                    // missing '}' or out of space
                } else {
                    fmt_p = fmt_p.offset(1);
                    *out_p = 0;

                    // Move our position in the output buffer
                    // to the beginning of the expression
                    out_p = t;

                    // { Evaluate the expression

                    // Store the current buffer number as a string variable
                    vim_snprintf(
                        tmp.as_ptr() as *const i8,
                        var_size(tmp),
                        S!("%d"),
                        (*curbuf).handle,
                    );
                    set_internal_string_var(S!("g:actual_curbuf"), tmp.as_ptr());

                    let save_curbuf = curbuf;
                    let save_curwin = curwin;
                    let save_VIsual_active = VIsual_active;
                    curwin = wp;
                    curbuf = wp.w_buffer;
                    // Visual mode is only valid in the current window.
                    if curwin != save_curwin {
                        VIsual_active = false_0;
                    }

                    // Note: The result stored in `t` is unused.
                    str = eval_to_string_safe(out_p, &mut t, use_sandbox);

                    curwin = save_curwin;
                    curbuf = save_curbuf;
                    VIsual_active = save_VIsual_active;

                    // Remove the variable we just stored
                    do_unlet(S!("g:actual_curbuf"), 15, true_0);

                    // }

                    // Check if the evaluated result is a number.
                    // If so, convert the number to an int and free the string.
                    if !str.is_null() && *str != 0 {
                        if *skipdigits(str) == 0 {
                            num = atoi(str as *const i8) as i64;
                            XFREE_CLEAR(&mut (str as *mut u8));
                            itemisflag = false;
                        }
                    }
                }
            }

            STL_LINE => {
                num = if (*wp.w_buffer).b_ml.ml_flags & ML_EMPTY != 0 {
                    0
                } else {
                    wp.w_cursor.lnum
                };
            }

            STL_NUMLINES => {
                num = (*wp.w_buffer).b_ml.ml_line_count;
            }

            STL_COLUMN => {
                num = if State & INSERT == 0 && empty_line {
                    0
                } else {
                    (wp.w_cursor.col) + 1
                } as i64;
            }

            STL_VIRTCOL | STL_VIRTCOL_ALT => {
                // In list mode virtcol needs to be recomputed
                let mut virtcol = wp.w_virtcol;
                if wp.w_onebuf_opt.wo_list != 0 && wp.w_p_lcs_chars.tab1 == NUL {
                    wp.w_onebuf_opt.wo_list = false_0;
                    let wp_raw = wp as *mut win_T; // getvcol wants 2 mut refs
                    getvcol(wp, &mut (*wp_raw).w_cursor, None, Some(&mut virtcol), None);
                    wp.w_onebuf_opt.wo_list = true_0;
                }
                virtcol += 1;
                // Don't display %V if it's the same as %c.
                if !(opt == STL_VIRTCOL_ALT
                    && virtcol
                        == (if State & INSERT == 0 && empty_line {
                            0
                        } else {
                            (wp.w_cursor.col) + 1
                        }))
                {
                    num = virtcol as i64;
                }
            }

            STL_PERCENTAGE => {
                num = wp.w_cursor.lnum * 100 / (*wp.w_buffer).b_ml.ml_line_count;
            }

            STL_ALTPERCENT => {
                // Store the position percentage in our temporary buffer.
                // Note: We cannot store the value in `num` because
                //       `get_rel_pos` can return a named position. Ex: "Top"
                get_rel_pos(wp, tmp.as_mut_ptr(), TMPLEN as i32);
                str = tmp.as_ptr();
            }

            STL_ARGLISTSTAT => {
                fillable = false;

                // Note: This is important because `append_arg_number` starts appending
                //       at the end of the null-terminated string.
                //       Setting the first byte to null means it will place the argument
                //       number string at the beginning of the buffer.
                tmp[0] = 0;

                // Note: The call will only return true if it actually
                //       appended data to the `tmp` buffer.
                if append_arg_number(wp, tmp.as_mut_ptr(), var_size(tmp) as i32, false) {
                    str = tmp.as_ptr();
                }
            }

            STL_KEYMAP => {
                fillable = false;
                if get_keymap_str(wp, S!("<%s>"), tmp.as_mut_ptr(), TMPLEN as i32) != 0 {
                    str = tmp.as_ptr();
                }
            }

            STL_PAGENUM => {
                num = printer_page_num;
            }

            STL_BUFNO => {
                num = (*wp.w_buffer).handle as i64;
            }

            STL_OFFSET_X | STL_OFFSET => {
                if opt == STL_OFFSET_X {
                    base = kNumBaseHexadecimal;
                }
                let l =
                    ml_find_line_or_offset(wp.w_buffer, wp.w_cursor.lnum, ptr::null_mut(), false);
                num = if (*wp.w_buffer).b_ml.ml_flags & ML_EMPTY != 0 || l < 0 {
                    0
                } else {
                    (l + 1)
                        + (if State & INSERT == 0 && empty_line {
                            0
                        } else {
                            wp.w_cursor.col
                        }) as i64
                };
            }

            STL_BYTEVAL_X | STL_BYTEVAL => {
                if opt == STL_BYTEVAL_X {
                    base = kNumBaseHexadecimal;
                }
                num = byteval as i64;
                if num == NL as i64 {
                    num = 0;
                } else if num == CAR as i64 && get_fileformat(wp.w_buffer) == EOL_MAC {
                    num = NL as i64;
                }
            }

            STL_ROFLAG | STL_ROFLAG_ALT => {
                itemisflag = true;
                if (*wp.w_buffer).b_p_ro != 0 {
                    str = if opt == STL_ROFLAG_ALT {
                        S!(",RO")
                    } else {
                        gettext("[RO]")
                    };
                }
            }

            STL_HELPFLAG | STL_HELPFLAG_ALT => {
                itemisflag = true;
                if (*wp.w_buffer).b_help {
                    str = if opt == STL_HELPFLAG_ALT {
                        S!(",HLP")
                    } else {
                        gettext("[Help]")
                    };
                }
            }

            STL_FILETYPE => {
                // Copy the filetype if it is not null and the formatted string will fit
                // in the temporary buffer
                // (including the brackets and null terminating character)
                if *(*wp.w_buffer).b_p_ft != 0
                    && strlen((*wp.w_buffer).b_p_ft as *mut i8) < (TMPLEN - 3) as u64
                {
                    vim_snprintf(
                        tmp.as_mut_ptr() as *mut i8,
                        var_size(tmp),
                        S!("[%s]"),
                        (*wp.w_buffer).b_p_ft,
                    );
                    str = tmp.as_ptr();
                }
            }

            STL_FILETYPE_ALT => {
                itemisflag = true;
                // Copy the filetype if it is not null and the formatted string will fit
                // in the temporary buffer
                // (including the comma and null terminating character)
                if *(*wp.w_buffer).b_p_ft != 0
                    && strlen((*wp.w_buffer).b_p_ft as *mut i8) < (TMPLEN - 2) as u64
                {
                    vim_snprintf(
                        tmp.as_mut_ptr() as *mut i8,
                        var_size(tmp),
                        S!(",%s"),
                        (*wp.w_buffer).b_p_ft,
                    );
                    // Uppercase the file extension
                    for t in tmp.iter_mut().take_while(|t| **t != 0) {
                        *t = toupper(*t as i32) as u8;
                    }
                    str = tmp.as_ptr();
                }
            }

            STL_PREVIEWFLAG | STL_PREVIEWFLAG_ALT => {
                itemisflag = true;
                if wp.w_onebuf_opt.wo_pvw != 0 {
                    str = if opt == STL_PREVIEWFLAG_ALT {
                        S!(",PRV")
                    } else {
                        gettext("[Preview]")
                    };
                }
            }

            STL_QUICKFIX => {
                if bt_quickfix(wp.w_buffer) {
                    str = if !wp.w_llist_ref.is_null() {
                        c_gettext(msg_loclist) as *const u8
                    } else {
                        c_gettext(msg_qflist) as *const u8
                    };
                }
            }
            STL_MODIFIED | STL_MODIFIED_ALT => {
                itemisflag = true;
                match (
                    opt == STL_MODIFIED_ALT,
                    bufIsChanged(wp.w_buffer),
                    !MODIFIABLE(wp.w_buffer),
                ) {
                    (false, true, false) => str = S!("[+]"),
                    (true, true, false) => str = S!(",+"),
                    (false, false, true) => str = S!("[-]"),
                    (true, false, true) => str = S!(",-"),
                    (false, true, true) => str = S!("[+-]"),
                    (true, true, true) => str = S!(",+-"),
                    _ => {}
                }
            }

            STL_HIGHLIGHT => {
                // { The name of the highlight is surrounded by `#`
                let t = fmt_p;
                while *fmt_p != '#' as u8 && *fmt_p != 0 {
                    fmt_p = fmt_p.offset(1)
                }
                // }

                // Create a highlight item based on the name
                if *fmt_p == '#' as u8 {
                    items[curitem].type_0 = Highlight;
                    items[curitem].start = out_p;
                    items[curitem].minwid = -syn_namen2id(t, fmt_p.offset_from(t) as i32);
                    curitem += 1;
                    fmt_p = fmt_p.offset(1);
                }
                continue;
            }
            _ => {}
        }

        // If we made it this far, the item is normal and starts at
        // our current position in the output buffer.
        // Non-normal items would have `continued`.
        items[curitem].start = out_p;
        items[curitem].type_0 = Normal;

        // Copy the item string into the output buffer
        if !str.is_null() && *str != 0 {
            // { Skip the leading `,` or ` ` if the item is a flag
            //  and the proper conditions are met
            let mut t = str;
            if itemisflag {
                if *t.offset(0) != 0
                    && *t.offset(1) != 0
                    && (!prevchar_isitem && *t == ',' as u8 || prevchar_isflag && *t == ' ' as u8)
                {
                    t = t.offset(1);
                }
                prevchar_isflag = true;
            }
            // }

            let mut l = vim_strsize(t) as i64;

            // If this item is non-empty, record that the last thing
            // we put in the output buffer was an item
            if l > 0 {
                prevchar_isitem = true;
            }

            // If the item is too wide, truncate it from the beginning
            if l > maxwid as i64 {
                while l >= maxwid as i64 {
                    if has_mbyte {
                        l -= ptr2cells(t) as i64;
                        t = t.offset(mb_ptr2len(t) as isize);
                    } else {
                        l -= byte2cells(*t as i32) as i64;
                        t = t.offset(1);
                    }
                }

                // Early out if there isn't enough room for the truncation marker
                if out_p >= out_end_p {
                    break;
                }

                // Add the truncation marker
                *out_p = '<' as u8;
                out_p = out_p.offset(1);
            }

            // If the item is right aligned and not wide enough,
            // pad with fill characters.
            if minwid > 0 {
                while l < minwid as i64 && out_p < out_end_p {
                    // Don't put a "-" in front of a digit.
                    if l + 1 == minwid as i64 && fillchar == '-' as u8 && ascii_isdigit(*t) {
                        *out_p = ' ' as u8;
                    } else {
                        *out_p = fillchar;
                    }
                    out_p = out_p.offset(1);
                    l += 1;
                }
                minwid = 0;
            } else {
                // Note: The negative value denotes a left aligned item.
                //       Here we switch the minimum width back to a positive value.
                minwid *= -1;
            }

            // { Copy the string text into the output buffer
            while *t != 0 && out_p < out_end_p {
                *out_p = *t;
                t = t.offset(1);
                out_p = out_p.offset(1);
                // Change a space by fillchar, unless fillchar is '-' and a
                // digit follows.
                if fillable
                    && *out_p.offset(-1) == ' ' as u8
                    && (!ascii_isdigit(*t) || fillchar != '-' as u8)
                {
                    *out_p.offset(-1) = fillchar;
                }
            }
            // }

            // For left-aligned items, fill any remaining space with the fillchar
            while l < minwid as i64 && out_p < out_end_p {
                *out_p = fillchar;
                out_p = out_p.offset(1);
                l += 1;
            }
        } else if num >= 0 {
            // Otherwise if the item is a number, copy that to the output buffer.
            if out_p.offset(20) > out_end_p {
                break; // not sufficient space
            }
            prevchar_isitem = true;

            // { Build the formatting string
            let mut nstr: [u8; 20] = [0; 20];
            let mut t = nstr.as_mut_ptr();
            if opt == STL_VIRTCOL_ALT {
                *t = '-' as i32 as u8;
                t = t.offset(1);
                minwid -= 1;
            }
            *t = '%' as u8;
            t = t.offset(1);
            if zeropad {
                *t = '0' as u8;
                t = t.offset(1);
            }

            // Note: The `*` means we take the width as one of the arguments
            *post_off!(t) = '*' as u8;
            *post_off!(t) = if base == kNumBaseHexadecimal {
                'X'
            } else {
                'd'
            } as u8;
            *t = 0;
            // }

            // { Determine how many characters the number will take up when printed
            //  Note: We have to cast the base because the compiler uses
            //        unsigned ints for the enum values.
            let mut num_chars = 1;
            for _ in it!(n = num; n >= base as i64; n / base as i64) {
                num_chars += 1;
            }

            // VIRTCOL_ALT takes up an extra character because
            // of the `-` we added above.
            if opt == STL_VIRTCOL_ALT {
                num_chars += 1;
            }
            // }

            assert!(out_end_p >= out_p);
            let remaining_buf_len = out_end_p.offset_from(out_p) as size_t + 1;

            // If the number is going to take up too much room
            // Figure out the approximate number in "scientific" type notation.
            // Ex: 14532 with maxwid of 4 -> '14>3'
            if num_chars > maxwid as i64 {
                // Add two to the width because the power piece will take
                // two extra characters
                num_chars += 2;

                // How many extra characters there are
                let n = num_chars - maxwid as i64;

                // { Reduce the number by base^n
                while post_dec!(num_chars) > maxwid as i64 {
                    num /= base as i64;
                }
                // }

                // { Add the format string for the exponent bit
                *post_off!(t) = '>' as u8;
                *post_off!(t) = '%' as u8;
                // Use the same base as the first number
                *t = *t.offset(-3);
                t = t.offset(1);
                *t = 0;
                // }

                vim_snprintf(
                    out_p as *mut i8,
                    remaining_buf_len,
                    nstr.as_ptr() as *const i8,
                    0,
                    num,
                    n,
                );
            } else {
                vim_snprintf(
                    out_p as *mut i8,
                    remaining_buf_len,
                    nstr.as_ptr() as *const i8,
                    minwid,
                    num,
                );
            }

            // Advance the output buffer position to the end of the
            // number we just printed
            out_p = out_p.offset(strlen(out_p as *mut i8) as isize);
        } else {
            // Otherwise, there was nothing to print so mark the item as empty
            items[curitem].type_0 = Empty;
        }

        // Only free the string buffer if we allocated it.
        // Note: This is not needed if `str` is pointing at `tmp`
        if opt == STL_VIM_EXPR {
            xfree(str as *mut u8);
        }

        if num >= 0 || !itemisflag && !str.is_null() && *str != 0 {
            prevchar_isflag = false; // Item not NULL, but not a flag
        }

        // Item processed, move to the next
        curitem += 1;
    }

    *out_p = 0;
    let mut itemcnt = curitem;

    // Free the format buffer if we allocated it internally
    if usefmt != fmt {
        xfree(usefmt);
    }

    // We have now processed the entire statusline format string.
    // What follows is post-processing to handle alignment and highlighting.

    let mut width = vim_strsize(out);
    if maxwidth > 0 && width > maxwidth {
        // Result is too long, must truncate somewhere.
        let mut item_idx = 0;
        let mut trunc_p: *mut u8;

        // If there are no items, truncate from beginning
        if itemcnt == 0 {
            trunc_p = out
        } else {
            // Otherwise, look for the truncation item

            // Default to truncating at the first item
            trunc_p = items[0].start;
            item_idx = 0;

            for i in 0..itemcnt {
                if items[i].type_0 == Trunc {
                    // Truncate at %< items.
                    trunc_p = items[i].start;
                    item_idx = i;
                    break;
                }
            }
        }

        // If the truncation point we found is beyond the maximum
        // length of the string, truncate the end of the string.
        if width - vim_strsize(trunc_p) >= maxwidth {
            // If we are using a multi-byte encoding, walk from the beginning of the
            // string to find the last character that will fit.
            if has_mbyte {
                trunc_p = out;
                width = 0;
                loop {
                    width += ptr2cells(trunc_p);
                    if width >= maxwidth {
                        break;
                    }

                    // Note: Only advance the pointer if the next
                    //       character will fit in the available output space
                    trunc_p = trunc_p.offset(mb_ptr2len(trunc_p) as isize);
                }
            } else {
                // Otherwise put the truncation point at the end, leaving enough room
                // for a single-character truncation marker
                trunc_p = out.offset(maxwidth as isize - 1);
            }

            // Ignore any items in the statusline that occur after
            // the truncation point
            for i in 0..itemcnt {
                if items[i].start > trunc_p {
                    itemcnt = i;
                    break;
                }
            }

            // Truncate the output
            *post_off!(trunc_p) = '>' as u8;
            *trunc_p = 0
        } else {
            // Truncate at the truncation point we found

            // { Determine how many bytes to remove
            let mut trunc_len: i64;
            if has_mbyte {
                trunc_len = 0;
                while width >= maxwidth {
                    width -= ptr2cells(trunc_p.offset(trunc_len as isize));
                    trunc_len += mb_ptr2len(trunc_p.offset(trunc_len as isize)) as i64;
                }
            } else {
                // Truncate an extra character so we can insert our `<`.
                trunc_len = (width - maxwidth) as i64 + 1;
            }
            // }

            // { Truncate the string
            let trunc_end_p = trunc_p.offset(trunc_len as isize);
            STRMOVE(trunc_p.offset(1), trunc_end_p);

            // Put a `<` to mark where we truncated at
            *trunc_p = '<' as u8;

            if (width + 1) < maxwidth {
                // Advance the pointer to the end of the string
                trunc_p = trunc_p.offset(strlen(trunc_p as *mut i8) as isize);
            }

            // Fill up for half a double-wide character.
            while pre_inc!(width) < maxwidth {
                *post_off!(trunc_p) = fillchar;
                *trunc_p = 0;
            }
            // }

            // { Change the start point for items based on
            //  their position relative to our truncation point

            // Note: The offset is one less than the truncation length because
            //       the truncation marker `<` is not counted.
            let item_offset = trunc_len - 1;

            for i in item_idx..itemcnt {
                // Items starting at or after the end of the truncated section need
                // to be moved backwards.
                if items[i].start >= trunc_end_p {
                    items[i].start = items[i].start.offset(-item_offset as isize);
                } else {
                    // Anything inside the truncated area is set to start
                    // at the `<` truncation character.
                    items[i].start = trunc_p;
                }
            }
            // }
        }
        width = maxwidth;
    } else if width < maxwidth
        && strlen(out as *mut i8) + (maxwidth - width) as u64 + 1 < outlen as _
    {
        // If there is room left in our statusline, and room left in our buffer,
        // add characters at the separate marker (if there is one) to
        // fill up the available space.

        // Find how many separators there are, which we will use when
        // figuring out how many groups there are.
        let mut num_separators: usize = 0;
        for i in 0..itemcnt {
            if items[i].type_0 == Separate {
                num_separators += 1;
            }
        }

        // If we have separated groups, then we deal with it now
        if num_separators != 0 {
            // Create an array of the start location for each
            // separator mark.
            let mut separator_locations: [usize; STL_MAX_ITEM] = [0; STL_MAX_ITEM];
            let mut index = 0;
            for i in 0..itemcnt {
                if items[i].type_0 == Separate {
                    separator_locations[index] = i;
                    index += 1;
                }
            }

            let standard_spaces = (maxwidth - width) as usize / num_separators;
            let final_spaces = (maxwidth - width) as usize - standard_spaces * (num_separators - 1);

            for i in 0..num_separators {
                let dislocation = if i == num_separators - 1 {
                    final_spaces
                } else {
                    standard_spaces
                };
                let seploc = items[separator_locations[i]]
                    .start
                    .offset(dislocation as isize);
                STRMOVE(seploc, items[separator_locations[i]].start);
                for s in it!(s = items[separator_locations[i]].start; s < seploc; s.offset(1)) {
                    *s = fillchar;
                }

                for item_idx in it!(i = separator_locations[i] + 1; i < itemcnt; i + 1) {
                    items[item_idx].start = items[item_idx].start.offset(dislocation as isize);
                }
            }

            width = maxwidth
        }
    }

    // Store the info about highlighting.
    if !hltab.is_null() {
        let mut sp = hltab;
        for l in 0..itemcnt {
            if items[l].type_0 == Highlight {
                (*sp).start = items[l].start;
                (*sp).userhl = items[l].minwid;
                sp = sp.offset(1);
            }
        }
        (*sp).start = ptr::null_mut();
        (*sp).userhl = 0;
    }

    // Store the info about tab pages labels.
    if !tabtab.is_null() {
        let mut cur_tab_rec = tabtab;
        for l in 0..itemcnt {
            if items[l].type_0 == TabPage {
                (*cur_tab_rec).start = items[l].start as *mut i8;
                if items[l].minwid == 0 {
                    (*cur_tab_rec).def.type_0 = kStlClickDisabled;
                    (*cur_tab_rec).def.tabnr = 0;
                } else {
                    let mut tabnr = items[l].minwid;
                    if items[l].minwid > 0 {
                        (*cur_tab_rec).def.type_0 = kStlClickTabSwitch;
                    } else {
                        (*cur_tab_rec).def.type_0 = kStlClickTabClose;
                        tabnr = -tabnr;
                    }
                    (*cur_tab_rec).def.tabnr = tabnr;
                }
                (*cur_tab_rec).def.func = ptr::null_mut();
                cur_tab_rec = cur_tab_rec.offset(1);
            } else if items[l].type_0 == ClickFunc {
                (*cur_tab_rec).start = items[l].start as *mut i8;
                (*cur_tab_rec).def.type_0 = kStlClickFuncRun;
                (*cur_tab_rec).def.tabnr = items[l].minwid;
                (*cur_tab_rec).def.func = items[l].cmd;
                cur_tab_rec = cur_tab_rec.offset(1);
            }
        }
        (*cur_tab_rec).start = ptr::null_mut();
        (*cur_tab_rec).def.type_0 = kStlClickDisabled;
        (*cur_tab_rec).def.tabnr = 0;
        (*cur_tab_rec).def.func = ptr::null_mut();
    }

    // When inside update_screen we do not want redrawing a stausline, ruler,
    // title, etc. to trigger another redraw, it may cause an endless loop.
    if updating_screen != 0 {
        must_redraw = save_must_redraw;
        (*curwin).w_redr_type = save_redr_type;
    }

    return width;
}

/*
 * Get relative cursor position in window into "buf[buflen]", in the form 99%,
 * using "Top", "Bot" or "All" when appropriate.
 */
#[no_mangle]
pub unsafe extern "C" fn get_rel_pos(wp: *const win_T, buf: *mut u8, buflen: i32) {
    // Need at least 3 chars for writing.
    if buflen < 3 {
        return;
    }
    let wp = wp.as_ref().unwrap();

    let mut above: i64; // number of lines above window
    let below: i64; // number of lines below window

    above = wp.w_topline - 1;
    above += (diff_check_fill(wp, wp.w_topline) - wp.w_topfill) as i64;
    if wp.w_topline == 1 && wp.w_topfill >= 1 {
        // All buffer lines are displayed and there is an indication
        // of filler lines, that can be considered seeing all lines.
        above = 0;
    }
    below = (*wp.w_buffer).b_ml.ml_line_count - wp.w_botline + 1;
    if below <= 0 {
        xstrlcpy(
            buf as *mut i8,
            if above == 0 {
                gettext("All")
            } else {
                gettext("Bot")
            },
            buflen as size_t,
        );
    } else if above <= 0 {
        xstrlcpy(buf as *mut i8, gettext("Top"), buflen as size_t);
    } else {
        vim_snprintf(
            buf as *mut i8,
            buflen as size_t,
            S!("%2d%%"),
            if above > 1000000 {
                (above / ((above + below) / 100)) as i32
            } else {
                (above * 100 / (above + below)) as i32
            },
        );
    }
}

/// Append (file 2 of 8) to "buf[buflen]", if editing more than one file.
///
/// @param          wp        window whose buffers to check
/// @param[in,out]  buf       string buffer to add the text to
/// @param          buflen    length of the string buffer
/// @param          add_file  if true, add "file" before the arg number
///
/// @return true if it was appended.
unsafe extern "C" fn append_arg_number(
    wp: *const win_T,
    buf: *mut u8,
    buflen: i32,
    add_file: bool,
) -> bool {
    // Nothing to do
    if ARGCOUNT() <= 1 {
        return false;
    }
    let wp = wp.as_ref().unwrap();

    let mut p = buf.offset(strlen(buf as *mut i8) as isize); // go to the end of the buffer

    // Early out if the string is getting too long
    if p.offset_from(buf) + 35 >= buflen as isize {
        return false;
    }

    *post_off!(p) = ' ' as u8;
    *post_off!(p) = '(' as u8;
    if add_file {
        strcpy(p as *mut i8, S!("file "));
        p = p.offset(5);
    }
    vim_snprintf(
        p as *mut i8,
        buflen as usize - p.offset_from(buf) as usize,
        if wp.w_arg_idx_invalid != 0 {
            S!("(%d) of %d)")
        } else {
            S!("%d of %d)")
        },
        wp.w_arg_idx + 1,
        ARGCOUNT(),
    );
    return true;
}

/*
 * Make "ffname" a full file name, set "sfname" to "ffname" if not NULL.
 * "ffname" becomes a pointer to allocated memory (or NULL).
 */
#[no_mangle]
pub unsafe extern "C" fn fname_expand(
    _buf: *mut buf_T,
    ffname: *mut *mut u8,
    sfname: *mut *mut u8,
) {
    if (*ffname).is_null() {
        return; // if no file name given, nothing to do
    }
    if (*sfname).is_null() {
        *sfname = *ffname; // if no short file name given, use ffname
    }
    *ffname = fix_fname(*ffname as *mut i8) as *mut u8; // expand to full path

    if cfg!(target_os = "windows") {
        //TODO: Windows
    }
}

/*
 * Get the file name for an argument list entry.
 */
#[no_mangle]
pub unsafe extern "C" fn alist_name(aep: *const aentry_T) -> *mut u8 {
    let aep = aep.as_ref().unwrap();
    let bp: *mut buf_T;

    // Use the name from the associated buffer if it exists.
    bp = buflist_findnr(aep.ae_fnum);
    if bp.is_null() || (*bp).b_fname.is_null() {
        return aep.ae_fname;
    }
    return (*bp).b_fname;
}

/*
 * do_arg_all(): Open up to 'count' windows, one for each argument.
 */
#[no_mangle]
pub unsafe extern "C" fn do_arg_all(
    mut count: i32,
    forceit: i32,   // hide buffers in current windows
    keep_tabs: i32, // keep current tabs, for ":tab drop file"
) {
    let forceit = forceit != 0;
    let keep_tabs = keep_tabs != 0;
    let opened: *mut u8; // Array of weight for which args are open:
                         //  0: not opened
                         //  1: opened in other tab
                         //  2: opened in curtab
                         //  3: opened in curtab and curwin

    let opened_len: i32; // length of opened[]
    let mut use_firstwin = false; // use first window for arglist
    let mut split_ret = OK;
    let mut alist: &mut alist_T; // argument list to be used
    let had_tab = cmdmod.tab;
    let mut new_curwin: *mut win_T = ptr::null_mut();
    let mut new_curtab: *mut tabpage_T = ptr::null_mut();

    assert!(!firstwin.is_null()); // satisfy coverity

    if ARGCOUNT() <= 0 {
        /* Don't give an error message.  We don't want it when the ":all"
         * command is in the .vimrc. */
        return;
    }
    setpcmark();

    opened_len = ARGCOUNT();
    opened = xcalloc(opened_len as size_t, 1) as *mut u8;

    /* Autocommands may do anything to the argument list.  Make sure it's not
     * freed while we are working here by "locking" it.  We still have to
     * watch out for its size to be changed. */
    alist = (*curwin).w_alist.as_mut().unwrap();
    alist.al_refcount += 1;

    let old_curwin: *mut win_T = curwin;
    let old_curtab: *mut tabpage_T = curtab;

    /*
     * Try closing all windows that are not in the argument list.
     * Also close windows that are not full width;
     * When 'hidden' or "forceit" set the buffer becomes hidden.
     * Windows that have a changed buffer and can't be hidden won't be closed.
     * When the ":tab" modifier was used do this for all tab pages.
     */
    if had_tab > 0 {
        goto_tabpage_tp(first_tabpage, true_0, true_0);
    }
    loop {
        let mut tpnext: *mut tabpage_T = (*curtab).tp_next;
        let mut wpnext = firstwin;
        while let Some(wp) = wpnext.as_mut() {
            wpnext = wp.w_next;
            let buf = wp.w_buffer.as_mut().unwrap();
            let mut last_i = None;
            if !(buf.b_ffname.is_null()
                || !keep_tabs && (buf.b_nwindows > 1 || wp.w_width != Columns))
            {
                // check if the buffer in this window is in the arglist
                for i in 0..opened_len {
                    if i < alist.al_ga.ga_len
                        && ((*AARGLIST(alist).offset(i as isize)).ae_fnum == buf.handle
                            || path_full_compare(
                                alist_name(&mut *AARGLIST(alist).offset(i as isize)),
                                buf.b_ffname,
                                true,
                                true,
                            ) as u32
                                & kEqualFiles
                                != 0)
                    {
                        last_i = Some(i);
                        let mut weight = 1;

                        if old_curtab == curtab {
                            weight += 1;
                            if old_curwin == wp as *mut _ {
                                weight += 1;
                            }
                        }

                        if weight > *opened.offset(i as isize) as i32 {
                            *opened.offset(i as isize) = weight as u8;
                            if i == 0 {
                                if !new_curwin.is_null() {
                                    (*new_curwin).w_arg_idx = opened_len;
                                }
                                new_curwin = wp as *mut _;
                                new_curtab = curtab;
                            }
                        } else if keep_tabs {
                            last_i = None;
                        }

                        if wp.w_alist != alist {
                            /* Use the current argument list for all windows
                             * containing a file from it. */
                            alist_unlink(wp.w_alist);
                            wp.w_alist = alist;
                            (*wp.w_alist).al_refcount += 1;
                        }
                        break;
                    }
                }
            }
            wp.w_arg_idx = last_i.unwrap_or(opened_len);

            if last_i.is_none() && !keep_tabs {
                // close this window
                if buf_hide(buf) || forceit || buf.b_nwindows > 1 || !bufIsChanged(buf) {
                    /* If the buffer was changed, and we would like to hide it,
                     * try autowriting. */
                    if !buf_hide(buf) && buf.b_nwindows <= 1 && bufIsChanged(buf) {
                        let mut bufref = bufref_T::default();
                        set_bufref(&mut bufref, buf);
                        autowrite(buf, false_0);
                        // Check if autocommands removed the window.
                        if !win_valid(wp) || !bufref_valid(&mut bufref) {
                            wpnext = firstwin; // Start all over...
                            continue;
                        }
                    }
                    // don't close last window
                    if ONE_WINDOW!() && ((*first_tabpage).tp_next.is_null() || had_tab == 0) {
                        use_firstwin = true;
                    } else {
                        win_close(wp, !buf_hide(buf) && !bufIsChanged(buf));
                        // check if autocommands removed the next window
                        if !win_valid(wpnext) {
                            // start all over...
                            wpnext = firstwin;
                        }
                    }
                }
            }
        }
        // Without the ":tab" modifier only do the current tab page.
        if had_tab == 0 || tpnext.is_null() {
            break;
        }
        // check if autocommands removed the next tab page
        if !valid_tabpage(tpnext) {
            tpnext = first_tabpage; // start all over...
        }
        goto_tabpage_tp(tpnext, true_0, true_0);
    }

    /*
     * Open a window for files in the argument list that don't have one.
     * ARGCOUNT may change while doing this, because of autocommands.
     */
    if count > opened_len || count <= 0 {
        count = opened_len
    }

    // Don't execute Win/Buf Enter/Leave autocommands here.
    autocmd_no_enter += 1;
    autocmd_no_leave += 1;
    let last_curwin: *mut win_T = curwin;
    let last_curtab: *mut tabpage_T = curtab;
    win_enter(lastwin, false);
    // ":drop all" should re-use an empty window to avoid "--remote-tab"
    // leaving an empty tab page when executed locally.
    if keep_tabs
        && BUFEMPTY()
        && (*curbuf).b_nwindows == 1
        && (*curbuf).b_ffname.is_null()
        && (*curbuf).b_changed == 0
    {
        use_firstwin = true;
    }

    for i in 0..min(count, opened_len) {
        if got_int != 0 {
            break;
        }
        if alist as *mut alist_T == &mut global_alist && i == global_alist.al_ga.ga_len - 1 {
            arg_had_last = true;
        }
        if *opened.offset(i as isize) > 0 {
            // Move the already present window to below the current window
            if (*curwin).w_arg_idx != i {
                FOR_ALL_WINDOWS_IN_TAB!(wp, curtab, {
                    if (*wp).w_arg_idx == i {
                        if keep_tabs {
                            new_curwin = wp;
                            new_curtab = curtab;
                        } else {
                            win_move_after(wp, curwin);
                        }
                        break;
                    }
                });
            }
        } else if split_ret == OK {
            if !use_firstwin {
                // split current window
                let p_ea_save = p_ea;
                p_ea = true_0; // use space from all windows
                split_ret = win_split(0, WSP_ROOM | WSP_BELOW);
                p_ea = p_ea_save;
                if split_ret == FAIL {
                    continue;
                }
            } else {
                // first window: do autocmd for leaving this buffer
                autocmd_no_leave -= 1;
            }

            /*
             * edit file "i"
             */
            (*curwin).w_arg_idx = i;
            if i == 0 {
                new_curwin = curwin;
                new_curtab = curtab;
            }
            do_ecmd(
                0,
                alist_name(&mut *AARGLIST(alist).offset(i as isize)),
                ptr::null_mut(),
                ptr::null_mut(),
                ECMD_ONE,
                (if buf_hide((*curwin).w_buffer) || bufIsChanged((*curwin).w_buffer) {
                    ECMD_HIDE
                } else {
                    0
                }) + ECMD_OLDBUF,
                curwin,
            );
            if use_firstwin {
                autocmd_no_leave += 1
            }
            use_firstwin = false;
        }
        os_breakcheck();

        // When ":tab" was used open a new tab for a new window repeatedly.
        if had_tab > 0 && tabpage_index(ptr::null_mut()) as i64 <= p_tpm {
            cmdmod.tab = 9999;
        }
    }

    // Remove the "lock" on the argument list.
    alist_unlink(alist);

    autocmd_no_enter -= 1;
    // restore last referenced tabpage's curwin
    if last_curtab != new_curtab {
        if valid_tabpage(last_curtab) {
            goto_tabpage_tp(last_curtab, true_0, true_0);
        }
        if win_valid(last_curwin) {
            win_enter(last_curwin, false);
        }
    }
    // to window with first arg
    if valid_tabpage(new_curtab) {
        goto_tabpage_tp(new_curtab, true_0, true_0);
    }
    if win_valid(new_curwin) {
        win_enter(new_curwin, false);
    }

    autocmd_no_leave -= 1;
    xfree(opened);
}

/// Return TRUE if "buf" is a prompt buffer.
#[no_mangle]
pub unsafe extern "C" fn bt_prompt(buf: *const buf_T) -> i32 {
    return buf
        .as_ref()
        .map_or(false, |buf| *buf.b_p_bt.offset(0) == 'p' as u8) as i32;
}

/*
 * Open a window for a number of buffers.
 */
#[no_mangle]
pub unsafe extern "C" fn ex_buffer_all(eap: *const exarg_T) {
    let eap = eap.as_ref().unwrap();
    let mut wp: *mut win_T;
    let mut wpnext: *mut win_T;
    let mut split_ret = OK;
    let mut open_wins = 0;
    let count: i64; // Maximum number of windows to open.
    let all: i32; // When true also load inactive buffers.
    let had_tab = cmdmod.tab;
    let mut tpnext: *mut tabpage_T;

    if eap.addr_count == 0 {
        count = 9999; // make as many windows as possible
    } else {
        count = eap.line2; // make as many windows as specified
    }
    if eap.cmdidx == CMD_unhide || eap.cmdidx == CMD_sunhide {
        all = false_0;
    } else {
        all = true_0;
    }

    setpcmark();

    /*
     * Close superfluous windows (two windows for the same buffer).
     * Also close windows that are not full-width.
     */
    if had_tab > 0 {
        goto_tabpage_tp(first_tabpage, true_0, true_0);
    }
    loop {
        tpnext = (*curtab).tp_next;
        wpnext = firstwin;
        while let Some(wp) = wpnext.as_mut() {
            wpnext = wp.w_next;
            if ((*wp.w_buffer).b_nwindows > 1
                || (if cmdmod.split & WSP_VERT != 0 {
                    ((wp.w_height + wp.w_status_height) as i64)
                        < Rows as i64 - p_ch - tabline_height() as i64
                } else {
                    wp.w_width != Columns
                })
                || had_tab > 0 && wp as *mut _ != firstwin)
                && !ONE_WINDOW!()
                && !(wp.w_closing || (*wp.w_buffer).b_locked > 0)
            {
                win_close(wp, false);
                wpnext = firstwin; // just in case an autocommand does
                                   // something strange with windows
                                   // start all over...
                tpnext = first_tabpage;
                open_wins = 0;
            } else {
                open_wins += 1;
            }
        }

        // Without the ":tab" modifier only do the current tab page.
        if had_tab == 0 || tpnext.is_null() {
            break;
        }
        goto_tabpage_tp(tpnext, true_0, true_0);
    }

    //
    // Go through the buffer list.  When a buffer doesn't have a window yet,
    // open one.  Otherwise move the window to the right position.
    // Watch out for autocommands that delete buffers or windows!
    //
    // Don't execute Win/Buf Enter/Leave autocommands here.
    autocmd_no_enter += 1;
    win_enter(lastwin, false);
    autocmd_no_leave += 1;
    for buf in it!(buf = &firstbuf; !buf.is_null(); &(**buf).b_next) {
        let buf = *buf;
        if !((open_wins as i64) < count) {
            break;
        }

        // Check if this buffer needs a window
        if all == 0 && (*buf).b_ml.ml_mfp.is_null() || (*buf).b_p_bl == 0 {
            continue;
        }

        if had_tab != 0 {
            // With the ":tab" modifier don't move the window.
            if (*buf).b_nwindows > 0 {
                wp = lastwin; // buffer has a window, skip it
            } else {
                wp = ptr::null_mut();
            }
        } else {
            // Check if this buffer already has a window
            wp = firstwin;
            while !wp.is_null() {
                if (*wp).w_buffer == buf {
                    break;
                }
                wp = (*wp).w_next
            }
            // If the buffer already has a window, move it
            if !wp.is_null() {
                win_move_after(wp, curwin);
            }
        }

        if wp.is_null() && split_ret == OK {
            let mut bufref = bufref_T::default();
            set_bufref(&mut bufref, buf);
            // Split the window and put the buffer in it.
            let p_ea_save = p_ea;
            p_ea = true_0; // use space from all windows
            split_ret = win_split(0, WSP_ROOM | WSP_BELOW);
            open_wins += 1;
            p_ea = p_ea_save;
            if split_ret == FAIL {
                continue;
            }

            // Open the buffer in this window.
            swap_exists_action = SEA_DIALOG;
            set_curbuf(buf, DOBUF_GOTO);
            if !bufref_valid(&mut bufref) {
                // Autocommands deleted the buffer.
                swap_exists_action = SEA_NONE;
                break;
            }
            if swap_exists_action == SEA_QUIT {
                let mut cs = cleanup_T::default();

                // Reset the error/interrupt/exception state here so that
                // aborting() returns false when closing a window.
                enter_cleanup(&mut cs);

                // User selected Quit at ATTENTION prompt; close this window.
                win_close(curwin, true);
                open_wins -= 1;
                swap_exists_action = SEA_NONE;
                swap_exists_did_quit = true_0;

                /* Restore the error/interrupt/exception state if not
                 * discarded by a new aborting error, interrupt, or uncaught
                 * exception. */
                leave_cleanup(&mut cs);
            } else {
                handle_swap_exists(ptr::null_mut());
            }
        }
        os_breakcheck();
        if got_int != 0 {
            vgetc(); // only break the file loading, not the rest
            break;
        }
        // Autocommands deleted the buffer or aborted script processing!!!
        if aborting() != 0 {
            break;
        }
        // When ":tab" was used open a new tab for a new window repeatedly.
        if had_tab > 0 && tabpage_index(ptr::null_mut()) as i64 <= p_tpm {
            cmdmod.tab = 9999;
        }
    }
    autocmd_no_enter -= 1;
    win_enter(firstwin, false); // back to first window
    autocmd_no_leave -= 1;

    /*
     * Close superfluous windows.
     */
    wp = lastwin;
    while open_wins as i64 > count {
        let r = buf_hide((*wp).w_buffer)
            || !bufIsChanged((*wp).w_buffer)
            || autowrite((*wp).w_buffer, false_0) == OK;
        if !win_valid(wp) {
            // BufWrite Autocommands made the window invalid, start over
            wp = lastwin;
        } else if r {
            win_close(wp, !buf_hide((*wp).w_buffer));
            open_wins -= 1;
            wp = lastwin;
        } else {
            wp = (*wp).w_prev;
            if wp.is_null() {
                break;
            }
        }
    }
}

/*
 * do_modelines() - process mode lines for the current file
 *
 * "flags" can be:
 * OPT_WINONLY	    only set options local to window
 * OPT_NOWIN	    don't set options local to window
 *
 * Returns immediately if the "ml" option isn't set.
 */
#[no_mangle]
pub unsafe extern "C" fn do_modelines(flags: i32) {
    let mut nmlines: i64 = p_mls;
    static mut entered: i32 = 0;

    if (*curbuf).b_p_ml == 0 || nmlines == 0 {
        return;
    }

    /* Disallow recursive entry here.  Can happen when executing a modeline
     * triggers an autocommand, which reloads modelines with a ":do". */
    if entered != 0 {
        return;
    }

    entered += 1;
    for lnum in 1..=(*curbuf).b_ml.ml_line_count {
        if !(lnum <= nmlines) {
            break;
        }
        if chk_modeline(lnum, flags) == FAIL {
            nmlines = 0;
        }
    }

    for lnum in (1..=(*curbuf).b_ml.ml_line_count).rev() {
        if !(lnum > nmlines && lnum > (*curbuf).b_ml.ml_line_count - nmlines) {
            break;
        }
        if chk_modeline(lnum, flags) == FAIL {
            nmlines = 0;
        }
    }
    entered -= 1;
}

/*
 * chk_modeline() - check a single line for a mode string
 * Return FAIL if an error encountered.
 */
unsafe fn chk_modeline(
    lnum: linenr_T,
    flags: i32, // Same as for do_modelines().
) -> i32 {
    let mut s: *mut u8;
    let mut e: *mut u8;
    let linecopy: *mut u8; // local copy of any modeline found
    let mut prev: i32;
    let mut vers: intmax_t = 0;
    let mut retval = OK;
    let save_sourcing_name: *mut u8;
    let save_sourcing_lnum: linenr_T;

    prev = -1;
    cfor!(s = ml_get(lnum); *s != 0; s = s.offset(1); {
        if prev == -1 || ascii_isspace(prev) {
            if prev != -1 && strncmp(s as *mut i8, S!("ex:"), 3) == 0
                || strncmp(s as *mut i8, S!("vi:"), 3) == 0
            {
                break;
            }
            // Accept both "vim" and "Vim".
            if (*s.offset(0) == 'v' as u8 || *s.offset(0) == 'V' as u8)
                && *s.offset(1) == 'i' as u8
                && *s.offset(2) == 'm' as u8
            {
                if *s.offset(3) == '<' as u8
                    || *s.offset(3) == '=' as u8
                    || *s.offset(3) == '>' as u8
                {
                    e = s.offset(4);
                } else {
                    e = s.offset(3);
                }
                if !try_getdigits(&mut e as *mut *mut _ as *mut *const _, &mut vers) {
                    continue;
                }
                if *e == ':' as u8
                    && (*s.offset(0) != 'V' as u8
                        || strncmp(skipwhite(e.offset(1)) as *mut i8, S!("set"), 3) == 0)
                    && (*s.offset(3) == ':' as u8
                        || VIM_VERSION_100 as i64 >= vers && ascii_isdigit(*s.offset(3))
                        || (VIM_VERSION_100 as i64) < vers && *s.offset(3) == '<' as u8
                        || VIM_VERSION_100 as i64 > vers && *s.offset(3) == '>' as u8
                        || VIM_VERSION_100 as i64 == vers && *s.offset(3) == '=' as u8)
                {
                    break;
                }
            }
        }
        prev = *s as i32;
    });

    if *s == 0 {
        return retval;
    }

    loop {
        // skip over "ex:", "vi:" or "vim:"
        s = s.offset(1);
        if !(*s.offset(-1) != ':' as u8) {
            break;
        }
    }

    linecopy = vim_strsave(s); // copy the line, it will change
    s = linecopy;

    save_sourcing_lnum = sourcing_lnum;
    save_sourcing_name = sourcing_name;
    sourcing_lnum = lnum; // prepare for emsg()
    sourcing_name = S!("modelines") as *mut _;

    let mut end = false;
    while end == false {
        s = skipwhite(s) as *mut _;
        if *s == 0 {
            break;
        }

        /*
         * Find end of set command: ':' or end of line.
         * Skip over "\:", replacing it with ":".
         */
        cfor!(e = s; *e != ':' as u8 && *e != 0; e = e.offset(1); {
            if *e.offset(0) == '\\' as u8 && *e.offset(1) == ':' as u8 {
                memmove(e, e.offset(1), strlen(e.offset(1) as *mut i8) + 1);
            }
        });
        if *e == 0 {
            end = true;
        }

        /*
         * If there is a "set" command, require a terminating ':' and
         * ignore the stuff after the ':'.
         * "vi:set opt opt opt: foo" -- foo not interpreted
         * "vi:opt opt opt: foo" -- foo interpreted
         * Accept "se" for compatibility with Elvis.
         */
        if strncmp(s as *mut i8, S!("set "), 4) == 0 || strncmp(s as *mut i8, S!("se "), 3) == 0 {
            if *e != ':' as u8 {
                // no terminating ':'?
                break;
            } else {
                end = true;
                s = vim_strchr(s, ' ' as i32).offset(1)
            }
        }
        *e = NUL as u8; // truncate the set command

        if *s != 0 {
            // skip over an empty "::"
            let secure_save = secure;
            let save_current_sctx = current_sctx;
            current_sctx.sc_sid = SID_MODELINE;
            current_sctx.sc_seq = 0;
            current_sctx.sc_lnum = 0;
            // Make sure no risky things are executed as a side effect.
            secure = 1;

            retval = do_set(s, OPT_MODELINE | OPT_LOCAL | flags);

            secure = secure_save;
            current_sctx = save_current_sctx;
            if retval == FAIL {
                break; // stop if error found
            }
        }
        s = e.offset(1); // advance to next part
    }

    sourcing_lnum = save_sourcing_lnum;
    sourcing_name = save_sourcing_name;

    xfree(linecopy);

    return retval;
}

// Return true if "buf" is a help buffer.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn bt_help(buf: *const buf_T) -> bool {
    buf.as_ref().map_or(false, |buf| buf.b_help)
}

// Return true if "buf" is a normal buffer, 'buftype' is empty.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn bt_normal(buf: *const buf_T) -> bool {
    buf.as_ref().map_or(false, |buf| *buf.b_p_bt.offset(0) == 0)
}

// Return true if "buf" is the quickfix buffer.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn bt_quickfix(buf: *const buf_T) -> bool {
    buf.as_ref()
        .map_or(false, |buf| *buf.b_p_bt.offset(0) == 'q' as u8)
}

// Return true if "buf" is a terminal buffer.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn bt_terminal(buf: *const buf_T) -> bool {
    buf.as_ref()
        .map_or(false, |buf| *(*buf).b_p_bt.offset(0) == 't' as u8)
}

// Return true if "buf" is a "nofile", "acwrite" or "terminal" buffer.
// This means the buffer name is not a file name.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn bt_nofile(buf: *const buf_T) -> bool {
    buf.as_ref().map_or(false, |buf| {
        *buf.b_p_bt.offset(0) == 'n' as u8 && *buf.b_p_bt.offset(2) == 'f' as u8
            || *buf.b_p_bt.offset(0) == 'a' as u8
            || !buf.terminal.is_null()
            || *buf.b_p_bt.offset(0) == 'p' as u8
    })
}

// Return true if "buf" is a "nowrite", "nofile" or "terminal" buffer.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn bt_dontwrite(buf: *const buf_T) -> bool {
    buf.as_ref().map_or(false, |buf| {
        *buf.b_p_bt.offset(0) == 'n' as u8
            || !buf.terminal.is_null()
            || *buf.b_p_bt.offset(0) == 'p' as u8
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn bt_dontwrite_msg(buf: *const buf_T) -> bool {
    if bt_dontwrite(buf) {
        emsg(gettext("E382: Cannot write, \'buftype\' option is set"));
        return true;
    }
    return false;
}

// Return true if the buffer should be hidden, according to 'hidden', ":hide"
// and 'bufhidden'.
#[no_mangle]
pub unsafe extern "C" fn buf_hide(buf: *const buf_T) -> bool {
    // 'bufhidden' overrules 'hidden' and ":hide", check it first
    match *(*buf).b_p_bh.offset(0) as char {
        'u' | // "unload"
        'w' | // "wipe"
        'd' => false, // "delete"
        'h' => true, // "hide"
        _ => p_hid != 0 || cmdmod.hide,
    }
}

/*
 * Return special buffer name.
 * Returns NULL when the buffer has a normal file name.
 */
#[no_mangle]
pub unsafe extern "C" fn buf_spname(buf: *mut buf_T) -> *const u8 {
    let buf = buf.as_mut().unwrap();
    if bt_quickfix(buf) {
        let mut win: *mut win_T = ptr::null_mut();
        let mut tp: *mut tabpage_T = ptr::null_mut();

        /*
         * For location list window, w_llist_ref points to the location list.
         * For quickfix window, w_llist_ref is NULL.
         */
        if find_win_for_buf(buf, &mut win, &mut tp) && !(*win).w_llist_ref.is_null() {
            return c_gettext(msg_loclist) as *mut u8;
        } else {
            return c_gettext(msg_qflist) as *mut u8;
        }
    }
    // There is no _file_ when 'buftype' is "nofile", b_sfname
    // contains the name as specified by the user.
    if bt_nofile(buf) {
        if !buf.b_fname.is_null() {
            return buf.b_fname;
        }
        if bt_prompt(buf) != 0 {
            return gettext("[Prompt]");
        }
        return gettext("[Scratch]");
    }
    if buf.b_fname.is_null() {
        return gettext("[No Name]");
    }
    return ptr::null_mut();
}

/// Find a window for buffer "buf".
/// If found true is returned and "wp" and "tp" are set to
/// the window and tabpage.
/// If not found, false is returned.
///
/// @param       buf  buffer to find a window for
/// @param[out]  wp   stores the found window
/// @param[out]  tp   stores the found tabpage
///
/// @return true if a window was found for the buffer.
#[no_mangle]
pub unsafe extern "C" fn find_win_for_buf(
    buf: *mut buf_T,
    wp: *mut *mut win_T,
    tp: *mut *mut tabpage_T,
) -> bool {
    *wp = ptr::null_mut();
    *tp = ptr::null_mut();
    FOR_ALL_TAB_WINDOWS!(tp2, wp2, {
        if (*wp2).w_buffer == buf {
            *tp = tp2;
            *wp = wp2;
            return true;
        }
    });
    return false;
}

#[no_mangle]
pub unsafe extern "C" fn buf_signcols(buf: *mut buf_T) -> i32 {
    let buf = buf.as_mut().unwrap();

    if buf.b_signcols_max == -1 {
        let mut sign: *mut signlist_T; // a sign in the signlist
        buf.b_signcols_max = 0;
        let mut linesum = 0;
        let mut curline = 0;

        FOR_ALL_SIGNS_IN_BUF!(buf, sign, {
            if (*sign).lnum > curline {
                if linesum > buf.b_signcols_max {
                    buf.b_signcols_max = linesum;
                }
                curline = (*sign).lnum;
                linesum = 0;
            }
            linesum += 1;
        });
        if linesum > buf.b_signcols_max {
            buf.b_signcols_max = linesum;
        }

        // Check if we need to redraw
        if buf.b_signcols_max != buf.b_signcols {
            buf.b_signcols = buf.b_signcols_max;
            redraw_buf_later(buf, NOT_VALID);
        }
    }

    return buf.b_signcols;
}

/*
 * Set 'buflisted' for curbuf to "on" and trigger autocommands if it changed.
 */
#[no_mangle]
pub unsafe extern "C" fn set_buflisted(on: i32) {
    if on != (*curbuf).b_p_bl {
        (*curbuf).b_p_bl = on;
        if on != 0 {
            apply_autocmds(
                EVENT_BUFADD,
                ptr::null_mut(),
                ptr::null_mut(),
                false,
                curbuf,
            );
        } else {
            apply_autocmds(
                EVENT_BUFDELETE,
                ptr::null_mut(),
                ptr::null_mut(),
                false,
                curbuf,
            );
        }
    }
}

/// Read the file for "buf" again and check if the contents changed.
/// Return true if it changed or this could not be checked.
///
/// @param  buf  buffer to check
///
/// @return true if the buffer's contents have changed
#[no_mangle]
pub unsafe extern "C" fn buf_contents_changed(buf: *mut buf_T) -> bool {
    let buf = buf.as_mut().unwrap();
    let mut differ = true;

    // Allocate a buffer without putting it in the buffer list.
    let newbuf = buflist_new(ptr::null_mut(), ptr::null_mut(), 1, BLN_DUMMY);
    if newbuf.is_null() {
        return true;
    }

    // Force the 'fileencoding' and 'fileformat' to be equal.
    let mut ea = exarg_T::default();
    prep_exarg(&mut ea, buf);

    // set curwin/curbuf to buf and save a few things
    let mut aco = aco_save_T::default();
    aucmd_prepbuf(&mut aco, newbuf);

    if ml_open(curbuf) == OK
        && readfile(
            buf.b_ffname,
            buf.b_fname,
            0,
            0,
            MAXLNUM,
            &mut ea,
            READ_NEW | READ_DUMMY,
        ) == OK
    {
        // compare the two files line by line
        if buf.b_ml.ml_line_count == (*curbuf).b_ml.ml_line_count {
            differ = false;
            for lnum in 1..=(*curbuf).b_ml.ml_line_count {
                if strcmp(
                    ml_get_buf(buf, lnum, 0 != 0) as *mut i8,
                    ml_get(lnum) as *mut i8,
                ) != 0
                {
                    differ = true;
                    break;
                }
            }
        }
    }
    xfree(ea.cmd);

    // restore curwin/curbuf and a few other things
    aucmd_restbuf(&mut aco);

    if curbuf != newbuf {
        wipe_buffer(newbuf, false_0); // safety check
    }

    return differ;
}

/*
 * Wipe out a buffer and decrement the last buffer number if it was used for
 * this buffer.  Call this to wipe out a temp buffer that does not contain any
 * marks.
 */
#[no_mangle]
pub unsafe extern "C" fn wipe_buffer(
    buf: *mut buf_T,
    aucmd: i32, // When true trigger autocommands.
) {
    if aucmd == 0 {
        // Don't trigger BufDelete autocommands here.
        block_autocmds();
    }
    close_buffer(ptr::null_mut(), buf, DOBUF_WIPE, false);
    if aucmd == 0 {
        unblock_autocmds();
    }
}

/// Creates or switches to a scratch buffer. :h special-buffers
/// Scratch buffer is:
///   - buftype=nofile bufhidden=hide noswapfile
///   - Always considered 'nomodified'
///
/// @param bufnr     Buffer to switch to, or 0 to create a new buffer.
///
/// @see curbufIsChanged()
#[no_mangle]
pub unsafe extern "C" fn buf_open_scratch(bufnr: handle_T, bufname: *mut i8) {
    do_ecmd(
        bufnr,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        ECMD_ONE,
        ECMD_HIDE,
        ptr::null_mut(),
    );
    setfname(curbuf, bufname as *mut u8, ptr::null_mut(), true);
    set_option_value(S!("bh"), 0, S!("hide"), OPT_LOCAL);
    set_option_value(S!("bt"), 0, S!("nofile"), OPT_LOCAL);
    set_option_value(S!("swf"), 0, ptr::null_mut(), OPT_LOCAL);
    RESET_BINDING(curwin);
}
