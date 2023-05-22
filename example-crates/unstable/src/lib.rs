#![no_std]
#![feature(core_intrinsics)]

extern crate no_std_compat2 as std;

use std::intrinsics::abort;

pub fn assert(cond: bool) {
    if !cond {
        unsafe {
            abort();
        }
    }
}
