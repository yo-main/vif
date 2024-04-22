pub use error::TypingError;
use vif_error::VifError;
use vif_objects::ast::Function;
mod callable;
mod error;
mod mutability;
mod references;
mod typer;

pub fn run_typing_checks(function: &mut Function) -> Result<(), VifError> {
    mutability::check_mutability(function)
}
