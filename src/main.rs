mod assembler;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(version, about = "Assembler for the RISC16 CPU", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Assembler {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: String,
    },
}

fn main() -> Result<()> {
   let cli = Cli::parse();

    match &cli.command {
        Commands::Assembler { input, output } => {
            assembler::run(input, output)?;
        }
    }

    Ok(())
}
