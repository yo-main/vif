use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use vif_ast::build_ast;
use vif_ast::print_ast_tree;
use vif_ast::AstError;
use vif_compiler::compile;
use vif_compiler::disassemble_application;
use vif_loader::log;
use vif_loader::CONFIG;
use vif_objects::ast::Function;
use vif_typing::run_typing_checks;
use vif_vm::interpret;

pub struct Vif {}

impl Vif {
    pub fn init() -> Self {
        Vif {}
    }

    pub fn run(&mut self) {
        match &CONFIG.entrypoint {
            Some(path) => self.run_file(path),
            _ => self.run_prompt(),
        };
    }

    fn run_file(&mut self, path: &PathBuf) {
        match fs::read_to_string(&path) {
            Ok(content) => self.exec(content.as_str()),
            _ => println!(
                "{}",
                format!("Could not read file {}", path.to_str().unwrap())
            ),
        };
    }

    fn run_prompt(&mut self) {
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
    }

    fn exec(&mut self, content: &str) {
        let ast = match self.build_ast(content) {
            Ok(ast) => ast,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        if CONFIG.ast {
            print_ast_tree(&ast);
            return;
        }

        let (function, globals) = match compile(&ast) {
            Err(e) => {
                println!("Compiler error! {e}");
                return;
            }
            Ok(r) => r,
        };

        if CONFIG.assembly {
            disassemble_application(&function, &globals);
        } else {
            match interpret(function, globals) {
                Ok(_) => log::info!("Interpreter says Bye"),
                Err(e) => println!("Intepreter error: {e}"),
            };
        }
    }

    fn build_ast(&self, content: &str) -> Result<Function, String> {
        let mut ast = match build_ast(content) {
            Ok(ast) => ast,
            Err(errors) => return Err(errors[0].format(content)),
        };

        match run_typing_checks(&mut ast) {
            Err(err) => Err(err.format(content)),
            _ => Ok(ast),
        }
    }
}
