/*
 * tb-test sandbox kernel module
 */

#include <linux/module.h>
#include <linux/printk.h>
#include <linux/debugfs.h>
#include <linux/xarray.h>
#include <linux/mutex.h>


static struct {
	struct dentry *tb_debug_dir;
	int id;
} tb;

DEFINE_XARRAY_ALLOC(tb_xa);

char *my_data = "some data";

static ssize_t tb_xa_read(struct file *file, char __user *buf,
				size_t size, loff_t *ppos)
{
	pr_info("tb_xa_read\n");
	unsigned long id = 0;
	char *entry;
	xa_for_each(&tb_xa, id, entry) {
		pr_info("got id %lu = %s\n", id, entry);
	}
	return 0;
}

static ssize_t tb_xa_add(struct file *file, const char __user *buf,
				size_t size, loff_t *ppos)
{
	int id;
	int ret;

	pr_info("tb_xa_add\n");

	ret = xa_alloc(&tb_xa, &id, my_data, xa_limit_32b, 0);
	if (ret) {
		pr_info("xa_alloc failed: %d\n", ret);
		return size;
	}

	xa_set_mark(&tb_xa, id, XA_MARK_0);

	pr_info("added id %d\n", id);

	return size;
}

static ssize_t tb_xa_remove(struct file *file, const char __user *buf,
				size_t size, loff_t *ppos)
{
	pr_info("tb_xa_remove\n");
	unsigned long id = 0;
	char *entry;

	entry = xa_find(&tb_xa, &id, ULONG_MAX, XA_MARK_0);
	if (entry) {
		entry = xa_erase(&tb_xa, id);

		if (entry) {
			pr_info("removed id %d, data: %s\n", tb.id, entry);
			tb.id++;
		}
	}

	return size;
}

static const struct file_operations tb_xa_add_ops = {
	.owner		= THIS_MODULE,
	.write		= tb_xa_add,
};

static const struct file_operations tb_xa_remove_ops = {
	.owner		= THIS_MODULE,
	.write		= tb_xa_remove,
};

static const struct file_operations tb_xa_read_ops = {
	.owner		= THIS_MODULE,
	.read		= tb_xa_read,
};

static void tb_test_exit(void)
{
	pr_info("tb_test exit");
	debugfs_remove(tb.tb_debug_dir);
}

static int __init tb_test_init(void)
{
	pr_info("tb_test entry\n");

	tb.tb_debug_dir = debugfs_create_dir("tb_test", NULL);
	debugfs_create_file("xa_add", 0644, tb.tb_debug_dir, NULL, &tb_xa_add_ops);
	debugfs_create_file("xa_remove", 0644, tb.tb_debug_dir, NULL, &tb_xa_remove_ops);
	debugfs_create_file("xa_read", 0644, tb.tb_debug_dir, NULL, &tb_xa_read_ops);

	return 0;
}

MODULE_LICENSE("GPL");

module_exit(tb_test_exit);
module_init(tb_test_init);
