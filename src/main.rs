mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::utils::parse_u16;
use risc16::assembler::assemble_to_file;
use risc16::emulator::{EmulatorOptions, InputFormat, MemoryFormat, emulate_file};

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
        #[arg(short = 'f', long, default_value = "asm", value_parser = clap::value_parser!(InputFormat))]
        input_format: InputFormat,
        #[arg(short, long)]
        step: bool,
        #[arg(short, long)]
        cycles: Option<u64>,
        #[arg(short = 'r', long, default_value_t = false)]
        show_registers: bool,
        #[arg(short = 'F', long, default_value_t = false)]
        show_flags: bool,
        #[arg(short = 'm', long, default_value_t = false)]
        show_memory: bool,
        #[arg(long, default_value_t = 0x0100, value_parser = parse_u16)]
        memory_start: u16,
        #[arg(long, default_value_t = 0x0110, value_parser = parse_u16)]
        memory_end: u16,
        #[arg(long, default_value = "hex", value_parser = clap::value_parser!(MemoryFormat))]
        memory_format: MemoryFormat,
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
