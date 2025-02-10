use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use vif_ast::build_ast;
use vif_ast::print_ast_tree;
use vif_llvm::compile_and_build_binary;
use vif_llvm::compile_and_execute;
use vif_llvm::execute_llvm_from_stdin;
use vif_llvm::get_llvm_ir;
use vif_loader::Action;
use vif_loader::Print;
use vif_loader::CONFIG;
use vif_objects::ast::Function;
use vif_typing::run_typing_checks;

pub struct Vif {}

impl Vif {
    pub fn init() -> Self {
        Vif {}
    }

    pub fn run(&mut self) -> Result<(), String> {
        match &CONFIG.action {
            Action::Execute(path) => self.execute_file(path)?,
            Action::Build(path) => self.build_binary(path)?,
            Action::ExecuteFromStdin => execute_llvm_from_stdin().map_err(|e| format!("{e}"))?,
            Action::Print(print_action) => match print_action {
                Print::Assembly(path) => {
                    let llvm_ir = self.get_llvm_ir(path)?;
                    print!("{}", llvm_ir);
                }
                Print::Ast(path) => self.get_ast(path).and_then(|ast| {
                    print_ast_tree(&ast);
                    Ok(())
                })?,
            },
        };
        Ok(())
    }

    fn get_llvm_ir(&self, path: &PathBuf) -> Result<String, String> {
        self.get_ast(&path)
            .and_then(|ast| get_llvm_ir(&ast).map_err(|e| format!("{e}")))
    }

    fn get_ast(&self, path: &PathBuf) -> Result<Function, String> {
        self.read_file(&path)
            .and_then(|content| self.build_ast(content.as_str()))
    }

    fn execute_file(&self, path: &PathBuf) -> Result<(), String> {
        self.get_ast(&path)
            .and_then(|ast| compile_and_execute(&ast).map_err(|e| format!("{e}")))
    }

    fn build_binary(&self, path: &PathBuf) -> Result<(), String> {
        self.get_ast(&path)
            .and_then(|ast| compile_and_build_binary(&ast).map_err(|e| format!("{e}")))
    }

    fn read_file(&self, path: &PathBuf) -> Result<String, String> {
        fs::read_to_string(&path).or(Err(format!(
            "Could not read file {}",
            path.to_string_lossy()
        )))
    }

    fn run_prompt(&mut self) {
        loop {
            let mut line = String::new();
            print!(">>> ");
            io::stdout().flush().unwrap();
            match io::stdin().read_line(&mut line) {
                Ok(_) => break,
                // Ok(_) => self.exec(line.as_str()),
                Err(_) => break,
            }
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
