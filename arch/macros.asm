#once

#include "<std>/types.asm"
#include "<std>/instructions.asm"

#ruledef {
    MOV {rd: register}, {imm: u16} => {
        high_byte = (imm >> 8)
        low_byte = (imm & 0xff)
        asm {
            MOVH {rd}, {high_byte}
            MOVL {rd}, {low_byte}
        }
    }

    MOV {rd: register}, {rs: register} => asm {
        ADD {rd}, {rs}, r0
    }

    TST {rs: register} => asm {
        CMP {rs}, r0
    }

    INC {rd: register} => asm {
        ADDI {rd}, 1
    }

    DEC {rd: register} => asm {
        SUBI {rd}, 1
    }
}