mod prelude;
use prelude::*;

fn main() {
    let message = "Hello World!\n";

    // Construct the program.
    let mut program = Program::new();

    let data_block = program.new_block(".data");
    data_block.push(Box::new(RawData(message.as_bytes().to_vec())));

    let start = program.new_block("_main");
	start.push(Box::new(MovData::new(Register::EBX, Value::UInt(1))));
	start.push(Box::new(MovData::new(Register::ECX, Value::Pointer(".data".to_string()))));
	start.push(Box::new(MovData::new(Register::EDX, Value::UInt(message.len() as u32))));
    start.push(Box::new(MovData::new(Register::EDI, Value::UInt(5))));

    let loop_blk = program.new_block("_loop");
	loop_blk.push(Box::new(MovData::new(Register::EAX, Value::UInt(4))));
	loop_blk.push(Box::new(Int(0x80)));                                    
    loop_blk.push(Box::new(Dec(Register::EDI)));
    loop_blk.push(Box::new(JMPData(JumpConditional::NotZero, Value::RelPointer("_loop".to_string()))));
                                                                              
    let exit = program.new_block("_exit");
	exit.push(Box::new(MovData::new(Register::EAX, Value::UInt(1))));
	exit.push(Box::new(MovData::new(Register::EBX, Value::UInt(0))));
	exit.push(Box::new(Int(0x80))); 

    // Set the entry point
    program.set_entrypoint("_main");

    // Write the ELF binary
    let elf = elf::ELF::new_x86(program);
    elf.save("a.out").expect("failed to save elf binary.");

    println!("Saved to 'a.out'");
}
