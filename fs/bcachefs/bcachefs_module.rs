// SPDX-License-Identifier: GPL-2.0

//! bcachefs
//!
//! Rust code for bcachefs.

#![allow(non_camel_case_types)]

pub mod bindings;
pub mod bkey;
pub mod btree;
pub mod btree_debugfs;
pub mod errcode;
pub mod fs;
pub mod printbuf;

const __LOG_PREFIX: &[u8] = b"bcachefs\0";
