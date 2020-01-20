use crate::grid_defs::schar_T;
use crate::memory::*;
use std::ptr;

pub type sattr_T = i16;

const CELLBYTES: usize = std::mem::size_of::<schar_T>();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UCell {
    pub data: [libc::c_char; CELLBYTES + 1],
    pub attr: sattr_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UGrid {
    pub row: libc::c_int,
    pub col: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub cells: *mut *mut UCell,
}

#[no_mangle]
pub unsafe extern "C" fn ugrid_init(mut grid: *mut UGrid) {
    (*grid).cells = ptr::null_mut();
}
#[no_mangle]
pub unsafe extern "C" fn ugrid_free(grid: *mut UGrid) {
    destroy_cells(grid);
}
#[no_mangle]
pub unsafe extern "C" fn ugrid_resize(
    mut grid: *mut UGrid,
    width: libc::c_int,
    height: libc::c_int,
) {
    destroy_cells(grid);
    (*grid).cells =
        xmalloc((height as libc::size_t).wrapping_mul(std::mem::size_of::<*mut UCell>()))
            as *mut *mut UCell;
    for i in 0..height {
        let ref mut fresh0 = *(*grid).cells.offset(i as isize);
        *fresh0 = xcalloc(width as libc::size_t, std::mem::size_of::<UCell>()) as *mut UCell;
    }
    (*grid).width = width;
    (*grid).height = height;
}
#[no_mangle]
pub unsafe extern "C" fn ugrid_clear(grid: *mut UGrid) {
    clear_region(
        grid,
        0,
        (*grid).height - 1,
        0,
        (*grid).width - 1,
        0 as sattr_T,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ugrid_clear_chunk(
    grid: *mut UGrid,
    row: libc::c_int,
    col: libc::c_int,
    endcol: libc::c_int,
    attr: sattr_T,
) {
    clear_region(grid, row, row, col, endcol - 1, attr);
}
#[no_mangle]
pub unsafe extern "C" fn ugrid_goto(mut grid: *mut UGrid, row: libc::c_int, col: libc::c_int) {
    (*grid).row = row;
    (*grid).col = col;
}
#[no_mangle]
pub unsafe extern "C" fn ugrid_scroll(
    grid: *mut UGrid,
    top: libc::c_int,
    bot: libc::c_int,
    left: libc::c_int,
    right: libc::c_int,
    count: libc::c_int,
) {
    // Compute start/stop/step for the loop below
    let start: libc::c_int;
    let stop: libc::c_int;
    let step: libc::c_int;
    if count > 0 {
        start = top;
        stop = bot - count + 1;
        step = 1;
    } else {
        start = bot;
        stop = top - count - 1;
        step = -1;
    }

    let mut i: libc::c_int;

    // Copy cell data
    i = start;
    while i != stop {
        let target_row: *mut UCell = (*(*grid).cells.offset(i as isize)).offset(left as isize);
        let source_row: *mut UCell =
            (*(*grid).cells.offset((i + count) as isize)).offset(left as isize);
        c_assert!(right >= left && left >= 0);
        memcpy(
            target_row,
            source_row,
            std::mem::size_of::<UCell>().wrapping_mul(
                (right as libc::size_t)
                    .wrapping_sub(left as libc::size_t)
                    .wrapping_add(1),
            ),
        );
        i += step
    }
}
unsafe extern "C" fn clear_region(
    grid: *mut UGrid,
    top: libc::c_int,
    bot: libc::c_int,
    left: libc::c_int,
    right: libc::c_int,
    attr: sattr_T,
) {
    for row in top..=bot {
        let row_cells: *mut UCell = *(*grid).cells.offset(row as isize);
        for curcol in left..=right {
            let mut cell: *mut UCell = row_cells.offset(curcol as isize);
            (*cell).data[0] = ' ' as libc::c_char;
            (*cell).data[1] = 0;
            (*cell).attr = attr;
        }
    }
}
unsafe extern "C" fn destroy_cells(grid: *mut UGrid) {
    if !(*grid).cells.is_null() {
        for i in 0..(*grid).height {
            xfree(*(*grid).cells.offset(i as isize) as *mut libc::c_void);
        }
        XFREE_CLEAR(&mut (*grid).cells);
    };
}
