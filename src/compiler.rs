use crate::{
    chunk::Chunk,
    opcode::Opcode,
    tokenizer::{Scanner, Token, TokenType},
    vm::CompileErr,
};

pub(crate) fn compile(source: &str, chunk: &mut Chunk) -> Result<(), CompileErr> {
    let mut parser = Parser {
        current_chunk: chunk,
        scanner: Scanner::new(source),
        current: None,
        previous: None,
    };
    // Get the scanner started.
    parser.advance()?;
    // Parse an expression.
    // scanner.expression();
    // Validate that we are at the end of the source code.
    parser.consume(TokenType::Eof, "Expected end of expression.")?;
    end_compiler(&mut parser);
    Ok(())
}

fn end_compiler(parser: &mut Parser) {
    parser.emit_return();
}

struct Parser<'src, 'chunk> {
    current_chunk: &'chunk mut Chunk,
    scanner: Scanner<'src>,
    current: Option<Token<'src>>,
    previous: Option<Token<'src>>,
}

impl<'src, 'chunk> Parser<'src, 'chunk> {
    fn advance(&mut self) -> Result<(), CompileErr> {
        self.previous = self.current;
        loop {
            let curr = self.scanner.scan_token();
            let tt = curr.token_type;
            self.current = Some(curr);
            if tt != TokenType::Error {
                break;
            }
            // errorAtCurrent(self.current.start)
            return Err(CompileErr::BadToken {
                line: self.scanner.line,
            });
        }
        Ok(())
    }

    /// Read the next token, validate it has the expected type.
    fn consume(&self, expected: TokenType, msg: &'static str) -> Result<(), CompileErr> {
        match self.current {
            Some(Token { token_type, .. }) if token_type == expected => Ok(()),
            _ => Err(CompileErr::Other {
                line: self.scanner.line,
                msg,
            }),
        }
    }
    fn emit_byte(&mut self, byte: u8) {
        self.current_chunk
            .write(byte, self.previous.map(|t| t.line).unwrap());
    }

    fn emit_bytes(&mut self, a: u8, b: u8) {
        self.emit_byte(a);
        self.emit_byte(b);
    }

    fn emit_return(&mut self) {
        self.emit_byte(Opcode::Return as u8);
    }
}
