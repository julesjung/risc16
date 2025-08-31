# RISC16

[![GitHub License](https://img.shields.io/github/license/julesjung/risc16)](https://github.com/julesjung/risc16/blob/main/LICENSE.txt)
[![GitHub Release](https://img.shields.io/github/v/release/julesjung/risc16?include_prereleases)](https://github.com/julesjung/risc16/releases/latest)

RISC16 is a custom **16-bit RISC CPU architecture** featuring a Rust-based emulator and assembler support via [customasm](https://github.com/hlorenzi/customasm).
It implements arithmetic, logic, branching, and memory instructions, with 8 general-purpose registers (r0-r7).
This project provides a simple platform for experimenting with ISA design and emulator development.

## Features

- 16-bit CPU with 8 general-purpose registers
- `r0` hard-wired to zero, `r7` used as stack pointer
- ALU with ADD, ADC, SUB, SBB, AND, OR, XOR, NOT, shifts, rotates
- Full flag support (zero, carry, signed, overflow)
- Branching and jumping with signed offsets
- Load/store for 8-bit and 16-bit memory access
- Emulator written in Rust with a simple CLI
- Program assembly with [customasm](https://github.com/hlorenzi/customasm)

## Installation

### Using Cargo

Make sure that you have Rust installed on your system. If not, download and install it from [the official website](https://www.rust-lang.org/tools/install). Then, you can directly install the `risc16` cli using Cargo:

```sh
cargo install --git https://github.com/julesjung/risc16.git risc16
```

### Using pre-built binaries

You can find the pre-built binaries on the [GitHub releases page](https://github.com/julesjung/risc16/releases). Download the one corresponding to your operating system and architecture.

## Example

Assemble and run a simple Fibonacci program:

```sh
risc16 emulate examples/fibonacci.bin
```

## Usage

You can see all available commands and options with:

```sh
risc16 help
```

## License

This project is licensed under the GPLv3 license. See the LICENSE file for more details.