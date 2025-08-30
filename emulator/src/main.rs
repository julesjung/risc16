use std::{fs::read};

use anyhow::{bail, Result};
use clap::Parser;

#[derive(Parser)]
struct Args {
    input: String
}

enum Instruction {
    Add { destination_register: usize, source_register_a: usize, source_register_b: usize },
    AddImmediate { destination_register: usize, immediate: u16 },
    SubtractImmediate { destination_register: usize, immediate: u16},
    Compare { source_register_a: usize, source_register_b: usize },
    MoveImmediate { destination_register: usize, upper_byte: bool, immediate: u16 },
    Load { destination_register: usize, source_register: usize, upper_byte: bool },
    Store { source_register: usize, destination_register: usize, upper_byte: bool },
    JumpWithOffset { offset: i16 },
    BranchIfNotEqual { immediate: i16 },
    BranchIfCarry { immediate: i16 },
    BranchIfSigned { immediate: i16 },
    Halt,
}

struct Flags {
    carry: bool,
    overflow: bool,
    zero: bool,
    signed: bool
}

impl Flags {
    fn new() -> Self {
        Self { carry: false, zero: false, signed: false }
    }
}

struct Cpu {
    memory: [u8; 65536],
    registers: [u16; 8],
    flags: Flags,
    program_counter: u16,
    halted: bool,
}

impl Cpu {
    fn new() -> Self {
        Self { memory: [0; 65536], registers: [0; 8], flags: Flags::new(), program_counter: 0, halted: false }
    }

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
            0x0 => match instruction & 0x7 {
                0x0000 => Ok(Instruction::Add {
                    destination_register: ((instruction >> 9) & 0x7) as usize,
                    source_register_a: ((instruction >> 6) & 0x7) as usize,
                    source_register_b: ((instruction >> 3) & 0x7) as usize
                }),
                _ => bail!("Unknown instruction: {instruction:0x}")
            },
            0x2 => match (instruction & 0x0100) != 0 {
                false => Ok(Instruction::AddImmediate {
                    destination_register: ((instruction >> 9) & 0x7) as usize,
                    immediate: instruction & 0x00ff
                }),
                true => Ok(Instruction::SubtractImmediate {
                    destination_register: ((instruction >> 9) & 0x7) as usize,
                    immediate: instruction & 0x00ff
                })
            },
            0x3 => Ok(Instruction::Compare {
                source_register_a: ((instruction >> 9) & 0x7) as usize,
                source_register_b: ((instruction >> 6) & 0x7) as usize
            }),
            0x4 => Ok(Instruction::MoveImmediate {
                destination_register: ((instruction >> 9) & 0x7) as usize,
                upper_byte: (instruction & 0x0100) != 0,
                immediate: instruction & 0x00ff,
            }),
            0x5 => Ok(Instruction::Load {
                destination_register: ((instruction >> 9) & 0x7) as usize,
                source_register: ((instruction >> 5) & 0x7) as usize,
                upper_byte: (instruction & 0x0100) != 0
            }),
            0x6 => Ok(Instruction::Store {
                source_register: ((instruction >> 9) & 0x7) as usize,
                destination_register: ((instruction >> 5) & 0x7) as usize,
                upper_byte: (instruction & 0x0100) != 0
            }),
            0x7 => Ok(Instruction::JumpWithOffset {
                offset: ((instruction << 4) as i16) >> 4
            }),
            0xa => match (instruction >> 9) & 0x7 {
                0x1 => {
                    Ok(Instruction::BranchIfNotEqual {
                        immediate: ((instruction << 7) as i16) >> 7
                    })
                },
                0x2 => {
                    Ok(Instruction::BranchIfCarry {
                        immediate: ((instruction << 7) as i16) >> 7
                    })
                },
                0x4 => {
                    Ok(Instruction::BranchIfSigned {
                        immediate: ((instruction << 7) as i16) >> 7
                    })
                }
                _ => bail!("Unknown instruction: {instruction:0x}")
            },
            0xf => Ok(Instruction::Halt),
            _ => bail!("Unknown instruction: {instruction:0x}")
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add { destination_register, source_register_a, source_register_b } => {
                (self.registers[destination_register], self.flags.carry) =
                    self.registers[source_register_a].overflowing_add(self.registers[source_register_b]);
            },
            Instruction::AddImmediate { destination_register, immediate } => {
                (self.registers[destination_register], self.flags.carry) =
                    self.registers[destination_register].overflowing_add(immediate);
            },
            Instruction::SubtractImmediate { destination_register, immediate } => {
                (self.registers[destination_register], self.flags.carry) =
                    self.registers[destination_register].overflowing_sub(immediate);
                self.flags.zero = self.registers[destination_register] == 0;
            },
            Instruction::Compare { source_register_a, source_register_b } => {
                let result: i16 = self.registers[source_register_a].wrapping_sub(self.registers[source_register_b]) as i16;
                self.flags.zero = result == 0;
                self.flags.signed = result < 0;
            },
            Instruction::MoveImmediate { destination_register, upper_byte, immediate } => {
                self.registers[destination_register] = if upper_byte {
                    (self.registers[destination_register] & 0x00FF) | immediate << 8
                } else {
                    (self.registers[destination_register] & 0xFF00) | immediate
                }
            },
            Instruction::Load { destination_register, source_register, upper_byte } => {
                let value = self.read_byte(self.registers[source_register]) as u16;
                self.registers[destination_register] = value;
            },
            Instruction::Store { source_register, destination_register, upper_byte } => {
                let value = if upper_byte {
                    (self.registers[source_register] >> 8) as u8
                } else {
                    (self.registers[source_register] & 0xff) as u8
                };
                self.write_byte(self.registers[destination_register], value);
            },
            Instruction::JumpWithOffset { offset } => {
                self.program_counter = self.program_counter.wrapping_add_signed(offset << 1);
            },
            Instruction::BranchIfNotEqual { immediate } => {
                if !self.flags.zero {
                    self.program_counter = self.program_counter.wrapping_add_signed(immediate << 1);
                }
            },
            Instruction::BranchIfCarry { immediate } => {
                if self.flags.carry {
                    self.program_counter = self.program_counter.wrapping_add_signed(immediate << 1);
                }
            },
            Instruction::BranchIfSigned { immediate } => {
                if self.flags.signed {
                    self.program_counter = self.program_counter.wrapping_add_signed(immediate << 1);
                }
            },
            Instruction::Halt => self.halted = true,
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
    let mut cpu = Cpu::new();
    let args = Args::parse();
    let program = read(args.input)?;
    for (index, byte) in program.iter().enumerate() {
        cpu.write_byte(index as u16, *byte);
    }
    cpu.run()?;
    println!("{:?}", &cpu.memory[0x0100..0x0110]);
    Ok(())
}

