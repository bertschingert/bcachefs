/* SPDX-License-Identifier: GPL-2.0 */
#ifndef _BCACHEFS_RUST_H
#define _BCACHEFS_RUST_H

#ifdef CONFIG_BCACHEFS_RUST
void bch_debugfs_setup_btree(void *parent_dentry,
				void **btree_debug_array,
				const char *name,
				struct btree_debug *bd);
void bch_debugfs_cleanup(void *btree_debug_pointers);
#endif

#endif /* _BCACHEFS_RUST_H */
