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
            Self::MovImmediate { register, value } => 5,
            Self::MovRM32R32 { dest, src } => if *dest == Register::ESP || *dest == Register::EBP { 3 } else { 2 },
            Self::MovMemory { addr, register } => { if *register == Register::EAX { 6 } else { 7 } },
            Self::Inc(_) => 1, 
            Self::Dec(_) => 1, 
            Self::Jump { condition, addr } => { if *condition == JumpCondition::None { 5 } else { 6 } },
            Self::AddImmediate { register, value } => { if *register == Register::EAX { 6 } else { 7 } },
            Self::SubImmediate { register, value } => { if *register == Register::EAX { 6 } else { 7 } },
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
            Instruction::MovImmediate { register, value } => {
                data.push(0xB8 + *register as u8);
                data.extend_from_slice(&value.as_vec(&self, cur_addr));
            }
            Instruction::MovRM32R32 { dest, src } => {
                // See table 2-2 of intel manual
                data.push(0x89);
                let op = *src as u8;
                let rm = *dest as u8;
                match dest {
                    Register::ESP => { data.push((rm << 3) | op); data.push(0x24); }
                    Register::EBP => { data.push(0b01000000 | (rm << 3) | op); data.push(0x00); }
                    Register::EAX | Register::ECX | Register::EDX | Register::EBX | Register::ESI 
                        | Register::EDI => data.push((op << 3) | rm),
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
                });
                data.extend_from_slice(&addr.as_vec(&self, cur_addr));

            }
            Instruction::Inc(register) => {
                data.push(0x40 + *register as u8);
            }
            Instruction::Dec(register) => {
                data.push(0x48 + *register as u8);
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
                if *register != Register::EAX { data.push(0x81); }

                data.push(match register {
                    Register::EAX => 0x05,
                    Register::ECX => 0xC1,
                    Register::EDX => 0xC2,
                    Register::EBX => 0xC3,
                    Register::ESP => 0xC4,
                    Register::EBP => 0xC5,
                    Register::ESI => 0xC6,
                    Register::EDI => 0xC7,
                });
                data.extend_from_slice(&value.as_vec(&self, cur_addr));
            }
            Instruction::SubImmediate { register, value } => {
                if *register != Register::EAX { data.push(0x81); }

                data.push(match register {
                    Register::EAX => 0x2D,
                    Register::ECX => 0xE9,
                    Register::EDX => 0xEA,
                    Register::EBX => 0xEB,
                    Register::ESP => 0xEC,
                    Register::EBP => 0xED,
                    Register::ESI => 0xEE,
                    Register::EDI => 0xEF,
                });
                data.extend_from_slice(&value.as_vec(&self, cur_addr));
            }
            Instruction::ByteSwap(register)  => {
                data.push(0x0f);
                data.push(0xC8 + *register as u8);
            }
            Instruction::And(dest, src) => {
                // See table 2-2 of intel manual
                data.push(0x21);
                let op = *src as u8;
                let rm = *dest as u8;
                data.push(0b11000000 | (op << 3) | rm);
            }
            Instruction::Or(dest, src) => {
                // See table 2-2 of intel manual
                data.push(0x09);
                let op = *src as u8;
                let rm = *dest as u8;
                data.push(0b11000000 | (op << 3) | rm);
            }
            Instruction::XOr(dest, src) => {
                // See table 2-2 of intel manual
                data.push(0x31);
                let op = *src as u8;
                let rm = *dest as u8;
                data.push(0b11000000 | (op << 3) | rm);
            }
        }

        data
    }
}
