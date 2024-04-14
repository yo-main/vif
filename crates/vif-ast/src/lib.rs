mod debug;
mod error;
mod parser;
pub use debug::print_ast_tree;
use error::AstError;
use parser::Parser;
use vif_loader::log;
use vif_objects::ast::Function;
use vif_scanner::Scanner;

pub fn build_ast(content: &str) -> Result<Function, Vec<AstError>> {
    let scanner = Scanner::new(content);
    let mut parser = Parser::new(scanner);
    let success = parser.build();
    if !success {
        return Err(parser.get_errors());
    }

    let ast = parser.get_ast();
    log::debug!("########### AST ##########");
    for token in ast.body.iter() {
        log::debug!("{:?}", token);
    }
    log::debug!("########### AST ##########");
    Ok(ast)
}
