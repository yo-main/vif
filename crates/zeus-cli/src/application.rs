use crate::error::ZeusError;
use crate::error::ZeusErrorType;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use zeus_vm::vm::VM;
// use zeus_vm::vm::VM;

pub struct Zeus<'c> {
    vm: VM<'c>,
}

impl<'c> Zeus<'c> {
    pub fn init() -> Self {
        Zeus { vm: VM::new() }
    }

    fn run(&mut self, content: String) -> Result<(), ZeusError> {
        self.vm.interpret(content);

        // let mut tokenizer = Tokenizer::new(content.as_str());
        // tokenizer.scan_tokens();
        // let mut parser = Parser::new(tokenizer.tokens);
        // let is_ok = parser.parse();
        // if is_ok {
        //     self.interpreter.interpret(parser.statements);
        // } else {
        //     println!("errors: {:?}", parser.errors);
        // }

        Ok(())
    }

    pub fn run_file(&mut self, path: PathBuf) -> Result<(), ZeusError> {
        match fs::read_to_string(&path) {
            Ok(content) => self.run(content)?,
            _ => {
                return Err(ZeusError::new(
                    format!("Could not read file {}", path.to_str().unwrap()),
                    ZeusErrorType::FileNotFound,
                ))
            }
        };

        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), ZeusError> {
        loop {
            let mut line = String::new();
            print!(">>> ");
            io::stdout().flush().unwrap();
            match io::stdin().read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => match self.run(line) {
                    Err(error) => print!("Failed to parse command"),
                    _ => (),
                },
                Err(_) => break,
            }
        }

        Ok(())
    }
}
