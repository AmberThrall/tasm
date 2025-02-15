mod prelude;
use prelude::*;

fn main() {
    let mut elf = elf::ELF::new_x86();
    elf.push_instruction(&vec![0xB8, 0x04, 0x00, 0x00, 0x00]);  // eax <- 4 (write)
    elf.push_instruction(&vec![0xBB, 0x01, 0x00, 0x00, 0x00]);  // ebx <- 1 (stdout)
    elf.push_instruction(&vec![0xB9, 0x76, 0x80, 0x04, 0x08]);  // ecx <- buf
    elf.push_instruction(&vec![0xBA, 0x0C, 0x00, 0x00, 0x00]);  // edx <- count 
    elf.push_instruction(&vec![0xCD, 0x80]);                    // int 80 (syscall)

    elf.push_instruction(&vec![0xB8, 0x01, 0x00, 0x00, 0x00]);  // eax <- 1 (exit)
    elf.push_instruction(&vec![0xBB, 0x00, 0x00, 0x00, 0x00]);  // ebx <- 0 (param)
    elf.push_instruction(&vec![0xCD, 0x80]);                    // int 80 (syscall)
    
    elf.push_instruction(b"HELLO WORLD\n");

    elf.save("a.out").expect("failed to save elf binary.");
}
