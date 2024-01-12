use crate::tokenizer::{Scanner, TokenType};

pub(crate) fn compile(source: &str) {
    let mut scanner = Scanner::new(source);
    let mut line: Option<usize> = None;
    loop {
        let token = scanner.scan_token();
        if Some(token.line) != line {
            print!("{:4} ", token.line);
            line = Some(token.line);
        } else {
            print!("   | ")
        }
        println!("{:2} '{}'", token.token_type as u8, token.lexeme);
        if token.token_type == TokenType::Eof {
            break;
        }
    }
}
