use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Write};
use anyhow::{bail, ensure, Context, Result};

static INSTRUCTIONS: &[Instruction] = &[
    Instruction { mnemonic: "LI", format: Format::ImmediateToRegister, opcode: 0x1000 },
];

#[repr(u8)]
#[derive(Debug)]
enum Argument {
    Register(u16),
    RegisterByte(u16, bool),
    Immediate(u16),
}

#[derive(Debug)]
enum Format {
    RegisterToRegister,
    MemoryToRegister,
    RegisterToMemory,
    ImmediateToRegister,
    ToRegister,
    ToMemory,
    Nullary
}

#[derive(Debug)]
struct Instruction {
    mnemonic: &'static str,
    format: Format,
    opcode: u16,
}

pub fn run(input_file: &str, output_file: &str) -> Result<()> {
    let input = BufReader::new(File::open(input_file)
        .with_context(|| format!("Failed to open input file `{input_file}`"))?
    );

    let mut binary: Vec<u16> = Vec::new();

    for (line_index, line) in input.lines().enumerate() {
        let line = line.with_context(|| format!("Failed to read line {}", line_index + 1))?;

        let mut token: String = String::new();
        let mut mnemonic: String = String::new();
        let mut arguments: Vec<Argument> = Vec::new();

        for character in line.chars() {
            if character == ';' {
                break;
            } else if character == ' ' || character == ',' {
                if !token.is_empty() {
                    push_token(&mut mnemonic, &mut arguments, token.clone(), line_index)?;
                }
                token.clear();
            } else if character.is_ascii() {
                token.push(character.to_ascii_uppercase());
            } else {
                bail!("Found unicode character `{}` on line {}", character, line_index + 1)
            }
        }

        if !token.is_empty() {
            push_token(&mut mnemonic, &mut arguments, token, line_index)?;
        }

        if mnemonic.is_empty() {
            continue;
        }

        binary.push(encode(&mnemonic, &arguments, line_index)?);
    }

    let mut output = BufWriter::new(File::create(output_file)
        .with_context(|| format!("Failed to create output file `{output_file}`"))?);

    for word in &binary {
        output.write_all(&word.to_le_bytes())?;
    }

    Ok(())
}

fn push_token(mnemonic: &mut String, arguments: &mut Vec<Argument>, token: String, line_index: usize) -> Result<()> {
    if mnemonic.is_empty() {
        *mnemonic = token;
    } else if let Some(identifier) = token.strip_prefix("R") {
        let identifier: u16 = identifier.parse()
            .with_context(|| format!("Invalid register identifier `{}` on line {}", token, line_index + 1))?;
        ensure!(identifier < 8, "Invalid register identifier `{}` on line {}", token, line_index + 1);
        arguments.push(Argument::Register(identifier))
    } else if let Some(identifier) = token.strip_prefix("L") {
        let identifier: u16 = identifier.parse()
            .with_context(|| format!("Invalid register identifier `{}` on line {}", token, line_index + 1))?;
        ensure!(identifier < 8, "Invalid register identifier `{}` on line {}", token, line_index + 1);
        arguments.push(Argument::RegisterByte(identifier, false))
    } else if let Some(identifier) = token.strip_prefix("H") {
        let identifier: u16 = identifier.parse()
            .with_context(|| format!("Invalid register identifier `{}` on line {}", token, line_index + 1))?;
        ensure!(identifier < 8, "Invalid register identifier `{}` on line {}", token, line_index + 1);
        arguments.push(Argument::RegisterByte(identifier, true))
    } else if let Some(value) = token.strip_prefix("#") {
        let value: u16 = value.parse()
            .with_context(|| format!("Invalid immediate `{}` on line {}", token, line_index + 1))?;
        arguments.push(Argument::Immediate(value))
    } else {
        bail!("Unrecognised token `{}` on line {}", token, line_index + 1);
    }

    Ok(())
}

fn find_instruction(mnemonic: &str) -> Option<&'static Instruction> {
    INSTRUCTIONS.iter().find(|i| i.mnemonic == mnemonic)
}

fn encode(mnemonic: &String, arguments: &Vec<Argument>, line_index: usize) -> Result<u16> {
    let instruction = find_instruction(mnemonic.as_str())
        .with_context(|| format!("Unknown instruction `{}` on line {}", mnemonic, line_index + 1))?;

    let mut result = instruction.opcode;

    match instruction.format {
        Format::ImmediateToRegister => {
        let Argument::RegisterByte(rd, sel) = arguments[0] else {
            bail!("Argument 1 should be a register byte on line {}", line_index + 1);
        };
        let Argument::Immediate(imm) = arguments[1] else {
            bail!("Argument 2 should be a register on line {}", line_index + 1);
        };

        result += rd << 9;
        result += (sel as u16) << 8;
        result += imm;

        Ok(result)
        },
        _ => bail!("Format not yet implemented"),
    }
}