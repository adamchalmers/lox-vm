mod chunk;
mod compiler;
mod opcode;
mod tokenizer;
mod value;
mod vm;

use std::process::exit;

use vm::Vm;

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let res = if let Some(filepath) = args.next() {
        run_file(filepath)
    } else {
        repl()
    };
    match res {
        Ok(()) => {}
        Err(Error::Io(err)) => {
            eprintln!("{err}");
            exit(1);
        }
        Err(Error::Vm(vm::Error::Runtime(err))) => {
            eprintln!("{err}");
            exit(2);
        }
        Err(Error::Vm(vm::Error::Compile(err))) => {
            eprintln!("{err}");
            exit(3);
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("The lox VM returned an error: {0}")]
    Vm(#[from] vm::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

fn repl() -> Result<(), Error> {
    let mut lines = std::io::stdin().lines();
    let mut vm = Vm::new();
    print!("> ");
    while let Some(line) = lines.next() {
        vm.interpret(&line?)?;
        print!("> ");
    }
    Ok(())
}

fn run_file(filepath: String) -> Result<(), Error> {
    let f = std::fs::read_to_string(filepath)?;
    let mut vm = Vm::new();
    for line in f.lines() {
        vm.interpret(&line)?;
    }
    Ok(())
}
