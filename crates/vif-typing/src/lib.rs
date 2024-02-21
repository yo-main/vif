use vif_objects::ast::Function;
mod mutability;

pub fn run_typing_checks(mut function: Function) -> Function {
    mutability::check_function(&mut function);
    function
}
