use crate::{Chunk, Op, Value};

enum InterpretError {
    CompileTime,
    RunTime,
}

struct VM {
    chunk: Chunk,
    ip: u8,
    value_stack: Vec<Value>,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            value_stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self) -> Result<(), InterpretError> {
        loop {
            match self.next_instruction() {
                Op::Return => {
                    // Debug printout for when we return from a function.
                    println!("{:?}", self.value_stack.pop().ok_or(InterpretError::RunTime)?);
                    return Ok(());
                }
                Op::Constant(index) => {
                    self.value_stack
                        .push(self.chunk.get_constant(index).ok_or(InterpretError::RunTime)?);
                }
            }
        }
    }

    fn next_instruction(&self) -> Op {
        Op::Return
    }
}
