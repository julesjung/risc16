# RISC16 Spec Sheet

## 1. Architecture Overview

- **Word Size**: 16 bits  
- **Registers**: R0–R7 (General purpose), PC (Program Counter), SP (Stack Pointer), FLAGS
- **Memory**: 64KB addressable space (16-bit addresses)
- **Endianness**: Little-endian
- **Instruction Width**: 16 bits

## 2. Instruction Formats

### Register to register

| Length | Offset | Description          |
| ------ | ------ | -------------------- |
| 4      | 12     | Opcode               |
| 6      | 6      | Sub-opcode           |
| 3      | 3      | Destination register |
| 3      | 0      | Source register      |

### Memory to register

| Length | Offset | Description          |
| ------ | ------ | -------------------- |
| 4      | 12     | Opcode               |
| 6      | 6      | Sub-opcode           |
| 3      | 3      | Destination register |
| 3      | 0      | Address register     |

### Register to memory

| Length | Offset | Description      |
| ------ | ------ | ---------------- |
| 4      | 12     | Opcode           |
| 6      | 6      | Sub-opcode       |
| 3      | 3      | Address register |
| 3      | 0      | Source register  |

### To register

| Length | Offset | Description          |
| ------ | ------ | -------------------- |
| 4      | 12     | Opcode               |
| 6      | 6      | Sub-opcode           |
| 3      | 3      | Destination register |
| 3      | 0      | Unused               |

### Immediate to register

| Length | Offset | Description             |
| ------ | ------ | ----------------------- |
| 4      | 12     | Opcode                  |
| 3      | 9      | Destination register    |
| 1      | 8      | Register byte selection |
| 8      | 0      | Immediate value         |

### Absolute jumps

| Length | Offset | Description      |
| ------ | ------ | ---------------- |
| 4      | 12     | Opcode           |
| 6      | 6      | Sub-opcode       |
| 3      | 3      | Address register |
| 3      | 0      | Unused           |

### Relative jumps

| Length | Offset | Description      |
| ------ | ------ | ---------------- |
| 4      | 12     | Opcode           |
| 4      | 8      | Sub-opcode       | 
| 8      | 0      | Signed offset    |

### Nullary

| Length | Offset | Description |
| ------ | ------ | ----------- |
| 4      | 12     | Opcode      |
| 12     | 0      | Unused      |

## 3. Instruction Set

### Register to register

| Opcode | Sub-opcode | Mnemonic      | Description                    |
| ------ | ---------- | ------------- | ------------------------------ |
| `0x0`  | `0x01`     | `ADD rd, rs`  | Add                            |
| `0x0`  | `0x02`     | `SUB rd, rs`  | Subtract                       |
| `0x0`  | `0x03`     | `AND rd, rs`  | Bitwise AND                    |
| `0x0`  | `0x04`     | `OR  rd, rs`  | Bitwise OR                     |
| `0x0`  | `0x05`     | `XOR rd, rs`  | Bitwise XOR                    |
| `0x0`  | `0x06`     | `SHL rd, rs`  | Logical shift left             |
| `0x0`  | `0x07`     | `SHR rd, rs`  | Logical shift right            |
| `0x0`  | `0x11`     | `SWAP rd, rs` | Swap contents of two registers |
| `0x0`  | `0x12`     | `COPY rd, rs` | Move rs into rd                |
| `0x0`  | `0x13`     | `CMP rd, rs`  | Compare rd - rs, sets flags    |
| `0x0`  | `0x17`     | `MOV rd, rs`  | Set rd from rs                 |
| `0x0`  | `0x18`     | `MOV SP, rs`  | Set Stack Pointer              |

### Memory to register

| Opcode | Sub-opcode | Mnemonic      | Description                    |
| ------ | ---------- | ------------- | ------------------------------ |
| `0x0`  | `0x08`     | `LD rd, [rs]` | Load from memory               |

### Register to memory

| Opcode | Sub-opcode | Mnemonic      | Description                    |
| ------ | ---------- | ------------- | ------------------------------ |
| `0x0`  | `0x09`     | `ST [rd], rs` | Store to memory                |

### To register

| Opcode | Sub-opcode | Mnemonic      | Description                    |
| ------ | ---------- | ------------- | ------------------------------ |
| `0x0`  | `0x0A`     | `INC rd`      | Increment                      |
| `0x0`  | `0x0B`     | `DEC rd`      | Decrement                      |
| `0x0`  | `0x0C`     | `CLR rd`      | Clear (set to 0)               |
| `0x0`  | `0x0D`     | `NEG rd`      | Two’s complement negate        |
| `0x0`  | `0x0E`     | `NOT rd`      | Bitwise NOT                    |
| `0x0`  | `0x0F`     | `PUSH rd`     | Push register to stack         |
| `0x0`  | `0x10`     | `POP rd`      | Pop register from stack        |

### Immediate to register

| Opcode | Mnemonic         | Description                           |
| ------ | ---------------- | ------------------------------------- |
| `0x2`  | `LI rd, #imm8`   | Load immediate 8-bit value            |
| `0x3`  | `ADDI rd, #imm8` | Add 8-bit immediate to register       |
| `0x4`  | `SUBI rd, #imm8` | Subtract 8-bit immediate from reg     |
| `0x5`  | `CMPI rd, #imm8` | Compare register with 8-bit immediate |

### Absolute jumps

| Opcode | Sub-opcode | Mnemonic      | Description                    |
| ------ | ---------- | ------------- | ------------------------------ |
| `0x0`  | `0x14`     | `JMP [rd]`  | Absolute jump: PC ← rd              |
| `0x0`  | `0x15`     | `CALL [rd]`  | Push PC to stack, jump to rd        |
| `0x0`  | `0x19`     | `JEQ [rd]`   | Jump if equal (Z=1)                 |
| `0x0`  | `0x1A`     | `JNE [rd]`   | Jump if not equal (Z=0)             |
| `0x0`  | `0x1B`     | `JGT [rd]`   | Jump if greater (Z=0, S=O)          |
| `0x0`  | `0x1C`     | `JLT [rd]`   | Jump if less (S≠O)                  |
| `0x0`  | `0x1D`     | `JGE [rd]`   | Jump if greater or equal (S=O)      |
| `0x0`  | `0x1E`     | `JLE [rd]`   | Jump if less or equal (Z=1 or S≠O)  |
| `0x0`  | `0x22`     | `JC [rd]`    | Jump if carry (C=1)                 |
| `0x0`  | `0x23`     | `JNC [rd]`   | Jump if no carry (C=0)              |
| `0x0`  | `0x24`     | `JO [rd]`    | Jump if overflow (O=1)              |
| `0x0`  | `0x25`     | `JNO [rd]`   | Jump if no overflow (O=0)           |
| `0x0`  | `0x26`     | `JS [rd]`    | Jump if negative (S=1)              |
| `0x0`  | `0x27`     | `JNS [rd]`   | Jump if positive (S=0)              |
| `0x0`  | `0x28`     | `JZ [rd]`    | Jump if zero (Z=1)                  |
| `0x0`  | `0x29`     | `JNZ [rd]`   | Jump if not zero (Z=0)              |

### Relative jumps

| Opcode | Sub-opcode | Mnemonic      | Description                         |
|--------|------------|---------------|-------------------------------------|
| `0x1`  | `0x0`      | `JMP #offset` | Relative jump                       |
| `0x1`  | `0x1`      | `CALL #offset`| Push PC to stack, jump to offset    |
| `0x1`  | `0x2`      | `JEQ #offset` | Jump if equal (Z=1)                 |
| `0x1`  | `0x3`      | `JNE #offset` | Jump if not equal (Z=0)             |
| `0x1`  | `0x4`      | `JGT #offset` | Jump if greater (Z=0, S=O)          |
| `0x1`  | `0x5`      | `JLT #offset` | Jump if less (S≠O)                  |
| `0x1`  | `0x6`      | `JGE #offset` | Jump if greater or equal (S=O)      |
| `0x1`  | `0x7`      | `JLE #offset` | Jump if less or equal (Z=1 or S≠O)  |
| `0x1`  | `0x8`      | `JC #offset`  | Jump if carry (C=1)                 |
| `0x1`  | `0x9`      | `JNC #offset` | Jump if no carry (C=0)              |
| `0x1`  | `0xA`      | `JO #offset`  | Jump if overflow (O=1)              |
| `0x1`  | `0xB`      | `JNO #offset` | Jump if no overflow (O=0)           |
| `0x1`  | `0xC`      | `JS #offset`  | Jump if negative (S=1)              |
| `0x1`  | `0xD`      | `JNS #offset` | Jump if positive (S=0)              |
| `0x1`  | `0xE`      | `JZ #offset`  | Jump if zero (Z=1)                  |
| `0x1`  | `0xF`      | `JNZ #offset` | Jump if not zero (Z=0)              |

### Nullary

| Opcode | Sub-opcode | Mnemonic      | Description                    |
| ------ | ---------- | ------------- | ------------------------------ |
| `0x0`  | `0x00`     | `NOP`         | No operation                   |
| `0x0`  | `0x16`     | `RET`         | Pop address from stack into PC |
| `0x0`  | `0x1F`     | `HLT`         | Halt processor                 |
| `0x0`  | `0x21`     | `CLRFLAGS`    | Clear `FLAGS` register         |

> Remaining sub-opcode space: `0x22` to `0x3F` available

## 4. Registers

| Register | Binary code | Description     |
| -------- | ----------- | --------------- |
| R0       | 000         | General purpose |
| R1       | 001         | General purpose |
| R2       | 010         | General purpose |
| R3       | 011         | General purpose |
| R4       | 100         | General purpose |
| R5       | 101         | General purpose |
| R6       | 110         | General purpose |
| R7       | 111         | General purpose |
| PC       | N/A         | Program Counter |
| SP       | N/A         | Stack Pointer   |
| FLAGS    | N/A         | Status Flags    |

## 5. Immediate Encoding with Register Byte Access

### Syntax

Use `Lx` or `Hx` in `LI`, `ADDI`, etc. to load or modify only 8 bits of a register.