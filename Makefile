
clean:
	rm -f init

init: 
	gcc -static init.c -o init

initramfs:
	chmod +x init-bin
	#cp source/samples/rust/rust_minimal.ko .
	#ls init-bin rust_minimal.ko |cpio -o --format=newc > ramdisk.img                                                  