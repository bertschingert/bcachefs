#include <linux/debugfs.h>

/* C functions */

int tb_register_debugfs_files(void);
void tb_cleanup_debugfs_files(void);

/* Rust functions */

void *tb_rs_open(void);
void tb_rs_release(void *);
ssize_t tb_rs_read(void *, char *, size_t, loff_t *);
