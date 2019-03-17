# Kicksat Sprite example application

A work in progress application for the Kicksat Sprite. This requires the also work in progress [atsamd-rs kicksat board fork](https://github.com/ryankurte/atsamd/tree/board/kicksat/boards/kicksat).

## Status
Builds, runs on board (if you can convince it to load), needs more pins and peripherals to be defined and implemented.

Note that the uf2 bootloader has boot protection and a bootloader offset set that must be cleared using atmel studio before loading will work (set `NVM BOOTPROT` fuse to `0x0F`).

## Setup

- `curl https://sh.rustup.rs -sSf | sh` to install rust
- `rustup target add thumbv7m-none-eabi` to add cortex-m4 target
- `git clone https://github.com/ryankurte/kicksat-sprite-rs` to clone this repository
- `git clone https://github.com/ryankurte/atsamd --branch board/kicksat --depth=1` to clone the atsamd-rs branch with kicksat support

***the `atsamd` project MUST exist alongside the `kicksat-sprite-rs` for building to work***

## Building

- `cargo build` to build `target/thumbv7m-none-eabi/debug/kicksat-app` elf file
- `cargo objcopy --bin kicksat-app -- -O binary target/thumbv7m-none-eabi/debug/kicksat-app.bin` to convert elf -> bin
- `JLinkExe -device atsamd51g19 -speed 4000 -if SWD -CommanderScript jlinkcmd.txt` to flash bin to board

## Resources

- [Rust embedded books](https://docs.rust-embedded.org)
- [Rust cortex-m quickstart](https://github.com/rust-embedded/cortex-m-quickstart)
- [Atsamd kicksat board fork](https://github.com/ryankurte/atsamd/tree/board/kicksat/boards/kicksat)

