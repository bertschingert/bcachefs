
use crate::uaccess;
use crate::pr_info;
use crate::c_str;
use bindings;

#[derive(Debug)]
pub struct DebugfsDentry {
    raw: *mut bindings::dentry,
    // should this get a list of children dentries created below it?
    // for making it so you can't drop a parent before the child
}

// SAFETY: TODO
unsafe impl Sync for DebugfsDentry {}

// TODO:
// - should this return a result?
// - take an Option<&DebugfsDentry> as an arg
pub fn debugfs_create_dir() -> Option<DebugfsDentry> {
    let name = c_str!("rust_test_dir");
    let name = name.as_bytes_with_nul().as_ptr() as *const i8;

    // Safety: TODO
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

extern "C" fn debugfs_read(
    f: *mut bindings::file,
    uptr: *mut core::ffi::c_char,
    len: usize,
    off: *mut bindings::loff_t,
    ) -> isize {
    let off = unsafe { &mut *off };

    if *off > 0 {
        return 0;
    }

    let mut writer = uaccess::UserSlice::new(uptr as *mut core::ffi::c_void, len).writer();

    writer.write_slice("hello from Rust :)\n".as_bytes());

    *off += 1;

    pr_info!("in debugfs read Rust handler :)\n");

    return 19;
}

const DEBUGFS_FILE_OPERATIONS: bindings::file_operations = bindings::file_operations {
    owner: core::ptr::null_mut(),
    llseek: None,
    read: Some(debugfs_read),
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
    open: None,
    flush: None,
    release: None,
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

pub fn debugfs_create_file(parent: &DebugfsDentry) -> Option<DebugfsDentry> {
    let name = c_str!("rust_test_file");
    let name = name.as_bytes_with_nul().as_ptr() as *const i8;

    let debugfs_file = unsafe {
        bindings::debugfs_create_file(name,
                                      0755,
                                      parent.raw,
                                      core::ptr::null_mut::<core::ffi::c_void>(),
                                      &DEBUGFS_FILE_OPERATIONS,
                                      )
    };

    if debugfs_file.is_null() {
        pr_info!("failed to create debugfs file\n");
        None
    } else {
        Some(DebugfsDentry {
            raw: debugfs_file,
        })
    }
}


impl DebugfsDentry {
    pub fn debugfs_remove(&self) {
        unsafe { bindings::debugfs_remove(self.raw); }
    }
}

/* not doing drop for now...
impl Drop for DebugfsDentry {
    fn drop(&mut self) {
        // Safety: TODO
        unsafe { bindings::debugfs_remove(self.raw); }
    }
}
*/
