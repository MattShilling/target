#!/bin/bash

# ST32 Cube Programmer expects the executables to have a file extension.
# We are building ELF executables.
# - cargo-readobj target/thumbv7em-none-eabihf/debug/target -- --file-headers
# Add the .elf suffix.
# We could probably load the file via ST32 cube CLI or something.
rm -f target/thumbv7em-none-eabihf/debug/target.elf
cargo build
mv -f target/thumbv7em-none-eabihf/debug/target \
    target/thumbv7em-none-eabihf/debug/target.elf

rm -f target/thumbv7em-none-eabihf/release/target.elf
cargo build --release
mv -f target/thumbv7em-none-eabihf/release/target \
    target/thumbv7em-none-eabihf/release/target.elf