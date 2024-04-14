use crate::error::VifError;
use crate::error::VifErrorType;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use vif_ast::build_ast;
use vif_ast::print_ast_tree;
use vif_compiler::compile;
use vif_compiler::disassemble_application;
use vif_loader::log;
use vif_loader::CONFIG;
use vif_typing::run_typing_checks;
use vif_vm::interpret;

pub struct Vif {}

impl Vif {
    pub fn init() -> Self {
        Vif {}
    }

    pub fn run(&mut self) -> Result<(), VifError> {
        match &CONFIG.entrypoint {
            Some(path) => self.run_file(path),
            _ => self.run_prompt(),
        }
    }

    fn exec(&mut self, content: &str) {
        match compile(content) {
            Ok((function, globals)) => {
                if CONFIG.assembly {
                    let (function, globals) = compile(content).unwrap();
                    disassemble_application(&function, &globals);
                } else if CONFIG.ast {
                    match build_ast(content) {
                        Ok(ast) => match run_typing_checks(ast) {
                            Ok(typed_ast) => print_ast_tree(&typed_ast),
                            Err(error) => println!("Error: {error}"),
                        },
                        Err(errors) => {
                            for error in errors.iter() {
                                println!("Error: {error}")
                            }
                        }
                    }
                } else {
                    match interpret(function, globals) {
                        Ok(_) => log::info!("Interpreter says Bye"),
                        Err(e) => println!("Intepreter error: {e}"),
                    };
                }
            }
            Err(e) => println!("Compiler error! {e}"),
        };
    }

    fn run_file(&mut self, path: &PathBuf) -> Result<(), VifError> {
        match fs::read_to_string(&path) {
            Ok(content) => self.exec(content.as_str()),
            _ => {
                return Err(VifError::new(
                    format!("Could not read file {}", path.to_str().unwrap()),
                    VifErrorType::FileNotFound,
                ))
            }
        };

        Ok(())
    }

    fn run_prompt(&mut self) -> Result<(), VifError> {
        loop {
            let mut line = String::new();
            print!(">>> ");
            io::stdout().flush().unwrap();
            match io::stdin().read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => self.exec(line.as_str()),
                Err(_) => break,
            }
        }

        Ok(())
    }
}
