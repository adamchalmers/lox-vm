use crate::{
    chunk::Chunk,
    opcode::{CouldNotDecodeOpcode, Opcode},
    Value,
};

#[derive(Debug, Default)]
pub struct Vm {
    chunk: Chunk,
    /// Instruction pointer
    ip: usize,
    stack: Vec<Value>,
}

impl Vm {
    pub fn init(&mut self) {
        self.stack.clear();
    }

    pub fn interpret(&mut self, chunk: Chunk) -> Result<(), Error> {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    fn run(&mut self) -> Result<(), Error> {
        loop {
            if cfg!(feature = "trace") {
                print!("          ");
                for slot in &self.stack {
                    print!("[");
                    slot.print();
                    print!("]");
                }
                println!();
                self.chunk.disassemble_instruction(self.ip);
            }
            let instruction = self.read_byte();
            match Opcode::try_from(instruction) {
                Ok(Opcode::Return) => {
                    let val = self.stack.pop().ok_or(RuntimeErr::StackEmpty)?;
                    val.print();
                    println!();
                    return Ok(());
                }
                Ok(Opcode::Constant) => {
                    let constant = self.read_constant().clone();
                    self.stack.push(constant);
                }
                Err(e) => return Err(RuntimeErr::from(e).into()),
            }
        }
    }

    fn read_constant(&mut self) -> &Value {
        let i = self.read_byte();
        &self.chunk.constants[i as usize]
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[allow(dead_code)]
    #[error("Compile error")]
    Compile,
    #[error("Runtime error: {0}")]
    Runtime(#[from] RuntimeErr),
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeErr {
    #[error("{0}")]
    CouldNotDecodeOpcode(#[from] CouldNotDecodeOpcode),
    #[error("tried to read from the stack, but it was empty")]
    StackEmpty,
}
