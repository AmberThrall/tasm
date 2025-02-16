use super::*;
use super::new_parser::Node;

pub struct CodeGenerator {
    program: Program,
    entry_point: String,
    current_block: usize,
}

impl CodeGenerator {
    pub fn generate(root: &Node) -> Program {
        let mut gen = CodeGenerator { 
            program: Program::new(), 
            entry_point: "__entry_point__".to_string(),
            current_block: 0, 
        };

        gen.program.new_block("__entry_point__");
        gen.process(&root);

        gen.program.set_entrypoint(&gen.entry_point);
        gen.program
    }

    fn push_instr(&mut self, instr: Instruction) {
        self.program.get_block_mut(self.current_block).unwrap().push(instr);
    }

    fn process(&mut self, node: &Node) {
        match node {
            Node::Program(stmts) => {
                for stmt in stmts {
                    self.process(stmt);
                }
            }
            Node::Label(label) => {
                self.program.new_block(&label);
                self.current_block += 1;
            } 
            Node::Entry(label) => self.entry_point = label.clone(),
            Node::Db(data) => self.push_instr(Instruction::RawData(data.to_vec())),
            Node::Int(x) => self.push_instr(Instruction::Int(*x)),
            Node::Inc(reg) => self.push_instr(Instruction::Inc(*reg)),
            Node::Dec(reg) => self.push_instr(Instruction::Dec(*reg)),
            Node::Jump { condition, label } => self.push_instr(Instruction::Jump { condition: *condition, addr: Value::RelPointer(label.clone()) }),
            Node::JumpImm { condition, addr } => self.push_instr(Instruction::Jump { condition: *condition, addr: Value::UInt(*addr) }),
            Node::MovImm(reg, x) => self.push_instr(Instruction::MovImmediate { register: *reg, value: Value::UInt(*x) }),
            Node::MovImmPointer(reg, label) => self.push_instr(Instruction::MovImmediate { register: *reg, value: Value::Pointer(label.clone()) }),
            Node::Newline => (),
        }
    }
}
