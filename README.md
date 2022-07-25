# target

## Prereqs
- Rust
    - https://www.rust-lang.org/tools/install
    - https://docs.rust-embedded.org/book/intro/install.html
    - https://docs.rust-embedded.org/book/intro/tooling.html
        - `rustup target add thumbv7em-none-eabihf`
        - `cargo install cargo-binutils`
        - `rustup component add llvm-tools-preview` 
        - `cargo install cargo-generate`
   
- https://www.st.com/en/development-tools/stm32cubeprog.html


# STM32

STM32 Peripheral Access Crates:
- https://github.com/stm32-rs/stm32-rs

Guides:
- https://github.com/rust-embedded/cortex-m-quickstart

## STM32F405RG
- https://www.sparkfun.com/products/17712
    - Embedded 1024 KB flash 
- https://github.com/stm32-rs/stm32f4xx-hal
- https://github.com/adamgreig/stm32f4-demo
- Reference manual: https://www.st.com/resource/en/reference_manual/dm00031020-stm32f405-415-stm32f407-417-stm32f427-437-and-stm32f429-439-advanced-arm-based-32-bit-mcus-stmicroelectronics.pdf

NOTE: The STM32F405 chip has an embedded bootloader code is located in ROM system memory (`0x1FFF 0000 - 0x1FFF 77FF`). This is seperate from the embedded Flash memory (0x0800 0000 - 0x081F FFFF).

# STM32 MPU

Board:
- https://www.st.com/en/evaluation-tools/stm32mp157f-dk2.html#documentation

Bare Metal:
- https://github.com/4ms/stm32mp1-baremetal

Rust prospects:
- https://github.com/geraldstanje/rust-arm-cortex-a9

# Concurrency
- https://github.com/rtic-rs/cortex-m-rtic

