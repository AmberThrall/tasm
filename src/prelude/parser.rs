use pest::Parser;
use std::u32;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar="prelude/grammar.pest"]
pub struct ASMParser;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Label(String),
    Instruction {
        mnemonic: String,
        arguments: Vec<ASTNode>,
    },
    Address(Box<ASTNode>),
    Identifier(String),
    Number(u32),
    String(String)
}


pub fn parse(source: &str) -> Result<ASTNode, pest::error::Error<Rule>> {
    let pairs = ASMParser::parse(Rule::program, source)?.next().unwrap();
    Ok(build_ast(pairs))
}

fn build_ast(pair: Pair<Rule>) -> ASTNode {
    match pair.as_rule() {
        Rule::program => {
            let mut statements = Vec::new();
            for record in pair.into_inner() {
                match record.as_rule() {
                    Rule::statement => statements.push(build_ast(record)),
                    _ => (),
                }
            }
            ASTNode::Program(statements)
        },
        Rule::statement => {
            let child = pair.into_inner().next().unwrap();
            build_ast(child)
        },
        Rule::label => {
            let label = pair.into_inner().next().unwrap();
            ASTNode::Label(label.as_str().to_string())
        },
        Rule::instruction => {
            let mut pairs = pair.into_inner();
            let ident = pairs.next().unwrap();

            if let Some(pairs) = pairs.next() {
                let mut arguments = Vec::new();
                for record in pairs.into_inner() {
                    arguments.push(build_ast(record));
                }
                ASTNode::Instruction { mnemonic: ident.as_str().to_string(), arguments } 
            } else {
                ASTNode::Instruction { mnemonic: ident.as_str().to_string(), arguments: Vec::new() }
            }
        },
        Rule::argument => build_ast(pair.into_inner().next().unwrap()),
        Rule::value => {
            let pair = pair.into_inner().next().unwrap();
            match pair.as_rule() {
                Rule::identifier => ASTNode::Identifier(pair.as_str().to_string()),
                Rule::address => {
                    let pair = pair.into_inner().next().unwrap();
                    ASTNode::Address(Box::new(build_ast(pair)))
                }
                Rule::number => ASTNode::Number(pair.as_str().parse::<i64>().unwrap() as u32),
                Rule::hex_number => {
                    let s = pair.as_str().to_string();
                    let without_prefix = s.trim_start_matches("0x");
                    let x = u32::from_str_radix(without_prefix, 16).unwrap();
                    ASTNode::Number(x)
                },
                Rule::string => {
                    let s = pair.as_str();
                    ASTNode::String(s[1..s.len()-1].to_string())
                },
                _ => panic!("Something went wrong, got a {:?}", pair)
            }
        },
        _ => panic!("Something wrong happened (got a {:?}).", pair)
    }
}

