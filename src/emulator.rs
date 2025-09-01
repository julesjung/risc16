use crate::assembler::assemble_to_binary;
use crate::instructions::Instruction;
use anyhow::{Result, bail};
use std::{fs::read, str::FromStr};

#[derive(Debug, Default)]
pub struct Flags {
    pub carry: bool,
    pub overflow: bool,
    pub zero: bool,
    pub signed: bool,
}

pub struct Cpu {
    pub memory: [u8; 65536],
    pub registers: [u16; 8],
    pub flags: Flags,
    pub program_counter: u16,
    pub halted: bool,
}

pub struct EmulatorOptions {
    pub step: bool,
    pub cycles: Option<u64>,
    pub show_registers: bool,
    pub show_flags: bool,
    pub show_memory: bool,
    pub memory_start: u16,
    pub memory_end: u16,
    pub memory_format: MemoryFormat,
}

#[derive(Clone)]
pub enum InputFormat {
    Asm,
    Bin,
}

impl FromStr for InputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "asm" => Ok(InputFormat::Asm),
            "bin" => Ok(InputFormat::Bin),
            _ => anyhow::bail!("invalid input format"),
        }
    }
}

#[derive(Clone)]
pub enum MemoryFormat {
    Hex,
    Dec,
    Bin,
}

impl FromStr for MemoryFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "hex" => Ok(MemoryFormat::Hex),
            "dec" => Ok(MemoryFormat::Dec),
            "bin" => Ok(MemoryFormat::Bin),
            _ => anyhow::bail!("invalid input format"),
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            memory: [0; 65536],
            registers: [0; 8],
            flags: Flags::default(),
            program_counter: 0,
            halted: false,
        }
    }
}

impl Cpu {
    pub fn run(&mut self) -> Result<()> {
        while !self.halted {
            let instruction = self.fetch();
            let instruction = self.decode(instruction)?;
            self.execute(instruction);
        }
        Ok(())
    }

    fn fetch(&mut self) -> u16 {
        let instruction = self.read_word(self.program_counter);
        self.program_counter = self.program_counter.wrapping_add(2);
        instruction
    }

    fn decode(&self, instruction: u16) -> Result<Instruction> {
        match instruction >> 12 {
            0x0 => {
                let rd = ((instruction >> 9) & 0b111) as usize;
                let ra = ((instruction >> 6) & 0b111) as usize;
                let rb = ((instruction >> 3) & 0b111) as usize;
                match instruction & 0b111 {
                    0b000 => Ok(Instruction::Add { rd, ra, rb }),
                    0b001 => Ok(Instruction::AddWithCarry { rd, ra, rb }),
                    0b010 => Ok(Instruction::Subtract { rd, ra, rb }),
                    0b011 => Ok(Instruction::SubtractWithBorrow { rd, ra, rb }),
                    0b100 => Ok(Instruction::And { rd, ra, rb }),
                    0b101 => Ok(Instruction::Or { rd, ra, rb }),
                    0b110 => Ok(Instruction::Xor { rd, ra, rb }),
                    0b111 => Ok(Instruction::Not { rd, ra }),
                    _ => unreachable!(),
                }
            }
            0x1 => {
                let rd = ((instruction >> 9) & 0b111) as usize;
                let rs = ((instruction >> 6) & 0b111) as usize;
                let imm = (instruction >> 2) & 0b1111;
                match instruction & 0b11 {
                    0b00 => Ok(Instruction::LogicalLeftShift { rd, rs, imm }),
                    0b01 => Ok(Instruction::LogicalRightShift { rd, rs, imm }),
                    0b10 => Ok(Instruction::ArithmeticRightShift { rd, rs, imm }),
                    0b11 => Ok(Instruction::RotateRight { rd, rs, imm }),
                    _ => unreachable!(),
                }
            }
            0x2 => {
                let rd: usize = ((instruction >> 9) & 0b111) as usize;
                let imm = (instruction >> 1) & 0b1111_1111;
                match instruction & 0b1 {
                    0b0 => Ok(Instruction::AddImmediate { rd, imm }),
                    0b1 => Ok(Instruction::SubImmediate { rd, imm }),
                    _ => unreachable!(),
                }
            }
            0x3 => {
                let ra = ((instruction >> 9) & 0b111) as usize;
                let rb = ((instruction >> 6) & 0b111) as usize;
                match instruction & 0b11 {
                    0b00 => Ok(Instruction::Compare { ra, rb }),
                    0b01 => bail!("Unknown instruction: 0x{instruction:04x}"),
                    0b10 => Ok(Instruction::CompareLowBytes { ra, rb }),
                    0b11 => Ok(Instruction::CompareHighBytes { ra, rb }),
                    _ => unreachable!(),
                }
            }
            0x4 => {
                let rs: usize = ((instruction >> 9) & 0b111) as usize;
                let imm = (instruction >> 1) & 0b1111_1111;
                match instruction & 0b1 {
                    0b0 => Ok(Instruction::CompareImmediateWithLowByte { rs, imm }),
                    0b1 => Ok(Instruction::CompareImmediateWithHighByte { rs, imm }),
                    _ => unreachable!(),
                }
            }
            0x5 => {
                let rd: usize = ((instruction >> 9) & 0b111) as usize;
                let imm = (instruction >> 1) & 0b1111_1111;
                match instruction & 0b1 {
                    0b0 => Ok(Instruction::MoveImmediateToLowByte { rd, imm }),
                    0b1 => Ok(Instruction::MoveImmediateToHighByte { rd, imm }),
                    _ => unreachable!(),
                }
            }
            0x6 => {
                let rd = ((instruction >> 9) & 0b111) as usize;
                let rs = ((instruction >> 6) & 0b111) as usize;
                match instruction & 0b11 {
                    0b00 => Ok(Instruction::LoadWord { rd, rs }),
                    0b01 => bail!("Unknown instruction: 0x{instruction:04x}"),
                    0b10 => Ok(Instruction::LoadToLowByte { rd, rs }),
                    0b11 => Ok(Instruction::LoadToHighByte { rd, rs }),
                    _ => unreachable!(),
                }
            }
            0x7 => {
                let rs = ((instruction >> 9) & 0b111) as usize;
                let rd = ((instruction >> 6) & 0b111) as usize;
                match instruction & 0b11 {
                    0b00 => Ok(Instruction::StoreWord { rs, rd }),
                    0b01 => bail!("Unknown instruction: 0x{instruction:04x}"),
                    0b10 => Ok(Instruction::StoreFromLowByte { rs, rd }),
                    0b11 => Ok(Instruction::StoreFromHighByte { rs, rd }),
                    _ => unreachable!(),
                }
            }
            0x8 => {
                let offset = instruction & 0b1111_1111_1111;
                let offset = ((offset << 4) as i16) >> 4;
                Ok(Instruction::JumpToOffset { offset })
            }
            0x9 => {
                let rs = ((instruction >> 9) & 0b111) as usize;
                Ok(Instruction::JumpToPointer { rs })
            }
            0xa => {
                let offset = (instruction >> 3) & 0b1_1111_1111;
                let offset = ((offset << 7) as i16) >> 7;
                match instruction & 0b111 {
                    0b000 => Ok(Instruction::BranchIfCarry { offset }),
                    0b001 => Ok(Instruction::BranchIfNotCarry { offset }),
                    0b010 => Ok(Instruction::BranchIfOverflow { offset }),
                    0b011 => Ok(Instruction::BranchIfNotOverflow { offset }),
                    0b100 => Ok(Instruction::BranchIfZero { offset }),
                    0b101 => Ok(Instruction::BranchIfNotZero { offset }),
                    0b110 => Ok(Instruction::BranchIfSigned { offset }),
                    0b111 => Ok(Instruction::BranchIfNotSigned { offset }),
                    _ => unreachable!(),
                }
            }
            0xf => Ok(Instruction::Halt),
            _ => bail!("Unknown instruction: 0x{instruction:04x}"),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add { rd, ra, rb } => {
                let ra = self.registers[ra];
                let rb = self.registers[rb];
                let (result, carry) = ra.overflowing_add(rb);
                self.registers[rd] = result;
                self.flags.carry = carry;
                let xor1 = ra ^ result;
                let xor2 = rb ^ result;
                self.flags.overflow = ((xor1 & xor2) & 0x8000) != 0;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            }
            Instruction::AddWithCarry { rd, ra, rb } => {
                let ra = self.registers[ra];
                let rb = self.registers[rb];
                let carry = self.flags.carry as u16;
                let (result, carry1) = ra.overflowing_add(rb);
                let (result, carry2) = result.overflowing_add(carry);
                self.registers[rd] = result;
                self.flags.carry = carry1 | carry2;
                let xor1 = ra ^ result;
                let xor2 = rb ^ result;
                self.flags.overflow = ((xor1 & xor2) & 0x8000) != 0;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            }
            Instruction::Subtract { rd, ra, rb } => {
                let ra = self.registers[ra];
                let rb = self.registers[rb];
                let (result, borrow) = ra.overflowing_sub(rb);
                self.registers[rd] = result;
                self.flags.carry = borrow;
                let xor1 = ra ^ rb;
                let xor2 = ra ^ result;
                self.flags.overflow = ((xor1 & xor2) & 0x8000) != 0;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            }
            Instruction::SubtractWithBorrow { rd, ra, rb } => {
                let ra = self.registers[ra];
                let rb = self.registers[rb];
                let borrow = self.flags.carry as u16;
                let (result, borrow1) = ra.overflowing_sub(rb);
                let (result, borrow2) = result.overflowing_sub(borrow);
                self.registers[rd] = result;
                self.flags.carry = borrow1 | borrow2;
                let xor1 = ra ^ rb;
                let xor2 = ra ^ result;
                self.flags.overflow = ((xor1 & xor2) & 0x8000) != 0;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            }
            Instruction::And { rd, ra, rb } => {
                let ra = self.registers[ra];
                let rb = self.registers[rb];
                let result = ra & rb;
                self.registers[rd] = result;
                self.flags.carry = false;
                self.flags.overflow = false;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            }
            Instruction::Or { rd, ra, rb } => {
                let ra = self.registers[ra];
                let rb = self.registers[rb];
                let result = ra | rb;
                self.registers[rd] = result;
                self.flags.carry = false;
                self.flags.overflow = false;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            }
            Instruction::Xor { rd, ra, rb } => {
                let ra = self.registers[ra];
                let rb = self.registers[rb];
                let result = ra ^ rb;
                self.registers[rd] = result;
                self.flags.carry = false;
                self.flags.overflow = false;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            }
            Instruction::Not { rd, ra } => {
                let result = !self.registers[ra];
                self.registers[rd] = result;
                self.flags.carry = false;
                self.flags.overflow = false;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            }
            Instruction::LogicalLeftShift { rd, rs, imm } => {
                let rs = self.registers[rs];
                let result = rs << imm;
                self.registers[rd] = result;
                self.flags.carry = ((rs >> (16 - imm)) & 1) != 0;
                self.flags.overflow = false;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            }
            Instruction::LogicalRightShift { rd, rs, imm } => {
                let rs = self.registers[rs];
                let result = rs >> imm;
                self.registers[rd] = result;
                self.flags.carry = ((rs >> (imm - 1)) & 1) != 0;
                self.flags.overflow = false;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            }
            Instruction::ArithmeticRightShift { rd, rs, imm } => {
                let rs = self.registers[rs] as i16;
                let result = (rs >> imm) as u16;
                self.registers[rd] = result;
                self.flags.carry = (((rs as u16) >> (imm - 1)) & 1) != 0;
                self.flags.overflow = false;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            }
            Instruction::RotateRight { rd, rs, imm } => {
                let rs = self.registers[rs];
                let result = (rs >> imm) | (rs << (16 - imm));
                self.registers[rd] = result;
                self.flags.carry = (result >> 15) & 1 != 0;
                self.flags.overflow = false;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            }
            Instruction::AddImmediate { rd, imm } => {
                let ra = self.registers[rd];
                let (result, carry) = ra.overflowing_add(imm);
                self.registers[rd] = result;
                self.flags.carry = carry;
                let xor1 = ra ^ result;
                let xor2 = imm ^ result;
                self.flags.overflow = ((xor1 & xor2) & 0x8000) != 0;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            }
            Instruction::SubImmediate { rd, imm } => {
                let ra = self.registers[rd];
                let (result, borrow) = ra.overflowing_sub(imm);
                self.registers[rd] = result;
                self.flags.carry = borrow;
                let xor1 = ra ^ imm;
                let xor2 = ra ^ result;
                self.flags.overflow = ((xor1 & xor2) & 0x8000) != 0;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            }
            Instruction::Compare { ra, rb } => {
                let ra = self.registers[ra];
                let rb = self.registers[rb];
                let (result, borrow) = ra.overflowing_sub(rb);
                self.flags.carry = borrow;
                let xor1 = ra ^ rb;
                let xor2 = ra ^ result;
                self.flags.overflow = ((xor1 & xor2) & 0x8000) != 0;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            }
            Instruction::CompareLowBytes { ra, rb } => {
                let ra = self.registers[ra] as u8;
                let rb = self.registers[rb] as u8;
                let (result, borrow) = ra.overflowing_sub(rb);
                self.flags.carry = borrow;
                let xor1 = ra ^ rb;
                let xor2 = ra ^ result;
                self.flags.overflow = ((xor1 & xor2) & 0x80) != 0;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x80) != 0;
            }
            Instruction::CompareHighBytes { ra, rb } => {
                let ra = (self.registers[ra] >> 8) as u8;
                let rb = (self.registers[rb] >> 8) as u8;
                let (result, borrow) = ra.overflowing_sub(rb);
                self.flags.carry = borrow;
                let xor1 = ra ^ rb;
                let xor2 = ra ^ result;
                self.flags.overflow = ((xor1 & xor2) & 0x80) != 0;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x80) != 0;
            }
            Instruction::CompareImmediateWithLowByte { rs, imm } => {
                let rs = self.registers[rs] as u8;
                let imm = imm as u8;
                let (result, borrow) = rs.overflowing_sub(imm);
                self.flags.carry = borrow;
                let xor1 = rs ^ imm;
                let xor2 = rs ^ result;
                self.flags.overflow = ((xor1 & xor2) & 0x80) != 0;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x80) != 0;
            }
            Instruction::CompareImmediateWithHighByte { rs, imm } => {
                let rs = (self.registers[rs] >> 8) as u8;
                let imm = imm as u8;
                let (result, borrow) = rs.overflowing_sub(imm);
                self.flags.carry = borrow;
                let xor1 = rs ^ imm;
                let xor2 = rs ^ result;
                self.flags.overflow = ((xor1 & xor2) & 0x80) != 0;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x80) != 0;
            }
            Instruction::MoveImmediateToLowByte { rd, imm } => {
                self.registers[rd] = (self.registers[rd] & 0xFF00) | imm;
            }
            Instruction::MoveImmediateToHighByte { rd, imm } => {
                self.registers[rd] = (self.registers[rd] & 0x00FF) | imm << 8;
            }
            Instruction::LoadWord { rd, rs } => {
                let value = self.read_word(self.registers[rs]);
                self.registers[rd] = value;
            }
            Instruction::LoadToLowByte { rd, rs } => {
                let value = self.read_byte(self.registers[rs]) as u16;
                self.registers[rd] = (self.registers[rd] & 0xFF00) | value;
            }
            Instruction::LoadToHighByte { rd, rs } => {
                let value = self.read_byte(self.registers[rs]) as u16;
                self.registers[rd] = (self.registers[rd] & 0x00FF) | value << 8;
            }
            Instruction::StoreWord { rs, rd } => {
                let value = self.registers[rs];
                self.write_word(self.registers[rd], value);
            }
            Instruction::StoreFromLowByte { rs, rd } => {
                let value = (self.registers[rs] & 0xff) as u8;
                self.write_byte(self.registers[rd], value);
            }
            Instruction::StoreFromHighByte { rs, rd } => {
                let value = (self.registers[rs] >> 8) as u8;
                self.write_byte(self.registers[rd], value);
            }
            Instruction::JumpToOffset { offset } => {
                self.program_counter = self.program_counter.wrapping_add_signed(offset << 1);
            }
            Instruction::JumpToPointer { rs } => {
                self.program_counter = self.registers[rs];
            }
            Instruction::BranchIfCarry { offset } => {
                if self.flags.carry {
                    self.program_counter = self.program_counter.wrapping_add_signed(offset << 1);
                }
            }
            Instruction::BranchIfNotCarry { offset } => {
                if !self.flags.carry {
                    self.program_counter = self.program_counter.wrapping_add_signed(offset << 1);
                }
            }
            Instruction::BranchIfOverflow { offset } => {
                if self.flags.overflow {
                    self.program_counter = self.program_counter.wrapping_add_signed(offset << 1);
                }
            }
            Instruction::BranchIfNotOverflow { offset } => {
                if !self.flags.overflow {
                    self.program_counter = self.program_counter.wrapping_add_signed(offset << 1);
                }
            }
            Instruction::BranchIfZero { offset } => {
                if self.flags.zero {
                    self.program_counter = self.program_counter.wrapping_add_signed(offset << 1);
                }
            }
            Instruction::BranchIfNotZero { offset } => {
                if !self.flags.zero {
                    self.program_counter = self.program_counter.wrapping_add_signed(offset << 1);
                }
            }
            Instruction::BranchIfSigned { offset } => {
                if self.flags.signed {
                    self.program_counter = self.program_counter.wrapping_add_signed(offset << 1);
                }
            }
            Instruction::BranchIfNotSigned { offset } => {
                if !self.flags.signed {
                    self.program_counter = self.program_counter.wrapping_add_signed(offset << 1);
                }
            }
            Instruction::Halt => self.halted = true,
        }
    }
}

pub trait Memory {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);

    fn read_word(&self, address: u16) -> u16 {
        let low_byte = self.read_byte(address) as u16;
        let high_byte = self.read_byte(address.wrapping_add(1)) as u16;
        (high_byte << 8) | low_byte
    }

    fn write_word(&mut self, address: u16, value: u16) {
        let high_byte = (value >> 8) as u8;
        let low_byte = value as u8;
        self.write_byte(address, low_byte);
        self.write_byte(address.wrapping_add(1), high_byte);
    }
}

impl Memory for Cpu {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

pub fn emulate_binary(binary: Vec<u8>, options: EmulatorOptions) -> Result<()> {
    let mut cpu = Cpu::default();
    for (index, byte) in binary.iter().enumerate() {
        cpu.write_byte(index as u16, *byte);
    }
    cpu.run()?;

    if options.show_registers {
        println!("{:?}", cpu.registers);
    }

    if options.show_flags {
        println!("{:?}", cpu.flags);
    }

    if options.show_memory {
        let range_start = options.memory_start as usize;
        let range_end = options.memory_end as usize;

        let data = &cpu.memory[range_start..range_end];
        for (i, chunk) in data.chunks(16).enumerate() {
            let addr = range_start + i * 16;
            print!("{:04x}: ", addr);

            for byte in chunk {
                match options.memory_format {
                    MemoryFormat::Hex => print!("{:02x} ", byte),
                    MemoryFormat::Dec => print!("{:03} ", byte),
                    MemoryFormat::Bin => print!("{:08b} ", byte),
                };
            }
            println!();
        }
    }

    Ok(())
}

pub fn emulate_file(
    input: String,
    input_format: InputFormat,
    options: EmulatorOptions,
) -> Result<()> {
    match input_format {
        InputFormat::Asm => {
            let binary = assemble_to_binary(input.as_str())?;
            emulate_binary(binary, options)
        }
        InputFormat::Bin => {
            let binary = read(input.as_str())?;
            emulate_binary(binary, options)
        }
    }
}
