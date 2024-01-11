mod tokenizer;

fn main() {
    let mut chunk = Chunk::default();
    let constant = chunk.add_constant(Value::from(1.2));
    chunk.write(Opcode::Constant as u8, 123);
    chunk.write(constant, 123);
    chunk.write(Opcode::Return as u8, 123);
    chunk.disassemble("test chunk");
}

#[derive(Debug)]
#[repr(u8)]
enum Opcode {
    Constant,
    Return,
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for Opcode {
    type Error = CouldNotDecodeOpcode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let opcode = match value {
            0 => Self::Constant,
            1 => Self::Return,
            _ => return Err(CouldNotDecodeOpcode(value)),
        };
        Ok(opcode)
    }
}

#[derive(Debug)]
struct CouldNotDecodeOpcode(u8);

#[derive(Default)]
struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    fn len(&self) -> usize {
        self.code.len()
    }
    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        (self.constants.len() - 1) as u8
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {name} ==");
        let mut offset = 0usize;
        while offset < self.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");

        // Print the line information.
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ")
        } else {
            print!("{:4} ", self.lines[offset])
        }

        match Opcode::try_from(self.code[offset]) {
            Ok(Opcode::Return) => simple_instruction("OP_RETURN", offset),
            Ok(Opcode::Constant) => self.constant_instruction("OP_CONSTANT", offset),
            Err(CouldNotDecodeOpcode(x)) => {
                println!("Unknown opcode {x}");
                offset + 1
            }
        }
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant: u8 = self.code[offset + 1];
        print!("{name:<16} {constant:4} ");
        self.constants[constant as usize].print();
        println!();
        offset + 2
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name:<16}");
    offset + 1
}

#[derive(Debug)]
struct Value(f64);

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl Value {
    fn print(&self) {
        print!("'{}'", self.0);
    }
}
