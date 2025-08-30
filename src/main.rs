mod emulator;

use std::{fs::{self, read}, process::Command};
use anyhow::{bail, Result};
use clap::{Parser, Subcommand};

use crate::emulator::{Cpu, Memory};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Assemble { input: String },
    Emulate { input: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Assemble { input } => assemble(input),
        Commands::Emulate { input } => emulate(input)
    }
    
}

fn assemble(input: String) -> Result<()> {
    fs::create_dir_all("build")?;
    let output = format!("build/{input}.bin");
    let input = format!("programs/{input}.asm");
    let result = Command::new("customasm")
        .args(["assembler/architecture.asm", &input, "-o", &output, "-f", "binary", "-q"])
        .status();

    if let Ok(result) = result {
        if !result.success() {
            bail!("Unable to assemble the program");
        }
        Ok(())
    } else {
        bail!("Unable to run customasm. Make sure it is correctly installed");
    }
}

fn emulate(input: String) -> Result<()> {
    let input = format!("build/{input}.bin");
    let program = read(input)?;

    let mut cpu = Cpu::default();
    for (index, byte) in program.iter().enumerate() {
        cpu.write_byte(index as u16, *byte);
    }
    cpu.run()?;
    println!("{:?}", &cpu.memory[0x0100..0x0110]);
    
    Ok(())
}
