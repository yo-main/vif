mod error;
mod parser;
use parser::Parser;
use vif_objects::ast::Stmt;
use vif_scanner::Scanner;

pub fn build_ast(content: String) -> Option<Vec<Stmt>> {
    let scanner = Scanner::new(content.as_str());
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
    for token in ast.iter() {
        log::debug!("{:?}", token);
    }
    log::debug!("########### AST ##########");
    Some(ast)
}
