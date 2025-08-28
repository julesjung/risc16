#once

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
    SAL {rd: register}, {rs: register}, {imm: u4} => le(0x1 @ rd @ rs @ imm @ 0b10)
    SAR {rd: register}, {rs: register}, {imm: u4} => le(0x1 @ rd @ rs @ imm @ 0b11)
    CMP {ra: register}, {rb: register} => le(0x2 @ ra @ rb @ 0b000000)
    MOVL {rd: register}, {imm: u8} => le(0x3 @ rd @ 0b0 @ imm)
    MOVH {rd: register}, {imm: u8} => le(0x3 @ rd @ 0b1 @ imm)
    LDL {rd: register}, [{rs: register}] => le(0x4 @ rd @ 0b0 @ rs)
    LDH {rd: register}, [{rs: register}] => le(0x4 @ rd @ 0b1 @ rs)
    STL {rs: register}, [{rd: register}] => le(0x5 @ rs @ 0b0 @ rd @ 0b00000)
    STH {rs: register}, [{rd: register}] => le(0x5 @ rs @ 0b1 @ rd @ 0b00000)
    HLT => le(0xf000)
}