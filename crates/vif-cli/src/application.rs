use crate::error::VifError;
use crate::error::VifErrorType;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use vif_vm::interpret;

pub struct Vif {}

impl Vif {
    pub fn init() -> Self {
        Vif {}
    }

    fn run(&mut self, content: String) -> Result<(), VifError> {
        match interpret(content) {
            Ok(_) => log::info!("Interpreter says Bye"),
            Err(e) => println!("Intepreter error: {e}"),
        }

        Ok(())
    }

    pub fn run_file(&mut self, path: PathBuf) -> Result<(), VifError> {
        match fs::read_to_string(&path) {
            Ok(content) => self.run(content)?,
            _ => {
                return Err(VifError::new(
                    format!("Could not read file {}", path.to_str().unwrap()),
                    VifErrorType::FileNotFound,
                ))
            }
        };

        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), VifError> {
        loop {
            let mut line = String::new();
            print!(">>> ");
            io::stdout().flush().unwrap();
            match io::stdin().read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => match self.run(line) {
                    Err(error) => print!("Failed to parse command: {error}"),
                    _ => (),
                },
                Err(_) => break,
            }
        }

        Ok(())
    }
}
