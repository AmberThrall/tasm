use std::fmt::Pointer;

use super::{Addr, Instr};

pub struct ProgramBlock {
    label: String,
    len: usize,
    instrs: Vec<Box<dyn Instr>>,
}

pub struct Program {
    pub offset: Addr,
    pub entry_point: Addr,
    blocks: Vec<ProgramBlock> 
}

impl ProgramBlock {
    /// Pushes an instruction to the block.
    pub fn push(&mut self, instr: Box<dyn Instr>) {
        self.len += instr.len();
        self.instrs.push(instr);
    }

    /// Gets the length of the block
    pub fn len(&self) -> usize {
        self.len
    }
}

impl Program {
    /// Constructs a new program with no blocks.
    pub fn new() -> Program {
        Program {
            offset: Addr::default(),
            entry_point: Addr::default(),
            blocks: Vec::new(),
        }
    }

    /// Pushes an instruction block to the program labeled by 'label'.
    pub fn new_block(&mut self, label: &str) -> &mut ProgramBlock {
        self.blocks.push(ProgramBlock {
            label: label.to_string(),
            len: 0,
            instrs: Vec::new(),
        });

        self.blocks.last_mut().unwrap()
    }

    /// Gets a mutable refence to a block by index
    pub fn get_block_mut(&mut self, idx: usize) -> Option<&mut ProgramBlock> {
        self.blocks.get_mut(idx)
    }

    /// Gets the length of the program in bytes.
    pub fn len(&self) -> usize {
        let mut len = 0;
        for block in &self.blocks {
            len += block.len();
        }
        len
    }

    pub fn set_entrypoint(&mut self, label: &str) {
        let addr = self.get_addr(label).unwrap_or_default();
        self.entry_point = addr;
    }

    /// Looks up address of the start of the block labeled by 'label'.
    pub fn get_addr(&self, label: &str) -> Option<Addr> {
        let mut addr = self.offset;

        for block in &self.blocks {
            if block.label == label {
                return Some(addr);
            }
            addr += block.len() as u64;
        }

        None
    }

    /// Converts the program into a vector of bytes.
    pub fn as_vec(&self) -> Vec<u8> {
        let mut addr = self.offset;

        let mut dump = Vec::new();
        for block in &self.blocks {
            for instr in &block.instrs {
                addr += instr.len() as u64;
                let data = instr.as_vec(&self, addr);
                dump.extend_from_slice(&data);
            }
        }

        dump
    }
}
