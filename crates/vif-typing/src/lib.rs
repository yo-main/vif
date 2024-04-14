pub use error::TypingError;
use vif_objects::ast::Function;
mod callable;
mod error;
mod mutability;
mod references;
mod typer;

pub fn run_typing_checks(function: Function) -> Result<Function, TypingError> {
    mutability::check_mutability(function)
}
