use anyhow::{Result, bail};
use customasm::{asm, diagn, util};
use std::fs;

const STD_FILES: &[(&str, &str)] = &[
    (
        "<std>/architecture.asm",
        include_str!("../arch/architecture.asm"),
    ),
    ("<std>/banks.asm", include_str!("../arch/banks.asm")),
    (
        "<std>/instructions.asm",
        include_str!("../arch/instructions.asm"),
    ),
    ("<std>/macros.asm", include_str!("../arch/macros.asm")),
    ("<std>/types.asm", include_str!("../arch/types.asm")),
];

pub fn assemble_to_binary(input: &str) -> Result<Vec<u8>> {
    let program = fs::read_to_string(input)?;

    let mut report = diagn::Report::new();
    let mut fileserver = util::FileServerMock::new();

    fileserver.add_std_files(STD_FILES);
    fileserver.add(input, program);

    let opts = asm::AssemblyOptions::new();

    let assembly = asm::assemble(
        &mut report,
        &opts,
        &mut fileserver,
        &["<std>/architecture.asm", input],
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
