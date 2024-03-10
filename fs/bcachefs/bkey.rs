// SPDX-License-Identifier: GPL-2.0

#![allow(missing_docs, non_camel_case_types)]

use crate::bindings;
use crate::btree::BtreeIter;
use crate::fs::Fs;
use core::ffi::CStr;
use core::fmt;
use core::marker::PhantomData;

#[repr(C)]
pub struct BkeySC<'a> {
    pub k: &'a bindings::bkey,
    pub v: &'a bindings::bch_val,
    pub(crate) iter: PhantomData<&'a mut BtreeIter<'a>>,
}

impl<'a, 'b> BkeySC<'a> {
    pub unsafe fn to_raw(&self) -> bindings::bkey_s_c {
        bindings::bkey_s_c {
            k: self.k,
            v: self.v,
        }
    }

    pub fn to_text(&'a self, fs: &'b Fs) -> BkeySCToText<'a, 'b> {
        BkeySCToText { k: self, fs }
    }
}

pub struct BkeySCToText<'a, 'b> {
    k: &'a BkeySC<'a>,
    fs: &'b Fs,
}

impl<'a, 'b> fmt::Display for BkeySCToText<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            printbuf_to_formatter(f, |buf| {
                bindings::bch2_bkey_val_to_text(buf, self.fs.raw, self.k.to_raw())
            })
        }
    }
}

pub fn printbuf_to_formatter<F>(f: &mut fmt::Formatter<'_>, func: F) -> fmt::Result
where
    F: Fn(*mut bindings::printbuf),
{
    let mut buf = bindings::printbuf::new();

    func(&mut buf);

    let s = unsafe { CStr::from_ptr(buf.buf) };
    // f.write_str(&s.to_string_lossy())
    f.write_str(&s.to_str().unwrap_or("printbuf_to_formatter: error\n"))
}

impl bindings::printbuf {
    fn new() -> bindings::printbuf {
        let mut buf: bindings::printbuf = Default::default();

        buf.set_heap_allocated(true);
        buf
    }
}
