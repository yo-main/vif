use error::TypingError;
use vif_objects::ast::Function;
mod error;
mod mutability;

pub fn run_typing_checks(function: Function) -> Result<Function, TypingError> {
    mutability::check_mutability(function)
}
