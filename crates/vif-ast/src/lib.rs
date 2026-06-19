mod debug;
mod error;
mod objects;
// mod old_parser;
mod parser;

pub use debug::print_ast_tree;
pub use error::AstError;
use parser::Parser;
use vif_loader::log;
use vif_scanner::Scanner;

pub use crate::objects::Assign;
pub use crate::objects::Binary;
pub use crate::objects::Condition;
pub use crate::objects::Entrypoint;
pub use crate::objects::Expr;
pub use crate::objects::ExprBody;
pub use crate::objects::Function;
pub use crate::objects::FunctionParameter;
pub use crate::objects::Logical;
pub use crate::objects::LogicalOperator;
pub use crate::objects::LoopKeyword;
pub use crate::objects::Operator;
pub use crate::objects::Return;
pub use crate::objects::Stmt;
pub use crate::objects::TypeAnnotation;
pub use crate::objects::Unary;
pub use crate::objects::UnaryOperator;
pub use crate::objects::Value;
pub use crate::objects::Variable;
pub use crate::objects::While;

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
