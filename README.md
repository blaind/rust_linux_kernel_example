# Rust - Linux kernel module example

This example shows how to build a Rust Linux kernel module, and debug it using gdb.

## Prequisites

    apt-get install make gcc cpio

## Running

Do-it-all-command, which

- builds the module
- builds init-binary
- creates initramfs
- launches qemu

Run it:

    make run_kernel

It should print a few lines, and be in idle mode.

Attach gdb to the emulator:

    make gdb

Now, you should see the kernel booting up and emulator eventually breaking in the rust module init

Emulator (run_kernel) should print:

    ...
    [    1.210080] Run /init as init process
    init - entered main()
    init - loading kernel module (iteration=0)
    init - module file fd=3

gdb should print:

    Hardware assisted breakpoint 1 at 0xffffffffa000013f: file samples/rust/rust_minimal.rs, line 36.

    Breakpoint 1, rust_minimal::{impl#1}::init (name=0xffffffffa0003000, module=0xffffffffa00035a8) at samples/rust/rust_minimal.rs:36
    36	samples/rust/rust_minimal.rs: No such file or directory.
    (gdb)
