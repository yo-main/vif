mod debug;
mod error;
mod parser;
pub use debug::print_ast_tree;
use parser::Parser;
use vif_loader::log;
use vif_objects::ast::Function;
use vif_scanner::Scanner;

pub fn build_ast(content: &str) -> Option<Function> {
    let scanner = Scanner::new(content);
    let mut parser = Parser::new(scanner);
    let res = parser.build();
    if !res {
        for res in parser.get_errors() {
            println!("ERROR: {}", res);
        }
        return None;
    }

    let ast = parser.get_ast();
    log::debug!("########### AST ##########");
    for token in ast.body.iter() {
        log::debug!("{:?}", token);
    }
    log::debug!("########### AST ##########");
    Some(ast)
}
