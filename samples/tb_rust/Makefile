# SPDX-License-Identifier: GPL-2.0

obj-$(CONFIG_SAMPLE_RUST_TB)		+= tb_rust.o

tb_rust-y	+= tb_debugfs_entry.o \
		tb_rust_main.o

always-$(CONFIG_SAMPLE_RUST_TB)		+= bindings/bindings_generated.rs

$(obj)/tb_rust_main.o: $(src)/bindings/bindings_generated.rs

$(obj)/bindings/bindings_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^#|^$$' $(srctree)/$(src)/bindgen_parameters)

$(obj)/bindings/bindings_generated.rs: $(src)/bindings/bindings_helper.h  \
	$(src)/bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
