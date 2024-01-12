use crate::{
    opcode::{CouldNotDecodeOpcode, Opcode},
    value::Value,
};

#[derive(Default, Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    fn len(&self) -> usize {
        self.code.len()
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        (self.constants.len() - 1) as u8
    }

    #[allow(dead_code)]
    pub fn disassemble(&self, name: &str) {
        println!("== {name} ==");
        let mut offset = 0usize;
        while offset < self.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");

        // Print the line information.
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ")
        } else {
            print!("{:4} ", self.lines[offset])
        }

        match Opcode::try_from(self.code[offset]) {
            Ok(Opcode::Return) => simple_instruction("OP_RETURN", offset),
            Ok(Opcode::Negate) => simple_instruction("OP_NEGATE", offset),
            Ok(Opcode::Add) => simple_instruction("OP_ADD", offset),
            Ok(Opcode::Sub) => simple_instruction("OP_SUBTRACT", offset),
            Ok(Opcode::Mul) => simple_instruction("OP_MULTIPLY", offset),
            Ok(Opcode::Div) => simple_instruction("OP_DIVIDE", offset),
            Ok(Opcode::Constant) => self.constant_instruction("OP_CONSTANT", offset),
            Err(CouldNotDecodeOpcode { opcode }) => {
                println!("Unknown opcode {opcode}");
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
