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
        match gen.process_node(root) {
            Ok(_) => {},
            Err(e) => panic!("Error occured: {}", e),
        }

        gen.program.set_entrypoint(&gen.entry_point);
        gen.program
    }

    fn process_node(&mut self, node: ASTNode) -> Result<(), String> {
        match node {
            ASTNode::Program(stmts) => {
                for stmt in stmts {
                    self.process_node(stmt)?;
                }
            }
            ASTNode::Label(label) => {
                self.program.new_block(&label);
                self.current_block += 1;
            }
            ASTNode::Instruction { mnemonic, arguments } => self.process_instr(mnemonic, arguments)?,
            _ => return Err(format!("Unimplemented: {:?}", node)),
        }
        Ok(())
    }

    fn process_instr(&mut self, mnemonic: String, arguments: Vec<ASTNode>) -> Result<(),String> {
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
            "mov" => self.mov(arguments)?,
            "int" => self.int(arguments)?,
            "inc" => self.inc(arguments)?,
            "dec" => self.dec(arguments)?,
            "jmp" => self.jmp(JumpCondition::None, arguments)?,
            "jo" => self.jmp(JumpCondition::Overflow, arguments)?,
            "jno" => self.jmp(JumpCondition::NotOverflow, arguments)?,
            "jb" | "jnae" | "jc" => self.jmp(JumpCondition::Carry, arguments)?,
            "jnb" | "jae" | "jnc" => self.jmp(JumpCondition::NotCarry, arguments)?,
            "jz" | "je" => self.jmp(JumpCondition::Zero, arguments)?,
            "jnz" | "jne" => self.jmp(JumpCondition::NotZero, arguments)?,
            "jbe" | "jna" => self.jmp(JumpCondition::CarryOrZero, arguments)?,
            "jnbe" | "ja" => self.jmp(JumpCondition::NotCarryAndNotZero, arguments)?,
            "js" => self.jmp(JumpCondition::Sign, arguments)?,
            "jns" => self.jmp(JumpCondition::NotSign, arguments)?,
            "jp" | "jpe" => self.jmp(JumpCondition::Parity, arguments)?,
            "jnp" | "jpo" => self.jmp(JumpCondition::NotParity, arguments)?,
            "jl" | "jnge" => self.jmp(JumpCondition::Less, arguments)?,
            "jnl" | "jge" => self.jmp(JumpCondition::NotLess, arguments)?,
            "jle" | "jng" => self.jmp(JumpCondition::NotGreater, arguments)?,
            "jnle" | "jg" => self.jmp(JumpCondition::Greater, arguments)?,
            "add" => self.add(arguments)?,
            "sub" => self.sub(arguments)?,
            "bswap" => self.bswap(arguments)?,
            "and" => self.and(arguments)?,
            "or" => self.or(arguments)?,
            "xor" => self.xor(arguments)?,
            _ => {
                return Err(format!("unknown instruction '{}'.", mnemonic));
            }
        }

        Ok(())
    }

    fn mov(&mut self, arguments: Vec<ASTNode>) -> Result<(), String> {
        if arguments.len() != 2 {
            return Err(format!("wrong number of arguments passed to mov (got {}, expected 2).", arguments.len()));
        }

        match &arguments[0] {
            ASTNode::Identifier(_) => {
                let register = get_register(&arguments[0])?;
                if let Ok(reg2) = get_register(&arguments[1]) {

                }
                else {
                    let value = match &arguments[1] {
                        ASTNode::Number(x) => if register.bits() == 32 { Value::UInt(*x) } else { Value::UByte(*x as u8) },
                        ASTNode::Identifier(s) => Value::Pointer(s.to_string()),
                        _ => return Err("second argument of mov is invalid, expected number or label.".to_string())
                    };
                    self.program.get_block_mut(self.current_block).unwrap().push(
                        Instruction::MovImmediate { register, value }
                    );
                }
            }
            ASTNode::Address(arg0) => {
                let mut addr = None;
                let arg0 = (**arg0).clone();
                
                match arg0 {
                    ASTNode::Identifier(ref s) => {
                        if let Ok(dest) = get_register(&arg0) {
                            let src = get_register(&arguments[1])?;                         
                            self.program.get_block_mut(self.current_block).unwrap().push(
                                Instruction::MovRM32R32 { dest, src }
                            );
                            return Ok(())
                        } else {
                            addr = Some(Value::Pointer(s.to_string()));
                        }
                    }
                    ASTNode::Number(x) => addr = Some(Value::UInt(x)),
                    _ => return Err(format!("invalid address given to mov ({:?})", arg0)),
                }

                self.program.get_block_mut(self.current_block).unwrap().push(
                    Instruction::MovMemory { addr: addr.unwrap(), register: get_register(&arguments[1])? }
                );
            }
            _ => return Err(format!("invalid first argument to mov ({:?})", arguments[0]))
        }
        

        
        Ok(())
    }

    fn int(&mut self, arguments: Vec<ASTNode>) -> Result<(), String> {
        if arguments.len() != 1 {
            return Err(format!("wrong number of arguments passed to int (got {}, expected 1).", arguments.len()));
        }

        let value = match &arguments[0] {
            ASTNode::Number(x) => x,
            _ => return Err("argument of int is invalid, expected byte.".to_string())
        };

        self.program.get_block_mut(self.current_block).unwrap().push(Instruction::Int(*value as u8));
        Ok(())
    }

    fn inc(&mut self, arguments: Vec<ASTNode>) -> Result<(),String> {
        if arguments.len() != 1 {
            return Err(format!("wrong number of arguments passed to inc (got {}, expected 1).", arguments.len()));
        }
        let register = get_register(&arguments[0])?;
        self.program.get_block_mut(self.current_block).unwrap().push(Instruction::Inc(register));
        Ok(())
    }

    fn dec(&mut self, arguments: Vec<ASTNode>) -> Result<(),String> {
        if arguments.len() != 1 {
            return Err(format!("wrong number of arguments passed to dec (got {}, expected 1).", arguments.len()));
        }
        let register = get_register(&arguments[0])?;
        self.program.get_block_mut(self.current_block).unwrap().push(Instruction::Dec(register));
        Ok(())
    }

    fn jmp(&mut self, condition: JumpCondition, arguments: Vec<ASTNode>) -> Result<(), String> {
        if arguments.len() != 1 {
            return Err(format!("wrong number of arguments passed to jmp (got {}, expected 1).", arguments.len()));
        }

        let value = match &arguments[0] {
            ASTNode::Number(x) => Value::UInt(*x),
            ASTNode::Identifier(s) => Value::RelPointer(s.to_string()),
            _ => return Err("first argument of mov is invalid, expected number or label.".to_string())
        };

        self.program.get_block_mut(self.current_block).unwrap().push(
            Instruction::Jump { condition, addr: value }
        );
        Ok(())
    }
    
    fn add(&mut self, arguments: Vec<ASTNode>) -> Result<(), String> {
        if arguments.len() != 2 {
            return Err(format!("wrong number of arguments passed to add (got {}, expected 2).", arguments.len()));
        }

        let register = get_register(&arguments[0])?;
        let value = match &arguments[1] {
            ASTNode::Number(x) => if register.bits() == 8 { Value::UByte(*x as u8) } else { Value::UInt(*x) },
            ASTNode::Identifier(s) => Value::Pointer(s.to_string()),
            _ => return Err("second argument of add is invalid, expected number or label.".to_string())
        };

        self.program.get_block_mut(self.current_block).unwrap().push(
            Instruction::AddImmediate { register, value }
        );
        Ok(())
    }

    fn sub(&mut self, arguments: Vec<ASTNode>) -> Result<(), String> {
        if arguments.len() != 2 {
            return Err(format!("wrong number of arguments passed to sub (got {}, expected 2).", arguments.len()));
        }

        let register = get_register(&arguments[0])?;
        let value = match &arguments[1] {
            ASTNode::Number(x) => if register.bits() == 8 { Value::UByte(*x as u8) } else { Value::UInt(*x) },
            ASTNode::Identifier(s) => Value::Pointer(s.to_string()),
            _ => return Err("second argument of sub is invalid, expected number or label.".to_string())
        };

        self.program.get_block_mut(self.current_block).unwrap().push(
            Instruction::SubImmediate { register, value }
        );
        Ok(())
    }

    fn bswap(&mut self, arguments: Vec<ASTNode>) -> Result<(),String> {
        if arguments.len() != 1 {
            return Err(format!("wrong number of arguments passed to bswap (got {}, expected 1).", arguments.len()));
        }
        let register = get_register(&arguments[0])?;
        self.program.get_block_mut(self.current_block).unwrap().push(Instruction::ByteSwap(register));
        Ok(())
    }

    fn and(&mut self, arguments: Vec<ASTNode>) -> Result<(),String> {
        if arguments.len() != 2 {
            return Err(format!("wrong number of arguments passed to and (got {}, expected 2).", arguments.len()));
        }
        let a = get_register(&arguments[0])?;
        let b = get_register(&arguments[1])?;
        self.program.get_block_mut(self.current_block).unwrap().push(Instruction::And(a, b));
        Ok(())
    }

    fn or(&mut self, arguments: Vec<ASTNode>) -> Result<(),String> {
        if arguments.len() != 2 {
            return Err(format!("wrong number of arguments passed to or (got {}, expected 2).", arguments.len()));
        }
        let a = get_register(&arguments[0])?;
        let b = get_register(&arguments[1])?;
        self.program.get_block_mut(self.current_block).unwrap().push(Instruction::Or(a, b));
        Ok(())
    }

    fn xor(&mut self, arguments: Vec<ASTNode>) -> Result<(),String> {
        if arguments.len() != 2 {
            return Err(format!("wrong number of arguments passed to xor (got {}, expected 2).", arguments.len()));
        }
        let a = get_register(&arguments[0])?;
        let b = get_register(&arguments[1])?;
        self.program.get_block_mut(self.current_block).unwrap().push(Instruction::XOr(a, b));
        Ok(())
    }
}

fn get_register(argument: &ASTNode) -> Result<Register, String> {
    match argument {
        ASTNode::Identifier(s) => match Register::try_from(s.clone()) {
            Ok(r) => Ok(r),
            Err(s) => Err(s),
        },
        _ => Err("not an identifier.".to_string()), 
    }
}

