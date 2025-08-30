use std::{fs::read};

use anyhow::{bail, Result};
use clap::Parser;

#[derive(Parser)]
struct Args {
    input: String
}

enum Instruction {
    Add { rd: usize, ra: usize, rb: usize },
    AddWithCarry { rd: usize, ra: usize, rb: usize },
    Subtract { rd: usize, ra: usize, rb: usize },
    SubtractWithBorrow { rd: usize, ra: usize, rb: usize },
    And { rd: usize, ra: usize, rb: usize },
    Or { rd: usize, ra: usize, rb: usize },
    Xor { rd: usize, ra: usize, rb: usize },
    Not { rd: usize, ra: usize },
    Halt,
}

#[derive(Default)]
struct Flags {
    carry: bool,
    overflow: bool,
    zero: bool,
    signed: bool
}

struct Cpu {
    memory: [u8; 65536],
    registers: [u16; 8],
    flags: Flags,
    program_counter: u16,
    halted: bool,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu { memory: [0; 65536], registers: [0; 8], flags: Flags::default(), program_counter: 0, halted: false }
    }
}

impl Cpu {
    fn run(&mut self) -> Result<()> {
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
                    _ => unreachable!()
                }
            },
            0xf => Ok(Instruction::Halt),
            _ => bail!("Unknown instruction: 0x{instruction:04x}")
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
            },
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
            },
            Instruction::Subtract { rd,ra, rb } => {
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
            },
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
            },
            Instruction::And { rd, ra, rb } => {
                let ra = self.registers[ra];
                let rb = self.registers[rb];
                let result = ra & rb;
                self.registers[rd] = result;
                self.flags.carry = false;
                self.flags.overflow = false;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            },
            Instruction::Or { rd, ra, rb } => {
                let ra = self.registers[ra];
                let rb = self.registers[rb];
                let result = ra | rb;
                self.registers[rd] = result;
                self.flags.carry = false;
                self.flags.overflow = false;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            },
            Instruction::Xor { rd, ra, rb } => {
                let ra = self.registers[ra];
                let rb = self.registers[rb];
                let result = ra ^ rb;
                self.registers[rd] = result;
                self.flags.carry = false;
                self.flags.overflow = false;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            },
            Instruction::Not { rd, ra } => {
                let result = !self.registers[ra];
                self.registers[rd] = result;
                self.flags.carry = false;
                self.flags.overflow = false;
                self.flags.zero = result == 0;
                self.flags.signed = (result & 0x8000) != 0;
            },
            Instruction::Halt => self.halted = true
        }
    }
}

trait Memory {
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

fn main() -> Result<()> {
    let mut cpu = Cpu::default();
    let args = Args::parse();
    let program = read(args.input)?;
    for (index, byte) in program.iter().enumerate() {
        cpu.write_byte(index as u16, *byte);
    }
    cpu.run()?;
    println!("{:?}", &cpu.memory[0x0100..0x0110]);
    Ok(())
}

