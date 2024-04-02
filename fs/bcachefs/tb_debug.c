/* my debug testing code */

#include "debug.h"

#include <linux/debugfs.h>

struct tb_state {
	struct bch_fs *c;
	void *data;
	size_t pos;
	size_t len;
};

static int tb_open(struct inode *inode, struct file *file)
{
	pr_info("in tb_open");

	struct bch_fs *c = inode->i_private;
	struct tb_state *s = kzalloc(sizeof(struct tb_state), GFP_KERNEL);

	if (!s)
		return -ENOMEM;

	s->c = c;
	file->private_data = s;

	return 0;
}

static int tb_release(struct inode *inode, struct file *file)
{
	pr_info("in tb_release");

	kfree(file->private_data);

	return 0;
}

static ssize_t tb_read(struct file *f, char __user *buf, size_t size,
			loff_t *ppos)
{
	pr_info("in tb_read");

	struct tb_state *s = f->private_data;

	size_t amt = s->len - s->pos;
	size_t bytes = min_t(size_t, size, amt);

	unsigned long n = copy_to_user(buf, s->data, bytes);
	if (n < 0)
		return -EFAULT;

	s->pos += bytes;

	return bytes;
}

static ssize_t tb_write(struct file *f, const char __user *buf,
			size_t size, loff_t *ppos)
{
	pr_info("in tb_write");

	if (size > 4096) {
		return -ERANGE;
	}

	void *data = kzalloc(size, GFP_KERNEL);

	if (!data)
		return -ENOMEM;

	unsigned long n = copy_from_user(data, buf, size);
	if (n < 0)
		return -EFAULT;

	struct tb_state *s = f->private_data;
	s->data = data;
	s->len = size;
	s->pos = 0;

	return size;
}

static const struct file_operations test_ops = {
	.owner		= THIS_MODULE,
	.open		= tb_open,
	.release	= tb_release,
	.read		= tb_read,
	.write		= tb_write,
};

void tb_debug_init(struct bch_fs *c, struct dentry *d)
{
	pr_info("tb_debug_init\n");

	struct dentry *tb = debugfs_create_dir("tb_debug", d);

	debugfs_create_file("test", 0400, tb, c, &test_ops);
}
