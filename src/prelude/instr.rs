use super::{Program, Register, utils::*, Value};
use std::any::Any;

pub trait Instr {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_vec(&self, program: &Program) -> Vec<u8>;
    fn len(&self) -> usize;
}

/// Dummy Instruction for adding raw data (useful for text).
pub struct RawData(pub Vec<u8>);

impl Instr for RawData {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn as_vec(&self, _program: &Program) -> Vec<u8> {
        self.0.clone()
    }
    fn len(&self) -> usize { self.0.len() }
}

/// Interrupt instruction
pub struct Int(pub u8);

impl Instr for Int {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn as_vec(&self, _program: &Program) -> Vec<u8> {
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
    fn as_vec(&self, program: &Program) -> Vec<u8> {
        let mut dump = Vec::new();

        dump.push(match self.r {
            Register::EAX => 0xB8,
            Register::EBX => 0xBB,
            Register::ECX => 0xB9,
            Register::EDX => 0xBA,
        });

        dump.extend_from_slice(&self.value.as_vec(program));

        dump
    }
}
