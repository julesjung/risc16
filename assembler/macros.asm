#once

#include "registers.asm"
#include "instructions.asm"

#ruledef {
    MOV {rd: register}, {imm: u16} => {
        high_byte = (imm >> 8)
        low_byte = (imm & 0xff)
        asm {
            MOVH {rd}, {high_byte}
            OVL {rd}, {low_byte}
        }
    }

    LDL {rd: register}, [{addr: u16}] => asm {
        MOV r6, {addr}
        LDL {rd}, [r6]
    }

    STL {rs: register}, [{addr: u16}] => asm {
        MOV r6, {addr}
        STL {rs}, [r6]
    }
}