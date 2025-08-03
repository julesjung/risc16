### RRI type

| Length | Offset | Description          |
| ------ | ------ | -------------------- |
| 5      | 11     | Opcode               |
| 3      | 8      | Destination register |
| 3      | 5      | Source register      |
| 5      | 0      | Immediate            |

### RI type

| Length | Offset | Description          |
| ------ | ------ | -------------------- |
| 5      | 11     | Opcode               |
| 3      | 8      | Destination register |
| 8      | 0      | Immediate            |

### I type

| Length | Offset | Description          |
| ------ | ------ | -------------------- |
| 5      | 11     | Opcode               |
| 8      | 0      | Immediate            |

## Instruction set

| Opcode | Instruction          | Format | Description                                                              |
| ------ | -------------------- | ------ | ------------------------------------------------------------------------ |
| `0x00` | `LUI rd, #u8`        | RI     | Load immediate into rd upper byte                                        |
| `0x01` | `JMP #i11`           | I      | Add `#i11 * 2` to `PC`                                                   |
| `0x02` | `LW rd, rs, #i5`     | RRI    | Add `#i5` to `rs` and loads a word from resulting memory address to `rd` |