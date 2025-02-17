use super::*;
use super::Node;

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
            Node::Mov(reg1, reg2) => self.push_instr(Instruction::Mov(*reg1, *reg2)),
            Node::MovImm(reg, x) => match reg.bits() {
                8 => self.push_instr(Instruction::MovImmediate { register: *reg, value: Value::UByte(*x as u8) }),
                16 => self.push_instr(Instruction::MovImmediate { register: *reg, value: Value::UShort(*x as u16) }),
                32 => self.push_instr(Instruction::MovImmediate { register: *reg, value: Value::UInt(*x) }),
                _ => panic!("unreachable code"),
            }
            Node::MovImmPointer(reg, label) => self.push_instr(Instruction::MovImmediate { register: *reg, value: Value::Pointer(label.clone()) }),
            Node::MovMemory(addr, reg) => self.push_instr(Instruction::MovMemory { addr: Value::UInt(*addr), register: *reg }),
            Node::MovMemoryPointer(label, reg) => self.push_instr(Instruction::MovMemory { addr: Value::Pointer(label.clone()), register: *reg }),
            Node::MovMemoryRegister(dest, reg) => self.push_instr(Instruction::MovMemoryReg { dest: *dest, src: *reg  }),
            Node::MovFromMemory(register, addr) => self.push_instr(Instruction::MovFromMemory(*register, Value::UInt(*addr))),
            Node::MovFromMemoryPointer(register, label) => self.push_instr(Instruction::MovFromMemory(*register, Value::Pointer(label.clone()))),
            Node::MovFromMemoryRegister(dest, src) => self.push_instr(Instruction::MovFromMemoryReg(*dest, *src)),
            Node::Add(dest, src) => self.push_instr(Instruction::Add(*dest, *src)),
            Node::AddImm(reg, x) => match reg.bits() {
                8 => self.push_instr(Instruction::AddImmediate { register: *reg, value: Value::UByte(*x as u8) }),
                16 => self.push_instr(Instruction::AddImmediate { register: *reg, value: Value::UShort(*x as u16) }),
                32 => self.push_instr(Instruction::AddImmediate { register: *reg, value: Value::UInt(*x) }),
                _ => panic!("unreachable code"),
            }
            Node::AddImmPointer(reg, label) => self.push_instr(Instruction::AddImmediate { register: *reg, value: Value::Pointer(label.clone()) }),
            Node::Sub(dest, src) => self.push_instr(Instruction::Sub(*dest, *src)),
            Node::SubImm(reg, x) => match reg.bits() {
                8 => self.push_instr(Instruction::SubImmediate { register: *reg, value: Value::UByte(*x as u8) }),
                16 => self.push_instr(Instruction::SubImmediate { register: *reg, value: Value::UShort(*x as u16) }),
                32 => self.push_instr(Instruction::SubImmediate { register: *reg, value: Value::UInt(*x) }),
                _ => panic!("unreachable code"),
            }
            Node::Mul(reg) => self.push_instr(Instruction::Multiply(*reg)),
            Node::Div(reg) => self.push_instr(Instruction::Divide(*reg)),
            Node::SubImmPointer(reg, label) => self.push_instr(Instruction::SubImmediate { register: *reg, value: Value::Pointer(label.clone()) }),
            Node::And(a, b) => self.push_instr(Instruction::And(*a, *b)),
            Node::Or(a, b) => self.push_instr(Instruction::Or(*a, *b)),
            Node::XOr(a, b) => self.push_instr(Instruction::XOr(*a, *b)),
            Node::CMP(a, b) => self.push_instr(Instruction::Compare(*a, *b)),
            Node::CMPImm(reg, x) => match reg.bits() {
                8 => self.push_instr(Instruction::CompareImmediate(*reg, Value::UByte(*x as u8))),
                16 => self.push_instr(Instruction::CompareImmediate(*reg, Value::UShort(*x as u16))),
                32 => self.push_instr(Instruction::CompareImmediate(*reg, Value::UInt(*x))),
                _ => panic!("unreachable code"),
            }
            Node::CMPImmPointer(reg, label) => self.push_instr(Instruction::CompareImmediate(*reg, Value::Pointer(label.clone()))),
            Node::BSWAP(reg) => self.push_instr(Instruction::ByteSwap(*reg)),
            Node::Push(reg) => self.push_instr(Instruction::Push(*reg)),
            Node::Pop(reg) => self.push_instr(Instruction::Pop(*reg)),
            Node::Call(addr) => self.push_instr(Instruction::Call(Value::UInt(*addr))),
            Node::CallPointer(label) => self.push_instr(Instruction::Call(Value::RelPointer(label.clone()))),
            Node::CallRegister(register) => self.push_instr(Instruction::CallRegister(*register)),
            Node::Return => self.push_instr(Instruction::Return),
            _ => (),
        }
    }
}
