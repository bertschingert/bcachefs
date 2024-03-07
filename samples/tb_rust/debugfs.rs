#![allow(missing_docs)]

use kernel::uaccess;
use kernel::pr_info;
use kernel::c_str;
use kernel::str::CString;

use kernel::prelude::*;

use super::bindings;

pub struct DebugfsDentry {
    pub raw: *mut bindings::dentry,
}

unsafe impl Sync for DebugfsDentry {}

pub fn debugfs_create_dir() -> Option<DebugfsDentry> {
    let name = c_str!("tbrs");
    let name = name.as_bytes_with_nul().as_ptr() as *const i8;

    let debugfs_dir = unsafe {
        bindings::debugfs_create_dir(name, core::ptr::null_mut::<bindings::dentry>())
    };

    if debugfs_dir.is_null() {
        None
    } else {
        Some(DebugfsDentry {
            raw: debugfs_dir,
        })
    }
}

impl DebugfsDentry {
    pub fn debugfs_remove(&self) {
        unsafe { bindings::debugfs_remove(self.raw); }
    }
}

pub struct DebugfsDisplayItem<T: core::fmt::Display> {
    raw: *mut bindings::dentry,
    _type: core::marker::PhantomData<T>,
}

impl<'a, T> DebugfsDisplayItem<T>
where T: core::fmt::Display
{
    const DISPLAY_FILE_OPS: bindings::file_operations = bindings::file_operations {
        owner: core::ptr::null_mut(),
        llseek: None,
        read: Some(Self::debugfs_read),
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
        open: Some(Self::debugfs_open),
        flush: None,
        release: Some(Self::debugfs_release),
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

    extern "C" fn debugfs_open(inode: *mut bindings::inode,
                               file: *mut bindings::file) -> core::ffi::c_int
    {
        let inode = unsafe { &*inode };
        let file = unsafe { &mut *file };

        let item = inode.i_private as *mut &T;

        let state = match Box::try_new(DebugfsState {
            buf: None,
            item,
        }) {
            Ok(s) => s,
            Err(_) => return -14,
        };

        file.private_data = Box::into_raw(state) as *mut core::ffi::c_void;

        0
    }

    extern "C" fn debugfs_release(_inode: *mut bindings::inode,
                                  _file: *mut bindings::file) -> core::ffi::c_int
    {
        0
    }

    extern "C" fn debugfs_read(file: *mut bindings::file,
                               uptr: *mut core::ffi::c_char,
                               len: usize,
                               off: *mut bindings::loff_t) -> isize
    {
        pr_info!("debugfs_read\n");
        let file = unsafe { &*file };
        let state = file.private_data;
        let mut state = unsafe { Box::from_raw(state as *mut DebugfsState<'_, T>) };
        pr_info!("debugfs_read: item: {:?}", state.item);

        if state.buf.is_some() && state.buf.as_ref().unwrap().len() == 0 {
            return 0;
        }

        let item = unsafe { &*state.item };

        if state.buf.is_none() {
            let data = match CString::try_from_fmt(kernel::fmt!("tb:debugfs:read: {}\n", item)) {
                Ok(d) => d,
                Err(_) => return -14,
            };

            let data = match Box::try_new(data) {
                Ok(d) => d,
                Err(_) => return -14,
            };

            state.buf = Some(data);
        }

        let mut writer = uaccess::UserSlice::new(uptr as *mut core::ffi::c_void, len).writer();

        let written = state.flush_buf(len, &mut writer);

        if written > 0 {
            unsafe { *off += written as i64 };
        }

        Box::into_raw(state);

        written
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
                                          &Self::DISPLAY_FILE_OPS,
                                          )
        };

        DebugfsDisplayItem {
            raw: debugfs_file,
            _type: Default::default(),
        }
    }
}

struct DebugfsState<'a, T>
where T: core::fmt::Display
{
    buf: Option<Box<kernel::str::CString>>,
    item: *mut &'a T,
}

impl<T> core::fmt::Display for DebugfsState<'_, T>
where T: core::fmt::Display
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "buf: {:?}", self.buf)
    }
}

impl<T> DebugfsState<'_, T>
where T: core::fmt::Display
{
    fn flush_buf(&mut self, len: usize, writer: &mut uaccess::UserSliceWriter) -> isize {
        if let Some(buf) = &self.buf {
            let n = core::cmp::min(len, buf.len());

            if n > 0 {
                match writer.write_slice(&buf[0..n]) {
                    Ok(_) => {},
                    Err(_) => return -14,
                };

                let remainder = match CString::try_from(&buf[n..]) {
                    Ok(s) => s,
                    Err(_) => return -14,
                };

                let new_buf = match Box::try_new(remainder) {
                    Ok(b) => Some(b),
                    Err(_) => return -14,
                };

                self.buf = new_buf;

                let n: isize = n.try_into().unwrap();
                return n;
            }
        }

        0
    }
}
