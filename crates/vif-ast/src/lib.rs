mod debug;
mod error;
mod objects;
mod old_parser;
mod parser;

pub use debug::print_ast_tree;
pub use error::AstError;
use parser::Parser;
use vif_loader::log;
use vif_scanner::Scanner;

pub use crate::objects::Entrypoint;

pub fn build_ast(content: &str) -> Result<Entrypoint, Vec<AstError>> {
    let mut scanner = Scanner::new(content);

    match Parser::build(&mut scanner) {
        Err(e) => Err(e),
        Ok(ast) => {
            log::debug!("########### AST ##########");
            for token in ast.body.iter() {
                log::debug!("{:?}", token);
            }
            log::debug!("########### AST ##########");
            Ok(ast)
        }
    }
}
