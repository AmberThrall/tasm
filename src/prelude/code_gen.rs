use std::collections::HashMap;
use super::*;
use super::{Node, Token};

#[derive(Debug, Clone)]
enum Expr {
    BinaryOp {
        op: Token,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    PC,
    Pointer(String),
    Number(u32),
}

impl Expr {
    pub fn print(&self) {
        self.print_impl(0);
    }

    fn print_impl(&self, depth: usize) {
        for _ in 0..depth { print!("   "); }
        match self {
            Expr::BinaryOp { op, lhs, rhs } => {
                println!("BinaryOp({:?})", op);
                lhs.print_impl(depth + 1);
                rhs.print_impl(depth + 1);
            }
            _ => println!("{:?}", self),
        }
    }
}

fn precedence(token: &Token) -> usize {
    match token {
        Token::Plus => 0,
        Token::Minus => 0,
        Token::Multiply => 1,
        Token::Divide => 1,
        _ => panic!("not an operator!"),
    }
}

pub struct CodeGenerator {
    program: Program,
    entry_point: String,
    block_addrs: HashMap<String, u32>,
    variables: HashMap<String, u32>,
    current_block: usize,
}

impl CodeGenerator {
    pub fn generate(root: &Node) -> Program {
        let mut gen = CodeGenerator { 
            program: Program::new(), 
            entry_point: "__entry_point__".to_string(),
            block_addrs: HashMap::new(),
            variables: HashMap::new(),
            current_block: 0, 
        };

        gen.program.new_block("__entry_point__");
        gen.block_addrs.insert("__entry_point__".to_string(), 0);
        gen.process(&root);

        gen.program.set_entrypoint(&gen.entry_point);
        gen.program
    }

    fn push_instr(&mut self, instr: Instruction) {
        self.program.get_block_mut(self.current_block).unwrap().push(instr);
    }

    fn lookup_pointer(&self, ident: &str) -> Value {
        match self.variables.get(ident) {
            Some(v) => Value::UInt(*v),
            None => Value::Pointer(ident.to_string())
        }
    }

    fn lookup_rel_pointer(&self, ident: &str) -> Value {
        match self.variables.get(ident) {
            Some(v) => Value::UInt(*v),
            None => Value::RelPointer(ident.to_string())
        }
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
                self.block_addrs.insert(label.clone(), self.program.len() as u32);
                self.current_block += 1;
            } 
            Node::Entry(label) => self.entry_point = label.clone(),
            Node::DS(len) => self.push_instr(Instruction::RawData(vec![0; *len as usize])),
            Node::Db(data) => self.push_instr(Instruction::RawData(data.to_vec())),
            Node::DW(data) => {
                let mut new_data = Vec::new();
                for word in data {
                    new_data.extend_from_slice(&utils::dump_word(*word, Endianness::Little));
                }
                self.push_instr(Instruction::RawData(new_data));
            }
            Node::DL(data) => {
                let mut new_data = Vec::new();
                for word in data {
                    new_data.extend_from_slice(&utils::dump_dword(*word, Endianness::Little));
                }
                self.push_instr(Instruction::RawData(new_data));
            }
            Node::Int(x) => self.push_instr(Instruction::Int(*x)),
            Node::Inc(reg) => self.push_instr(Instruction::Inc(*reg)),
            Node::Dec(reg) => self.push_instr(Instruction::Dec(*reg)),
            Node::Jump { condition, label } => self.push_instr(Instruction::Jump { condition: *condition, addr: self.lookup_rel_pointer(label) }),
            Node::JumpImm { condition, addr } => self.push_instr(Instruction::Jump { condition: *condition, addr: Value::UInt(*addr) }),
            Node::Mov(reg1, reg2) => self.push_instr(Instruction::Mov(*reg1, *reg2)),
            Node::MovImm(reg, x) => match reg.bits() {
                8 => self.push_instr(Instruction::MovImmediate { register: *reg, value: Value::UByte(*x as u8) }),
                16 => self.push_instr(Instruction::MovImmediate { register: *reg, value: Value::UShort(*x as u16) }),
                32 => self.push_instr(Instruction::MovImmediate { register: *reg, value: Value::UInt(*x) }),
                _ => panic!("unreachable code"),
            }
            Node::MovImmPointer(reg, label) => self.push_instr(Instruction::MovImmediate { register: *reg, value: self.lookup_pointer(label) }),
            Node::MovMemory(addr, reg) => self.push_instr(Instruction::MovMemory { addr: Value::UInt(*addr), register: *reg }),
            Node::MovMemoryPointer(label, reg) => self.push_instr(Instruction::MovMemory { addr: self.lookup_pointer(label), register: *reg }),
            Node::MovMemoryRegister(dest, reg) => self.push_instr(Instruction::MovMemoryReg { dest: *dest, src: *reg  }),
            Node::MovFromMemory(register, addr) => self.push_instr(Instruction::MovFromMemory(*register, Value::UInt(*addr))),
            Node::MovFromMemoryPointer(register, label) => self.push_instr(Instruction::MovFromMemory(*register, self.lookup_pointer(label))),
            Node::MovFromMemoryRegister(dest, src) => self.push_instr(Instruction::MovFromMemoryReg(*dest, *src)),
            Node::Add(dest, src) => self.push_instr(Instruction::Add(*dest, *src)),
            Node::AddImm(reg, x) => match reg.bits() {
                8 => self.push_instr(Instruction::AddImmediate { register: *reg, value: Value::UByte(*x as u8) }),
                16 => self.push_instr(Instruction::AddImmediate { register: *reg, value: Value::UShort(*x as u16) }),
                32 => self.push_instr(Instruction::AddImmediate { register: *reg, value: Value::UInt(*x) }),
                _ => panic!("unreachable code"),
            }
            Node::AddImmPointer(reg, label) => self.push_instr(Instruction::AddImmediate { register: *reg, value: self.lookup_pointer(label) }),
            Node::Sub(dest, src) => self.push_instr(Instruction::Sub(*dest, *src)),
            Node::SubImm(reg, x) => match reg.bits() {
                8 => self.push_instr(Instruction::SubImmediate { register: *reg, value: Value::UByte(*x as u8) }),
                16 => self.push_instr(Instruction::SubImmediate { register: *reg, value: Value::UShort(*x as u16) }),
                32 => self.push_instr(Instruction::SubImmediate { register: *reg, value: Value::UInt(*x) }),
                _ => panic!("unreachable code"),
            }
            Node::Mul(reg) => self.push_instr(Instruction::Multiply(*reg)),
            Node::Div(reg) => self.push_instr(Instruction::Divide(*reg)),
            Node::SubImmPointer(reg, label) => self.push_instr(Instruction::SubImmediate { register: *reg, value: self.lookup_pointer(label) }),
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
            Node::CMPImmPointer(reg, label) => self.push_instr(Instruction::CompareImmediate(*reg, self.lookup_pointer(label))),
            Node::BSWAP(reg) => self.push_instr(Instruction::ByteSwap(*reg)),
            Node::Push(reg) => self.push_instr(Instruction::Push(*reg)),
            Node::Pop(reg) => self.push_instr(Instruction::Pop(*reg)),
            Node::Call(addr) => self.push_instr(Instruction::Call(Value::UInt(*addr))),
            Node::CallPointer(label) => self.push_instr(Instruction::Call(self.lookup_rel_pointer(label))),
            Node::CallRegister(register) => self.push_instr(Instruction::CallRegister(*register)),
            Node::Return => self.push_instr(Instruction::Return),
            Node::Not(register) => self.push_instr(Instruction::Not(*register)),
            Node::Neg(register) => self.push_instr(Instruction::Neg(*register)),
            Node::SHL(register) => self.push_instr(Instruction::ShiftLeft(*register)),
            Node::SHR(register) => self.push_instr(Instruction::ShiftRight(*register)),
            Node::EQU(ident, expr) => {
                let expr = self.build_expr(expr);
                let value = self.evaluate_expr(&expr);
                self.variables.insert(ident.clone(), value);
            }
            _ => (),
        }
    }

    fn evaluate_expr(&self, expr: &Expr) -> u32 {
        match expr {
            Expr::Number(v) => *v,
            Expr::PC => self.program.len() as u32,
            Expr::Pointer(label) => match self.block_addrs.get(label) {
                Some(x) => *x,
                None => {
                    println!("Warning: unknown label '{}' in expression.", label);
                    0
                },
            }
            Expr::BinaryOp { op, lhs, rhs } => {
                let a = self.evaluate_expr(lhs);
                let b = self.evaluate_expr(rhs);
                
                match op {
                    Token::Plus => a + b,
                    Token::Minus => a - b,
                    Token::Multiply => a * b,
                    Token::Divide => a / b,
                    _ => 0,
                }
            }
        }
    }

    fn build_expr(&self, node: &Node) -> Expr {
        match node {
            Node::Pointer(label) => match self.variables.get(label) {
                Some(v) => Expr::Number(*v),
                None => Expr::Pointer(label.to_string())
            }
            Node::Integer(v) => Expr::Number(*v),
            Node::Dollar => Expr::PC,
            Node::ParenExpr(node) => self.build_expr(node),
            Node::Expr(nodes)  => {
                let mut peekable = nodes.iter().peekable();
                let lhs = self.build_expr(peekable.next().unwrap());
                self.build_expr_climber(lhs, &mut peekable, 0)
            }
            _ => panic!("unreachable code. Got a {:?}.", node),
        }
    }

    fn build_expr_climber(&self, mut lhs: Expr, nodes: &mut core::iter::Peekable<std::slice::Iter<'_, Node>>, min_precedence: usize) -> Expr {
        let mut peek = nodes.peek();
        while peek.is_some() {
            let operator = match peek.unwrap() {
                Node::Operator(op) => op, 
                _ => panic!("not an operator!"),
            };
            if precedence(&operator) < min_precedence { break; }

            nodes.next();
            let lookahead = nodes.next();
            let mut rhs = self.build_expr(lookahead.unwrap());

            peek = nodes.peek();
            while peek.is_some() {
                match peek.unwrap() {
                    Node::Operator(op) => if precedence(&op) <= min_precedence { break }
                    _ => break,
                }               

                rhs = self.build_expr_climber(rhs.clone(), nodes, precedence(&operator));
                peek = nodes.peek();
            }

            lhs = Expr::BinaryOp { 
                op: operator.clone(), 
                lhs: Box::new(lhs), 
                rhs: Box::new(rhs),
            }
        }
        lhs
    }
}
