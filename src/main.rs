mod prelude;
use prelude::*;
use clap::Parser;
use std::path::PathBuf;
use std::fs;
use std::os::unix::fs::PermissionsExt;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file
    input: PathBuf,

    /// Output file
    #[arg(short, long, default_value = "a.out")]
    output: PathBuf,
}

fn main() {
    // Handle the arguments
    let args = Args::parse();

    // Load and parse the code.
    let code: String = fs::read_to_string(args.input).expect("failed to open file."); 
    let ast = match prelude::Parser::parse(&code) {
        Ok(node) => node,
        Err(e) => {
            println!("Error on line {}: {}", e.line_no, e.message);
            return;
        }
    };

    // Generate the code
    let program = CodeGenerator::generate(&ast);

    // Write the ELF binary
    let elf = elf::ELF::new_x86(program);
    elf.save(args.output.clone()).expect("failed to save elf binary.");

    // Set the permissions
    fs::set_permissions(args.output, fs::Permissions::from_mode(0o755)).expect("failed to set permissions.");
}
