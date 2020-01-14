use super::typval;

// Head of list of all dictionaries
#[no_mangle]
pub static mut gc_first_dict: *mut typval::dict_T = 0 as *mut typval::dict_T;
// Head of list of all lists
#[no_mangle]
pub static mut gc_first_list: *mut typval::list_T = 0 as *mut typval::list_T;
