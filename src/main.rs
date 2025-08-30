mod assembler;
mod emulator;

use std::env;
use std::fs::File;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::assembler::assemble;
use crate::emulator::{emulate};

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
        Commands::Assemble { input, output } => assemble(input, output),
        Commands::Emulate { input, format } => {
            match format.as_str() {
                "asm" => {
                    let program_name = input
                        .split('/')
                        .next_back()
                        .unwrap()
                        .strip_suffix(".asm")
                        .unwrap();
                    let tmp_dir = env::temp_dir();
                    let output = tmp_dir.join(format!("{program_name}.bin"));
                    let _ = File::create(&output);
                    let output = output.to_str().unwrap();
                    assemble(input, output)?;
                    emulate(output)
                },
                "bin" => {
                    emulate(input)
                }
                _ => unreachable!()
            }
        }
    }
    
}
