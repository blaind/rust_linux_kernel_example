# Rust - Linux kernel module example

This example shows how to build a Rust Linux kernel module, and debug it using gdb.

This should enable relatively rapid iteration when developing (e.g. edit module file, run `make debug_kernel`, attach gdb with `make gdb`, repeat)

## Prequisites

Build dependencies:

    sudo apt-get install make gcc cpio

Fetch Rust-For-Linux:

    git submodule update --init

Rust-For-Linux prequisites, see [quick-start.rst](https://github.com/Rust-for-Linux/linux/blob/d9b2e84c0700782f26c9558a3eaacbe1f78c01e8/Documentation/rust/quick-start.rst). In brief (note! this overrides rustup config and installs specific version of bindgen)

    cd linux
    rustup override set $(scripts/min-tool-version.sh rustc)
    rustup component add rust-src
    cargo install --locked --version $(scripts/min-tool-version.sh bindgen) bindgen
    make LLVM=1 rustavailable
    # should print: "Rust is available!"

## Building

Build the kernel module. Source located at `sample_module` path.

This uses `kernel-config` as a baseline config

    make build_kernel

This will take a while

## Running without gdb

Do-it-all-command, which

- builds the module (`rust_minimal.rs`)
- builds init-binary
- creates initramfs
- launches qemu

(see Makefile for details)

Run it:

    make run_kernel

This should open up a busybox shell, with the module already loaded.

## Running with gdb

### 1. Init, build module & Start qemu

Do-it-all-command, which

- builds the module (`rust_minimal.rs`)
- builds init-binary
- creates initramfs
- launches qemu

(see Makefile for details)

Run it:

    make debug_kernel

It should print a few lines, and be in idle mode. Leave it running

### 2. Attach gdb to the qemu instance

Open another terminal window, and attach gdb to the emulator:

    make gdb

Now, you should see the qemu-kernel booting up and emulator eventually breaking in the rust module init

Emulator (debug_kernel) should print:

    ...
    [    1.210080] Run /init as init process
    init - entered main()
    init - loading kernel module (iteration=0)
    init - module file fd=3

gdb should print:

    Hardware assisted breakpoint 1 at 0xffffffffa0000474: file samples/rust/rust_minimal.rs, line 21.

    Breakpoint 1, rust_minimal::{impl#0}::init (_name=..., _module=<optimized out>) at samples/rust/rust_minimal.rs:21
    21	        pr_info!("Rust minimal sample (init)\n");
    (gdb)

## Troubleshooting

### Finding module address

Hopefully the module address stays the same (for symbols). If not, this may help (but please make a pull request for documenting a proper way to find the address)

Look for the module kernel message:

    [    5.532793] kobject: 'rust_minimal' (ffffffffa0002090): kobject_release, parent ffff888000928250 (delayed 2000)

Here the address is `0xffffffffa0002090`.

TODO: why +0x2090 to base address `0xffffffffa0000000`?

Update gdb.config `add-symbol-file` section with the base address.
