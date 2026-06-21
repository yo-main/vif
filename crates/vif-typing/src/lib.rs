mod callable;
mod error;
mod mutability;
mod objects;
mod references;
pub mod type_merger;
mod typer;
use vif_ast as ast;
pub use vif_ast::Span;

pub use crate::objects::Assert;
pub use crate::objects::Assign;
pub use crate::objects::Binary;
pub use crate::objects::Call;
pub use crate::objects::Callable;
pub use crate::objects::CallableParameter;
pub use crate::objects::Condition;
pub use crate::objects::Entrypoint;
pub use crate::objects::Expr;
pub use crate::objects::ExprBody;
pub use crate::objects::Function;
pub use crate::objects::Logical;
pub use crate::objects::LogicalOperator;
pub use crate::objects::LoopKeyword;
pub use crate::objects::Operator;
pub use crate::objects::Return;
pub use crate::objects::Stmt;
pub use crate::objects::Type;
pub use crate::objects::Unary;
pub use crate::objects::UnaryOperator;
pub use crate::objects::Value;
pub use crate::objects::Variable;
pub use crate::objects::While;

pub fn run_typing_checks(entrypoint: &ast::Entrypoint) -> Result<Entrypoint, error::TypingError> {
    // first pass
    Ok(typer::BottomUpTyper::run(
        entrypoint,
        type_merger::SoftTypeMerger {},
    )?)

    // second pass, with functions parameters typed hopefully
    // typer::BottomUpTyper::run(entrypoint, type_merger::HardTypeMerger {})?;

    // Ok(())
    // mutability::check_mutability(function)
}
