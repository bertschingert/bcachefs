// SPDX-License-Identifier: GPL-2.0

//! Rust playground testing module

#![allow(missing_docs)]

use kernel::c_str;
use kernel::debugfs;
use kernel::prelude::*;

pub mod my_collection;

module! {
    type: RustPlayground,
    name: "rust_playground",
    author: "Thomas Bertschinger",
    description: "Rust playground testing module",
    license: "GPL",
}

struct RustPlayground {
    debug_dir: debugfs::Dentry,
    data: MyWrapper,
}

struct MyWrapper {
    inner: *mut my_collection::MyObject,
}

unsafe impl Sync for MyWrapper {}

impl kernel::Module for RustPlayground {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust playground testing module: entry\n");

        let my_obj = my_collection::MyObject { data: 3 };

        let my_obj = Box::try_new(my_obj)?;

        let my_obj = Box::into_raw(my_obj);

        let debug_dir = debugfs::create_dir(c_str!("tbrs"), None);
        unsafe {
            let _debug_reader = debugfs::DebugfsReader::create_file(
                c_str!("tbrs_reader"),
                0400,
                Some(debug_dir),
                my_obj,
            );
        }

        Ok(RustPlayground {
            debug_dir,
            data: MyWrapper { inner: my_obj },
        })
    }
}

impl Drop for RustPlayground {
    fn drop(&mut self) {
        self.debug_dir.remove();
        let _ = unsafe { Box::from_raw(self.data.inner) };
        pr_info!("Rust playground testing module: exit\n");
    }
}
