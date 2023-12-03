#![feature(iter_advance_by)]

mod callframe;
mod error;
mod vm;

use std::collections::HashMap;

use callframe::CallFrame;
use error::InterpreterError;
use zeus_compiler::compile;

pub fn interpret(content: String) -> Result<(), InterpreterError> {
    let mut stack = Vec::new();
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
