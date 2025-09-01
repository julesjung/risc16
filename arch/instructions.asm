#once

#include "<std>/types.asm"

#ruledef {
    ADD {rd: register}, {ra: register}, {rb: register} => le(0x0 @ rd @ ra @ rb @ 0b000)
    ADC {rd: register}, {ra: register}, {rb: register} => le(0x0 @ rd @ ra @ rb @ 0b001)
    SUB {rd: register}, {ra: register}, {rb: register} => le(0x0 @ rd @ ra @ rb @ 0b010)
    SBB {rd: register}, {ra: register}, {rb: register} => le(0x0 @ rd @ ra @ rb @ 0b011)
    AND {rd: register}, {ra: register}, {rb: register} => le(0x0 @ rd @ ra @ rb @ 0b100)
    OR {rd: register}, {ra: register}, {rb: register} => le(0x0 @ rd @ ra @ rb @ 0b101)
    XOR {rd: register}, {ra: register}, {rb: register} => le(0x0 @ rd @ ra @ rb @ 0b110)
    NOT {rd: register}, {ra: register} => le(0x0 @ rd @ ra @ 0b000 @ 0b111)

    SHL {rd: register}, {rs: register}, {imm: u4} => le(0x1 @ rd @ rs @ imm @ 0b00)
    SHR {rd: register}, {rs: register}, {imm: u4} => le(0x1 @ rd @ rs @ imm @ 0b01)
    SAR {rd: register}, {rs: register}, {imm: u4} => le(0x1 @ rd @ rs @ imm @ 0b10)
    ROR {rd: register}, {rs: register}, {imm: u4} => le(0x1 @ rd @ rs @ imm @ 0b11)

    ADDI {rd: register}, {imm: i8} => le(0x2 @ rd @ imm @ 0b0)
    SUBI {rd: register}, {imm: i8} => le(0x2 @ rd @ imm @ 0b1)

    CMP {ra: register}, {rb: register} => le(0x3 @ ra @ rb @ 0b000000)
    CMPL {ra: register}, {rb: register} => le(0x3 @ ra @ rb @ 0b000010)
    CMPH {ra: register}, {rb: register} => le(0x3 @ ra @ rb @ 0b000011)
    
    CMPL {rs: register}, {imm: i8} => le(0x4 @ ra @ imm @ 0b0)
    CMPH {rs: register}, {imm: i8} => le(0x4 @ ra @ imm @ 0b1)

    MOVL {rd: register}, {imm: i8} => le(0x5 @ rd @ imm @ 0b0)
    MOVH {rd: register}, {imm: i8} => le(0x5 @ rd @ imm @ 0b1)

    LDW {rd: register}, [{rs: register}] => le(0x6 @ rd @ rs @ 0b000000)
    LDL {rd: register}, [{rs: register}] => le(0x6 @ rd @ rs @ 0b000010)
    LDH {rd: register}, [{rs: register}] => le(0x6 @ rd @ rs @ 0b000011)
    
    STW {rs: register}, [{rd: register}] => le(0x7 @ rs @ rd @ 0b000000)
    STL {rs: register}, [{rd: register}] => le(0x7 @ rs @ rd @ 0b000010)
    STH {rs: register}, [{rd: register}] => le(0x7 @ rs @ rd @ 0b000011)

    JMP {offset: off12} => le(0x8 @ offset)

    JMP [{rs: register}] => le(0x9 @ rs @ 0b00000)

    BC  {offset: off9} => le(0xa @ offset @ 0b000)
    BNC {offset: off9} => le(0xa @ offset @ 0b001)
    BO  {offset: off9} => le(0xa @ offset @ 0b010)
    BNO {offset: off9} => le(0xa @ offset @ 0b011)
    BZ  {offset: off9} => le(0xa @ offset @ 0b100)
    BNZ {offset: off9} => le(0xa @ offset @ 0b101)
    BS  {offset: off9} => le(0xa @ offset @ 0b110)
    BNS {offset: off9} => le(0xa @ offset @ 0b111)

    ; BSET {rd: register, imm: u4}
    ; BCLR {rd: register, imm: u4}
    ; BTST {rd: register, imm: u4}

    ; ANDI {rd: register, imm: u8}
    ; ORI  {rd: register, imm: u8}

    ; PUSH {rs: regiser}
    ; POP {rd: register}
    ; CALL {offset: off12}
    ; RET

    HLT => le(0xf000)
}