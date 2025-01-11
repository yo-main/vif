use vif_objects::ast::Function;
mod callable;
mod error;
mod mutability;
mod references;
mod typer;

pub fn run_typing_checks(function: &mut Function) -> Result<(), error::TypingError> {
    let mut references = references::References::new();
    // first pass
    let bottom_up_typer = typer::BottomUpTyper::new();
    bottom_up_typer.run(function, &mut references)?;

    // second pass, with functions parameters typed hopefully
    bottom_up_typer.run(function, &mut references)?;

    mutability::check_mutability(function)
}
