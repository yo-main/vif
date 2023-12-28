mod error;
mod parser;
use parser::Parser;
use zeus_objects::ast::Stmt;
use zeus_scanner::Scanner;

pub fn build_ast(content: String) -> Vec<Stmt> {
    let scanner = Scanner::new(content.as_str());
    let mut parser = Parser::new(scanner);
    parser.build();
    parser.get_ast()
}
