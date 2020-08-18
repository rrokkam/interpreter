use std::fmt::{self, Debug};

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Return,
    Constant(usize),
}

#[derive(Debug, Clone, Copy)]
pub struct Value(f64);

impl Value {
    pub fn new(value: f64) -> Self {
        Value(value)
    }
}

#[derive(Default)]
pub struct ChunkBuilder {
    name: String,
    code: Vec<Op>,
    constants: Vec<Value>,
}

impl ChunkBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn build(self) -> Chunk {
        Chunk {
            name: self.name,
            code: self.code,
            constants: self.constants,
        }
    }

    pub fn op_return(mut self) -> Self {
        self.code.push(Op::Return);
        self
    }

    pub fn op_constant(mut self, value: Value) -> Self {
        self.code.push(Op::Constant(self.constants.len()));
        self.constants.push(value);
        self
    }
}

pub struct Chunk {
    name: String,
    code: Vec<Op>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn builder(name: impl Into<String>) -> ChunkBuilder {
        ChunkBuilder::new(name)
    }

    pub fn get_constant(&self, index: usize) -> Option<Value> {
        Some(*self.constants.get(index)?)
    }

    pub fn disassemble(&self) -> String {
        let mut disassembly = Vec::new();

        for (num, opcode) in self.code.iter().enumerate() {
            disassembly.push(format!("{:0<4} {}", num, self.disassemble_op(opcode)));
        }

        disassembly.join("\n")
    }

    fn disassemble_op(&self, op: &Op) -> String {
        match op {
            Op::Return => format!("{:?}", op),
            Op::Constant(index) => format!("{:?} {:?}", op, self.get_constant(*index).unwrap()),
        }
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "== {} ==", self.name)?;
        write!(f, "{}", self.disassemble())
    }
}
