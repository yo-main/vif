#![feature(iter_advance_by)]

mod callframe;
mod error;
pub mod vm;

use std::collections::HashMap;

pub use callframe::CallFrame;
pub use error::InterpreterError;
use zeus_compiler::compile;
use zeus_objects::stack::Stack;

pub fn interpret(content: String) -> Result<(), InterpreterError> {
    let mut stack = Stack::new();
    let mut variables = HashMap::new();

    let (function, globals) = match compile(content) {
        Ok(c) => c,
        Err(e) => return Err(InterpreterError::CompileError(format!("{}", e))),
    };

    let mut vm = vm::VM::new(
        &mut stack,
        &mut variables,
        &globals,
        CallFrame::new(&function, 0, 0),
    );

    vm.interpret_loop()?;

    Ok(())
}
