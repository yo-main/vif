mod callable;
mod error;
mod mutability;
mod objects;
mod references;
pub mod type_merger;
mod typer;
use vif_ast::Entrypoint;

pub use crate::objects::Type;

pub fn run_typing_checks(entrypoint: &mut Entrypoint) -> Result<(), error::TypingError> {
    // first pass
    typer::BottomUpTyper::run(entrypoint, type_merger::SoftTypeMerger {})?;

    // second pass, with functions parameters typed hopefully
    typer::BottomUpTyper::run(entrypoint, type_merger::HardTypeMerger {})?;

    Ok(())
    // mutability::check_mutability(function)
}
