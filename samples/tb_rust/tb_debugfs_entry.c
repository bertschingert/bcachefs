#include <linux/debugfs.h>

#include "tb_rust.h"

struct dentry *tb_rust_debugfs_root;

static int tb_open(struct inode *inode, struct file *file) {
	file->private_data = tb_rs_open();

	return 0;
}

static int tb_release(struct inode *inode, struct file *file) {
	tb_rs_release(file->private_data);

	return 0;
}

static ssize_t tb_read(struct file *f, char __user *buf, size_t n, loff_t *off) {
	return tb_rs_read(f->private_data, buf, n, off);
}

const struct file_operations tb_fops = {
	.open = tb_open,
	.release = tb_release,
	.read = tb_read,
};

int tb_register_debugfs_files(void) {
	struct dentry *tb_rust_file;

	tb_rust_debugfs_root = debugfs_create_dir("tb_rust", NULL);

	tb_rust_file = debugfs_create_file("tb_rust_test", 0777,
                                   tb_rust_debugfs_root, NULL, &tb_fops);

	return 0;
}

void tb_cleanup_debugfs_files(void) {
	debugfs_remove_recursive(tb_rust_debugfs_root);
}
