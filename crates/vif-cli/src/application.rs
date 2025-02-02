use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use vif_ast::build_ast;
use vif_ast::print_ast_tree;
use vif_llvm::compile_and_execute;
use vif_llvm::get_llvm_ir;
use vif_loader::log;
use vif_loader::CONFIG;
use vif_objects::ast::Function;
use vif_typing::run_typing_checks;

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

        let compiled_code = get_llvm_ir(&ast).unwrap();

        if CONFIG.assembly {
            println!("{}", compiled_code);
            fs::write("here.ll", compiled_code).unwrap();
            return;
        }

        compile_and_execute(&ast).unwrap();
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
