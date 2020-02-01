extern "C" {
    pub fn os_proc_tree_kill(pid: libc::c_int, sig: libc::c_int) -> bool;
}
