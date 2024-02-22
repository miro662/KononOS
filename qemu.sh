#!/bin/bash

TARGET_FILE="target/riscv64gc-unknown-none-elf/debug/kononos"

case $1 in
    help)
        echo "syntax: $0 [lldb/help]"
        ;;
    lldb)
        cargo build
        qemu-system-riscv64 -M virt -kernel $TARGET_FILE -bios none -serial stdio -display none -S -s &
        rust-lldb -o "gdb-remote localhost:1234" $TARGET_FILE
        ;;
    *) 
        cargo build
        qemu-system-riscv64 -M virt -kernel $TARGET_FILE -bios none -serial stdio -display none
        ;;
esac