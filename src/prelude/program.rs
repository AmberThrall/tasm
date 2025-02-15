use super::{Addr, Instr};

pub struct ProgramBlock {
    label: String,
    len: usize,
    instrs: Vec<Box<dyn Instr>>,
}

pub struct Program {
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
            entry_point: Addr { addr: 0, vaddr: 0 },
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

    /// Gets the length of the program in bytes.
    pub fn len(&self) -> usize {
        let mut len = 0;
        for block in &self.blocks {
            len += block.len();
        }
        len
    }

    /// Looks up address of the start of the block labeled by 'label'.
    pub fn get_addr(&self, label: &str) -> Option<Addr> {
        let mut addr = self.entry_point;

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
        let mut addr = self.entry_point;

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
