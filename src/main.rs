use std::fmt::{self, Display};

#[derive(Debug)]
enum OpCode {
    Return,
    Constant(usize),
}

impl Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let name = match self {
            Self::Return => "OP_RETURN".to_string(),
            Self::Constant(index) => format!("OP_CONSTANT {:0<4}", index),
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug)]
struct Value(f64);

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default)]
struct Chunk {
    name: String,
    code: Vec<OpCode>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new(name: impl Into<String>, code: Vec<OpCode>, constants: Vec<Value>) -> Self {
        let name = name.into();
        Chunk {
            name,
            code,
            constants,
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "== {} ==", self.name)?;
        for (num, opcode) in self.code.iter().enumerate() {
            match opcode {
                OpCode::Return => writeln!(f, "{:0<4} {}", num, opcode)?,
                OpCode::Constant(index) => writeln!(
                    f,
                    "{:0<4} {} {}",
                    num,
                    opcode,
                    self.constants.get(*index).unwrap()
                )?,
            }
        }
        Ok(())
    }
}

fn main() {
    let constants = vec![Value(1.2)];
    let bytecode = vec![OpCode::Constant(0), OpCode::Return];
    let chunk = Chunk::new("Test chunk", bytecode, constants);
    println!("{}", chunk);
}
