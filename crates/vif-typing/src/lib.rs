use vif_objects::ast::Function;
mod callable;
mod error;
mod mutability;
mod references;
pub mod type_merger;
mod typer;

pub fn run_typing_checks(function: &mut Function) -> Result<(), error::TypingError> {
    let mut references = references::References::new();
    // first pass
    typer::BottomUpTyper::new(type_merger::SoftTypeMerger {}).run(function, &mut references)?;

    // second pass, with functions parameters typed hopefully
    typer::BottomUpTyper::new(type_merger::HardTypeMerger {}).run(function, &mut references)?;

    mutability::check_mutability(function)
}
