use std::u32;

use logos::{Logos, Lexer};

use super::lexer::Token;
use super::{Register, JumpCondition};

#[derive(Debug)]
pub enum Node {
    Program(Vec<Node>),
    Label(String),
    Entry(String),
    Db(Vec<u8>),
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
    AddImm(Register, u32),
    AddImmPointer(Register, String),
    SubImm(Register, u32),
    SubImmPointer(Register, String),
    And(Register, Register),
    Or(Register, Register),
    XOr(Register, Register),
    Register(Register),
    Integer(u32),
    Pointer(String),
    Newline
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
            _ => println!("{:?}", self),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub line_no: usize,
    pub token: Option<Token>,
}

pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
    next: Option<Token>,
    line_no: usize,
}

impl<'a> Parser<'a> {
    /// Parse a string returning an abstract syntax tree.
    pub fn parse(str: &str) -> Result<Node, Error> {
        let lexer = Token::lexer(str);
        let mut parser = Parser { 
            lexer,
            next: None,
            line_no: 1 
        };  

        parser.march(); // Feed the first token in
        parser.program()
    }

    ///// Helper Functions /////
    fn peek(&self) -> Option<Token> {
        self.next.clone()
    }

    fn march(&mut self) -> Option<Token> {
        let ret = self.next.clone();
        self.next = self.lexer.next().map(|x| x.unwrap());
        ret
    }

    fn error(&self, msg: &str) -> Result<Node,Error> {
        Err(Error {
            message: msg.to_string(),
            line_no: self.line_no,
            token: self.next.clone(),
        })
    }


    ///// Recursive descent parser /////
    // program ::= (whitespace statement newline | whitespace newline)*
    fn program(&mut self) -> Result<Node, Error> {
        let mut stmts = Vec::new();

        // repeat while there are still tokens left
        while let Some(token) = self.peek() {
            self.whitespace(); 
            match self.newline() {
                Ok(_) => (),
                Err(_) => {
                    let node = self.statement()?;
                    self.newline()?;
                    stmts.push(node);
                }
            }
        }

        Ok(Node::Program(stmts))
    }

    // statement ::= label_statement | instruction
    fn statement(&mut self) -> Result<Node, Error> {
        let token = self.peek();
        match token {
            Some(Token::Identifier(ident)) => match self.label_statement() {
                Ok(x) => Ok(x),
                Err(_) => self.error(&format!("unknown instruction '{}'.", ident)),
            },
            Some(Token::Entry) => self.entry_statement(),
            Some(Token::Db) => self.db_statement(),
            Some(Token::Int) => self.int_statement(),
            Some(Token::Inc) => self.inc_statement(),
            Some(Token::Dec) => self.dec_statement(),
            Some(Token::Jmp) => self.jump_statement(JumpCondition::None),
            Some(Token::JO) => self.jump_statement(JumpCondition::Overflow),
            Some(Token::JNO) => self.jump_statement(JumpCondition::NotOverflow),
            Some(Token::JB) | Some(Token::JNAE) | Some(Token::JC) => self.jump_statement(JumpCondition::Carry),
            Some(Token::JNB) | Some(Token::JAE) | Some(Token::JNC) => self.jump_statement(JumpCondition::NotCarry),
            Some(Token::JZ) | Some(Token::JE) => self.jump_statement(JumpCondition::Zero),
            Some(Token::JNZ) | Some(Token::JNE) => self.jump_statement(JumpCondition::NotZero),
            Some(Token::JBE) | Some(Token::JNA) => self.jump_statement(JumpCondition::CarryOrZero),
            Some(Token::JNBE) | Some(Token::JA) => self.jump_statement(JumpCondition::NotCarryAndNotZero),
            Some(Token::JS) => self.jump_statement(JumpCondition::Sign),
            Some(Token::JNS) => self.jump_statement(JumpCondition::NotSign),
            Some(Token::JP) | Some(Token::JPE) => self.jump_statement(JumpCondition::Parity),
            Some(Token::JNP) | Some(Token::JPO) => self.jump_statement(JumpCondition::NotParity),
            Some(Token::JL) | Some(Token::JNGE) => self.jump_statement(JumpCondition::Less),
            Some(Token::JNL) | Some(Token::JGE) => self.jump_statement(JumpCondition::NotLess),
            Some(Token::JLE) | Some(Token::JNG) => self.jump_statement(JumpCondition::NotGreater),
            Some(Token::JNLE) | Some(Token::JG) => self.jump_statement(JumpCondition::Greater),
            Some(Token::Mov) => self.mov_statement(),
            Some(Token::Add) => self.add_statement(),
            Some(Token::Sub) => self.sub_statement(),
            Some(Token::And) => self.and_statement(),
            Some(Token::Or) => self.or_statement(),
            Some(Token::Xor) => self.xor_statement(),
            _ => self.error("expected a statement."),
        }
    }

    // label_statement ::= identifier whitespace COLON
    fn label_statement(&mut self) -> Result<Node, Error> {
        let ident = match self.march() {
            Some(Token::Identifier(x)) => x,
            _ => { self.error("label requires a name.")?; String::new() } ,
        };

        self.whitespace();

        match self.march() {
            Some(Token::Colon) => Ok(Node::Label(ident)),
            _ => self.error("expected ':' in label statement."),
        }
    }

    // entry_statement ::= ENTRY required_whitespace identifier
    fn entry_statement(&mut self) -> Result<Node, Error> {
        self.march();
        if !self.required_whitespace() { return self.error("expected whitespace after 'entry'."); }
        match self.march() {
            Some(Token::Identifier(x)) => Ok(Node::Entry(x)),
            _ => self.error("invalid argument passed to 'entry', expected label."),
        }
    }

    // db_statement ::= DB required_whitespace db_statment (COMMA whitespace db_statement)*
    fn db_statement(&mut self) -> Result<Node, Error> {
        self.march();
        if !self.required_whitespace() { return self.error("expected whitespace after 'db'."); }
        let mut data = Vec::new();

        match self.db_argument() {
            Ok(bytes) => data.extend_from_slice(&bytes),
            Err(msg) => return self.error(&msg),
        }

        while self.peek() == Some(Token::Comma) {
            self.march();
            self.whitespace();
            match self.db_argument() {
                Ok(bytes) => data.extend_from_slice(&bytes),
                Err(msg) => return self.error(&msg),
            }
        }

        Ok(Node::Db(data))
    }

    // db_argument ::= number | byte 
    fn db_argument(&mut self) -> Result<Vec<u8>, String> {
        match self.peek() {
            Some(Token::String(s)) => { self.march(); Ok(s.as_bytes().to_vec()) },
            Some(Token::Number(_)) | Some(Token::HexNumber(_)) => match self.byte() {
                Ok(x) => Ok(vec![x]),
                Err(e) => Err(format!("invalid argument passed to 'db' ({})", e))
            }
            _ => Err("invalid argument passed to 'db'.".to_string()),
        }
    }

    // int_statement ::= INT required_whitespace number
    fn int_statement(&mut self) -> Result<Node, Error> {
        self.march(); // INT 
        if !self.required_whitespace() {
            return self.error("expected whitespace after 'int'.");
        }
        match self.byte() {
            Ok(x) => Ok(Node::Int(x)),
            Err(e) => self.error(&format!("invalid argument for 'int' ({}).", e)),
        }
    }

    // inc_statement ::= INC required_whitespace register
    fn inc_statement(&mut self) -> Result<Node, Error> {
        self.march();
        if !self.required_whitespace() { return self.error("expected whitespace after 'inc'."); }
        
        match self.register() {
            Some(r) => Ok(Node::Inc(r)),
            None => self.error("invalid argument for 'inc', expected register."),
        }
    }

    // dec_statement ::= DEC required_whitespace register
    fn dec_statement(&mut self) -> Result<Node, Error> {
        self.march(); 
        if !self.required_whitespace() { return self.error("expected whitespace after 'dec'."); }
        
        match self.register() {
            Some(r) => Ok(Node::Dec(r)),
            None => self.error("invalid argument for 'dec', expected register."),
        }
    }

    // jump_statement ::= (JMP..) required_whitespace (IDENTIFIER | integer)
    fn jump_statement(&mut self, condition: JumpCondition) -> Result<Node, Error> {
        self.march();
        if !self.required_whitespace() { return self.error("expected whitespace after 'jmp'."); }

        match self.peek() {
            Some(Token::Identifier(label)) => { self.march(); Ok(Node::Jump { condition, label }) }
            _ => match self.integer() {
                Ok(addr) => Ok(Node::JumpImm { condition , addr }),
                Err(e) => self.error(&format!("invalid argument for 'jmp' ({})", e)),
            }
        }
    }

    // mov_statement ::= MOV required_whitespace reg_imm_or_reg_reg
    //              | MOV requied_whitespace [ ws (register | pointer | integer) ws ] ws , ws register
    fn mov_statement(&mut self) -> Result<Node, Error> {
        self.march();
        if !self.required_whitespace() { return self.error("expected whitespace after 'mov'."); }

        match self.peek() {
            Some(Token::LeftBracket) => {
                self.march();
                self.whitespace();
                let n = match self.peek() {
                    Some(Token::Identifier(x)) => { self.march(); Ok(Node::Pointer(x)) },
                    Some(Token::Number(_)) | Some(Token::HexNumber(_)) => match self.integer() {
                        Ok(x) => Ok(Node::Integer(x)),
                        Err(e) => self.error(&format!("invalid memory address in mov ({})", e)),
                    }
                    _ => match self.register() {
                        Some(x) => Ok(Node::Register(x)),
                        None => self.error(&format!("invalid memory address in mov (unknown register)")),
                    }
                }?;

                self.whitespace();
                if self.march() != Some(Token::RightBracket) { return self.error("expected ']'"); }
                self.whitespace();
                if self.march() != Some(Token::Comma) { return self.error("expected ','"); }
                self.whitespace();

                let register = self.register();
                if register.is_none() { return self.error("unknown register in mov."); }

                match n {
                    Node::Integer(x) => Ok(Node::MovMemory(x, register.unwrap())),
                    Node::Pointer(x) => Ok(Node::MovMemoryPointer(x.clone(), register.unwrap())),
                    Node::Register(x) => Ok(Node::MovMemoryRegister(x, register.unwrap())),
                    _ => self.error("unknown error occured."),
                }
            }
            // It's not a memory write
            _ => match self.reg_imm_or_reg_reg() {
                Ok((reg, n)) => match n {
                    Node::Register(reg2) => Ok(Node::Mov(reg, reg2)),
                    Node::Pointer(label) => Ok(Node::MovImmPointer(reg, label)),
                    Node::Integer(x) => Ok(Node::MovImm(reg, x)),
                    _ => self.error("invalid arguments to mov (unknown error)."),
                }
                Err(e) => self.error(&format!("invalid arguments to mov ({}).", e)),
            }
        }
    }

    // add_statement ::= ADD req_ws reg_imm 
    fn add_statement(&mut self) -> Result<Node, Error> {
        self.march();
        if !self.required_whitespace() { return self.error("expected whitespace after 'add'."); }

        match self.reg_imm() {
            Ok((reg, n)) => match n {
                Node::Pointer(label) => Ok(Node::AddImmPointer(reg, label)),
                Node::Integer(x) => Ok(Node::AddImm(reg, x)),
                _ => self.error("invalid arguments to add (unknown error)."),
            }
            Err(e) => self.error(&format!("invalid arguments to add ({}).", e)),
        }
    }

    // sub_statement ::= SUB req_ws reg_imm 
    fn sub_statement(&mut self) -> Result<Node, Error> {
        self.march();
        if !self.required_whitespace() { return self.error("expected whitespace after 'sub'."); }

        match self.reg_imm() {
            Ok((reg, n)) => match n {
                Node::Pointer(label) => Ok(Node::SubImmPointer(reg, label)),
                Node::Integer(x) => Ok(Node::SubImm(reg, x)),
                _ => self.error("invalid arguments to sub (unknown error)."),
            }
            Err(e) => self.error(&format!("invalid arguments to sub ({}).", e)),
        }
    }

    // and_statement ::= AND req_ws reg_reg
    fn and_statement(&mut self) -> Result<Node, Error> {
        self.march();
        if !self.required_whitespace() { return self.error("expected whitespace after 'and'."); }

        match self.reg_reg() {
            Ok((a,b)) => Ok(Node::And(a, b)),
            Err(e) => self.error(&format!("invalid argument passed to and ({})", e)),
        }
    }

    // or_statement ::= AND req_ws reg_reg
    fn or_statement(&mut self) -> Result<Node, Error> {
        self.march();
        if !self.required_whitespace() { return self.error("expected whitespace after 'or'."); }

        match self.reg_reg() {
            Ok((a,b)) => Ok(Node::Or(a, b)),
            Err(e) => self.error(&format!("invalid argument passed to or ({})", e)),
        }
    }

    // xor_statement ::= AND req_ws reg_reg
    fn xor_statement(&mut self) -> Result<Node, Error> {
        self.march();
        if !self.required_whitespace() { return self.error("expected whitespace after 'xor'."); }

        match self.reg_reg() {
            Ok((a,b)) => Ok(Node::XOr(a, b)),
            Err(e) => self.error(&format!("invalid argument passed to xor ({})", e)),
        }
    }
    // whitespace ::= WHITESPACE*
    fn whitespace(&mut self) {
        while let Some(token) = self.peek() {
            match token {
                Token::Whitespace => { self.march(); },
                _ => break,
            }
        }
    }

    // required_whitespace ::= WHITESPACE whitespace
    fn required_whitespace(&mut self) -> bool {
        match self.march() {
            Some(Token::Whitespace) => { self.whitespace(); true },
            _ => false
        }
    }

    // register
    fn register(&mut self) -> Option<Register> {
        match self.march() {
            Some(Token::AH) => Some(Register::AH),
            Some(Token::AL) => Some(Register::AL), 
            Some(Token::BH) => Some(Register::BH), 
            Some(Token::BL) => Some(Register::BL), 
            Some(Token::CH) => Some(Register::CH), 
            Some(Token::CL) => Some(Register::CL), 
            Some(Token::DH) => Some(Register::DH), 
            Some(Token::DL) => Some(Register::DL),
            Some(Token::EAX) => Some(Register::EAX),
            Some(Token::EBX) => Some(Register::EBX),
            Some(Token::ECX) => Some(Register::ECX),
            Some(Token::ESP) => Some(Register::ESP),
            Some(Token::EBP) => Some(Register::EBP),
            Some(Token::EDI) => Some(Register::EDI),
            Some(Token::ESI) => Some(Register::ESI),
            Some(Token::EDX) => Some(Register::EDX),
            _ => None,
        }
    }

    // byte ::= NUMBER | HEXNUMBER
    //      checks that its between 0 and 0xFF
    fn byte(&mut self) -> Result<u8, String> {
        match self.march() {
            Some(Token::Number(x)) => if (x as u64) > 0xFF { 
                Err(format!("{} > 255", x as u64))     
            } else { Ok(x as u8) }
            Some(Token::HexNumber(x)) => if (x as u64) > 0xFF { 
                Err(format!("{} > 255", x as u64))     
            } else { Ok(x as u8) }
            _ => Err("not a number".to_string()),
        }
    }

    // integer ::= NUMBER | HEXNUMBER
    //      checks that its a valid 32bit integer
    fn integer(&mut self) -> Result<u32, String> {
        match self.march() {
            Some(Token::Number(x)) => if (x as u64) > 0xFFFFFFFF { 
                Err(format!("{} > 0xFFFFFFFF", x as u64))     
            } else { Ok(x as u32) }
            Some(Token::HexNumber(x)) => if (x as u64) > 0xFFFFFFFF { 
                Err(format!("{} > 0xFFFFFFFF", x as u64))     
            } else { Ok(x as u32) }
            _ => Err("not a number".to_string()),
        }
    }

    // reg_reg ::= register ws COMMA ws register
    fn reg_reg(&mut self) -> Result<(Register, Register), String> {
        let a = self.register();
        self.whitespace();
        if self.march() != Some(Token::Comma) { return Err("missing ','".to_string()); }
        self.whitespace();
        let b = self.register();

        if a.is_none() || b.is_none() { return Err("unknown register".to_string()); }
        Ok((a.unwrap(), b.unwrap()))
    }

    // reg_imm ::= register ws COMMA ws (integer | identifier)
    fn reg_imm(&mut self) -> Result<(Register, Node), String> {
        let a = self.register();
        self.whitespace();
        if self.march() != Some(Token::Comma) { return Err("missing ','".to_string()); }
        self.whitespace();

        if a.is_none() { return Err("unknown register".to_string()); }

        match self.peek() {
            Some(Token::Identifier(x)) => { self.march(); Ok((a.unwrap(), Node::Pointer(x.clone()))) }
            _ => match self.integer() {
                Ok(x) => Ok((a.unwrap(), Node::Integer(x))),
                Err(e) => Err("not a integer".to_string()),
            }
        }
    }

    // reg_imm_or_reg_reg ::= reg_reg | reg_imm
    fn reg_imm_or_reg_reg(&mut self) -> Result<(Register, Node), String> {
        let a = self.register();
        self.whitespace();
        if self.march() != Some(Token::Comma) { return Err("missing ','".to_string()); }
        self.whitespace();

        if a.is_none() { return Err("unknown register".to_string()); }

        match self.peek() {
            Some(Token::Identifier(x)) => { self.march(); Ok((a.unwrap(), Node::Pointer(x.clone()))) },
            Some(Token::Number(_)) | Some(Token::HexNumber(_)) => match self.integer() {
                Ok(x) => Ok((a.unwrap(), Node::Integer(x))),
                Err(e) => Err(e),
            }
            _ => match self.register() {
                Some(reg) => Ok((a.unwrap(), Node::Register(reg))), 
                None => Err("unknown register".to_string()),
            }
        }
    }

    // newline ::= NEWLINE
    fn newline(&mut self) -> Result<Node, Error> {
        self.whitespace();
        let token = self.peek();
        match token {
            Some(Token::Newline) | None => { self.line_no += 1; self.march(); Ok(Node::Newline) }
            _ => self.error("expected new line."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let code = "
entry _start

_msg: 
    db \"Hello World!\",0xA

_start:
    mov ebx, 1      ; stdout
    mov ecx, _msg
    mov edx, 13    ; message length
    mov edi, 5      ; print it 5 times

_loop:
    mov eax, 4  ; write
    int 0x80

    dec edi
    jnz _loop

_exit:
    mov eax, 1      ; exit
    mov ebx, 0      ; status code 0
    int 0x80
";

        let node = Parser::parse(code).unwrap();
        node.print();
    }
}
