use crate::chunk::{Chunk, Value};
//use crate::vm::VM;
use std::error::Error;

mod chunk;
//mod vm;

fn main() -> Result<(), Box<dyn Error>> {
    let chunk = Chunk::builder("Test chunk")
        .op_constant(Value::new(1.2))
        .op_return()
        .build();

    println!("{:?}", chunk);

    //    let mut vm = VM::new(chunk);
    //    vm.interpret()?;
    Ok(())
}
