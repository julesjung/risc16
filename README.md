# RISC16

[![GitHub License](https://img.shields.io/github/license/julesjung/risc16)](https://github.com/julesjung/risc16/blob/main/LICENSE.txt)
[![GitHub Release](https://img.shields.io/github/v/release/julesjung/risc16?include_prereleases)](https://github.com/julesjung/risc16/releases)

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
cargo install risc16 --git https://github.com/julesjung/risc16.git
```

### Using pre-built binaries

You can find the pre-built binaries on the [GitHub releases page](https://github.com/julesjung/risc16/releases). Choose the latest release and download the binary corresponding to your operating system and architecture.

## Example

Create a file named `fibonacci.asm` and paste this assembly code in it :

```asm
; This program calculates the Fibonacci sequence using a loop.
; It stores each value in memory at an increasing address.
;
; Registers usage:
; - r1: last number
; - r2: current number
; - r3: memory pointer
; - r4: maximum value
; - r5: next number

#bank code

    MOVL r2, 1      ; r2 = F(1)
    MOVH r3, 0x01   ; r3 = 0x0100 (initial memory address)
    MOVL r4, 0xff   ; r4 = 0x00ff (maximum value)

    INC r3          ; increment memory pointer
    STL r2, [r3]    ; store F(1) in memory
    
loop:
    ADD r5, r1, r2  ; r5 = r1 + r2
    CMP r4, r5      ; check if next number is within range
    BS end          ; if not, jump to the end of the program
    MOV r1, r2      ; r1 = r2
    MOV r2, r5      ; r2 = r5
    INC r3          ; increment memory pointer
    STL r2, [r3]    ; store next number in memory
    JMP loop        ; loop again

end:
    HLT             ; halt the program
```

More examples available in the `examples/` directory.

## Usage

You can see all available commands and options with:

```sh
risc16 help
```

## License

This project is licensed under the GPLv3 license. See the LICENSE file for more details.