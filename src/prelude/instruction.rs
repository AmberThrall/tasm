use std::fmt::Debug;

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
    Mov(Register, Register),
    MovImmediate { register: Register, value: Value },
    MovRM32R32 { dest: Register, src: Register },
    MovMemory { addr: Value, register: Register },
    Inc(Register),
    Dec(Register),
    Jump { condition: JumpCondition, addr: Value },
    AddImmediate { register: Register, value: Value },
    SubImmediate { register: Register, value: Value },
    ByteSwap(Register),
    And(Register, Register),
    Or(Register, Register),
    XOr(Register, Register),
}

impl Instruction {
    /// Get the length of the instruction in bytes.
    pub fn len(&self) -> usize {
        match self {
            Self::RawData(x) => x.len(),
            Self::Int(_) => 2,
            Self::Mov(_, _) => 2,
            Self::MovImmediate { register, value } => if register.bits() == 32 { 5 } else { 2 },
            Self::MovRM32R32 { dest, src } => if *dest == Register::ESP || *dest == Register::EBP { 3 } else { 2 },
            Self::MovMemory { addr, register } => { if *register == Register::EAX { 6 } else { 7 } },
            Self::Inc(_) => 1, 
            Self::Dec(_) => 1, 
            Self::Jump { condition, addr } => { if *condition == JumpCondition::None { 5 } else { 6 } },
            Self::AddImmediate { register, value } => { 
                let data_len = if register.bits() == 32 { 4 } else { 1 };
                if *register == Register::EAX { 1 + data_len } else { 2 + data_len } 
            },
            Self::SubImmediate { register, value } => { 
                let data_len = if register.bits() == 32 { 4 } else { 1 };
                if *register == Register::EAX { 1 + data_len } else { 2 + data_len } 
            },
            Self::ByteSwap(_) => 2, 
            Self::And(_, _) => 2,
            Self::Or(_, _) => 2,
            Self::XOr(_, _) => 2,
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
            Instruction::Mov(dest, src) => {
                // See table 2-2 of intel manual
                data.push(if dest.bits() == 32 { 0x89 } else { 0x88 });
                let op = src.offset();
                let rm = dest.offset();
                data.push(0b11000000 | (op << 3) | rm);
            }
            Instruction::MovImmediate { register, value } => {
                let start = match register.bits() {
                    8 => 0xB0,
                    32 => 0xB8,
                    _ => panic!("unsupported."),
                };
                data.push(start + register.offset());
                data.extend_from_slice(&value.as_vec(&self, cur_addr));
            }
            Instruction::MovRM32R32 { dest, src } => {
                // See table 2-2 of intel manual
                data.push(0x89);
                let op = src.offset();
                let rm = dest.offset();
                match dest {
                    Register::ESP => { data.push((rm << 3) | op); data.push(0x24); }
                    Register::EBP => { data.push(0b01000000 | (rm << 3) | op); data.push(0x00); }
                    Register::EAX | Register::ECX | Register::EDX | Register::EBX | Register::ESI 
                        | Register::EDI => data.push((op << 3) | rm),
                    _ => panic!("unsupported register."),
                }
            }
            Instruction::MovMemory { addr, register } => {
                if *register != Register::EAX { data.push(0x89); }

                data.push(match register {
                    Register::EAX => 0xA3,
                    Register::ECX => 0x0D,
                    Register::EDX => 0x15,
                    Register::EBX => 0x1D,
                    Register::ESP => 0x25,
                    Register::EBP => 0x2D,
                    Register::ESI => 0x35,
                    Register::EDI => 0x3D,
                    _ => panic!("unsupported register."),
                });
                data.extend_from_slice(&addr.as_vec(&self, cur_addr));

            }
            Instruction::Inc(register) => {
                data.push(0x40 + register.offset());
            }
            Instruction::Dec(register) => {
                data.push(0x48 + register.offset());
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
            Instruction::AddImmediate { register, value } => {
                match register {
                    Register::AL | Register::EAX => (),
                    _ => data.push(if register.bits() == 32 { 0x81 } else { 0x80 }),
                }

                data.push(match register {
                    Register::AL => 0x04,
                    Register::EAX => 0x05,
                    _ => 0xC0 + register.offset(),
                });
                data.extend_from_slice(&value.as_vec(&self, cur_addr));
            }
            Instruction::SubImmediate { register, value } => {
                match register {
                    Register::AL | Register::EAX => (),
                    _ => data.push(if register.bits() == 32 { 0x81 } else { 0x80 }),
                }

                data.push(match register {
                    Register::AL => 0x2C,
                    Register::EAX => 0x2D,
                    _ => 0xE8 + register.offset(),
                });
                data.extend_from_slice(&value.as_vec(&self, cur_addr));
            }
            Instruction::ByteSwap(register)  => {
                data.push(0x0f);
                data.push(0xC8 + register.offset());
            }
            Instruction::And(dest, src) => {
                // See table 2-2 of intel manual
                data.push(0x21);
                let op = src.offset();
                let rm = dest.offset();
                data.push(0b11000000 | (op << 3) | rm);
            }
            Instruction::Or(dest, src) => {
                // See table 2-2 of intel manual
                data.push(0x09);
                let op = src.offset();
                let rm = dest.offset();
                data.push(0b11000000 | (op << 3) | rm);
            }
            Instruction::XOr(dest, src) => {
                // See table 2-2 of intel manual
                data.push(0x31);
                let op = src.offset();
                let rm = dest.offset();
                data.push(0b11000000 | (op << 3) | rm);
            }
        }

        data
    }
}
