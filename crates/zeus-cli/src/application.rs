use crate::error::ZeusError;
use crate::error::ZeusErrorType;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use zeus_vm::interpret;

pub struct Zeus {}

impl Zeus {
    pub fn init() -> Self {
        Zeus {}
    }

    fn run(&mut self, content: String) -> Result<(), ZeusError> {
        match interpret(content) {
            Ok(_) => log::info!("Interpreter says Bye"),
            Err(e) => log::error!("Intepreter error: {e}"),
        }

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
                    Err(error) => print!("Failed to parse command: {error}"),
                    _ => (),
                },
                Err(_) => break,
            }
        }

        Ok(())
    }
}
