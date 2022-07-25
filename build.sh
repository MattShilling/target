#!/bin/bash

rm -f target/thumbv7em-none-eabihf/debug/target.elf
cargo build
mv -f target/thumbv7em-none-eabihf/debug/target \
    target/thumbv7em-none-eabihf/debug/target.elf

rm -f target/thumbv7em-none-eabihf/release/target.elf
cargo build --release
mv -f target/thumbv7em-none-eabihf/release/target \
    target/thumbv7em-none-eabihf/release/target.elf