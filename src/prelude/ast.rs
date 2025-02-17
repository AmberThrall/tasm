use super::lexer::Token;
use super::{Register, JumpCondition};

#[derive(Debug)]
pub enum Node {
    Program(Vec<Node>),
    Label(String),
    Entry(String),
    DS(u32),
    Db(Vec<u8>),
    DW(Vec<u16>),
    DL(Vec<u32>),
    Int(u8),
    Inc(Register),
    Dec(Register),
    Jump { condition: JumpCondition, label: String },
    JumpImm { condition: JumpCondition, addr: u32 },
    Mov(Register, Register),
    MovImm(Register, u32),
    MovImmPointer(Register, String),
    MovMemory(u32, Register),
    MovMemoryPointer(String, Register),
    MovMemoryRegister(Register, Register),
    MovFromMemory(Register, u32),
    MovFromMemoryPointer(Register, String),
    MovFromMemoryRegister(Register, Register),
    Add(Register, Register),
    AddImm(Register, u32),
    AddImmPointer(Register, String),
    Sub(Register, Register),
    SubImm(Register, u32),
    SubImmPointer(Register, String),
    Mul(Register),
    Div(Register),
    And(Register, Register),
    Or(Register, Register),
    XOr(Register, Register),
    CMP(Register, Register),
    CMPImm(Register, u32),
    CMPImmPointer(Register, String),
    BSWAP(Register),
    Push(Register),
    Pop(Register),
    Call(u32),
    CallPointer(String),
    CallRegister(Register),
    Return,
    Not(Register),
    Neg(Register),
    SHL(Register),
    SHR(Register),
    Register(Register),
    Integer(u32),
    Pointer(String),
    Newline,
    EQU(String, Box<Node>),
    Expr(Vec<Node>),    
    ParenExpr(Box<Node>),
    Operator(Token),
    Dollar,
}

impl Node {
    pub fn print(&self) {
        self.print_impl(0);
    }

    fn print_impl(&self, depth: usize) {
        for _ in 0..depth { print!("   "); }
        match self {
            Node::Program(stmts) => {
                println!("Program");
                for n in stmts { n.print_impl(depth + 1); }
            }
            Node::Label(ident) => println!("Label({})", ident),
            Node::EQU(ident, node) => {
                println!("EQU({})", ident);
                node.print_impl(depth + 1);
            }
            Node::Expr(nodes) => {
                println!("Expr");
                for node in nodes {
                    node.print_impl(depth + 1);
                }
            }
            Node::ParenExpr(node) => {
                println!("ParenExpr");
                node.print_impl(depth + 1);
            }
            _ => println!("{:?}", self),
        }
    }
}
