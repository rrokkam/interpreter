use crate::chunk::{Chunk, Op, Value};

#[derive(Debug)]
pub enum InterpretError {
    Compile,
    Runtime,
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
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
            let instruction = self.next_instruction()?;

            //#[cfg(debug)]
            {
                self.chunk.disassemble_op(&instruction);
                println!("{:?}", self.value_stack);
            }
            match instruction {
                Op::Return => return self.op_return(),
                Op::Constant(index) => self.op_constant(index)?,
                Op::Negate => self.op_negate()?,
            }
        }
    }

    fn op_return(&mut self) -> Result<(), InterpretError> {
        println!(
            "Returning {:?}",
            self.value_stack.pop().ok_or(InterpretError::Runtime)?
        );
        Ok(())
    }

    fn op_constant(&mut self, index: usize) -> Result<(), InterpretError> {
        self.value_stack.push(
            self.chunk
                .get_constant(index)
                .ok_or(InterpretError::Runtime)?,
        );
        println!("op_constant {:?}", self.chunk.get_constant(index).unwrap());
        Ok(())
    }

    fn op_negate(&mut self) -> Result<(), InterpretError> {
        let value = self.value_stack.pop().ok_or(InterpretError::Runtime)?;
        Ok(self.value_stack.push(-value))
    }

    fn next_instruction(&mut self) -> Result<Op, InterpretError> {
        let op = self.chunk.code(self.ip).ok_or(InterpretError::Runtime)?;
        self.ip += 1;
        Ok(op)
    }
}
