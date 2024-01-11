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
}

impl Vm {
    pub fn interpret(&mut self, chunk: Chunk) -> Result<(), Error> {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    fn run(&mut self) -> Result<(), Error> {
        loop {
            if cfg!(feature = "trace") {
                self.chunk.disassemble_instruction(self.ip);
            }
            match Opcode::try_from(self.chunk.code[self.ip]) {
                Ok(Opcode::Return) => return Ok(()),
                Ok(Opcode::Constant) => {
                    let constant = self.read_constant();
                    constant.print();
                    println!();
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
}
