use crate::*;

extern "C" {
    pub type qf_info_T;
}

pub fn GETFILE_SUCCESS(x: i32) -> bool {
    x <= 0
}
pub unsafe fn MODIFIABLE(buf: *const buf_T) -> bool {
    (*buf).b_p_ma != 0
}

pub type disptick_T = u16;

#[derive(Copy, Clone)]
#[repr(C)]
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
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum WinStyle {
    kWinStyleUnused = 0,
    kWinStyleMinimal, // Minimal UI: no number column, eob markers, etc
}
pub use WinStyle::*;
#[allow(dead_code)]
#[derive(Copy, Clone)]
#[repr(C)]
pub enum FloatRelative {
    kFloatRelativeEditor = 0,
    kFloatRelativeWindow = 1,
    kFloatRelativeCursor = 2,
}
pub use FloatRelative::*;
pub type FloatAnchor = libc::c_int;
pub type Window = handle_T;
pub type taggy_T = taggy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct taggy {
    pub tagname: *mut libc::c_uchar,
    pub fmark: fmark_T,
    pub cur_match: libc::c_int,
    pub cur_fnum: libc::c_int,
    pub user_data: *mut libc::c_uchar,
}

pub const MAX_MAPHASH: usize = 256;

pub type buf_T = file_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_buffer {
    pub handle: handle_T,
    pub b_ml: memline_T,
    pub b_next: *mut buf_T,
    pub b_prev: *mut buf_T,
    pub b_nwindows: libc::c_int,
    pub b_flags: libc::c_int,
    pub b_locked: libc::c_int,
    pub b_ffname: *mut u8,
    pub b_sfname: *mut u8,
    pub b_fname: *mut u8,
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
    pub b_orig_size: u64,
    pub b_orig_mode: libc::c_int,
    pub b_namedm: [fmark_T; NMARKS],
    pub b_visual: visualinfo_T,
    pub b_visual_mode_eval: libc::c_int,
    pub b_last_cursor: fmark_T,
    pub b_last_insert: fmark_T,
    pub b_last_change: fmark_T,
    pub b_changelist: [fmark_T; JUMPLISTSIZE],
    pub b_changelistlen: libc::c_int,
    pub b_new_change: bool,
    pub b_chartab: [u64; 4],
    pub b_maphash: [*mut mapblock_T; MAX_MAPHASH],
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
    pub b_u_line_ptr: *mut u8,
    pub b_u_line_lnum: linenr_T,
    pub b_u_line_colnr: colnr_T,
    pub b_scanned: bool,
    pub b_p_iminsert: libc::c_long,
    pub b_p_imsearch: libc::c_long,
    pub b_kmap_state: libc::c_short,
    pub b_kmap_ga: garray_T,
    pub b_p_initialized: bool,
    pub b_p_script_ctx: [LastSet; BV::COUNT as usize],
    pub b_p_ai: libc::c_int,
    pub b_p_ai_nopaste: libc::c_int,
    pub b_p_bkc: *mut u8,
    pub b_bkc_flags: libc::c_uint,
    pub b_p_ci: libc::c_int,
    pub b_p_bin: libc::c_int,
    pub b_p_bomb: libc::c_int,
    pub b_p_bh: *mut u8,
    pub b_p_bt: *mut u8,
    pub b_has_qf_entry: libc::c_int,
    pub b_p_bl: libc::c_int,
    pub b_p_channel: libc::c_long,
    pub b_p_cin: libc::c_int,
    pub b_p_cino: *mut u8,
    pub b_p_cink: *mut u8,
    pub b_p_cinw: *mut u8,
    pub b_p_com: *mut u8,
    pub b_p_cms: *mut u8,
    pub b_p_cpt: *mut u8,
    pub b_p_cfu: *mut u8,
    pub b_p_ofu: *mut u8,
    pub b_p_tfu: *mut u8,
    pub b_p_eol: libc::c_int,
    pub b_p_fixeol: libc::c_int,
    pub b_p_et: libc::c_int,
    pub b_p_et_nobin: libc::c_int,
    pub b_p_et_nopaste: libc::c_int,
    pub b_p_fenc: *mut u8,
    pub b_p_ff: *mut u8,
    pub b_p_ft: *mut u8,
    pub b_p_fo: *mut u8,
    pub b_p_flp: *mut u8,
    pub b_p_inf: libc::c_int,
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
    pub b_p_lisp: libc::c_int,
    pub b_p_menc: *mut u8,
    pub b_p_mps: *mut u8,
    pub b_p_ml: libc::c_int,
    pub b_p_ml_nobin: libc::c_int,
    pub b_p_ma: libc::c_int,
    pub b_p_nf: *mut u8,
    pub b_p_pi: libc::c_int,
    pub b_p_qe: *mut u8,
    pub b_p_ro: libc::c_int,
    pub b_p_sw: libc::c_long,
    pub b_p_scbk: libc::c_long,
    pub b_p_si: libc::c_int,
    pub b_p_sts: libc::c_long,
    pub b_p_sts_nopaste: libc::c_long,
    pub b_p_sua: *mut u8,
    pub b_p_swf: libc::c_int,
    pub b_p_smc: libc::c_long,
    pub b_p_syn: *mut u8,
    pub b_p_ts: libc::c_long,
    pub b_p_tw: libc::c_long,
    pub b_p_tw_nobin: libc::c_long,
    pub b_p_tw_nopaste: libc::c_long,
    pub b_p_wm: libc::c_long,
    pub b_p_wm_nobin: libc::c_long,
    pub b_p_wm_nopaste: libc::c_long,
    pub b_p_keymap: *mut u8,
    pub b_p_gp: *mut u8,
    pub b_p_mp: *mut u8,
    pub b_p_efm: *mut u8,
    pub b_p_ep: *mut u8,
    pub b_p_path: *mut u8,
    pub b_p_ar: libc::c_int,
    pub b_p_tags: *mut u8,
    pub b_p_tc: *mut u8,
    pub b_tc_flags: libc::c_uint,
    pub b_p_dict: *mut u8,
    pub b_p_tsr: *mut u8,
    pub b_p_ul: libc::c_long,
    pub b_p_udf: libc::c_int,
    pub b_p_lw: *mut u8,
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
    pub b_start_fenc: *mut u8,
    pub b_bad_char: libc::c_int,
    pub b_start_bomb: libc::c_int,
    pub b_bufvar: ScopeDictDictItem,
    pub b_vars: *mut dict_T,
    pub b_may_swap: bool,
    pub b_did_warn: bool,
    pub b_help: bool,
    pub b_spell: bool,
    pub b_prompt_text: *mut u8,
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
    pub update_channels: kvec_t<u64>,
    pub update_callbacks: kvec_t<BufUpdateCallbacks>,
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
pub struct file_buffer_update_callbacks {
    pub size: size_t,
    pub capacity: size_t,
    pub items: *mut BufUpdateCallbacks,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
    pub w_p_lcs_chars: window_w_p_lcs_chars,
    pub w_p_fcs_chars: window_w_p_fcs_chars,
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
    pub w_localdir: *mut u8,
    pub w_onebuf_opt: winopt_T,
    pub w_allbuf_opt: winopt_T,
    pub w_p_stl_flags: u32,
    pub w_p_fde_flags: u32,
    pub w_p_fdt_flags: u32,
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
pub type win_T = window_S;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_w_p_fcs_chars {
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
pub struct window_w_p_lcs_chars {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matchitem_T {
    pub next: *mut matchitem_T,
    pub id: libc::c_int,
    pub priority: libc::c_int,
    pub pattern: *mut libc::c_uchar,
    pub hlg_id: libc::c_int,
    pub match_0: regmmatch_T,
    pub pos: posmatch_T,
    pub hl: match_T,
    pub conceal_char: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub type posmatch_T = posmatch;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posmatch {
    pub pos: [llpos_T; 8],
    pub cur: libc::c_int,
    pub toplnum: linenr_T,
    pub botlnum: linenr_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct llpos_T {
    pub lnum: linenr_T,
    pub col: colnr_T,
    pub len: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winopt_T {
    pub wo_arab: libc::c_int,
    pub wo_bri: libc::c_int,
    pub wo_briopt: *mut u8,
    pub wo_diff: libc::c_int,
    pub wo_fdc: *mut u8,
    pub wo_fdc_save: *mut u8,
    pub wo_fen: libc::c_int,
    pub wo_fen_save: libc::c_int,
    pub wo_fdi: *mut u8,
    pub wo_fdl: libc::c_long,
    pub wo_fdl_save: libc::c_int,
    pub wo_fdm: *mut u8,
    pub wo_fdm_save: *mut u8,
    pub wo_fml: libc::c_long,
    pub wo_fdn: libc::c_long,
    pub wo_fde: *mut u8,
    pub wo_fdt: *mut u8,
    pub wo_fmr: *mut u8,
    pub wo_lbr: libc::c_int,
    pub wo_list: libc::c_int,
    pub wo_nu: libc::c_int,
    pub wo_rnu: libc::c_int,
    pub wo_nuw: libc::c_long,
    pub wo_wfh: libc::c_int,
    pub wo_wfw: libc::c_int,
    pub wo_pvw: libc::c_int,
    pub wo_rl: libc::c_int,
    pub wo_rlc: *mut u8,
    pub wo_scr: libc::c_long,
    pub wo_spell: libc::c_int,
    pub wo_cuc: libc::c_int,
    pub wo_cul: libc::c_int,
    pub wo_cc: *mut u8,
    pub wo_stl: *mut u8,
    pub wo_scb: libc::c_int,
    pub wo_diff_saved: libc::c_int,
    pub wo_scb_save: libc::c_int,
    pub wo_wrap: libc::c_int,
    pub wo_wrap_save: libc::c_int,
    pub wo_cocu: *mut u8,
    pub wo_cole: libc::c_long,
    pub wo_crb: libc::c_int,
    pub wo_crb_save: libc::c_int,
    pub wo_scl: *mut u8,
    pub wo_winhl: *mut u8,
    pub wo_fcs: *mut u8,
    pub wo_lcs: *mut u8,
    pub wo_winbl: libc::c_long,
    pub wo_script_ctx: [LastSet; 42],
}
pub type alist_T = arglist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arglist {
    pub al_ga: garray_T,
    pub al_refcount: libc::c_int,
    pub id: libc::c_int,
}
pub type wline_T = w_line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct w_line {
    pub wl_lnum: linenr_T,
    pub wl_size: u16,
    pub wl_valid: libc::c_char,
    pub wl_folded: libc::c_char,
    pub wl_lastlnum: linenr_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pos_save_T {
    pub w_topline_save: libc::c_int,
    pub w_topline_corr: libc::c_int,
    pub w_cursor_save: pos_T,
    pub w_cursor_corr: pos_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_T {
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
#[derive(Copy, Clone)]
#[repr(C)]
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
    pub b_syn_sync_id: i16,
    pub b_syn_sync_minlines: libc::c_long,
    pub b_syn_sync_maxlines: libc::c_long,
    pub b_syn_sync_linebreaks: libc::c_long,
    pub b_syn_linecont_pat: *mut libc::c_uchar,
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
    pub b_spell_ismw_mb: *mut libc::c_uchar,
    pub b_p_spc: *mut libc::c_uchar,
    pub b_cap_prog: *mut regprog_T,
    pub b_p_spf: *mut libc::c_uchar,
    pub b_p_spl: *mut libc::c_uchar,
    pub b_cjk: libc::c_int,
    pub b_syn_chartab: [libc::c_uchar; 32],
    pub b_syn_isk: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapblock_T {
    pub m_next: *mut mapblock_T,
    pub m_keys: *mut libc::c_uchar,
    pub m_str: *mut libc::c_uchar,
    pub m_orig_str: *mut libc::c_uchar,
    pub m_keylen: libc::c_int,
    pub m_mode: libc::c_int,
    pub m_noremap: libc::c_int,
    pub m_silent: libc::c_char,
    pub m_nowait: libc::c_char,
    pub m_expr: libc::c_char,
    pub m_script_ctx: sctx_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wininfo_T {
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
    pub di_key: [libc::c_uchar; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufUpdateCallbacks {
    pub on_lines: LuaRef,
    pub on_changedtick: LuaRef,
    pub on_detach: LuaRef,
    pub utf_sizes: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct syn_time_T {
    pub total: proftime_T,
    pub slowest: proftime_T,
    pub count: libc::c_long,
    pub match_0: libc::c_long,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufref_T {
    pub br_buf: *mut buf_T,
    pub br_fnum: libc::c_int,
    pub br_buf_free_count: libc::c_int,
}
impl Default for bufref_T {
    fn default() -> Self {
        bufref_T {
            br_buf: 0 as *mut buf_T,
            br_fnum: 0,
            br_buf_free_count: 0,
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
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
    pub tp_localdir: *mut u8,
}
pub type tabpage_T = tabpage_S;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct diffblock_S {
    pub df_next: *mut diff_T,
    pub df_lnum: [linenr_T; 8],
    pub df_count: [linenr_T; 8],
}
pub type diff_T = diffblock_S;
pub const BF_NEVERLOADED: libc::c_int = 0x4 as libc::c_int;
pub const BF_CHECK_RO: libc::c_int = 0x2 as libc::c_int;
pub const BF_WRITE_MASK: libc::c_int = BF_NOTEDITED + BF_NEW + BF_READERR;
pub const BF_NOTEDITED: libc::c_int = 0x8 as libc::c_int;
pub const BF_NEW: libc::c_int = 0x10 as libc::c_int;
pub const VALID_TOPLINE: libc::c_int = 0x80 as libc::c_int;
pub const BF_READERR: libc::c_int = 0x40 as libc::c_int;
pub const KEYMAP_INIT: i16 = 1;
pub const BF_DUMMY: libc::c_int = 0x80 as libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stl_hlrec {
    pub start: *mut u8,
    pub userhl: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argentry {
    pub ae_fname: *mut u8,
    pub ae_fnum: libc::c_int,
}
pub type aentry_T = argentry;

pub unsafe fn ALIST(win: *const win_T) -> *mut alist_T {
    (*win).w_alist
}
pub unsafe fn AARGLIST(al: &mut alist_T) -> *mut aentry_T {
    al.al_ga.ga_data as *mut aentry_T
}
pub unsafe fn ARGCOUNT() -> i32 {
    (*ALIST(curwin)).al_ga.ga_len
}
