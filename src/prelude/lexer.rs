use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Symbols
    #[token(",")]
    Comma,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token(":")]
    Colon,

    // Pseudo-instructions
    #[token("entry")]
    Entry,
    #[token("db")]
    Db,

    // Instructions
    #[token("mov")]
    Mov,
    #[token("int")]
    Int,
    #[token("inc")]
    Inc,
    #[token("dec")]
    Dec,
    #[token("jmp")]
    Jmp,
    #[token("jo")]
    JO,
    #[token("jno")]
    JNO,
    #[token("jb")]
    JB,
    #[token("jnae")]
    JNAE,
    #[token("jc")]
    JC,
    #[token("jnb")]
    JNB,
    #[token("jae")]
    JAE,
    #[token("jnc")]
    JNC,
    #[token("jz")]
    JZ,
    #[token("je")]
    JE,
    #[token("jnz")]
    JNZ,
    #[token("jne")]
    JNE,
    #[token("jbe")]
    JBE,
    #[token("jna")]
    JNA,
    #[token("jnbe")]
    JNBE,
    #[token("ja")]
    JA,
    #[token("js")]
    JS,
    #[token("jns")]
    JNS,
    #[token("jp")]
    JP,
    #[token("jpe")]
    JPE,
    #[token("jnp")]
    JNP,
    #[token("jpo")]
    JPO,
    #[token("jl")]
    JL,
    #[token("jnge")]
    JNGE,
    #[token("jnl")]
    JNL,
    #[token("jge")]
    JGE,
    #[token("jle")]
    JLE,
    #[token("jng")]
    JNG,
    #[token("jnle")]
    JNLE,
    #[token("jg")]
    JG,
    #[token("add")]
    Add,
    #[token("sub")]
    Sub,
    #[token("mul")]
    Mul,
    #[token("div")]
    Div,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("xor")]
    Xor,
    #[token("cmp")]
    CMP,
    #[token("bswap")]
    BSWAP,
    #[token("push")]
    Push,
    #[token("pop")]
    Pop,
    #[token("call")]
    Call,
    #[token("ret")]
    Ret,

    // Registers
    #[token("ah")] 
    AH,
    #[token("al")]
    AL, 
    #[token("bh")]
    BH, 
    #[token("bl")] 
    BL, 
    #[token("ch")]
    CH, 
    #[token("cl")]
    CL, 
    #[token("dh")]
    DH, 
    #[token("dl")]
    DL,
    #[token("ax")]
    AX,
    #[token("cx")]
    CX,
    #[token("dx")]
    DX,
    #[token("bx")]
    BX,
    #[token("sp")]
    SP,
    #[token("bp")]
    BP,
    #[token("si")]
    SI,
    #[token("di")]
    DI,
    #[token("eax")]
    EAX,
    #[token("ebx")]
    EBX,
    #[token("ecx")]
    ECX,
    #[token("esp")]
    ESP,
    #[token("ebp")]
    EBP,
    #[token("edi")]
    EDI,
    #[token("esi")]
    ESI,
    #[token("edx")]
    EDX,



    // Whitespace
    #[regex(r"(;.*)?[\n\r]")]
    Newline,
    #[regex(r"[ \t\f]+")]
    Whitespace,

    // Values
    #[regex("[_a-zA-Z][_a-zA-Z0-9]*", |lex| lex.slice().to_owned())]
    Identifier(String),
    #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, |lex| let s = lex.slice().to_owned(); s[1..s.len()-1].to_string())]
    String(String),
    #[regex("0x[0-9A-Fa-f]+", |lex| let s = lex.slice().to_owned(); u64::from_str_radix(&s[2..], 16).unwrap())]
    HexNumber(u64),
    #[regex("-?[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    Number(i64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer() {
        let mut lex = Token::lexer("_msg:
    db \"Hello World!\", 0xA
_start:
    mov eax, 5 ; eax <- 5
    int 0x80");

        assert_eq!(lex.next(), Some(Ok(Token::Identifier("_msg".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Colon)));
        assert_eq!(lex.next(), Some(Ok(Token::Newline)));
        assert_eq!(lex.next(), Some(Ok(Token::Whitespace)));
        assert_eq!(lex.next(), Some(Ok(Token::Db)));
        assert_eq!(lex.next(), Some(Ok(Token::Whitespace)));
        assert_eq!(lex.next(), Some(Ok(Token::String("Hello World!".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Comma)));
        assert_eq!(lex.next(), Some(Ok(Token::Whitespace)));
        assert_eq!(lex.next(), Some(Ok(Token::HexNumber(0xA))));
        assert_eq!(lex.next(), Some(Ok(Token::Newline)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier("_start".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Colon)));
        assert_eq!(lex.next(), Some(Ok(Token::Newline)));
        assert_eq!(lex.next(), Some(Ok(Token::Whitespace)));
        assert_eq!(lex.next(), Some(Ok(Token::Mov)));
        assert_eq!(lex.next(), Some(Ok(Token::Whitespace)));
        assert_eq!(lex.next(), Some(Ok(Token::EAX)));
        assert_eq!(lex.next(), Some(Ok(Token::Comma)));
        assert_eq!(lex.next(), Some(Ok(Token::Whitespace)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(5))));
        assert_eq!(lex.next(), Some(Ok(Token::Whitespace)));
        assert_eq!(lex.next(), Some(Ok(Token::Newline)));
        assert_eq!(lex.next(), Some(Ok(Token::Whitespace)));
        assert_eq!(lex.next(), Some(Ok(Token::Int)));
        assert_eq!(lex.next(), Some(Ok(Token::Whitespace)));
        assert_eq!(lex.next(), Some(Ok(Token::HexNumber(0x80))));
        assert_eq!(lex.next(), None);
    }
}
