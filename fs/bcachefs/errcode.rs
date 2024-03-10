// SPDX-License-Identifier: GPL-2.0

#![allow(missing_docs)]

use crate::bindings::bch_errcode;

pub fn errptr_to_result_c<T>(p: *const T) -> Result<*const T, bch_errcode> {
    let addr = p as usize;
    let max_err: isize = -4096;
    if addr > max_err as usize {
        let addr = addr as i32;
        let err: bch_errcode = unsafe { core::mem::transmute(-addr) };
        Err(err)
    } else {
        Ok(p)
    }
}
