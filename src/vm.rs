use crate::{
    chunk::Chunk,
    compiler::compile,
    opcode::{CouldNotDecodeOpcode, Opcode},
    value::Value,
};

#[derive(Debug)]
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

    pub fn new() -> Self {
        let mut slf = Self {
            chunk: Default::default(),
            ip: Default::default(),
            stack: Default::default(),
        };
        slf.init();
        slf
    }

    pub fn interpret(&mut self, source: &str) -> Result<(), Error> {
        let chunk = Chunk::default();
        self.chunk = chunk;
        self.ip = 0;
        compile(source, &mut self.chunk)?;
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
                    let val = self.pop()?;
                    val.print();
                    println!();
                    return Ok(());
                }
                Ok(Opcode::Negate) => {
                    let x = self.stack.pop().unwrap();
                    let x = Value::from(-x.0);
                    self.stack.push(x);
                }
                Ok(Opcode::Constant) => {
                    let constant = self.read_constant().clone();
                    self.stack.push(constant);
                }
                Ok(Opcode::Add) => {
                    let (a, b) = self.pop_two()?;
                    self.do_then_push(a, b, std::ops::Add::add)
                }
                Ok(Opcode::Sub) => {
                    let (a, b) = self.pop_two()?;
                    self.do_then_push(a, b, std::ops::Sub::sub)
                }
                Ok(Opcode::Mul) => {
                    let (a, b) = self.pop_two()?;
                    self.do_then_push(a, b, std::ops::Mul::mul)
                }
                Ok(Opcode::Div) => {
                    let (a, b) = self.pop_two()?;
                    self.do_then_push(a, b, std::ops::Div::div)
                }
                Err(e) => return Err(RuntimeErr::from(e).into()),
            }
        }
    }

    fn do_then_push<Op>(&mut self, a: Value, b: Value, op: Op)
    where
        Op: Fn(Value, Value) -> Value,
    {
        self.stack.push(op(a, b));
    }

    fn pop(&mut self) -> Result<Value, CompileErr> {
        let val = self.stack.pop().ok_or(CompileErr::StackEmpty)?;
        Ok(val.clone())
    }

    fn pop_two(&mut self) -> Result<(Value, Value), CompileErr> {
        let b = self.pop()?;
        let a = self.pop()?;
        Ok((a, b))
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
    #[error("Compile error: {0}")]
    Compile(#[from] CompileErr),
    #[error("Runtime error: {0}")]
    Runtime(#[from] RuntimeErr),
}

#[derive(Debug, thiserror::Error)]
pub enum CompileErr {
    #[error("tried to read from the stack, but it was empty")]
    StackEmpty,
    #[error("unexpected or invalid token in your source code at line {line}")]
    BadToken { line: usize },
    #[error("error at line {line}: {msg}")]
    Other { line: usize, msg: &'static str },
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeErr {
    #[error("{0}")]
    CouldNotDecodeOpcode(#[from] CouldNotDecodeOpcode),
}
