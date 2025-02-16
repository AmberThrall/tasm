use super::{Register, Value, Program, Addr};

/// Jump conditionals
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum JumpCondition {
    None,
    Overflow    = 0x80,
    NotOverflow = 0x81,
    Carry       = 0x82,
    NotCarry    = 0x83,
    Zero        = 0x84,
    NotZero     = 0x85,
    CarryOrZero = 0x86,
    NotCarryAndNotZero = 0x87,
    Sign        = 0x88,
    NotSign     = 0x89,
    Parity      = 0x8A,
    NotParity   = 0x8B,
    Less        = 0x8C,
    NotLess     = 0x8D,
    NotGreater  = 0x8E,
    Greater     = 0x8F,
}

pub enum Instruction {
    RawData(Vec<u8>),
    Int(u8),
    MovImmediate { register: Register, value: Value },
    Inc(Register),
    Dec(Register),
    Jump { condition: JumpCondition, addr: Value }
}

impl Instruction {
    /// Get the length of the instruction in bytes.
    pub fn len(&self) -> usize {
        match self {
            Self::RawData(x) => x.len(),
            Self::Int(_) => 2,
            Self::MovImmediate { register, value } => 5,
            Self::Inc(_) => 1, 
            Self::Dec(_) => 1, 
            Self::Jump { condition, addr } => { if *condition == JumpCondition::None { 5 } else { 6 } },
        }
    }
}

impl Program {
    pub fn encode_instruction(&self, instr: &Instruction, cur_addr: Addr) -> Vec<u8> {
        let mut data = Vec::new();

        match instr {
            Instruction::RawData(x) => data.extend_from_slice(x),
            Instruction::Int(x) => {
                data.push(0xCD);
                data.push(*x);
            }
            Instruction::MovImmediate { register, value } => {
                data.push(match register {
                    Register::EAX => 0xB8,
                    Register::ECX => 0xB9,
                    Register::EDX => 0xBA,
                    Register::EBX => 0xBB,
                    Register::ESP => 0xBC,
                    Register::EBP => 0xBD,
                    Register::ESI => 0xBE,
                    Register::EDI => 0xBF,
                });
                data.extend_from_slice(&value.as_vec(&self, cur_addr));
            }
            Instruction::Inc(register) => {
                data.push(match register {
                    Register::EAX => 0x40,
                    Register::ECX => 0x41,
                    Register::EDX => 0x42,
                    Register::EBX => 0x43,
                    Register::ESP => 0x44,
                    Register::EBP => 0x45,
                    Register::ESI => 0x46,
                    Register::EDI => 0x47,
                });
            }
            Instruction::Dec(register) => {
                data.push(match register {
                    Register::EAX => 0x48,
                    Register::ECX => 0x49,
                    Register::EDX => 0x4A,
                    Register::EBX => 0x4B,
                    Register::ESP => 0x4C,
                    Register::EBP => 0x4D,
                    Register::ESI => 0x4E,
                    Register::EDI => 0x4F,
                });
            }
            Instruction::Jump { condition, addr } => {
                match condition {
                    JumpCondition::None => { data.push(0xE9); },
                    _ => {
                        data.push(0x0F);
                        data.push(*condition as u8);
                    }
                }
                let addr_delta = addr.as_vec(&self, cur_addr);
                data.extend_from_slice(&addr_delta);
            }
        }

        data
    }
}
