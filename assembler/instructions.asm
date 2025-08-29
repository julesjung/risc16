#once

#include "addresses.asm"
#include "registers.asm"

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

    CMPL {ra: register}, {rb: register} => le(0x3 @ ra @ rb @ 0b000000)
    CMPH {ra: register}, {rb: register} => le(0x3 @ ra @ rb @ 0b000001)
    CMP {ra: register}, {rb: register} => le(0x3 @ ra @ rb @ 0b000010)

    CMPL {ra: register}, {imm: i8} => le(0x3 @ ra @ imm @ 0b0)
    CMPH {ra: register}, {imm: i8} => le(0x3 @ ra @ imm @ 0b1)

    MOVL {rd: register}, {imm: i8} => le(0x4 @ rd @ imm @ 0b0)
    MOVH {rd: register}, {imm: i8} => le(0x4 @ rd @ imm @ 0b1)

    LDL {rd: register}, [{rs: register}] => le(0x5 @ rd @ rs @ 0b000000)
    LDH {rd: register}, [{rs: register}] => le(0x5 @ rd @ rs @ 0b000001)
    LDW {rd: register}, [{rs: register}] => le(0x5 @ rd @ rs @ 0b000010)

    STL {rs: register}, [{rd: register}] => le(0x6 @ rs @ rd @ 0b000000)
    STH {rs: register}, [{rd: register}] => le(0x6 @ rs @ rd @ 0b000001)
    STW {rs: register}, [{rd: register}] => le(0x6 @ rs @ rd @ 0b000010)

    JMP {offset: off12} => le(0x7 @ offset)

    JMP [{rs: register}] => le(0x8 @ rs @ 0b00000)

    BZ {offset: off9} => le(0x9 @ offset @ 0b000)
    BNZ {offset: off9} => le(0x9 @ offset @ 0b001)
    BC  {offset: off9} => le(0x9 @ offset @ 0b010)
    BNC {offset: off9} => le(0x9 @ offset @ 0b011)
    BS  {offset: off9} => le(0x9 @ offset @ 0b100)
    BNS {offset: off9} => le(0x9 @ offset @ 0b101)

    HLT => le(0xf000)
}