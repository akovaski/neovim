extern crate libc;
use libc::c_int;
use std::num::FpCategory;

extern "C" {
    static rust_ffi_fp_nan: c_int;
    static rust_ffi_fp_infinite: c_int;
    static rust_ffi_fp_zero: c_int;
    static rust_ffi_fp_subnormal: c_int;
    static rust_ffi_fp_normal: c_int;
}

#[no_mangle]
extern "C" fn xisnan(d: f64) -> c_int {
    match d.is_nan() {
        true => 1,
        false => 0,
    }
}
#[no_mangle]
extern "C" fn xisinf(d: f64) -> c_int {
    match d.is_infinite() {
        true => 1,
        false => 0,
    }
}
#[no_mangle]
extern "C" fn xfpclassify(d: f64) -> c_int {
    unsafe {
        match d.classify() {
            FpCategory::Nan => rust_ffi_fp_nan,
            FpCategory::Infinite => rust_ffi_fp_infinite,
            FpCategory::Zero => rust_ffi_fp_zero,
            FpCategory::Subnormal => rust_ffi_fp_subnormal,
            FpCategory::Normal => rust_ffi_fp_normal,
        }
    }
}
