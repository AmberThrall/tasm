use super::{Instr, Program};

/// A Block of instructions
pub struct Block {
    instrs: Vec<Box<dyn Instr>>,
    len: usize,
}

impl Block {
    pub fn new() -> Block {
        Block {
            instrs: Vec::new(),
            len: 0,
        }
    }

    /// Pushes an instruction to the block. Returns the offset in memory.
    pub fn push(&mut self, instr: Box<dyn Instr>) -> u32 {
        let ret = self.len;
        self.len += instr.len();
        self.instrs.push(instr);

        ret as u32
    }

    /// Returns the total length of the block in bytes.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Represents the block as a vector of bytes
    pub fn as_vec(&self, program: &Program) -> Vec<u8> {
        let mut dump = Vec::new();
        for instr in &self.instrs {
            dump.extend_from_slice(&instr.as_vec(program));
        }
        dump
    }
}
