mod assembler;
mod emulator;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::assembler::assemble_to_file;
use crate::emulator::{EmulatorOptions, emulate_file};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Assemble {
        input: String,
        output: String,
    },
    Emulate {
        input: String,
        #[arg(short, long, default_value = "asm", value_parser = ["asm", "bin"])]
        input_format: String,
        #[arg(short, long)]
        step: bool,
        #[arg(short, long)]
        cycles: Option<u64>,
        #[arg(short = 'r', long, default_value_t = false)]
        show_registers: bool,
        #[arg(short = 'f', long, default_value_t = false)]
        show_flags: bool,
        #[arg(short = 'm', long, default_value_t = false)]
        show_memory: bool,
        #[arg(long, default_value_t = 0x0100)]
        memory_start: u16,
        #[arg(long, default_value_t = 0x0200)]
        memory_end: u16,
        #[arg(long, default_value = "hex", value_parser = ["hex", "bin"])]
        memory_format: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Assemble { input, output } => assemble_to_file(input, output),
        Commands::Emulate {
            input,
            input_format,
            step,
            cycles,
            show_registers,
            show_flags,
            show_memory,
            memory_start,
            memory_end,
            memory_format,
        } => emulate_file(
            input,
            input_format,
            EmulatorOptions {
                step,
                cycles,
                show_registers,
                show_flags,
                show_memory,
                memory_start,
                memory_end,
                memory_format,
            },
        ),
    }
}
