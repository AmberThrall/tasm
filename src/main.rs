mod prelude;
use prelude::*;

fn main() {
    let mut elf = elf::ELF::new_x86();
    elf.push_instruction(&vec![0xB8, 0x01, 0x00, 0x00, 0x00]);  // eax <- 1 (exit)
    elf.push_instruction(&vec![0xBB, 0x07, 0x00, 0x00, 0x00]);  // ebx <- 0 (param)
    elf.push_instruction(&vec![0xCD, 0x80]);                    // int 80 (syscall)

    elf.save("a.out").expect("failed to save elf binary.");
}
