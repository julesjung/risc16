use std::fs::read;

use anyhow::{bail, Result};
use clap::Parser;

#[derive(Parser)]
struct Args {
    input: String
}

enum Instruction {
    Add { destination_register: usize, source_register_a: usize, source_register_b: usize },
    LoadImmediate { destination_register: usize, upper_byte: bool, immediate: u16 },
    Halt,
}

struct Cpu {
    memory: [u8; 65536],
    registers: [u16; 8],
    program_counter: u16,
    halted: bool,
}

impl Cpu {
    fn new() -> Self {
        Self { memory: [0; 65536], registers: [0; 8], program_counter: 0, halted: false }
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
            0x3 => Ok(Instruction::LoadImmediate {
                destination_register: ((instruction >> 9) & 0x7) as usize,
                upper_byte: (instruction & 0x0100) != 0,
                immediate: instruction & 0x00ff,
            }),
            0xf => Ok(Instruction::Halt),
            _ => bail!("Unknown instruction: {instruction:0x}")
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add { destination_register, source_register_a, source_register_b } => {
                self.registers[destination_register] = self.registers[source_register_a].wrapping_add(self.registers[source_register_b])
            },
            Instruction::LoadImmediate { destination_register, upper_byte, immediate } => {
                self.registers[destination_register] = if upper_byte {
                    immediate << 8
                } else {
                    immediate
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
    println!("{:?}", cpu.registers);
    Ok(())
}

