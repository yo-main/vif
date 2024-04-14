#![feature(iter_advance_by)]

mod callframe;
mod error;
pub mod vm;

pub use callframe::CallFrame;
pub use error::InterpreterError;
use vif_objects::function::Function;
use vif_objects::global_store::GlobalStore;
use vif_objects::stack::Stack;
use vif_objects::variable_storage::VariableStore;

pub fn interpret(function: Function, globals: GlobalStore) -> Result<(), InterpreterError> {
    let mut stack = Stack::new();
    let mut variables = VariableStore::new();

    let mut vm = vm::VM::new(
        &mut stack,
        &mut variables,
        &globals,
        CallFrame::new(&function, 0, 0),
    );

    vm.interpret_loop()?;

    Ok(())
}
