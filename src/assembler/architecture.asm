#once

#subruledef register {
    r0 => 0b000
    r1 => 0b001
    r2 => 0b010
    r3 => 0b011
    r4 => 0b100
    r5 => 0b101
    r6 => 0b110
    r7 => 0b111
}

#subruledef off9 {
    {addr: u16} => {
		relative_address = (addr - pc - 2) >> 1
		assert(relative_address <=  0xff)
		assert(relative_address >= !0xff)
		relative_address`9
	}
    
}

#subruledef off12 {
    {addr: u16} => {
		relative_address = (addr - pc - 2) >> 1
		assert(relative_address <=  0x8ff)
		assert(relative_address >= !0x8ff)
		relative_address`12
	}
    
}

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

    BC  {offset: off9} => le(0x9 @ offset @ 0b000)
    BNC {offset: off9} => le(0x9 @ offset @ 0b001)
    BO  {offset: off9} => le(0x9 @ offset @ 0b010)
    BNO {offset: off9} => le(0x9 @ offset @ 0b011)
    BZ  {offset: off9} => le(0x9 @ offset @ 0b100)
    BNZ {offset: off9} => le(0x9 @ offset @ 0b101)
    BS  {offset: off9} => le(0x9 @ offset @ 0b110)
    BNS {offset: off9} => le(0x9 @ offset @ 0b111)

    HLT => le(0xf000)
}

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

#bankdef code
{
    #addr 0x0000
    #size 0x0100
    #outp 0
}

#bankdef data
{
    #addr 0x0100
    #size 0x0010
    #outp 8 * 0x0100
}