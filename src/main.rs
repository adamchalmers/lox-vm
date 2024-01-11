mod chunk;
mod opcode;
mod tokenizer;

use chunk::Chunk;
use opcode::Opcode;

fn main() {
    let mut chunk = Chunk::default();
    let constant = chunk.add_constant(Value::from(1.2));
    chunk.write(Opcode::Constant as u8, 123);
    chunk.write(constant, 123);
    chunk.write(Opcode::Return as u8, 123);
    chunk.disassemble("test chunk");
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
