pub mod addr;
pub mod elf;
pub mod instr;
pub mod block;
pub mod program;
mod utils;

pub use addr::*;
pub use instr::*;
pub use block::*;
pub use program::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Register {
    EAX, EBX, ECX, EDX,
}

pub enum Value {
    Byte(u8),
    Short(u16),
    Int(u32),
    Long(u64),
    Pointer(String),
}

impl Value {
    pub fn len(&self) -> usize {
        match &self {
            Value::Byte(_) => 1,
            Value::Short(_) => 2,
            Value::Int(_) => 4,
            Value::Long(_) => 8,
            Value::Pointer(_) => 4,
        }
    }

    pub fn as_vec(&self, program: &Program) -> Vec<u8> {
        match &self {
            Value::Byte(x) => vec![*x],
            Value::Short(x) => utils::dump_word(*x, Endianness::Little).to_vec(),
            Value::Int(x) => utils::dump_dword(*x, Endianness::Little).to_vec(),
            Value::Long(x) => utils::dump_qword(*x, Endianness::Little).to_vec(),
            Value::Pointer(label) => {
                let addr = program.get_addr(label).unwrap_or_default().vaddr as u32;
                utils::dump_dword(addr, Endianness::Little).to_vec()
            }
        }
    }
}
