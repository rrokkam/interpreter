use crate::chunk::{Chunk, Value};
use crate::vm::{InterpretError, VM};

mod chunk;
mod vm;

fn main() -> Result<(), InterpretError> {
    let chunk = Chunk::builder("Test chunk")
        .op_constant(3, Value::new(1.2))
        .op_return(4)
        .build();

    println!("{:?}", chunk);

    VM::new(chunk).interpret()
}
