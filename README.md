# Rust - Linux kernel module development example

## Prequisites

    apt-get install make gcc cpio

## Building

First, build the init binary & initramfs:

    make prepare

## Running

Then, connect to emulator with gdb:

    make gdb
