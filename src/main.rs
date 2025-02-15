mod prelude;
use prelude::*;

fn main() {
    let message = "Hello World!\n";

    // Construct the blocks 
    let mut start = Block::new();
	start.push(Box::new(MovData::new(Register::EAX, Value::UInt(4))));
	start.push(Box::new(MovData::new(Register::EBX, Value::UInt(1))));
	start.push(Box::new(MovData::new(Register::ECX, Value::Pointer(".data".to_string()))));
	start.push(Box::new(MovData::new(Register::EDX, Value::UInt(message.len() as u32))));
    start.push(Box::new(MovData::new(Register::EDI, Value::UInt(5))));

	start.push(Box::new(Int(0x80)));                                    
                                                                              
	start.push(Box::new(MovData::new(Register::EAX, Value::Int(1))));
	start.push(Box::new(MovData::new(Register::EBX, Value::Int(0))));
	start.push(Box::new(Int(0x80))); 
     
    let mut data_block = Block::new();
    data_block.push(Box::new(RawData(message.as_bytes().to_vec())));

    // Construct the program.
    let mut program = Program::new();
    program.push("_start", start);
    program.push(".data", data_block);

    // Write the ELF binary
    let elf = elf::ELF::new_x86(program);
    elf.save("a.out").expect("failed to save elf binary.");

    println!("Saved to 'a.out'");
}
