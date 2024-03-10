// SPDX-License-Identifier: GPL-2.0

#![allow(missing_docs)]

use core::cmp::Ordering;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use kernel::prelude::*;

use crate::bindings;
use crate::bkey::BkeySC;
use crate::errcode::errptr_to_result_c;
use crate::fs::Fs;
use bindings::bch_errcode;

use bindings::bpos as Bpos;

pub const fn spos(inode: u64, offset: u64, snapshot: u32) -> Bpos {
    Bpos {
        inode,
        offset,
        snapshot,
    }
}

pub const fn pos(inode: u64, offset: u64) -> Bpos {
    spos(inode, offset, 0)
}

pub const POS_MIN: Bpos = spos(0, 0, 0);
pub const POS_MAX: Bpos = spos(u64::MAX, u64::MAX, 0);
pub const SPOS_MAX: Bpos = spos(u64::MAX, u64::MAX, u32::MAX);

impl PartialEq for Bpos {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Bpos {}

impl PartialOrd for Bpos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bpos {
    fn cmp(&self, other: &Self) -> Ordering {
        let l_inode = self.inode;
        let r_inode = other.inode;
        let l_offset = self.offset;
        let r_offset = other.offset;
        let l_snapshot = self.snapshot;
        let r_snapshot = other.snapshot;

        l_inode
            .cmp(&r_inode)
            .then(l_offset.cmp(&r_offset))
            .then(l_snapshot.cmp(&r_snapshot))
    }
}

pub struct BtreeTrans<'f> {
    raw: *mut bindings::btree_trans,
    fs: PhantomData<&'f Fs>,
}

impl<'f> BtreeTrans<'f> {
    pub fn new(fs: &'f Fs) -> BtreeTrans<'_> {
        unsafe {
            BtreeTrans {
                raw: &mut *bindings::__bch2_trans_get(fs.raw, 0),
                fs: PhantomData,
            }
        }
    }

    pub fn unlock(&self) {
        unsafe { bindings::bch2_trans_unlock(self.raw) };
    }
}

impl<'f> Drop for BtreeTrans<'f> {
    fn drop(&mut self) {
        unsafe { bindings::bch2_trans_put(&mut *self.raw) }
    }
}

pub struct BtreeIter<'t> {
    raw: bindings::btree_iter,
    trans: PhantomData<&'t BtreeTrans<'t>>,
}

impl<'t> BtreeIter<'t> {
    pub fn new(
        trans: &'t BtreeTrans<'t>,
        btree: bindings::btree_id,
        pos: bindings::bpos,
        flags: u16,
    ) -> BtreeIter<'t> {
        unsafe {
            let mut iter: MaybeUninit<bindings::btree_iter> = MaybeUninit::uninit();

            bindings::bch2_trans_iter_init_outlined(
                trans.raw,
                iter.as_mut_ptr(),
                btree,
                pos,
                flags,
            );

            BtreeIter {
                raw: iter.assume_init(),
                trans: PhantomData,
            }
        }
    }

    pub fn peek_upto<'i>(
        &'i mut self,
        end: bindings::bpos,
    ) -> Result<Option<BkeySC<'_>>, bch_errcode> {
        unsafe {
            let k = bindings::bch2_btree_iter_peek_upto(&mut self.raw, end);
            errptr_to_result_c(k.k).map(|_| {
                if !k.k.is_null() {
                    Some(BkeySC {
                        k: &*k.k,
                        v: &*k.v,
                        iter: PhantomData,
                    })
                } else {
                    None
                }
            })
        }
    }

    pub fn peek(&mut self) -> Result<Option<BkeySC<'_>>, bch_errcode> {
        self.peek_upto(SPOS_MAX)
    }

    pub fn peek_and_restart(&mut self) -> Result<Option<BkeySC<'_>>, bch_errcode> {
        unsafe {
            let k = bindings::bch2_btree_iter_peek_and_restart_outlined(&mut self.raw);

            errptr_to_result_c(k.k).map(|_| {
                if !k.k.is_null() {
                    Some(BkeySC {
                        k: &*k.k,
                        v: &*k.v,
                        iter: PhantomData,
                    })
                } else {
                    None
                }
            })
        }
    }

    pub fn advance(&mut self) {
        unsafe {
            bindings::bch2_btree_iter_advance(&mut self.raw);
        }
    }

    pub fn cur_pos(&self) -> Bpos {
        self.raw.pos
    }
}

impl<'t> Drop for BtreeIter<'t> {
    fn drop(&mut self) {
        unsafe { bindings::bch2_trans_iter_exit(self.raw.trans, &mut self.raw) }
    }
}

pub struct BtreeNodeIter<'t> {
    _raw: bindings::btree_iter,
    trans: PhantomData<&'t BtreeTrans<'t>>,
}
