mod assembler;
mod emulator;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::assembler::assemble_to_file;
use crate::emulator::emulate_file;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Assemble {
        input: String,
        output: String
    },
    Emulate {
        input: String,
        #[arg(short, long, default_value = "asm", value_parser = ["asm", "bin"])]
        format: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Assemble { input, output } => assemble_to_file(input, output),
        Commands::Emulate { input, format } => emulate_file(input, format),
    }
    
}
