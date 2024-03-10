// SPDX-License-Identifier: GPL-2.0

#![allow(missing_docs)]

use crate::bindings;

#[derive(Clone)]
pub struct Fs {
    pub raw: *mut bindings::bch_fs,
}
