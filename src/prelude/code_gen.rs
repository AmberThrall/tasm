use super::*;

pub struct CodeGenerator {
    program: Program,
    entry_point: String,
    current_block: usize,
}

impl CodeGenerator {
    pub fn generate(root: ASTNode) -> Program {
        let mut gen = CodeGenerator { 
            program: Program::new(), 
            entry_point: "__entry_point__".to_string(),
            current_block: 0, 
        };

        gen.program.new_block("__entry_point__");
        gen.process_node(root);

        gen.program.set_entrypoint(&gen.entry_point);

        gen.program
    }

    fn process_node(&mut self, node: ASTNode) {
        match node {
            ASTNode::Program(stmts) => {
                for stmt in stmts {
                    self.process_node(stmt);
                }
            }
            ASTNode::Label(label) => {
                self.program.new_block(&label);
                self.current_block += 1;
            }
            ASTNode::Instruction { mnemonic, arguments } => self.process_instr(mnemonic, arguments),
            _ => println!("Unimplemented: {:?}", node),
        }
    }

    fn process_instr(&mut self, mnemonic: String, arguments: Vec<ASTNode>) {
        match mnemonic.as_str() {
            "global" => {
                if arguments.len() != 1 {
                    panic!("Error: wrong number of arguments passed to global.");
                }

                match &arguments[0] {
                    ASTNode::Identifier(label) => self.entry_point = label.to_string(),
                    _ => panic!("Error: invalid argument passed to global, expected label."),
                }
            },
            "db" => {
                let data: Vec<Vec<u8>> = arguments.iter().map(|x| {
                    match x {
                        ASTNode::Number(v) => vec![*v as u8],
                        ASTNode::String(v) => v.as_bytes().to_vec(),
                        _ => panic!("Error: invalid argument passed to db."), 
                    }
                }).collect();

                for d in data {
                    self.program.get_block_mut(self.current_block).unwrap().push(Instruction::RawData(d));
                }
            },
            "mov" => self.process_mov(arguments),
            "int" => self.process_int(arguments),
            "dec" => self.process_dec(arguments),
            "jmp" => self.process_jmp(JumpCondition::None, arguments),
            "jo" => self.process_jmp(JumpCondition::Overflow, arguments),
            "jno" => self.process_jmp(JumpCondition::NotOverflow, arguments),
            "jb" | "jnae" | "jc" => self.process_jmp(JumpCondition::Carry, arguments),
            "jnb" | "jae" | "jnc" => self.process_jmp(JumpCondition::NotCarry, arguments),
            "jz" | "je" => self.process_jmp(JumpCondition::Zero, arguments),
            "jnz" | "jne" => self.process_jmp(JumpCondition::NotZero, arguments),
            "jbe" | "jna" => self.process_jmp(JumpCondition::CarryOrZero, arguments),
            "jnbe" | "ja" => self.process_jmp(JumpCondition::NotCarryAndNotZero, arguments),
            "js" => self.process_jmp(JumpCondition::Sign, arguments),
            "jns" => self.process_jmp(JumpCondition::NotSign, arguments),
            "jp" | "jpe" => self.process_jmp(JumpCondition::Parity, arguments),
            "jnp" | "jpo" => self.process_jmp(JumpCondition::NotParity, arguments),
            "jl" | "jnge" => self.process_jmp(JumpCondition::Less, arguments),
            "jnl" | "jge" => self.process_jmp(JumpCondition::NotLess, arguments),
            "jle" | "jng" => self.process_jmp(JumpCondition::NotGreater, arguments),
            "jnle" | "jg" => self.process_jmp(JumpCondition::Greater, arguments),
            _ => {
                println!("Error: unknown instruction '{}'.", mnemonic);
            }
        }
    }

    fn process_mov(&mut self, arguments: Vec<ASTNode>) {
        if arguments.len() != 2 {
            panic!("Error: wrong number of arguments passed to mov.");
        }

        let register = match &arguments[0] {
            ASTNode::Identifier(s) => Register::try_from(s.clone()).expect("error: unknown register."),
            _ => panic!("Error: first argument of mov is invalid, expected register.")
        };

        let value = match &arguments[1] {
            ASTNode::Number(x) => Value::UInt(*x),
            ASTNode::Identifier(s) => Value::Pointer(s.to_string()),
            _ => panic!("Error: first argument of mov is invalid, expected number or label.")
        };

        self.program.get_block_mut(self.current_block).unwrap().push(
            Instruction::MovImmediate { register, value }
        );
    }

    fn process_int(&mut self, arguments: Vec<ASTNode>) {
        if arguments.len() != 1 {
            panic!("Error: wrong number of arguments passed to int.");
        }

        let value = match &arguments[0] {
            ASTNode::Number(x) => x,
            _ => panic!("Error: argument of int is invalid, expected byte.")
        };

        self.program.get_block_mut(self.current_block).unwrap().push(Instruction::Int(*value as u8));
    }

    fn process_dec(&mut self, arguments: Vec<ASTNode>) {
        if arguments.len() != 1 {
            panic!("Error: wrong number of arguments passed to int.");
        }

        let register = match &arguments[0] {
            ASTNode::Identifier(s) => Register::try_from(s.clone()).expect("error: unknown register."),
            _ => panic!("Error: argument of int is invalid, expected byte.")
        };

        self.program.get_block_mut(self.current_block).unwrap().push(Instruction::Dec(register));
    }

    fn process_jmp(&mut self, condition: JumpCondition, arguments: Vec<ASTNode>) {
        if arguments.len() != 1 {
            panic!("Error: wrong number of arguments passed to jmp.");
        }

        let value = match &arguments[0] {
            ASTNode::Number(x) => Value::UInt(*x),
            ASTNode::Identifier(s) => Value::RelPointer(s.to_string()),
            _ => panic!("Error: first argument of mov is invalid, expected number or label.")
        };

        self.program.get_block_mut(self.current_block).unwrap().push(
            Instruction::Jump { condition, addr: value }
        );
    }
}
