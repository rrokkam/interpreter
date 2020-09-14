use crate::chunk::Chunk;
use crate::chunk::Op;
use crate::lexer::{Token, Tokens};
use crate::InterpretError;
use std::iter::Peekable;

struct Parser<'source> {
    tokens: Peekable<Tokens<'source>>,
    had_error: bool,
    panicking: bool,
}

// Pratt's parsing algorithm: https://craftinginterpreters.com/compiling-expressions.html
pub fn compile(source: &str) -> Result<Chunk, InterpretError> {
    for token in Tokens::from(source) {
        expression(token);
    }

    Err(InterpretError::Compile)
}

fn expression(token: Token) {}

impl Parser<'_> {
    fn error(&mut self, token: &Token, message: &str) {
        self.had_error = true;
        if self.panicking {
            return;
        }
        eprintln!("Got an error at {:?}", token);
        eprintln!("{}", message);
    }
}

impl Iterator for Parser<'_> {
    type Item = Op;
    fn next(&mut self) -> Option<Op> {
        Some(Op::Return)
    }
}
