// SPDX-License-Identifier: GPL-2.0

//! Rust APIs for creating files and directories in the debugfs filesystem.

use kernel::str::*;
use kernel::uaccess;

use kernel::prelude::*;

use super::bindings;
use super::reader::*;

/// Type representing a file's mode bits.
pub type Mode = core::ffi::c_ushort;

/// Type representing a dentry in debugfs
pub struct Dentry {
    raw: *mut bindings::dentry,
}

/// Create a debugfs directory
///
/// If `parent` is None, then it is created in the debugfs root, otherwise under the specified
/// parent.
///
/// The C debugfs_create_dir() documentation states:
/// > it's expected that most callers should _ignore_ the errors returned by this function.
///
/// Therefore, if debugfs_dir is an error pointer, this is ignored, and callers should not try
/// to look at the Dentry value to determine if it's an error or not.
pub fn create_dir(name: &CStr, parent: Option<Dentry>) -> Dentry {
    let name = name.as_bytes_with_nul().as_ptr() as *const i8;

    let parent = match parent {
        Some(d) => d.raw,
        None => core::ptr::null_mut::<bindings::dentry>(),
    };

    // SAFETY: ??
    let debugfs_dir = unsafe { bindings::debugfs_create_dir(name, parent) };

    Dentry { raw: debugfs_dir }
}

impl Dentry {
    /// Construct a [`Dentry`] given a `struct dentry *` that was returned by a previous C
    /// call to `debugfs_create_dir()`.
    pub fn from_raw(raw: *mut core::ffi::c_void) -> Dentry {
        Dentry {
            raw: raw as *mut bindings::dentry,
        }
    }

    /// Remove a debugfs directory, and all of its children, from debugfs.
    ///
    /// This is intentionally not implemented using the Drop trait because the lifetimes of
    /// debugfs files are ended when their common parent is destroyed using debugfs_remove().
    ///
    /// By not putting debugfs_remove() in the Drop implementation, callers can drop
    /// `Dentry`s of children when convenient and only must retain the `Dentry` for
    /// the root, rather than being forced to retain a reference to every `Dentry`
    /// they create.
    pub fn remove(&self) {
        // SAFETY: ??
        unsafe {
            bindings::debugfs_remove(self.raw);
        }
    }
}

/// A type representing a debugfs file that responds to read() request by reading from the backing
/// BufRead type.
pub struct DebugfsReader<T> {
    _type: core::marker::PhantomData<T>,
}

struct DebugfsReaderState<U> {
    bufreader: U,
    // Reading can be done because there is no more data (then done contains 0), or an error
    // was encountered (then done contains the error code).
    done: Option<isize>,
}

impl<'a, T: IntoBufReader + Sync + Clone> DebugfsReader<T> {
    const READER_FILE_OPS: bindings::file_operations = bindings::file_operations {
        owner: core::ptr::null_mut(),
        llseek: None,
        read: Some(Self::read),
        write: None,
        read_iter: None,
        write_iter: None,
        iopoll: None,
        iterate_shared: None,
        poll: None,
        unlocked_ioctl: None,
        compat_ioctl: None,
        mmap: None,
        mmap_supported_flags: 0,
        open: Some(Self::open),
        flush: None,
        release: Some(Self::release),
        fsync: None,
        fasync: None,
        lock: None,
        get_unmapped_area: None,
        check_flags: None,
        flock: None,
        splice_write: None,
        splice_read: None,
        splice_eof: None,
        setlease: None,
        fallocate: None,
        show_fdinfo: None,
        copy_file_range: None,
        remap_file_range: None,
        fadvise: None,
        uring_cmd: None,
        uring_cmd_iopoll: None,
    };

    extern "C" fn open(inode: *mut bindings::inode, file: *mut bindings::file) -> core::ffi::c_int {
        // SAFETY: inode is guaranteed by the C API to be a valid pointer.
        let inode = unsafe { &*inode };
        // SAFETY: file is guaranteed by the C API to be a valid pointer.
        let file = unsafe { &mut *file };

        let object = inode.i_private as *const T;
        // SAFETY: it is valid to dereference the i_private pointer because of the safety
        // requirements of DebugfsReader::create_file().
        let object = unsafe { &*object };

        let state = match Box::try_new(DebugfsReaderState {
            bufreader: object.clone().into_bufreader(),
            done: None,
        }) {
            Ok(s) => s,
            Err(_) => return ENOMEM.to_errno(),
        };

        file.private_data = Box::into_raw(state) as *mut core::ffi::c_void;

        0
    }

    extern "C" fn release(
        _inode: *mut bindings::inode,
        file: *mut bindings::file,
    ) -> core::ffi::c_int {
        // SAFETY: file is guaranteed by the C API to be a valid pointer.
        let file = unsafe { &mut *file };

        // SAFETY: we set up the private_data in open(), and nothing could
        // have dropped it yet, so we know it is still a valid pointer.
        let _ = unsafe { Box::from_raw(file.private_data as *mut DebugfsReaderState<T>) };

        0
    }

    extern "C" fn read(
        file: *mut bindings::file,
        uptr: *mut core::ffi::c_char,
        len: usize,
        _off: *mut bindings::loff_t,
    ) -> isize {
        // SAFETY: file is guaranteed by the C API to be a valid pointer.
        let file = unsafe { &mut *file };
        let state = file.private_data as *mut DebugfsReaderState<<T as IntoBufReader>::BufReader>;
        // SAFETY: we set up the private_data in open(), and nothing could
        // have dropped it yet, so we know it is still a valid pointer.
        let state = unsafe { &mut *state };
        let bufreader = &mut state.bufreader;
        let mut writer = uaccess::UserSlice::new(uptr as *mut core::ffi::c_void, len).writer();

        match state.done {
            Some(ret) => return ret,
            None => {},
        };

        let mut written: usize = 0;

        while written < len {
            let data = match bufreader.fill_buf() {
                Ok(s) => s,
                Err(e) => return e.to_errno().try_into().unwrap(),
            };

            if data.len() == 0 {
                state.done = Some(0);
                break;
            }

            let n = core::cmp::min(data.len(), len - written);

            match writer.write_slice(&data[0..n]) {
                Ok(_) => {}
                Err(e) => {
                    if written > 0 {
                        state.done = Some(e.to_errno().try_into().unwrap());
                        break;
                    } else {
                        return e.to_errno().try_into().unwrap();
                    }
                }
            };

            bufreader.consume(n);
            written += n;
        }

        written.try_into().unwrap()
    }

    /// Create a debugfs file under the given parent (or the debugfs root if parent is None), with
    /// the given name and mode. `object` is a pointer to the type that will be used in the open()
    /// handler to run the type's `into_bufreader()` method.
    ///
    /// # SAFETY
    ///
    /// The caller must guarantee that `object`'s lifetime exceeds the lifetime of this debugfs file.
    /// The simplest way to do this is to ensure that debugfs_remove() is called, and completes,
    /// for a parent directory of this file before `object` is dropped.
    pub unsafe fn create_file(
        name: &CStr,
        mode: Mode,
        parent: Option<Dentry>,
        object: *mut T,
    ) -> Dentry {
        let name = name.as_bytes_with_nul().as_ptr() as *const i8;

        let parent = match parent {
            Some(d) => d.raw,
            None => core::ptr::null_mut::<bindings::dentry>(),
        };

        Dentry {
            // SAFETY: it is safe to create a debugfs file.
            raw: unsafe {
                bindings::debugfs_create_file(
                    name,
                    mode,
                    parent,
                    object as *mut core::ffi::c_void,
                    &Self::READER_FILE_OPS,
                )
            },
        }
    }
}
