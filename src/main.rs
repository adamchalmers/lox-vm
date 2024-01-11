use std::borrow::Cow;

mod tokenizer;

fn main() {
    let mut chunk = Chunk::default();
    let i = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant(i), 123);
    chunk.write(OpCode::Return, 123);
    chunk.disassemble("test chunk");
}

#[repr(u8)]
enum OpCode {
    Constant(usize),
    Return,
}

#[derive(Default)]
struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn write(&mut self, instr: OpCode, line: usize) {
        self.code.push(instr);
        self.lines.push(line);
    }

    fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {name} ==");
        let mut previous_line: Option<usize> = None;
        for (i, instr) in self.code.iter().enumerate() {
            let this_line = self.lines[i];
            let line_header = if let Some(prev) = previous_line {
                if prev == this_line {
                    Cow::Borrowed("|")
                } else {
                    this_line.to_string().into()
                }
            } else {
                this_line.to_string().into()
            };
            self.disassemble_instruction(i, instr, &line_header);
            previous_line = Some(this_line);
        }
    }

    fn disassemble_instruction(&self, i: usize, instr: &OpCode, line_header: &str) {
        print!("{i:04} {line_header:>4} ");
        match instr {
            OpCode::Return => println!("OpCode::Return"),
            OpCode::Constant(c) => {
                println!("OpCode::Constant {}", self.constants[*c])
            }
        }
    }
}

type Value = f64;
