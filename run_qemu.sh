#!/bin/bash

cargo build --release

qemu-system-riscv64 -machine virt -bios default -nographic -serial mon:stdio --no-reboot -kernel target/riscv64imac-unknown-none-elf/release/meos
