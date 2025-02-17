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
    MovMemoryReg { dest: Register, src: Register },
    MovMemory { addr: Value, register: Register },
    MovFromMemory(Register, Value),
    MovFromMemoryReg(Register, Register),
    Inc(Register),
    Dec(Register),
    Jump { condition: JumpCondition, addr: Value },
    Add(Register, Register),
    AddImmediate { register: Register, value: Value },
    Sub(Register, Register),
    SubImmediate { register: Register, value: Value },
    Multiply(Register),
    Divide(Register),
    ByteSwap(Register),
    And(Register, Register),
    Or(Register, Register),
    XOr(Register, Register),
    Compare(Register, Register),
    CompareImmediate(Register, Value),
    Push(Register),
    Pop(Register),
    Call(Value),
    CallRegister(Register),
    Return,
}

impl Instruction {
    /// Get the length of the instruction in bytes.
    pub fn len(&self) -> usize {
        match self {
            Self::RawData(x) => x.len(),
            Self::Int(_) => 2,
            Self::Mov(dest, _) => if dest.bits() == 16 { 3 } else { 2 },
            Self::MovImmediate { register, value } => match register.bits() {
                8 => 2,
                16 => 4,
                32 => 5,
                _ => 0,
            }
            Self::MovMemoryReg { dest, src } => {
                let offset = if src.bits() == 16 { 1 } else { 0 };
                if *dest == Register::ESP || *dest == Register::EBP { 3 + offset } else { 2 + offset }
            },
            Self::MovMemory { addr, register } => {
                let mut offset = if register.bits() == 16 { 1 } else { 0 };
                if *register == Register::AL || *register == Register::AX || *register == Register::EAX { offset + 5 } else { offset + 6 }
            },
            Self::MovFromMemory(register, addr) => {
                match *register {
                    Register::AL | Register::EAX => 5,
                    _ => 6,
                }
            }
            Self::MovFromMemoryReg(dest, _) => if dest.bits() == 16 { 3 } else { 2 },
            Self::Inc(r) => if r.bits() == 16 { 2 } else { 1 }, 
            Self::Dec(r) => if r.bits() == 16 { 2 } else { 1 }, 
            Self::Jump { condition, addr } => { if *condition == JumpCondition::None { 5 } else { 6 } },
            Self::Add(dest, _) => if dest.bits() == 16 { 3 } else { 2 },
            Self::AddImmediate { register, value } => { 
                let data_len = register.bits() / 8;
                if *register == Register::AL || *register == Register::EAX { 1 + data_len } else { 2 + data_len } 
            },
            Self::Sub(dest, _) => if dest.bits() == 16 { 3 } else { 2 },
            Self::SubImmediate { register, value } => { 
                let data_len = register.bits() / 8;
                if *register == Register::AL || *register == Register::EAX { 1 + data_len } else { 2 + data_len } 
            },
            Self::Multiply(r) => if r.bits() == 16 { 3 } else { 2 },
            Self::Divide(r) => if r.bits() == 16 { 3 } else { 2 },
            Self::ByteSwap(_) => 2, 
            Self::And(r, _) => if r.bits() == 16 { 3 } else { 2 },
            Self::Or(r, _) => if r.bits() == 16 { 3 } else { 2 },
            Self::XOr(r, _) => if r.bits() == 16 { 3 } else { 2 },
            Self::Compare(r, _) => if r.bits() == 16 { 3 } else { 2 },
            Self::CompareImmediate(register,_) => { 
                let data_len = register.bits() / 8;
                if *register == Register::AL || *register == Register::EAX { 1 + data_len } else { 2 + data_len } 
            },
            Self::Push(r) => if r.bits() == 16 { 2 } else { 1 },
            Self::Pop(r) => if r.bits() == 16 { 2 } else { 1 },
            Self::Call(_) => 5,
            Self::CallRegister(r) => if r.bits() == 16 { 2 } else { 1 },
            Self::Return => 1,
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
                if dest.bits() == 16 { data.push(0x66); }
                data.push(if dest.bits() == 8 { 0x88 } else { 0x89 });
                let op = src.offset();
                let rm = dest.offset();
                data.push(0b11000000 | (op << 3) | rm);
            }
            Instruction::MovImmediate { register, value } => {
                if register.bits() == 16 { data.push(0x66); }
                let start = match register.bits() {
                    8 => 0xB0,
                    16 | 32 => 0xB8,
                    _ => panic!("unsupported."),
                };
                data.push(start + register.offset());
                data.extend_from_slice(&value.as_vec(&self, cur_addr));
            }
            Instruction::MovMemoryReg { dest, src } => {
                // See table 2-2 of intel manual
                if src.bits() == 16 { data.push(0x66); }
                data.push(if src.bits() == 8 { 0x88 } else { 0x89 });

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
                if register.bits() == 16 { data.push(0x66); }
                match *register {
                    Register::AL | Register::AX | Register::EAX => (),
                    _ => if register.bits() == 8 { data.push(0x88) } else { data.push(0x89) },
                }

                data.push(match register {
                    Register::AL => 0xA2,
                    Register::AX | Register::EAX => 0xA3,
                    _ => 0x05 + register.offset() * 8,
                });
                data.extend_from_slice(&addr.as_vec(&self, cur_addr));

            }
            Instruction::MovFromMemory(register, addr) => {
                if register.bits() == 16 { data.push(0x66); }
                match *register {
                    Register::AL | Register::AX | Register::EAX => (),
                    _ => match register.bits() {
                        8 => data.push(0x8A),
                        16 | 32 => data.push(0x8B),
                        _ => panic!("unreachable code"),
                    }
                }

                data.push(match register {
                    Register::AL => 0xA0,
                    Register::AX | Register::EAX => 0xA1,
                    _ => 0x05 + register.offset() * 8,
                });
                data.extend_from_slice(&addr.as_vec(&self, cur_addr));

            }
            Instruction::MovFromMemoryReg(dest, src) => {
                // See Table 2-2
                if dest.bits() == 16 { data.push(0x66); }
                data.push(if dest.bits() == 8 { 0x8A } else { 0x8B });

                let op = dest.offset();
                let rm = src.offset();
                match src {
                    Register::ESP => { data.push((op << 3) | rm); data.push(0x24); }
                    Register::EBP => { data.push(0b01000000 | (op << 3) | rm); data.push(0x00); }
                    Register::EAX | Register::ECX | Register::EDX | Register::EBX | Register::ESI 
                        | Register::EDI => data.push((op << 3) | rm),
                    _ => panic!("unsupported register."),
                }
            }
            Instruction::Inc(register) => {
                if register.bits() == 16 { data.push(0x66); }
                data.push(0x40 + register.offset());
            }
            Instruction::Dec(register) => {
                if register.bits() == 16 { data.push(0x66); }
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
            Instruction::Add(dest, src) => {
                // See table 2-2 of intel manual
                if dest.bits() == 16 { data.push(0x66); }
                data.push(if dest.bits() == 8 { 0x00 } else { 0x01 });
                let op = src.offset();
                let rm = dest.offset();
                data.push(0b11000000 | (op << 3) | rm);
            }
            Instruction::AddImmediate { register, value } => {
                if register.bits() == 16 { data.push(0x66); }
                match register {
                    Register::AL | Register::AX | Register::EAX => (),
                    _ => data.push(if register.bits() == 8 { 0x80 } else { 0x81 }),
                }

                data.push(match register {
                    Register::AL => 0x04,
                    Register::AX | Register::EAX => 0x05,
                    _ => 0xC0 + register.offset(),
                });
                data.extend_from_slice(&value.as_vec(&self, cur_addr));
            }
            Instruction::Sub(dest, src) => {
                // See table 2-2 of intel manual
                if dest.bits() == 16 { data.push(0x66); }
                data.push(if dest.bits() == 32 { 0x29 } else { 0x28 });
                let op = src.offset();
                let rm = dest.offset();
                data.push(0b11000000 | (op << 3) | rm);
            }
            Instruction::SubImmediate { register, value } => {
                if register.bits() == 16 { data.push(0x66); }
                match register {
                    Register::AL | Register::AX | Register::EAX => (),
                    _ => data.push(if register.bits() == 32 { 0x81 } else { 0x80 }),
                }

                data.push(match register {
                    Register::AL => 0x2C,
                    Register::AX | Register::EAX => 0x2D,
                    _ => 0xE8 + register.offset(),
                });
                data.extend_from_slice(&value.as_vec(&self, cur_addr));
            }
            Instruction::Multiply(register) => {
                if register.bits() == 16 { data.push(0x66); }
                data.push(match register.bits() {
                    8 => 0xF6,
                    16 | 32 => 0xF7,
                    _ => panic!("unknown error."),
                });
                data.push(0xE0 + register.offset());
            }
            Instruction::Divide(register) => {
                if register.bits() == 16 { data.push(0x66); }
                data.push(match register.bits() {
                    8 => 0xF6,
                    16 | 32 => 0xF7,
                    _ => panic!("unknown error."),
                });
                data.push(0xF0 + register.offset());
            }
            Instruction::ByteSwap(register)  => {
                data.push(0x0f);
                data.push(0xC8 + register.offset());
            }
            Instruction::And(dest, src) => {
                // See table 2-2 of intel manual
                if dest.bits() == 16 { data.push(0x66); }
                data.push(if dest.bits() == 8 { 0x20 } else { 0x21 });
                let op = src.offset();
                let rm = dest.offset();
                data.push(0b11000000 | (op << 3) | rm);
            }
            Instruction::Or(dest, src) => {
                // See table 2-2 of intel manual
                if dest.bits() == 16 { data.push(0x66); }
                data.push(if dest.bits() == 8 { 0x08 } else { 0x09 });
                let op = src.offset();
                let rm = dest.offset();
                data.push(0b11000000 | (op << 3) | rm);
            }
            Instruction::XOr(dest, src) => {
                // See table 2-2 of intel manual
                if dest.bits() == 16 { data.push(0x66); }
                data.push(if dest.bits() == 8 { 0x30 } else { 0x31 });
                let op = src.offset();
                let rm = dest.offset();
                data.push(0b11000000 | (op << 3) | rm);
            }
            Instruction::Compare(dest, src) => {
                // See table 2-2 of intel manual
                if dest.bits() == 16 { data.push(0x66); }
                data.push(if dest.bits() == 8 { 0x38 } else { 0x39 });
                let op = src.offset();
                let rm = dest.offset();
                data.push(0b11000000 | (op << 3) | rm);
            }
            Instruction::CompareImmediate(register, value) => {
                if register.bits() == 16 { data.push(0x66); }
                match register {
                    Register::AL | Register::AX | Register::EAX => (),
                    _ => data.push(if register.bits() == 8 { 0x80 } else { 0x81 }),
                }

                data.push(match register {
                    Register::AL => 0x3C,
                    Register::AX | Register::EAX => 0x3D,
                    _ => 0xF8 + register.offset(),
                });
                data.extend_from_slice(&value.as_vec(&self, cur_addr));
            }
            Instruction::Push(register) => {
                if register.bits() == 16 { data.push(0x66); }
                data.push(0x50 + register.offset());
            }
            Instruction::Pop(register) => {
                if register.bits() == 16 { data.push(0x66); }
                data.push(0x58 + register.offset());
            }
            Instruction::Call(value) => {
                data.push(0xE8);
                data.extend_from_slice(&value.as_vec(&self, cur_addr));
            }
            Instruction::CallRegister(register) => {
                if register.bits() == 16 { data.push(0x66); }
                data.push(0xFF);
                data.push(0xD0 + register.offset());
            }
            Instruction::Return => data.push(0xC3),
        }

        data
    }
}
