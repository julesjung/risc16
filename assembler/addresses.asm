#once

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