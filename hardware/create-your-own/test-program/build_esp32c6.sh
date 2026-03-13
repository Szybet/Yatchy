#!/bin/bash

cargo build --release --no-default-features --features esp32c6 --target riscv32imac-unknown-none-elf
