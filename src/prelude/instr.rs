use super::{Program, Register, utils::*, Value, Addr};
use std::any::Any;

pub trait Instr {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_vec(&self, program: &Program, addr: Addr) -> Vec<u8>;
    fn len(&self) -> usize;
}

/// Dummy Instruction for adding raw data (useful for text).
pub struct RawData(pub Vec<u8>);

impl Instr for RawData {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn as_vec(&self, _program: &Program, _addr: Addr) -> Vec<u8> {
        self.0.clone()
    }
    fn len(&self) -> usize { self.0.len() }
}

/// Interrupt instruction
pub struct Int(pub u8);

impl Instr for Int {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn as_vec(&self, _program: &Program, _addr: Addr) -> Vec<u8> {
        vec![0xCD, 0x80]
    }
    fn len(&self) -> usize { 2 }
}

/// Move data into register instruction.
pub struct MovData {
    pub r: Register,
    pub value: Value,
}

impl MovData {
    pub fn new(r: Register, value: Value) -> MovData {
        MovData {
            r,
            value,
        }
    }
}

impl Instr for MovData {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn len(&self) -> usize { 5 }
    fn as_vec(&self, program: &Program, addr: Addr) -> Vec<u8> {
        let mut dump = Vec::new();

        dump.push(match self.r {
            Register::EAX => 0xB8,
            Register::ECX => 0xB9,
            Register::EDX => 0xBA,
            Register::EBX => 0xBB,
            Register::ESP => 0xBC,
            Register::EBP => 0xBD,
            Register::ESI => 0xBE,
            Register::EDI => 0xBF,
        });

        dump.extend_from_slice(&self.value.as_vec(program, addr));

        dump
    }
}

/// Decrement a register.
pub struct Dec(pub Register);

impl Instr for Dec {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn len(&self) -> usize { 1 }
    fn as_vec(&self, _program: &Program, _addr: Addr) -> Vec<u8> {
        let mut dump = Vec::new();

        dump.push(match self.0 {
            Register::EAX => 0x48,
            Register::ECX => 0x49,
            Register::EDX => 0x4A,
            Register::EBX => 0x4B,
            Register::ESP => 0x4C,
            Register::EBP => 0x4D,
            Register::ESI => 0x4E,
            Register::EDI => 0x4F,
        });

        dump
    }
}

/// Jump conditionals
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum JumpConditional {
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

/// Jump to a location
pub struct JMPData(pub JumpConditional, pub Value);

impl Instr for JMPData {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn len(&self) -> usize { 
        match &self.0 {
            JumpConditional::None => 5,
            _ => 6,
        }
    }
    fn as_vec(&self, program: &Program, addr: Addr) -> Vec<u8> {
        let mut dump = Vec::new();

        match &self.0 {
            JumpConditional::None => { dump.push(0xE9); },
            _ => {
                dump.push(0x0F);
                dump.push(self.0 as u8);
            }
        }

        let addr_delta = self.1.as_vec(program, addr);
        dump.extend_from_slice(&addr_delta);
        dump
    }
}

