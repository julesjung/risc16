use anyhow::{Result, bail};
use customasm::{asm, diagn, util};
use std::fs;

const RISC16_ARCHITECTURE: &str = include_str!("architecture.asm");

pub fn assemble_to_binary(input: &str) -> Result<Vec<u8>> {
    let program = fs::read_to_string(input)?;

    let mut report = diagn::Report::new();
    let mut fileserver = util::FileServerMock::new();
    fileserver.add("architecture.asm", RISC16_ARCHITECTURE);
    fileserver.add(input, program);

    let opts = asm::AssemblyOptions::new();

    let assembly = asm::assemble(
        &mut report,
        &opts,
        &mut fileserver,
        &["architecture.asm", input],
    );

    let result = assembly.output.map(|output| output.format_binary());

    report.print_all(&mut std::io::stderr().lock(), &fileserver, true);

    if let Some(result) = result {
        Ok(result)
    } else {
        bail!("unable to assemble program");
    }
}

pub fn assemble_to_file(input: String, output: String) -> Result<()> {
    let bytes = assemble_to_binary(input.as_str())?;
    fs::write(output, bytes)?;
    Ok(())
}
