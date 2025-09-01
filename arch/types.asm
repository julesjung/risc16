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