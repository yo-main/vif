#![feature(iter_advance_by)]

mod callframe;
mod error;
pub mod value;
mod vm;

use std::collections::HashMap;

use crate::callframe::CodeIterator;
use crate::vm::VM;
use callframe::CallFrame;
use error::InterpreterError;
use zeus_compiler::{compile, Application};

pub fn interpret(content: String) -> Result<(), InterpreterError> {
    let mut stack = Vec::new();
    let mut variables = HashMap::new();

    let function = match compile(content) {
        Ok(c) => c,
        Err(e) => return Err(InterpreterError::CompileError(format!("{}", e))),
    };

    let mut vm: VM<Application> = vm::VM {
        application: &function,
        stack: &mut stack,
        variables: &mut variables,
        call_frames: vec![CallFrame::new(&function, 0)],
    };

    vm.interpret_loop()?;

    Ok(())
}
