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

    /// Print the abstract syntax tree to the terminal
    #[arg(long, default_value_t = false)]
    print_ast: bool,
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
            std::process::exit(1);
        }
    };

    // Print ast
    if args.print_ast {
        println!("Abstract Syntax Tree:");
        ast.print();
    }

    // Generate the code
    let program = CodeGenerator::generate(&ast);

    // Write the ELF binary
    let elf = elf::ELF::new_x86(program);
    elf.save(args.output.clone()).expect("failed to save elf binary.");

    // Set the permissions
    fs::set_permissions(args.output, fs::Permissions::from_mode(0o755)).expect("failed to set permissions.");
}
