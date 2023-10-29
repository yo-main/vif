use crate::ast::AstVisitor;
use crate::errors::ZeusError;
use crate::interpreter;
use std::fs;
use std::io;
use std::io::Write;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::tokenizer::Tokenizer;
use std::path::PathBuf;

pub struct Zeus {
    interpreter: Interpreter,
}

impl Zeus {
    pub fn init() -> Self {
        Zeus {
            interpreter: Interpreter::new(),
        }
    }

    fn run(&mut self, content: String) -> Result<(), ZeusError> {
        let mut tokenizer = Tokenizer::new(content.as_str());
        tokenizer.scan_tokens();
        let mut parser = Parser::new(tokenizer.tokens);
        let is_ok = parser.parse();
        if is_ok {
            self.interpreter.interpret(parser.statements);
        } else {
            println!("errors: {:?}", parser.errors);
        }

        Ok(())
    }

    pub fn run_file(&mut self, path: PathBuf) -> Result<(), ZeusError> {
        match fs::read_to_string(path) {
            Ok(content) => self.run(content)?,
            _ => return Err(ZeusError::new("Couldn't open file")),
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
                    Err(error) => print!("Failed to parse command: {}", error.msg),
                    _ => (),
                },
                Err(_) => break,
            }
        }

        Ok(())
    }
}
