use std::fmt::{self, Display};

mod vm;

#[derive(Debug)]
enum Op {
    Return,
    Constant(usize),
}

#[derive(Debug, Clone, Copy)]
struct Value(f64);

#[derive(Debug, Default)]
struct Chunk {
    name: String,
    code: Vec<Op>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new(name: impl Into<String>, code: Vec<Op>, constants: Vec<Value>) -> Self {
        let name = name.into();
        Chunk {
            name,
            code,
            constants,
        }
    }

    fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, index: usize) -> Option<Value> {
        Some(self.constants.get(index)?.clone())
    }

    pub fn disassemble(&self) -> String {
        let mut disassembly = Vec::new();

        for (num, opcode) in self.code.iter().enumerate() {
            disassembly.push(format!("{:0<4} {}", num, self.disassemble_opcode(opcode)));
        }

        disassembly.join("\n")
    }

    pub fn disassemble_opcode(&self, opcode: &Op) -> String {
        match opcode {
            Op::Return => format!("{:?}", opcode),
            Op::Constant(index) => {
                format!("{:?} {:?}", opcode, self.constants.get(*index).unwrap())
            }
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "== {} ==", self.name)?;
        write!(f, "{}", self.disassemble())
    }
}

fn main() {
    let constants = vec![Value(1.2)];
    let bytecode = vec![Op::Constant(0), Op::Return];
    let chunk = Chunk::new("Test chunk", bytecode, constants);
    println!("{}", chunk);
}
