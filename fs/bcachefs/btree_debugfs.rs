// SPDX-License-Identifier: GPL-2.0

//! bcachefs debugfs

use kernel::container_of;
use kernel::debugfs;
use kernel::prelude::*;
use kernel::reader;

use crate::bindings;
use crate::btree;
use crate::fs;
use crate::printbuf::Printbuf;

/// `BtreeDebug` contains a pointer into the `btree_debug` array embedded in `struct bch_fs`. This
/// is used both to get the identity of the btree being examined, as well as a reference to `bch_fs`
/// using `container_of!`.
#[derive(Copy, Clone)]
struct BtreeDebug {
    bd: *const bindings::btree_debug,
}

// SAFETY: All access to to the btrees referred to by this type is safe for concurrent access
// by the C cdoe implementing the access, to `BtreeDebug` can be `sync`.
unsafe impl Sync for BtreeDebug {}

const N_BTREES: usize = bindings::btree_id::BTREE_ID_NR as usize;

type DebugArray = [BtreeDebug; N_BTREES];

/// Create a debugfs file for the btree `bd`.
#[no_mangle]
pub extern "C" fn bch_debugfs_setup_btree(
    parent: *mut core::ffi::c_void,
    debug_array: *mut *mut core::ffi::c_void,
    name: *const core::ffi::c_char,
    bd: *const bindings::btree_debug,
) {
    // SAFETY: This is a pointer to a field of `struct bch_fs` which we know is valid because
    // `bch_fs` must have been succesfully allocated if we reach this stage of setup.
    let debug_array = unsafe { &mut (*debug_array as *mut DebugArray) };

    if (*debug_array).is_null() {
        let a = match Box::try_new(
            [BtreeDebug {
                bd: core::ptr::null(),
            }; N_BTREES],
        ) {
            Ok(a) => a,
            // If there is a failure to set up debug structures, this is ignored so that the
            // filesystem can go on mounting. This matches existing debug init C code.
            Err(_) => return,
        };
        *debug_array = Box::into_raw(a);
    }

    // SAFETY: Name is static.
    let name = unsafe { CStr::from_char_ptr(name) };
    let parent = debugfs::Dentry::from_raw(parent);
    let btree_debug = BtreeDebug { bd };

    // SAFETY: The first dereference of debug_array is for the Rust reference, which is safe.
    // The second dereference is for the btree_debug_array field of struct bch_fs. This is valid
    // because we allocated it previously in this function.
    // bd is a valid pointer into bch_fs.
    let debug_entry = unsafe { &mut (**debug_array)[(*bd).id as usize] };

    (*debug_entry) = btree_debug;

    // SAFETY: btree_debug is guaranteed to live at least as long as the created debugfs file
    // because we do not drop btree_debug until after debugfs_remove() has completed for the parent
    // debugfs directory.
    unsafe {
        debugfs::DebugfsReader::create_file(name, 0400, Some(parent), debug_entry);
    }
}

/// Clean up the heap-allocated debugfs data.
#[no_mangle]
pub extern "C" fn bch_debugfs_cleanup(debug_array: *mut core::ffi::c_void) {
    if !debug_array.is_null() {
        // SAFETY: if allocating debug_array failed, then it would be null, so since it is not we
        // know it was originally succesfully allocated via a Box.
        let _ = unsafe { Box::from_raw(debug_array as *mut DebugArray) };
    }
}

impl reader::IntoBufReader for BtreeDebug {
    type BufReader = BchBtreeReader;

    fn into_bufreader(self) -> Self::BufReader {
        let fs: *const bindings::bch_fs = unsafe {
            // core::mem::offset_of! (user by container_of!) doesn't work for elements of inner
            // arrays, so we have to compute the base of the btree_debug array first. :(
            //
            // SAFETY: self.bd is a valid pointer into struct bch_fs, which must point to valid
            // memory because the FS being mounted still is a precondition for being able to open
            // the debugfs files.
            let btree_debug: *const bindings::btree_debug =
                self.bd.offset(-1 * (*self.bd).id as isize);

            // SAFETY: the btree_debug pointer is within the allocation of the struct bch_fs
            container_of!(btree_debug, bindings::bch_fs, btree_debug)
        };

        BchBtreeReader {
            fs: fs as *mut bindings::bch_fs,
            btree: self.clone(),
            buf: Printbuf::new(),
            buf_pos: 0,
            btree_pos: btree::POS_MIN,
        }
    }
}

struct BchBtreeReader {
    fs: *mut bindings::bch_fs,
    btree: BtreeDebug,
    buf: Printbuf,
    buf_pos: core::ffi::c_uint,
    btree_pos: bindings::bpos,
}

impl reader::BufRead for BchBtreeReader {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        if self.buf_pos >= self.buf.len() {
            // SAFETY: we know that self.fs is valid because the debugfs file would have
            // been destroyed already (meaning we couldn't get here) if the FS was unmounted.
            let fs = unsafe { fs::Fs::new(self.fs) };
            let trans = btree::BtreeTrans::new(&fs);
            // SAFETY: self.btree.bd points to a member of self.fs and is a valid pointer for
            // the same reason that one is.
            let btree_id = unsafe { (*self.btree.bd).id };
            let mut iter = btree::BtreeIter::new(
                &trans,
                btree_id,
                self.btree_pos,
                bindings::BTREE_ITER_PREFETCH | bindings::BTREE_ITER_ALL_SNAPSHOTS,
            );

            self.buf_pos = 0;
            self.buf.reset();

            let k = iter.peek_and_restart()?;

            let k = match k {
                Some(k) => k,
                None => {
                    return Ok(&[]);
                }
            };

            k.write_to_printbuf(&fs, &mut self.buf);
            self.buf.newline();

            iter.advance();
            self.btree_pos = iter.cur_pos();
        }

        Ok(&self.buf.as_slice()[(self.buf_pos as usize)..])
    }

    fn consume(&mut self, amt: usize) {
        self.buf_pos += amt as core::ffi::c_uint;
    }
}
