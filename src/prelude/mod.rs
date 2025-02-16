pub mod addr;
pub mod elf;
pub mod instruction;
pub mod program;
pub mod parser;
pub mod code_gen;
mod utils;

pub use addr::*;
pub use instruction::*;
pub use program::*;
pub use parser::*;
pub use code_gen::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Register {
    EAX = 0, ECX = 1, EDX = 2, EBX = 3, ESP = 4, EBP = 5, ESI = 6, EDI = 7,
}

impl TryFrom<String> for Register {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "eax" => Ok(Register::EAX),
            "ebx" => Ok(Register::EBX),
            "ecx" => Ok(Register::ECX),
            "esp" => Ok(Register::ESP),
            "ebp" => Ok(Register::EBP),
            "edi" => Ok(Register::EDI),
            "esi" => Ok(Register::ESI),
            "edx" => Ok(Register::EDX),
            _ => Err(format!("unkown register {}", s))
        }
    }
}

pub enum Value {
    UByte(u8),
    UShort(u16),
    UInt(u32),
    ULong(u64),
    Pointer(String),
    RelPointer(String),
}

impl Value {
    pub fn len(&self) -> usize {
        match &self {
            Value::UByte(_) => 1,
            Value::UShort(_) => 2,
            Value::UInt(_) => 4,
            Value::ULong(_) => 8,
            Value::Pointer(_) => 4,
            Value::RelPointer(_) => 4,
        }
    }

    pub fn as_vec(&self, program: &Program, addr: Addr) -> Vec<u8> {
        match &self {
            Value::UByte(x) => vec![*x],
            Value::UShort(x) => utils::dump_word(*x, Endianness::Little).to_vec(),
            Value::UInt(x) => utils::dump_dword(*x, Endianness::Little).to_vec(),
            Value::ULong(x) => utils::dump_qword(*x, Endianness::Little).to_vec(),
            Value::Pointer(label) => {
                let x = program.get_addr(label).unwrap_or_default().vaddr as u32;
                utils::dump_dword(x, Endianness::Little).to_vec()
            }
            Value::RelPointer(label) => {
                let x = program.get_addr(label).unwrap_or_default().addr as i32;
                let delta = x - (addr.addr as i32);
                utils::dump_dword(delta as u32, Endianness::Little).to_vec()
            }
        }
    }
}
