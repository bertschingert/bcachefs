/*
 * tb-test sandbox kernel module
 */

#include <linux/module.h>
#include <linux/printk.h>
#include <linux/debugfs.h>


static struct dentry *d;

static void tb_test_exit(void)
{
	pr_info("tb_test exit");
	debugfs_remove(d);
}

static int __init tb_test_init(void)
{
	pr_info("tb_test entry\n");

	d = debugfs_create_dir("tb_test", NULL);

	return 0;
}

MODULE_LICENSE("GPL");

module_exit(tb_test_exit);
module_init(tb_test_init);
