use crate::chunk::Chunk;
use crate::lexer::{Token, Tokens};
use crate::InterpretError;

// Pratt's parsing algorithm: https://craftinginterpreters.com/compiling-expressions.html
pub fn compile(source: &str) -> Result<Chunk, InterpretError> {
    for token in Tokens::from(source) {
        expression(token);
    }

    Err(InterpretError::Compile)
}

fn expression(token: Token) {}
