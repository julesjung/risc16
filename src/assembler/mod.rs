use std::fs::File;
use std::io::Write;
use std::env;
use std::process::Command;
use anyhow::{bail, Result};

const RISC16_ARCHITECTURE: &str = include_str!("architecture.asm");

pub fn assemble(input: &str, output: &str) -> Result<()> {
    let tmp_dir = env::temp_dir();
    let architecture = tmp_dir.join("risc16_architecture.asm");
    let mut file = File::create(&architecture).unwrap();
    file.write_all(RISC16_ARCHITECTURE.as_bytes()).unwrap();

    let result = Command::new("customasm")
        .args([architecture.to_str().unwrap(), input, "-o", output, "-f", "binary", "-q"])
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