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

    let mut vm = vm::VM {
        globals: &globals,
        stack: &mut stack,
        variables: &mut variables,
        frame: CallFrame::new(&function, 0, 0),
        previous_frames: Vec::new(),
    };

    vm.interpret_loop()?;

    Ok(())
}
