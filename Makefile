MODULE_NAME=rust_sample_module.ko

# Cleanup
clean:
	rm -f init *.ko ramdisk.img

# Prepare for emulation
prepare: init initramfs

# Build init binary
init: 
	gcc -static init.c -o init

# Connect gdb to emulator
gdb:
	gdb -x gdb.config

# Build kernel initramfs
initramfs:
	chmod +x init
	cp source/samples/rust/rust_minimal.ko ./${MODULE_NAME}
	ls init ${MODULE_NAME} |cpio -o --format=newc > ramdisk.img                                                  