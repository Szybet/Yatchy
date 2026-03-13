#!/bin/bash
export LIBCLANG_PATH="/home/szybet/.rustup/toolchains/esp/xtensa-esp32-elf-clang/esp-20.1.1_20250829/esp-clang/lib"
export PATH="/home/szybet/.rustup/toolchains/esp/xtensa-esp-elf/esp-15.2.0_20250920/xtensa-esp-elf/bin:$PATH"
cargo build --release --no-default-features --features esp32s3 --target xtensa-esp32s3-none-elf
