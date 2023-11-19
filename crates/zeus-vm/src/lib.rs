#![feature(iter_advance_by)]

mod error;
pub mod value;
mod vm;

use std::collections::HashMap;

use error::InterpreterError;
use zeus_compiler::compile;

pub fn interpret(content: String) -> Result<(), InterpreterError> {
    let chunk = match compile(content) {
        Ok(c) => c,
        Err(e) => return Err(InterpreterError::CompileError(format!("{}", e))),
    };

    let mut stack = Vec::new();
    let mut variables = HashMap::new();

    let mut vm = vm::VM::new(&chunk, &mut stack, &mut variables);
    vm.interpret_loop()?;

    Ok(())
}
