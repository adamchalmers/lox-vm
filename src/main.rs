mod chunk;
mod opcode;
mod tokenizer;
mod vm;

use chunk::Chunk;
use opcode::Opcode;
use vm::Vm;

fn main() {
    let mut chunk = Chunk::default();
    let constant = chunk.add_constant(Value::from(1.2));
    chunk.write(Opcode::Constant as u8, 123);
    chunk.write(constant, 123);
    chunk.write(Opcode::Return as u8, 123);
    chunk.disassemble("test chunk");

    let mut vm = Vm::default();
    vm.init();
    if let Err(e) = vm.interpret(chunk) {
        eprintln!("Error:\n{e:?}");
    }
}

#[derive(Debug, Clone)]
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
