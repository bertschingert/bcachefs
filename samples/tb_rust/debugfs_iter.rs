
#![allow(missing_docs)]

use kernel::pr_info;
use kernel::c_str;

use kernel::prelude::*;

use super::bindings;

use crate::debugfs::*;

pub struct DebugfsIter<T: IntoIterator>
where T: IntoIterator, <T as IntoIterator>::Item: core::fmt::Display
{
    raw: *mut bindings::dentry,
    _type: core::marker::PhantomData<T>,
}

impl<'a, T> DebugfsIter<T>
where T: IntoIterator, <T as IntoIterator>::Item: core::fmt::Display
{
    const ITER_FILE_OPS: bindings::file_operations = bindings::file_operations {
        owner: core::ptr::null_mut(),
        llseek: None,
        read: Some(Self::debugfs_read_iter),
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
        open: Some(Self::debugfs_open_iter),
        flush: None,
        release: Some(Self::debugfs_release_iter),
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

    extern "C" fn debugfs_open_iter(inode: *mut bindings::inode,
                               file: *mut bindings::file) -> core::ffi::c_int
    {
        let inode = unsafe { &*inode };
        let file = unsafe { &mut *file };

        let collection = inode.i_private as *mut &mut T;
        let collection = unsafe { &*collection };

        let mut my_iter = collection.into_iter();


        let state = match Box::try_new(DebugfsIterState {
            buf: None,
            my_iter: &mut my_iter,
        }) {
            Ok(s) => s,
            Err(_) => return -14,
        };

        file.private_data = Box::into_raw(state) as *mut core::ffi::c_void;

        0
    }

    extern "C" fn debugfs_release_iter(_inode: *mut bindings::inode,
                                  _file: *mut bindings::file) -> core::ffi::c_int
    {
        0
    }

    extern "C" fn debugfs_read_iter(file: *mut bindings::file,
                               _uptr: *mut core::ffi::c_char,
                               _len: usize,
                               _off: *mut bindings::loff_t) -> isize
    {
        pr_info!("debugfs_read iter\n");
        let _file = unsafe { &*file };

        0
    }

    pub fn new(parent: &DebugfsDentry,
               item: &'a T) -> Self
    {
        let item = Box::try_new(item).unwrap();
        let item = Box::into_raw(item);

        let name = c_str!("tbrs_file");
        let name = name.as_bytes_with_nul().as_ptr() as *const i8;
        let debugfs_file = unsafe {
            bindings::debugfs_create_file(name,
                                          0755,
                                          parent.raw,
                                          item as *mut core::ffi::c_void,
                                          &Self::ITER_FILE_OPS,
                                          )
        };

        DebugfsIter {
            raw: debugfs_file,
            _type: Default::default(),
        }
    }
}

struct DebugfsIterState<'a, U>
where U: Iterator, <U as Iterator>::Item: core::fmt::Display
{
    buf: Option<Box<kernel::str::CString>>,
    my_iter: &'a mut U,
}
