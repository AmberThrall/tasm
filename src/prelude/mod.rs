pub mod addr;
pub mod elf;
pub mod instruction;
pub mod program;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod code_gen;
mod utils;

pub use addr::*;
pub use instruction::*;
pub use program::*;
pub use lexer::*;
pub use ast::*;
pub use parser::*;
pub use code_gen::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Register {
    AH, AL, BH, BL, CH, CL,  DH, DL,
    AX, CX, DX, BX, SP, BP, SI, DI,
    EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI,
}

impl Register {
    pub fn bits(&self) -> usize {
        use Register::*;
        match &self {
            AL | CL | DL | BL | AH | CH | DH | BH => 8,
            AX | CX | DX | BX | SP | BP | SI | DI => 16,
            EAX | ECX | EDX | EBX | ESP | EBP | ESI | EDI => 32,
        }
    }

    pub fn offset(&self) -> u8 {
        use Register::*;
        match self {
            AL | AX | EAX => 0,
            CL | CX | ECX => 1,
            DL | DX | EDX => 2,
            BL | BX | EBX => 3,
            AH | SP | ESP => 4,
            CH | BP | EBP => 5,
            DH | SI | ESI => 6,
            BH | DI | EDI => 7,
        }
    }
}

impl TryFrom<String> for Register {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "ah" => Ok(Register::AH),
            "al" => Ok(Register::AL),
            "bh" => Ok(Register::BH),
            "bl" => Ok(Register::BL),
            "ch" => Ok(Register::CH),
            "cl" => Ok(Register::CL),
            "dh" => Ok(Register::DH),
            "dl" => Ok(Register::DL),
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
