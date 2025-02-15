use super::{Instr, Program, Addr};

/// A Block of instructions
pub struct Block {
    pub instrs: Vec<Box<dyn Instr>>,
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
    pub fn as_vec(&self, program: &Program, addr: Addr) -> Vec<u8> {
        let mut addr = addr;
        let mut dump = Vec::new();
        for instr in &self.instrs {
            addr += instr.len() as u64;
            let data = instr.as_vec(program, addr);
            dump.extend_from_slice(&data);
        }
        dump
    }
}
