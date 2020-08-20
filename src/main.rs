#![feature(peekable_next_if)]

use crate::chunk::{Chunk, Value};
use crate::vm::{InterpretError, VM};
use std::fs;
use std::io::{self, BufRead, Write};

mod chunk;
mod lexer;
mod vm;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => usage(),
    }?;
    old_main().unwrap();
    Ok(())
}

fn repl() -> Result<(), std::io::Error> {
    print!("> ");
    let mut stdout = io::stdout();
    stdout.flush()?;
    for line in io::stdin().lock().lines() {
        interpret(line?).unwrap();
        print!("> ");
        stdout.flush()?;
    }
    println!();
    Ok(())
}

fn run_file(filename: &str) -> Result<(), std::io::Error> {
    let source = fs::read_to_string(filename)
        .map_err(|_| std::process::exit(74))
        .unwrap();
    interpret(source).map_err(|err| match err {
        InterpretError::Compile => std::process::exit(65),
        InterpretError::Runtime => std::process::exit(70),
    })
}

fn interpret(line: String) -> Result<(), InterpretError> {
    println!("You typed: {}", line);
    Ok(())
}

fn usage() -> ! {
    eprintln!("Usage: cargo run [path]");
    std::process::exit(64);
}

fn old_main() -> Result<(), InterpretError> {
    let chunk = Chunk::builder("Test chunk")
        .op_constant(3, Value::new(1.2))
        .op_negate(4)
        .op_return(5)
        .build();

    println!("{:?}", chunk);

    VM::new(chunk).interpret()
}
