// SPDX-License-Identifier: GPL-2.0

//! Rust playground testing module

#![allow(missing_docs)]

use kernel::prelude::*;
// use kernel::debugfs;

pub mod bindings;

pub mod tb_debugfs;
pub mod debugfs;
pub mod debugfs_iter;

module! {
    type: RustPlayground,
    name: "rust_playground",
    author: "Thomas Bertschinger",
    description: "Rust playground testing module",
    license: "GPL",
}

struct RustPlayground {
    debug_dir: debugfs::DebugfsDentry,
}

impl kernel::Module for RustPlayground {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust playground testing module: entry\n");

        unsafe { bindings::tb_register_debugfs_files() };

        let debug_dir = debugfs::debugfs_create_dir().unwrap();
        // let debug_file = debugfs::debugfs_create_file(&debug_dir).unwrap();
        let _debug_display = debugfs::DebugfsDisplayItem::new(&debug_dir,
                                                              &"hello display!");

        Ok(RustPlayground {
            debug_dir: debug_dir,
        })
    }
}

impl Drop for RustPlayground {
    fn drop(&mut self) {
        unsafe { bindings::tb_cleanup_debugfs_files() };
        self.debug_dir.debugfs_remove();
        pr_info!("Rust playground testing module: exit\n");
    }
}
