SOURCE_MODULE=rust_minimal.ko

MODULE_NAME=rust_sample_module.ko

.PHONY: clean

# Prepare for emulation
prepare: clean kernel_module init initramfs

# Build kernel
build_kernel:
	cp linux/.config linux/.config-bak
	cp kernel-config linux/.config
	make LLVM=1 -C linux -j 8

# Cleanup
clean:
	rm -f init ${MODULE_NAME} ramdisk.img

# Build init binary
init: 
	gcc -static init.c -o init

# Connect gdb to emulator
gdb:
	gdb -x gdb.config

# Build kernel initramfs
initramfs:
	chmod +x init
	cp linux/samples/rust/${SOURCE_MODULE} ./${MODULE_NAME}
	ls init ${MODULE_NAME} |cpio -o --format=newc > ramdisk.img

# Build the kernel_module
kernel_module:
	make -C linux LLVM=1 samples/rust/${SOURCE_MODULE}

# Run the kernel
run_kernel: prepare
	echo "Starting kernel, remember to run 'make gdb'!"
	qemu-system-x86_64 \
		-kernel linux/arch/x86/boot/bzImage \
		-initrd ramdisk.img \
		-append "console=ttyS0 init=/init-bin root=/dev/hda1 nokaslr" \
		-serial stdio \
		-display none \
		-m 128 \
		-enable-kvm \
		-no-reboot \
		-cpu host \
		-monitor telnet:127.0.0.1:55555,server,nowait \
		-s -S
