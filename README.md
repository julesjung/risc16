# RISC16

[![GitHub License](https://img.shields.io/github/license/julesjung/risc16)](https://github.com/julesjung/risc16/blob/main/LICENSE.txt)
[![GitHub Release](https://img.shields.io/github/v/release/julesjung/risc16?include_prereleases)](https://github.com/julesjung/risc16/releases)

RISC16 is a custom **16-bit RISC CPU architecture** featuring a Rust-based emulator and assembler support via [customasm](https://github.com/hlorenzi/customasm).
It implements arithmetic, logic, branching, and memory instructions, with 8 general-purpose registers (r0-r7).
This project provides a simple platform for experimenting with ISA design and emulator development.

## Features

- **Registers**: 8 16-bit general-purpose registers (r0 fixed to zero, r7 as stack pointer)
- **ALU operations**: `ADD`, `ADC`, `SUB`, `SBB`, `AND`, `OR`, `XOR`, `NOT`, shifts, rotates
- **Flags**: `CARRY`, `OVERFLOW`, `ZERO`, `SIGNED`, 
- **Control flow**: conditional branches and jumps with signed offsets
- **Memory access**: 8-bit and 16-bit load/store instructions
- **Tools**:
    - Emulator written in Rust with a simple CLI
    - Assembler made with [customasm](https://github.com/hlorenzi/customasm)

## Installation

### From Cargo

Make sure that you have Rust installed on your system. Then, run:

```sh
cargo install risc16 --git https://github.com/julesjung/risc16.git
```

### From pre-built binaries

Open the latest release from [the releases page](https://github.com/julesjung/risc16/releases) and download the binary corresponding to your operating system and architecture.

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

More programs can be found in the `examples/` directory.

## Usage

You can see all available commands and options with:

```sh
risc16 help
```

## License

Licensed under the [GPLv3](https://github.com/julesjung/risc16/blob/main/LICENSE.txt).