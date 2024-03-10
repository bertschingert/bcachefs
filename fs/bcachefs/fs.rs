// SPDX-License-Identifier: GPL-2.0

#![allow(missing_docs)]

use crate::bindings;

#[derive(Clone)]
pub struct Fs {
    raw: *mut bindings::bch_fs,
}

impl Fs {
    /// # SAFETY
    ///
    /// This holds a raw pointer to a struct bch_fs representing a mounted bcachefs filesystem, but
    /// this reference is not accounted on the C side in any way. The caller who creates this Fs
    /// object must ensure that the filesystem's lifetime is greater than the lifetime of this Fs
    /// object. Once we have proper reference counting of struct bch_fs between C and Rust, we can
    /// remove this safety requirement.
    pub unsafe fn new(raw: *mut bindings::bch_fs) -> Fs {
        Fs { raw }
    }

    pub fn raw(&self) -> *mut bindings::bch_fs {
        self.raw
    }
}
