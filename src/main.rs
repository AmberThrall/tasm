mod prelude;
use prelude::*;
use std::fs;

fn main() {
    // Load and parse the code.
    let code: String = fs::read_to_string("hello.s").expect("failed to open file."); 
    let ast = match parse(&code) {
        Ok(node) => node,
        Err(e) => {
            println!("Parsing error occured: {}", e);
            return;
        }
    };

    let program = CodeGenerator::generate(ast);

    // Write the ELF binary
    let elf = elf::ELF::new_x86(program);
    elf.save("a.out").expect("failed to save elf binary.");

    println!("Saved to 'a.out'");
}
