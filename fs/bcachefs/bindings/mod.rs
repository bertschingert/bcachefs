// SPDX-License-Identifier: GPL-2.0

#![allow(
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    improper_ctypes,
    unsafe_op_in_unsafe_fn
)]

include!("bindings_special.rs");
include!("bindings_generated.rs");

impl From<bch_errcode> for kernel::error::Error {
    fn from(e: bch_errcode) -> Self {
        unsafe { kernel::error::Error::from_errno(__bch2_err_class(e as core::ffi::c_int)) }
    }
}
